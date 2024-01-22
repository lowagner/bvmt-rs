#![allow(dead_code)]

use crate::variables::*;

use bytemuck::{Pod, Zeroable};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Fragments<F: Variables> {
    array: Vec<Fragment>,
    /// The `Variables` here don't need to be settable, just self-descriptive.
    /// I.e., these variables are interpolated from the outputs of the vertex
    /// shader and *cannot* be set on each fragment, so they live here.
    variables_description: F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Fragment {
    index0: u16,
    index1: u16,
    index2: u16,
}
