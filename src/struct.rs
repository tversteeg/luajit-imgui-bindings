use anyhow::Result;

/// Represents an ImGui structure.
#[derive(Debug, Default)]
pub struct Struct {
    name: String,
    fields: Vec<Field>,
    location: Option<(String, i64)>,
}

impl Struct {
    /// Add a new struct from the parsed data.
    pub fn from_parsed(name: String, fields: Vec<Field>) -> Self {
        Self {
            name,
            fields,
            ..Default::default()
        }
    }

    /// Add location information.
    pub fn add_location(&mut self, filename: &str, line_number: i64) {
        self.location = Some((filename.to_string(), line_number));
    }

    /// Check if this type is the same as the string.
    pub fn is_same(&self, r#type: &str) -> bool {
        self.name == r#type
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
    /// Add a new struct field from the parsed data.
    pub fn from_parsed(name: String, template_type: Option<String>, type_: String) -> Self {
        Self {
            name,
            template_type,
            type_,
        }
    }
}
