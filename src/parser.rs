use crate::{
    function::{Arg, Function},
    json,
    r#enum::{Enum, Value},
    r#struct::{Field, Struct},
    r#type::Type,
    render::Render,
};
use anyhow::{anyhow, Result};
use indoc::indoc;
use itertools::Itertools;
use std::collections::HashMap;

/// How cimgui output files need to be parsed.
#[derive(Debug, Default)]
pub struct Parser {
    typedefs: Vec<json::Typedefs>,
    defs: Vec<json::Definitions>,
    structs: HashMap<String, json::Struct>,
    enums: HashMap<String, json::Enum>,
    locations: HashMap<String, json::Location>,
}

impl Parser {
    /// Construct a new parser object.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Parse type definitions defined in JSON.
    pub fn add_json_typedefs(&mut self, json: &str) -> Result<()> {
        let typedefs = json::Typedefs::from_str(json)?;

        self.typedefs.push(typedefs);

        Ok(())
    }

    /// Parse definitions defined in JSON.
    pub fn add_json_definitions(&mut self, json: &str) -> Result<()> {
        let defs = json::Definitions::from_str(json)?;

        self.defs.push(defs);

        Ok(())
    }

    /// Parse structs & enums defined in JSON.
    pub fn add_json_structs_and_enums(&mut self, json: &str) -> Result<()> {
        let structs_enums = json::StructsAndEnums::from_str(json)?;

        for (name, location) in structs_enums.locations.iter() {
            self.locations.insert(name.to_string(), location.clone());
        }
        for (name, r#enum) in structs_enums.enums.iter() {
            self.enums.insert(name.to_string(), r#enum.clone());
        }
        for (name, r#struct) in structs_enums.structs.iter() {
            self.structs.insert(name.to_string(), r#struct.clone());
        }

        Ok(())
    }

    /// Convert everything to usable data.
    pub fn parse(&self) -> Result<Data> {
        let mut types = vec![];

        types.append(
            &mut self
                .structs
                .iter()
                .map(|(name, r#struct)| {
                    let fields = r#struct
                        .0
                        .iter()
                        .map(|field| {
                            Field::from_parsed(
                                (&field.name).into(),
                                field.template_type.clone(),
                                field.r#type.clone(),
                            )
                        })
                        .collect();

                    Type::Struct(Struct::from_parsed(name.into(), fields))
                })
                .collect::<Vec<_>>(),
        );
        types.append(
            &mut self
                .enums
                .iter()
                .map(|(name, r#enum)| {
                    let values = r#enum
                        .0
                        .iter()
                        .map(|value| {
                            Value::from_parsed(
                                value.name.clone(),
                                value.value.clone(),
                                value.calc_value.unwrap_or(0),
                            )
                        })
                        .collect();

                    Type::Enum(Enum::from_parsed(name.clone(), values))
                })
                .collect::<Vec<_>>(),
        );

        // Add the location to each type when applicable
        self.locations.iter().for_each(|(name, location)| {
            types.iter_mut().for_each(|r#type| {
                if r#type.is_same(name) {
                    r#type.add_location(&location.filename(), location.line_number());
                }
            });
        });

        // Get an iterator of all methods & functions
        let (methods, functions): (Vec<_>, Vec<_>) = self
            .defs
            .iter()
            .flat_map(|defs| defs.0.iter())
            .flat_map(|(name, defs)| {
                defs.iter()
                    // Only parse non-templated functions
                    .filter(|def| !def.templated)
                    .map(move |def| {
                        let args = def
                            .args_t
                            .iter()
                            .map(|arg| {
                                Arg::from_parsed(
                                    arg.name.clone(),
                                    def.defaults.get(&arg.name).map(|s| s.clone()),
                                    arg.r#type.clone(),
                                )
                            })
                            .collect();

                        (
                            // Convert the string to an option
                            match def.struct_name.as_str() {
                                "" => None,
                                value => Some(value.clone()),
                            },
                            Function::from_parsed(
                                // Use the func name and if that's missing the cimgui name
                                def.func_name.as_ref().unwrap_or(&name.to_string()).into(),
                                args,
                                // Parse the location
                                def.location
                                    .as_ref()
                                    .map(|loc| (loc.filename().to_string(), loc.line_number())),
                                def.ret.clone(),
                                def.signature.clone(),
                            ),
                        )
                    })
            })
            // Split into functions and methods
            .partition(|(struct_name, _)| struct_name.is_some());

        // Add the methods to the structs
        for (struct_name, method) in methods.into_iter() {
            // Safe to unwrap because all instances that don't have a value are already partitioned
            let struct_name = struct_name.unwrap();
            match types.iter_mut().find(|r#type| r#type.is_same(struct_name)) {
                Some(r#type) => r#type.add_method(method)?,
                None => return Err(anyhow!("No struct \"{}\" for method found", struct_name)),
            }
        }

        // Extract just the functions
        let functions = functions.into_iter().map(|(_, func)| func).collect();

        Ok(Data { functions, types })
    }
}

/// Processed cimgui data.
#[derive(Debug, Default)]
pub struct Data {
    types: Vec<Type>,
    functions: Vec<Function>,
}

impl Data {
    /// Render the result as a Lua file.
    pub fn lua(&self) -> String {
        format!(
            indoc! {r#"
            local gui = {{}}
            gui.__index = gui

            {args_lua}
            --[[ Functions ]]

            {functions}
            return gui
        "#},
            args_lua = include_str!("lua/args.lua"),
            functions = self
                .functions
                .iter()
                .map(|func| func.lua(&self.types))
                .intersperse("\n".to_string())
                .collect::<String>()
        )
    }

    /// Render the result as cdefs.
    pub fn cdefs(&self) -> String {
        format!(
            indoc! {r#"
            return [[
            {cdefs}
            ]]
        "#},
            cdefs = self
                .functions
                .iter()
                .map(|func| func.cdef(&self.types))
                .intersperse("\n".to_string())
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn json() -> anyhow::Result<()> {
        let mut parser = super::Parser::new();

        // Parse typedefs
        parser.add_json_typedefs(include_str!(
            "../cimgui/generator/output/typedefs_dict.json"
        ))?;

        // Parse definitions
        parser.add_json_definitions(include_str!("../cimgui/generator/output/definitions.json"))?;
        parser.add_json_definitions(include_str!(
            "../cimgui/generator/output/impl_definitions.json"
        ))?;

        // Parse structs & enums
        parser.add_json_structs_and_enums(include_str!(
            "../cimgui/generator/output/structs_and_enums.json"
        ))?;

        // Parse everything
        let data = parser.parse()?;

        Ok(())
    }
}
