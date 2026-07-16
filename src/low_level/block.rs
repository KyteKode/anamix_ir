use std::collections::HashMap;
use id_arena::Id;
use either::Either;

type Input = (u8, LLArrayBlock, Option<LLArrayBlock>);
type Field = (String, Option<String>);

pub enum LLBlock {
    Object(LLObjectBlock),
    Array(LLArrayBlock),
}

pub struct LLObjectBlock {
    pub opcode: String,
    pub next: Option<Id<LLBlock>>,
    pub parent: Option<Id<LLBlock>>,
    pub inputs: HashMap<String, Input>,
    pub fields: HashMap<String, Field>,
    pub shadow: bool,
    pub top_level: bool,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub mutation: HashMap<String, String>
}

pub enum LLArrayBlock {
    ID(Id<LLBlock>),
    Array(u8, Either<String, f64>, Option<String>, Option<f64>, Option<f64>)
}