#![allow(dead_code)]

use crate::pixels::Pixels;
use crate::variables::Value;

/// For use with binding global values to shaders.
#[derive(Clone, Debug)]
pub enum Bind<'a> {
    /// Creates a uniform struct.
    Struct(u16, UniformStruct<'a>),
    Pixels(u16, &'a Pixels),
    // TODO: Sampler based on what the Pixels wants.
}

impl<'a> Bind<'a> {
    pub fn group(&self) -> u16 {
        match self {
            Bind::Struct(group, _) => *group,
            Bind::Pixels(group, _) => *group,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UniformStruct<'a> {
    pub name: &'a str,
    pub values: Vec<Value<'a>>,
}

pub fn get_uniform_value<'a>(
    uniform_struct: &'a UniformStruct,
    variable_name: &str,
) -> Option<Value<'a>> {
    for value in &uniform_struct.values {
        if value.name() == variable_name {
            return Some(*value);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dimensions::*;

    #[test]
    fn test_bind_group() {
        assert_eq!(
            Bind::Struct(
                12,
                UniformStruct {
                    name: "Whatever",
                    values: vec![]
                }
            )
            .group(),
            12,
        );

        assert_eq!(
            Bind::Pixels(32, &Pixels::new(Size2i::new(8, 4)),).group(),
            32,
        );
    }

    // TODO: test binding()

    // TODO: test get_uniform_value()
}
