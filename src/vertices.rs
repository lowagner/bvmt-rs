#![allow(dead_code)]

use crate::variables::*;

pub struct Vertices<V: Variables> {
    /// The `Variables` in `V` must be settable, because we are specifying
    /// the values of each vertex (e.g., position, color, etc.).
    array: Vec<V>,
}
