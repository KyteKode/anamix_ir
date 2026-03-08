use crate::{
    hat::Hat,
    command::Command
};

pub struct Thread<'a> {
    hat: Hat,
    blocks: Vec<Command<'a>>
}