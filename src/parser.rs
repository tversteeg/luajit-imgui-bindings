use crate::{json, r#struct::Struct};
use anyhow::Result;

/// How cimgui output files need to be parsed.
#[derive(Debug, Default)]
pub struct Parser {
    typedefs: Vec<json::Typedefs>,
    defs: Vec<json::Definitions>,
    structs_enums: Vec<json::StructsAndEnums>,
}

impl Parser {
    /// Construct a new parser object.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Parse type definitions defined in JSON.
    pub fn parse_json_typedefs(&mut self, json: &str) -> Result<()> {
        let typedefs = json::Typedefs::from_str(json)?;

        self.typedefs.push(typedefs);

        Ok(())
    }

    /// Parse definitions defined in JSON.
    pub fn parse_json_definitions(&mut self, json: &str) -> Result<()> {
        let defs = json::Definitions::from_str(json)?;

        self.defs.push(defs);

        Ok(())
    }

    /// Parse structs & enums defined in JSON.
    pub fn parse_json_structs_and_enums(&mut self, json: &str) -> Result<()> {
        let structs_enums = json::StructsAndEnums::from_str(json)?;

        self.structs_enums.push(structs_enums);

        Ok(())
    }

    /// Convert everything to usable data.
    pub fn parse(&self) -> Result<()> {
        Ok(())
    }
}

/// Processed cimgui data.
#[derive(Debug, Default)]
pub struct Data {
    structs: Vec<Struct>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn json() -> anyhow::Result<()> {
        let mut parser = super::Parser::new();

        // Parse typedefs
        parser.parse_json_typedefs(include_str!(
            "../cimgui/generator/output/typedefs_dict.json"
        ))?;

        // Parse definitions
        parser
            .parse_json_definitions(include_str!("../cimgui/generator/output/definitions.json"))?;
        parser.parse_json_definitions(include_str!(
            "../cimgui/generator/output/impl_definitions.json"
        ))?;

        // Parse structs & enums
        parser.parse_json_structs_and_enums(include_str!(
            "../cimgui/generator/output/structs_and_enums.json"
        ))?;

        // Parse everything
        parser.parse()?;

        Ok(())
    }
}
