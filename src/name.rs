use inflector::Inflector;

/// Represents a ImGui name string that can be properly converted to the requested case.
#[derive(Debug, Default, Clone)]
pub struct Name(String);

impl Name {
    /// The ImGui representation.
    pub fn imgui(&self) -> &str {
        &self.0
    }

    /// The converted Lua representation.
    pub fn lua(&self) -> String {
        self.0.to_snake_case()
    }
}

impl From<String> for Name {
    fn from(name: String) -> Self {
        Self(name)
    }
}

impl From<&String> for Name {
    fn from(name: &String) -> Self {
        Self(name.clone())
    }
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        Self(name.to_string())
    }
}
