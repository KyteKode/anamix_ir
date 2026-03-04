use crate::{
    block_in::NumberIn,
    field::{KeyOption, WhenGreaterThanMenu},
    project::{Backdrop, Broadcast},
};

pub enum Hat {
    WhenFlagClicked,
    WhenKeyPressed(KeyOption),
    WhenThisSpriteClicked,
    WhenBackdropSwitchesTo(Backdrop),
    WhenGreaterThan(NumberIn, WhenGreaterThanMenu),
    WhenBroadcastRecieved(Broadcast),

    StartAsClone,
}
