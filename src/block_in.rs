use crate::block::Block;

pub enum NumberIn {
    Literal(f64),
    Block(Block),
}
