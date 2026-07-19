use id_arena::Id;

use super::block::LLBlock;

pub(crate) fn id_string(id: &Id<LLBlock>) -> String {
    format!("b{:05}", id.index())
}