/// Represents an ImGui function, not a method.
#[derive(Debug)]
pub struct Function {
    /// The full function name.
    name: String,
    /// All the arguments of this function.
    args: Vec<Arg>,
    /// The return value.
    ret: Option<String>,
    /// Where the source file lives.
    location: Option<(String, i64)>,
    /// The C arguments signature.
    signature: String,
}

impl Function {
    /// Add a new function from the parsed data.
    pub fn from_parsed(
        name: String,
        args: Vec<Arg>,
        location: Option<(String, i64)>,
        ret: Option<String>,
        signature: String,
    ) -> Self {
        Self {
            name,
            args,
            location,
            ret,
            signature,
        }
    }

    /// Get the cdef definition of this function.
    pub fn cdef(&self) -> String {
        format!(
            "{} {}{};",
            self.ret.as_ref().unwrap_or(&String::new()),
            self.name,
            self.signature
        )
    }
}

/// Represent an ImGui function & method argument.
#[derive(Debug)]
pub struct Arg {
    name: String,
    default_value: Option<String>,
    r#type: String,
}

impl Arg {
    /// Add a new argument from the parsed data.
    pub fn from_parsed(name: String, default_value: Option<String>, r#type: String) -> Self {
        Self {
            name,
            default_value,
            r#type,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cdef() -> anyhow::Result<()> {
        let arg1 = super::Arg::from_parsed("first".to_string(), None, "char*".to_string());
        let arg2 = super::Arg::from_parsed("second".to_string(), None, "int".to_string());

        let func = super::Function::from_parsed(
            "func".to_string(),
            vec![arg1, arg2],
            None,
            Some("const char[512]".to_string()),
            "(char*, int)".to_string(),
        );

        assert_eq!(func.cdef(), "const char[512] func(char*, int);");

        Ok(())
    }
}
