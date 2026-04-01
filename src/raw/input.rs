use super::block::Block;

pub struct Input<'a> {
    pub input_type: InputType,
    pub data: InputData<'a>,
    pub obscured: Option<InputData<'a>>,
}

pub enum InputData<'a> {
    Internal(Block<'a>),
    Reference(&'a Block<'a>),
}

pub enum InputType {
    Shadow,         // 1
    NoShadow,       // 2
    ObscuredShadow, // 3
}
