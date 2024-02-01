use crate::pixels::Pixels;
use crate::variables::VariablesStruct;

/// For use with binding global values to shaders.
pub enum Binding<'a> {
    /// Creates a uniform struct.
    Struct(&'a VariablesStruct),
    Pixels(&'a Pixels),
}
