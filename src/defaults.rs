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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DefaultGlobals {
    view_transform: Matrix4f,
}

impl std::default::Default for DefaultGlobals {
    fn default() -> Self {
        Self {
            view_transform: Matrix4f::identity(),
        }
    }
}

impl Variables for DefaultGlobals {
    fn list() -> Vec<Variable> {
        vec![Variable::Matrix4f(Metadata {
            name: "view_transform".into(),
            // TODO: do we even need Location for Globals?
            // some live in bind groups, etc.
            location: Location::Index(0),
        })]
    }
}

impl Globals for DefaultGlobals {
    fn get(&self, name: &str) -> Value {
        match name {
            "view_transform" => Value::Matrix4f(self.view_transform),
            _ => panic!("invalid default global: {}", name),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_globals_can_iterate_safely() {
        let mut globals_count = 0;
        let globals = DefaultGlobals::default();
        for variable in DefaultGlobals::list() {
            let _ = globals.get(variable.name());
            globals_count += 1;
        }
        assert_eq!(globals_count, 1);
    }

    #[test]
    fn test_default_globals_initializes_view_transform_to_identity() {
        let globals = DefaultGlobals::default();
        assert_eq!(
            globals.get("view_transform"),
            Value::Matrix4f(Matrix4f::identity())
        );
    }

    #[test]
    fn test_default_globals_can_return_modified_view_transform() {
        let mut globals = DefaultGlobals::default();
        globals.view_transform.x = Vector4f::new(4.0, 3.0, 2.0, 1.0);
        globals.view_transform.y = Vector4f::new(40.0, 30.0, 20.0, 10.0);
        globals.view_transform.z = Vector4f::new(400.0, 300.0, 200.0, 100.0);
        globals.view_transform.w = Vector4f::new(4000.0, 3000.0, 2000.0, 1000.0);
        assert_eq!(
            globals.get("view_transform"),
            Value::Matrix4f(Matrix4f {
                x: Vector4f::new(4.0, 3.0, 2.0, 1.0),
                y: Vector4f::new(40.0, 30.0, 20.0, 10.0),
                z: Vector4f::new(400.0, 300.0, 200.0, 100.0),
                w: Vector4f::new(4000.0, 3000.0, 2000.0, 1000.0),
            }),
        );
    }
}
