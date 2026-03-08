use crate::field::MotionTarget;

pub enum Shadow<'a> {
    MotionGoToMenu(MotionTarget<'a>),
    MotionGlideToMenu(MotionTarget<'a>),
    MotionPointTowardsMenu(MotionTarget<'a>)
}