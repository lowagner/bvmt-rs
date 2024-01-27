use std::fmt;

use crate::dimensions::*;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default, Pod, Zeroable)]
pub struct DefaultVertexVariables {
    location: Vector3f,
}

impl Variables for DefaultVertexVariables {
    fn list(&self) -> Vec<Variable> {
        vec![Variable::Vector3f(Metadata {
            name: "location".into(),
            location: Location::Index(0),
        })]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct DefaultFragmentVariables {
    // Note that we don't need to add these variables into the `Fragment`s,
    // since this is only constructed on the GPU via the vertex shader.
}

impl Variables for DefaultFragmentVariables {
    fn list(&self) -> Vec<Variable> {
        vec![built_in(BuiltIn::ClipPosition)]
    }
}

/// A group of variables (field names + field values) that has some reflection
/// properties, i.e., the ability to return a list of all values.
pub trait Variables {
    fn list(&self) -> Vec<Variable>;
}

// TODO: pub trait Globals: Variables { fn get(Variable) -> Value; }

/// A description of a variable that can be represented in shader code.
#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    Vector2f(Metadata),
    Vector3f(Metadata),
    Vector4f(Metadata),
    // TODO: Pixels (e.g., Texture)
}

pub struct VariablesDeclaration<'a> {
    pub name: &'a str,
    pub variables: &'a Vec<Variable>,
}

impl<'a> fmt::Display for VariablesDeclaration<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "struct {} {{\n", self.name)?;
        for variable in self.variables {
            match variable {
                Variable::Vector2f(metadata) => write!(f, "    {}: vec2<f32>,\n", metadata)?,
                Variable::Vector3f(metadata) => write!(f, "    {}: vec3<f32>,\n", metadata)?,
                Variable::Vector4f(metadata) => write!(f, "    {}: vec4<f32>,\n", metadata)?,
            }
        }
        write!(f, "}}\n")
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
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Index(index) => write!(f, "@location({})", index),
            Location::BuiltIn(BuiltIn::ClipPosition) => write!(f, "@builtin(position)"),
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

#[cfg(test)]
mod test {
    use super::*;

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
            VariablesDeclaration {
                variables: &variables,
                name: "MySuperShaderStructX",
            }
        );
        assert_eq!(
            my_code,
            "struct MySuperShaderStructX {\n    \
            @builtin(position) position: vec4<f32>,\n    \
            @location(3) my_vector2f: vec2<f32>,\n    \
            @location(29) my_vector3f: vec3<f32>,\n    \
            @location(17) my_vector4f: vec4<f32>,\n\
            }\n",
        );
    }
}
