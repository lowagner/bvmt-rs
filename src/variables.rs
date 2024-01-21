use crate::dimensions::*;

/// A group of variables (field names + field values) that has some reflection
/// properties, i.e., the ability to return a list of all values.
pub trait Variables {
    // TODO: necessary? fn get(&self, field_name: &str) -> Variable;
    // TODO: necessary? fn set(&self, field_name: &str, new_value: Variable);
    fn list(&self) -> Vec<Variable>;
}

/// A variable that can be represented in shader code.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Variable {
    Vector2f(Metadata, Vector2f),
    Vector3f(Metadata, Vector3f),
    // TODO: Pixels (e.g., Texture)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Metadata {
    /// Location index for where this variable lives in the GPU buffer.
    Location(u16),
    /// Used if the variable is a built-in value.
    BuiltIn(BuiltIn),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BuiltIn {
    /// Should correspond to a Vector4f for clip coordinates.
    Position,
}
