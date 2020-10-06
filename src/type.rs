use crate::r#enum::Enum;
use crate::r#struct::Struct;

/// Represents any ImGui or C type.
#[derive(Debug)]
pub enum Type {
    Enum(Enum),
    Struct(Struct),
    C(String),
}
