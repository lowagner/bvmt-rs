#![allow(dead_code)]

use crate::binds::Bind;
use crate::variables::Value;

/// A group of bindings that also can be queried by name to get their values.
pub trait Globals {
    fn binds<'a>(&'a self) -> Vec<Bind<'a>>;
}
