#![allow(dead_code)]

use crate::dimensions::*;

pub struct Vertices<Variables> {
    array: Vec<Vertex<Variables>>,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vertex<Variables> {
    position: Vector3f,
    variables: Variables,
}
