use serde_json::Result;
use std::{collections::HashMap, str::FromStr};

/// Corresponds to definitions.json & impl_definitions.json
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Definitions(pub HashMap<String, Vec<Definition>>);

impl Definitions {
    pub fn from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Definition {
    pub args: String,
    #[serde(rename = "argsT")]
    pub args_t: Vec<ArgT>,
    #[serde(rename = "argsoriginal")]
    pub args_original: Option<String>,
    pub call_args: String,
    #[serde(rename = "cimguiname")]
    pub cimgui_name: String,
    pub defaults: HashMap<String, String>,
    #[serde(rename = "funcname")]
    pub func_name: Option<String>,
    pub location: Option<Location>,
    #[serde(rename = "ov_cimguiname")]
    pub ov_cimgui_name: String,
    pub ret: Option<String>,
    pub signature: String,
    #[serde(rename = "stname")]
    pub struct_name: String,
    #[serde(default)]
    pub templated: bool,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArgT {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Corresponds to structs_and_enums.json
#[derive(Debug, Clone, serde::Deserialize)]
pub struct StructsAndEnums {
    pub enums: HashMap<String, Enum>,
    pub locations: HashMap<String, Location>,
    pub structs: HashMap<String, Struct>,
}

impl StructsAndEnums {
    pub fn from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Enum(pub Vec<EnumValue>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub calc_value: Option<i64>,
    pub value: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Location(String);

impl Location {
    pub fn line_number(&self) -> i64 {
        i64::from_str(self.0.split(":").last().expect("Could not find ':' symbol"))
            .expect("Invalid string")
    }

    pub fn filename(&self) -> &str {
        self.0.split(":").next().expect("Could not find ':' symbol")
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Struct(pub Vec<Field>);

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Field {
    pub name: String,
    pub template_type: Option<String>,
    pub r#type: String,
}

/// Corresponds to typedefs_dict.json
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Typedefs(pub HashMap<String, String>);

impl Typedefs {
    pub fn from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() -> anyhow::Result<()> {
        // Parse the definitions
        super::Definitions::from_str(include_str!("../cimgui/generator/output/definitions.json"))?;
        super::Definitions::from_str(include_str!(
            "../cimgui/generator/output/impl_definitions.json"
        ))?;

        // Parse the structs & enums
        super::StructsAndEnums::from_str(include_str!(
            "../cimgui/generator/output/structs_and_enums.json"
        ))?;

        // Parse the typedefs
        super::Typedefs::from_str(include_str!(
            "../cimgui/generator/output/typedefs_dict.json"
        ))?;

        Ok(())
    }
}
