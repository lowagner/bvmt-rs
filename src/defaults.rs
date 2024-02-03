use crate::dimensions::*;
use crate::gpu::*;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug, Default, Pod, Zeroable)]
pub struct DefaultVertexVariables {
    location: Vector3f,
    // TODO: add color
}

impl Variables for DefaultVertexVariables {
    fn list() -> Vec<Variable> {
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
    // TODO: add color
}

impl Variables for DefaultFragmentVariables {
    fn list() -> Vec<Variable> {
        vec![built_in(BuiltIn::ClipPosition)]
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct DefaultGlobals {
    view: Matrix4f,
}

impl std::default::Default for DefaultGlobals {
    fn default() -> Self {
        Self {
            view: Matrix4f::identity(),
        }
    }
}

impl Globals for DefaultGlobals {
    fn binds<'a>(&'a self) -> Vec<Bind<'a>> {
        vec![Bind::Struct(
            0,
            UniformStruct {
                name: "Globals",
                values: vec![Value::Matrix4f("view", &self.view)],
            },
        )]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_globals_can_iterate_safely() {
        let mut globals_count = 0;
        let globals = DefaultGlobals::default();
        for bind in globals.binds() {
            match bind {
                Bind::Struct(_, uniform_struct) => {
                    for value in uniform_struct.values {
                        globals_count += 1;
                    }
                }
                bind => panic!("unexpected global: {:?}\n", bind),
            }
        }
        assert_eq!(globals_count, 1);
    }

    #[test]
    fn test_default_globals_uses_group_0() {
        let globals = DefaultGlobals::default();
        let binds = globals.binds();
        assert_eq!(binds.len(), 1);
        match &binds[0] {
            Bind::Struct(group, _) => {
                assert_eq!(*group, 0);
            }
            bind => panic!("unexpected global: {:?}\n", bind),
        }
    }

    #[test]
    fn test_default_globals_initializes_view_to_identity() {
        let globals = DefaultGlobals::default();
        match &globals.binds()[0] {
            Bind::Struct(_, uniform_struct) => {
                assert_eq!(uniform_struct.name, "Globals");
                assert_eq!(
                    uniform_struct.values,
                    vec![Value::Matrix4f("view", &Matrix4f::identity()),]
                );
            }
            bind => panic!("unexpected global: {:?}\n", bind),
        }
    }

    #[test]
    fn test_default_globals_can_return_modified_view() {
        let mut globals = DefaultGlobals::default();
        globals.view.x = Vector4f::new(4.0, 3.0, 2.0, 1.0);
        globals.view.y = Vector4f::new(40.0, 30.0, 20.0, 10.0);
        globals.view.z = Vector4f::new(400.0, 300.0, 200.0, 100.0);
        globals.view.w = Vector4f::new(4000.0, 3000.0, 2000.0, 1000.0);
        match &globals.binds()[0] {
            Bind::Struct(_, uniform_struct) => {
                assert_eq!(uniform_struct.name, "Globals");
                assert_eq!(
                    uniform_struct.values,
                    vec![Value::Matrix4f(
                        "view",
                        &Matrix4f {
                            x: Vector4f::new(4.0, 3.0, 2.0, 1.0),
                            y: Vector4f::new(40.0, 30.0, 20.0, 10.0),
                            z: Vector4f::new(400.0, 300.0, 200.0, 100.0),
                            w: Vector4f::new(4000.0, 3000.0, 2000.0, 1000.0),
                        }
                    )]
                );
            }
            bind => panic!("unexpected global: {:?}\n", bind),
        }
    }
}
