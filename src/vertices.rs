#![allow(dead_code)]

use crate::dimensions::*;
use crate::variables::*;

pub struct Vertices<V: Variables> {
    array: Vec<Vertex<V>>,
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Vertex<V: Variables> {
    position: Vector3f,
    variables: V,
}
