use crate::{
    name::Name,
    r#type::{Type, TypeList},
    render::Render,
};
use anyhow::{anyhow, Result};
use indoc::indoc;
use itertools::Itertools;

/// Represents an ImGui function, not a method.
#[derive(Debug)]
pub struct Function {
    /// The full function name.
    name: Name,
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
        name: Name,
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
}

impl Render for Function {
    fn lua(&self, types: &Vec<Type>) -> String {
        format!(
            indoc!(
                r#"
        function {module}.{name}({args})
        {checks}
            -- call
            {ret}
        end
        "#
            ),
            // TODO: Make this configurable
            module = "gui",
            name = self.name.lua(),
            args = self
                .args
                .iter()
                .map(|arg| arg.name())
                .intersperse(", ")
                .collect::<String>(),
            checks = self
                .args
                .iter()
                .map(|arg| arg
                    .check_string(types)
                    .expect("Could not build argument check"))
                .intersperse("\n".to_string())
                .collect::<String>(),
            ret = self.ret.as_ref().map(|ret| "return ret").unwrap_or("")
        )
    }

    fn doc(&self, _types: &Vec<Type>) -> String {
        format!("")
    }

    /// Get the cdef definition of this function.
    fn cdef(&self, _types: &Vec<Type>) -> String {
        format!(
            "{} {}{};",
            self.ret.as_ref().unwrap_or(&"void".to_string()),
            self.name.imgui(),
            self.signature
        )
    }
}

/// Represent an ImGui function & method argument.
#[derive(Debug, Default)]
pub struct Arg {
    name: String,
    default_value: Option<String>,
    r#type: String,
    index: u8,
}

impl Arg {
    /// Add a new argument from the parsed data.
    pub fn from_parsed(
        name: String,
        default_value: Option<String>,
        r#type: String,
        index: u8,
    ) -> Self {
        Self {
            name,
            default_value,
            r#type,
            index,
        }
    }

    /// The name of the argument.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the type reference of the argument.
    pub fn r#type<'a>(&'a self, types: &'a Vec<Type>) -> Result<&'a Type> {
        types
            .find(&self.r#type)
            .map_err(|err| anyhow!("Could not get type of argument \"{}\": {}", self.name, err))
    }

    /// The Lua argument check.
    pub fn check_string(&self, types: &Vec<Type>) -> Result<String> {
        Ok(format!(
            "    {name} = arg_check({name}, \"{type}\", {index})",
            // Use 'name' or 'name or default_value'
            name = self.default_value.as_ref().map_or(
                self.name().to_string(),
                |default_value| format!("{} or {}", self.name(), default_value)
            ),
            r#type = self.r#type(types)?.lua_primitive_type()?,
            index = self.index
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::render::Render;

    #[test]
    fn cdef() -> anyhow::Result<()> {
        let arg1 = super::Arg::from_parsed("first".to_string(), None, "char*".to_string(), 0);
        let arg2 = super::Arg::from_parsed("second".to_string(), None, "int".to_string(), 1);

        let func = super::Function::from_parsed(
            "func".into(),
            vec![arg1, arg2],
            None,
            Some("const char[512]".to_string()),
            "(char*, int)".to_string(),
        );

        assert_eq!(func.cdef(&vec![]), "const char[512] func(char*, int);");

        Ok(())
    }

    #[test]
    fn lua() -> anyhow::Result<()> {
        let arg1 = super::Arg::from_parsed("first".to_string(), None, "char*".to_string(), 1);
        let arg2 = super::Arg::from_parsed("second".to_string(), None, "int".to_string(), 2);

        let func = super::Function::from_parsed(
            "func".into(),
            vec![arg1, arg2],
            None,
            Some("const char[512]".to_string()),
            "(char*, int)".to_string(),
        );

        assert_eq!(
            func.lua(&super::Type::default_list()),
            indoc::indoc!(
                r#"
                function gui.func(first, second)
                    first = arg_check(first, "string", 1)
                    second = arg_check(second, "number", 2)
                    -- call
                    return ret
                end
                "#
            )
        );

        Ok(())
    }
}
