use id_arena::{Arena, Id};

use crate::low_level::block::{LLBlock, LLObjectBlock};

pub trait Block {
    fn lower(&self) -> LLObjectBlock;
}

pub struct BlockStack(pub Vec<Box<dyn Block>>);

impl BlockStack {
    pub fn lower(&self, arena: &mut Arena<LLBlock>) {
        let previous: Option<Id<LLBlock>> = None;

        for block in &self.0 {
            let mut low_block = block.lower();
            low_block.parent = previous;

            let block_id = arena.alloc(LLBlock::Object(low_block));

            if let Some(LLBlock::Object(obj)) = previous.map(|id| &mut arena[id]) {
                obj.next = Some(block_id);
            }
        }
    }
}
