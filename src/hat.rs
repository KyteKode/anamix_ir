use crate::{
    field::{KeyOption, WhenGreaterThanMenu},
    block_in::NumberIn,
    project::{Backdrop, Broadcast}
};

pub enum Hat {
    WhenFlagClicked,
    WhenKeyPressed(KeyOption),
    WhenThisSpriteClicked,
    WhenBackdropSwitchesTo(Backdrop),
    WhenGreaterThan(NumberIn, WhenGreaterThanMenu),
    WhenBroadcastRecieved(Broadcast),
    
    StartAsClone
}
