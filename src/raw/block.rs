use super::{arr_block::ArrayBlock, obj_block::ObjectBlock};

pub enum Block<'a> {
    Object(ObjectBlock<'a>),
    Array(ArrayBlock),
}
