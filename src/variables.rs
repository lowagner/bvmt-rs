use std::fmt;

use crate::bindings::*;
use crate::dimensions::*;
use crate::pixels::*;

pub use crate::defaults::{DefaultFragmentVariables, DefaultGlobals, DefaultVertexVariables};

/// A group of variables (field names + field values) that has some reflection
/// properties, i.e., the ability to return a list of all variable descriptions.
pub trait Variables {
    fn list() -> Vec<Variable>;
}

/// A group of variables that also can be queried by name to get their values.
pub trait Globals: Variables {
    fn get(&self, name: &str) -> Value;
    fn bindings<'a>(&'a self) -> Vec<Binding<'a>>;
}

/// A variable value (i.e., to pass to the GPU shader).
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Value {
    Vector2f(Vector2f),
    Vector3f(Vector3f),
    Vector4f(Vector4f),
    Matrix4f(Matrix4f),
}

/// A description of a variable that can be represented in shader code.
#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    Vector2f(Metadata),
    Vector3f(Metadata),
    Vector4f(Metadata),
    Matrix4f(Metadata),
}

impl Variable {
    pub fn name(&self) -> &str {
        match self {
            Variable::Vector2f(Metadata { name, .. }) => &name,
            Variable::Vector3f(Metadata { name, .. }) => &name,
            Variable::Vector4f(Metadata { name, .. }) => &name,
            Variable::Matrix4f(Metadata { name, .. }) => &name,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Metadata {
    pub name: String,
    pub location: Location,
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.location, self.name)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Location {
    /// Location index for where this variable lives in the GPU buffer.
    Index(u16),
    /// Used if the variable is a built-in value.
    BuiltIn(BuiltIn),
    /// Used if the variable is in a group binding.
    GroupBinding(u16, u16),
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Index(index) => write!(f, "@location({})", index),
            Location::BuiltIn(BuiltIn::ClipPosition) => write!(f, "@builtin(position)"),
            Location::GroupBinding(group, binding) => {
                write!(f, "@group({}) @binding({})", group, binding)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BuiltIn {
    /// Should correspond to a Vector4f for clip coordinates.
    ClipPosition,
}

pub fn built_in(built_in: BuiltIn) -> Variable {
    match built_in {
        BuiltIn::ClipPosition => Variable::Vector4f(Metadata {
            name: "clip_position".into(),
            location: Location::BuiltIn(built_in),
        }),
    }
}

pub struct VariablesStruct {
    pub name: String,
    pub variables: Vec<Variable>,
}

impl fmt::Display for VariablesStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct {} {{\n", self.name)?;
        for variable in &self.variables {
            match variable {
                Variable::Vector2f(metadata) => write!(f, "    {}: vec2<f32>,\n", metadata)?,
                Variable::Vector3f(metadata) => write!(f, "    {}: vec3<f32>,\n", metadata)?,
                Variable::Vector4f(metadata) => write!(f, "    {}: vec4<f32>,\n", metadata)?,
                Variable::Matrix4f(metadata) => write!(f, "    {}: mat4x4<f32>,\n", metadata)?,
            }
        }
        write!(f, "}}\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_variables_name() {
        assert_eq!(
            Variable::Vector2f(Metadata {
                location: Location::Index(0),
                name: "asdf_2f".into()
            })
            .name(),
            "asdf_2f",
        );
        assert_eq!(
            Variable::Vector3f(Metadata {
                location: Location::Index(0),
                name: "hey_hey_five".into()
            })
            .name(),
            "hey_hey_five",
        );
        assert_eq!(
            Variable::Vector4f(Metadata {
                location: Location::Index(0),
                name: "one_two_three".into()
            })
            .name(),
            "one_two_three",
        );
        assert_eq!(
            Variable::Matrix4f(Metadata {
                location: Location::Index(0),
                name: "quad_tree".into()
            })
            .name(),
            "quad_tree",
        );
    }

    #[test]
    fn test_variables_built_in() {
        assert_eq!(
            built_in(BuiltIn::ClipPosition),
            Variable::Vector4f(Metadata {
                location: Location::BuiltIn(BuiltIn::ClipPosition),
                name: "clip_position".into()
            })
        );
    }

    #[test]
    fn test_variables_write_struct() {
        let variables = vec![
            built_in(BuiltIn::ClipPosition),
            Variable::Vector2f(Metadata {
                name: "my_vector2f".into(),
                location: Location::Index(3),
            }),
            Variable::Vector3f(Metadata {
                name: "my_vector3f".into(),
                location: Location::Index(29),
            }),
            Variable::Vector4f(Metadata {
                name: "my_vector4f".into(),
                location: Location::Index(17),
            }),
        ];
        let my_code = format!(
            "{}",
            VariablesStruct {
                variables,
                name: "MySuperShaderStructX".into(),
            }
        );
        assert_eq!(
            my_code,
            "struct MySuperShaderStructX {\n    \
            @builtin(position) clip_position: vec4<f32>,\n    \
            @location(3) my_vector2f: vec2<f32>,\n    \
            @location(29) my_vector3f: vec3<f32>,\n    \
            @location(17) my_vector4f: vec4<f32>,\n\
            }\n",
        );
    }
}
