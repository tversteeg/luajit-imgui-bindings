use anyhow::Result;

/// Represents an ImGui structure.
#[derive(Debug)]
pub struct Struct {
    name: String,
    fields: Vec<Field>,
}

impl Struct {
    /// Add a new struct from the parsed data.
    pub fn from_parsed(name: &str, fields: Vec<Field>) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            fields,
        })
    }
}

/// Represents an ImGui structure field.
#[derive(Debug)]
pub struct Field {
    name: String,
    template_type: Option<String>,
    type_: String,
}

impl Field {
    /// Add a new struct from the parsed data.
    pub fn from_parsed(name: &str, template_type: Option<&str>, type_: &str) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            template_type: template_type.map(|x| x.to_string()),
            type_: type_.to_string(),
        })
    }
}
