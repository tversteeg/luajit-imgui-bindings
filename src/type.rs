use crate::{function::Function, r#enum::Enum, r#struct::Struct};
use anyhow::{anyhow, Result};

/// Represents any ImGui or C type.
#[derive(Debug)]
pub enum Type {
    Enum(Enum),
    Struct(Struct),
    C(String),
}

impl Type {
    /// Check if this type is the same as the string.
    pub fn is_same(&self, r#type: &str) -> bool {
        match self {
            Self::Enum(r#enum) => r#enum.is_same(r#type),
            Self::Struct(r#struct) => r#struct.is_same(r#type),
            Self::C(c) => c == r#type,
        }
    }

    /// Add location information to the type.
    pub fn add_location(&mut self, filename: &str, line_number: i64) {
        match self {
            Self::Enum(r#enum) => r#enum.add_location(filename, line_number),
            Self::Struct(r#struct) => r#struct.add_location(filename, line_number),
            _ => (),
        }
    }

    /// Add a method to the type (only applies to structs).
    pub fn add_method(&mut self, method: Function) -> Result<()> {
        match self {
            Self::Struct(r#struct) => Ok(r#struct.add_method(method)),
            Self::Enum(_) => Err(anyhow!("Cannot add method to enum")),
            Self::C(_) => Err(anyhow!("Cannot add method to C type")),
        }
    }
}
