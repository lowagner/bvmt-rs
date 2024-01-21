#![allow(dead_code)]

use bytemuck::{Pod, Zeroable};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Fragments {
    array: Vec<Fragment>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Pod, Zeroable)]
pub struct Fragment {
    index0: u16,
    index1: u16,
    index2: u16,
}
