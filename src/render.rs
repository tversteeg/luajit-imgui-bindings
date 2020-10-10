use crate::r#type::Type;

/// Implement this for structures that output Lua.
pub trait Render {
    /// Output Lua.
    fn lua(&self, types: &Vec<Type>) -> String;
    /// Output Lua documentation.
    fn doc(&self, types: &Vec<Type>) -> String;
    /// Output LuaJIT cdef.
    fn cdef(&self, types: &Vec<Type>) -> String;
}
