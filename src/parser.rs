use crate::{
    json,
    r#enum::{Enum, Value},
    r#struct::{Field, Struct},
    r#type::Type,
};
use anyhow::Result;
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
                                field.name.clone(),
                                field.template_type.clone(),
                                field.r#type.clone(),
                            )
                        })
                        .collect();

                    Type::Struct(Struct::from_parsed(name.clone(), fields))
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
            types.iter_mut().for_each(|mut r#type| {
                if r#type.is_same(name) {
                    r#type.add_location(&location.filename(), location.line_number());
                }
            });
        });

        Ok(Data { types })
    }
}

/// Processed cimgui data.
#[derive(Debug, Default)]
pub struct Data {
    types: Vec<Type>,
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

        dbg!(data);

        assert!(false);

        Ok(())
    }
}
