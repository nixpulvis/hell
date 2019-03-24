use interact::*;

/// Makes choices automatically when possible.
#[derive(Debug)]
pub struct Auto<'a, O: 'a + Observation, C: 'a + Choice>(pub &'a mut Choose<O, C>);

mod action;
mod feed;
