use super::{block::Block, field::Field, input::Input};
use std::collections::HashMap;

pub struct ObjectBlock<'a> {
    pub opcode: String,
    pub next: Option<&'a Block<'a>>,
    pub parent: Option<&'a Block<'a>>,
    pub inputs: HashMap<String, Input<'a>>,
    pub fields: HashMap<String, Field>,
    pub shadow: bool,
    pub top_level: bool,
}
