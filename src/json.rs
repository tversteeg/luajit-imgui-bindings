use serde_json::{Number, Result, Value};
use std::collections::HashMap;

#[derive(serde::Deserialize)]
struct StructsAndEnums {
    enums: HashMap<String, Enum>,
    locations: HashMap<String, Location>,
    structs: HashMap<String, Struct>,
}

#[derive(serde::Deserialize)]
struct Enum(Vec<EnumValue>);

#[derive(serde::Deserialize)]
struct EnumValue {
    name: String,
    calc_value: Option<Number>,
    value: Value,
}

#[derive(serde::Deserialize)]
struct Location(String);

#[derive(serde::Deserialize)]
struct Struct(Vec<StructVariant>);

#[derive(serde::Deserialize)]
struct StructVariant {
    name: String,
    template_type: Option<String>,
    #[serde(rename = "type")]
    type_: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse() {
        let _: super::StructsAndEnums = serde_json::from_str(include_str!(
            "../cimgui/generator/output/structs_and_enums.json"
        ))
        .unwrap();
    }
}
