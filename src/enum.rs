/// Represents an ImGui structure.
#[derive(Debug, Default)]
pub struct Enum {
    name: String,
    values: Vec<Value>,
    location: Option<(String, i64)>,
}

impl Enum {
    /// Add a new enum from the parsed data.
    pub fn from_parsed(name: String, values: Vec<Value>) -> Self {
        Self {
            name,
            values,
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

/// The value variant of an enum.
#[derive(Debug)]
pub struct Value {
    name: String,
    value: String,
    calculated_value: i64,
}

impl Value {
    /// Add a new enum field from the parsed data.
    pub fn from_parsed(name: String, value: String, calculated_value: i64) -> Self {
        Self {
            name,
            value,
            calculated_value,
        }
    }
}
