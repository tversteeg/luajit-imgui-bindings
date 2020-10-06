use serde_json::{Number, Result, Value};
use std::collections::HashMap;

/// Corresponds to definitions.json & impl_definitions.json
#[derive(Debug, serde::Deserialize)]
pub struct Definitions(HashMap<String, Vec<Definition>>);

impl Definitions {
    pub fn from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Definition {
    args: String,
    #[serde(rename = "argsT")]
    args_t: Vec<ArgT>,
    #[serde(rename = "argsoriginal")]
    args_original: Option<String>,
    call_args: String,
    #[serde(rename = "cimguiname")]
    cimgui_name: String,
    defaults: HashMap<String, String>,
    #[serde(rename = "funcname")]
    func_name: Option<String>,
    location: Option<String>,
    #[serde(rename = "ov_cimguiname")]
    ov_cimgui_name: String,
    ret: Option<String>,
    signature: String,
    #[serde(rename = "stname")]
    struct_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ArgT {
    name: String,
    #[serde(rename = "type")]
    type_: String,
}

/// Corresponds to structs_and_enums.json
#[derive(Debug, serde::Deserialize)]
pub struct StructsAndEnums {
    enums: HashMap<String, Enum>,
    locations: HashMap<String, Location>,
    structs: HashMap<String, Struct>,
}

impl StructsAndEnums {
    pub fn from_str(json: &str) -> Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Enum(Vec<EnumValue>);

#[derive(Debug, serde::Deserialize)]
pub struct EnumValue {
    name: String,
    calc_value: Option<Number>,
    value: Value,
}

#[derive(Debug, serde::Deserialize)]
pub struct Location(String);

#[derive(Debug, serde::Deserialize)]
pub struct Struct(Vec<Field>);

#[derive(Debug, serde::Deserialize)]
pub struct Field {
    name: String,
    template_type: Option<String>,
    #[serde(rename = "type")]
    type_: String,
}

/// Corresponds to typedefs_dict.json
#[derive(Debug, serde::Deserialize)]
pub struct Typedefs(HashMap<String, String>);

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
