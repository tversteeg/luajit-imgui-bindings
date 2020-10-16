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
    /// Get a list of default C types.
    pub fn default_list() -> Vec<Self> {
        vec!["char*", "int", "float"]
            .into_iter()
            .map(|t| Self::C(t.to_string()))
            .collect()
    }

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

    /// The Lua primitive type.
    pub fn lua_primitive_type(&self) -> Result<String> {
        match self {
            // Struct are always a table.
            Self::Struct(_) => Ok("table".to_string()),
            // Enums are always a number.
            Self::Enum(_) => Ok("number".to_string()),
            Self::C(c_type) => match c_type.as_str() {
                "int" | "float" => Ok("number".to_string()),
                "bool" => Ok("boolean".to_string()),
                "const char*" | "char*" => Ok("string".to_string()),
                other => Err(anyhow!("Unrecognized C type \"{}\"", other)),
            },
        }
    }
}

/// The trait for type lists.
pub trait TypeList {
    type Output;

    /// Find a type by it's ImGui name.
    fn find(&self, imgui_type: &str) -> Result<&Self::Output>;
}

impl TypeList for Vec<Type> {
    type Output = Type;

    fn find(&self, imgui_type: &str) -> Result<&Type> {
        self.iter()
            .find(|r#type| r#type.is_same(imgui_type))
            .ok_or_else(|| {
                anyhow!(
                    "ImGui type \"{}\" is not registered in type list",
                    imgui_type
                )
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lua_type() -> anyhow::Result<()> {
        assert_eq!(
            super::Type::C("int".to_string()).lua_primitive_type()?,
            "number"
        );
        assert_eq!(
            super::Type::C("const char*".to_string()).lua_primitive_type()?,
            "string"
        );

        Ok(())
    }
}
