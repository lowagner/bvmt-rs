use std::fmt;

use crate::dimensions::*;

pub use crate::defaults::{DefaultFragmentVariables, DefaultGlobals, DefaultVertexVariables};

/// A group of variables (field names + field values) that has some reflection
/// properties, i.e., the ability to return a list of all variable descriptions.
pub trait Variables {
    fn list() -> Vec<Variable>;
}

/// A variable value (i.e., to pass to the GPU shader).
/// Includes the name of the variable for reflection purposes.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Value<'a> {
    Vector2f(&'a str, &'a Vector2f),
    Vector3f(&'a str, &'a Vector3f),
    Vector4f(&'a str, &'a Vector4f),
    Matrix4f(&'a str, &'a Matrix4f),
}

/// A description of a variable that can be represented in shader code.
#[derive(Clone, PartialEq, Debug)]
pub enum Variable {
    Vector2f(Metadata),
    Vector3f(Metadata),
    Vector4f(Metadata),
    // TODO: think of a consistent way to do this.  probably need to reserve multiple locations (i.e., 4 Vector4f's).
    // Matrix4f(Metadata),
}

impl<'a> Value<'a> {
    pub fn name(&'a self) -> &'a str {
        match self {
            Value::Vector2f(name, _) => &name,
            Value::Vector3f(name, _) => &name,
            Value::Vector4f(name, _) => &name,
            Value::Matrix4f(name, _) => &name,
        }
    }
}

impl Variable {
    pub fn name(&self) -> &str {
        match self {
            Variable::Vector2f(Metadata { name, .. }) => &name,
            Variable::Vector3f(Metadata { name, .. }) => &name,
            Variable::Vector4f(Metadata { name, .. }) => &name,
        }
    }

    pub fn index(&self) -> Option<u16> {
        match self {
            Variable::Vector2f(Metadata { location, .. }) => location.index(),
            Variable::Vector3f(Metadata { location, .. }) => location.index(),
            Variable::Vector4f(Metadata { location, .. }) => location.index(),
        }
    }

    pub(crate) fn bytes_format(&self) -> (usize, wgpu::VertexFormat) {
        match self {
            Variable::Vector2f(_) => (Self::BYTES_PER_32 * 2, wgpu::VertexFormat::Float32x2),
            Variable::Vector3f(_) => (Self::BYTES_PER_32 * 3, wgpu::VertexFormat::Float32x3),
            Variable::Vector4f(_) => (Self::BYTES_PER_32 * 4, wgpu::VertexFormat::Float32x4),
            // TODO: doesn't exist, need to make multiple attributes...
            // Variable::Matrix4f(_) => wgpu::VertexFormat::Float32x4x4,
        }
    }

    /// Number of bytes in e.g. an f32 or i32.
    pub const BYTES_PER_32: usize = 4;
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

impl Location {
    /// Returns the index if this is an indexed location.
    fn index(&self) -> Option<u16> {
        match self {
            Location::Index(index) => Some(*index),
            _ => None,
        }
    }
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
            }
        }
        write!(f, "}}\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_values_name() {
        assert_eq!(
            Value::Vector2f("it".into(), &Vector2f::default()).name(),
            "it",
        );
        assert_eq!(
            Value::Vector3f("has".into(), &Vector3f::default()).name(),
            "has",
        );
        assert_eq!(
            Value::Vector4f("been".into(), &Vector4f::default()).name(),
            "been",
        );
        assert_eq!(
            Value::Matrix4f("excellent".into(), &Matrix4f::default()).name(),
            "excellent",
        );
    }

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

    #[test]
    fn test_variables_bytes_per_32() {
        assert_eq!(Variable::BYTES_PER_32, std::mem::size_of::<f32>());
        assert_eq!(Variable::BYTES_PER_32, std::mem::size_of::<i32>());
    }

    #[test]
    fn test_variables_bytes_format() {
        let metadata = Metadata {
            name: "whatever".into(),
            location: Location::Index(12345),
        };
        assert_eq!(
            Variable::Vector2f(metadata.clone()).bytes_format(),
            (
                std::mem::size_of::<f32>() * 2,
                wgpu::VertexFormat::Float32x2
            )
        );
        assert_eq!(
            Variable::Vector3f(metadata.clone()).bytes_format(),
            (
                std::mem::size_of::<f32>() * 3,
                wgpu::VertexFormat::Float32x3
            )
        );
        assert_eq!(
            Variable::Vector4f(metadata.clone()).bytes_format(),
            (
                std::mem::size_of::<f32>() * 4,
                wgpu::VertexFormat::Float32x4
            )
        );
    }

    #[test]
    fn test_variables_location_index() {
        assert_eq!(Location::Index(12345).index(), Some(12345));
        assert_eq!(Location::BuiltIn(BuiltIn::ClipPosition).index(), None);
        assert_eq!(Location::Index(9876).index(), Some(9876));
    }
}
