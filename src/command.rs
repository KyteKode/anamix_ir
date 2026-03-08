use crate::{
    block_in::*,
    shadow::Shadow,
    field::RotationStyle
};

pub enum Command<'a> {
    MotionMoveSteps(NumberIn),
    MotionTurnRight(NumberIn),
    MotionTurnLeft(NumberIn),
    MotionGoTo(Shadow<'a>),
    MotionGoToXY(NumberIn, NumberIn),
    MotionGlideTo(NumberIn, Shadow<'a>),
    MotionGlideSecsToXY(NumberIn, NumberIn, NumberIn),
    MotionPointInDirection(NumberIn),
    MotionPointTowards(Shadow<'a>),
    MotionChangeXBy(NumberIn),
    MotionSetX(NumberIn),
    MotionChangeYBy(NumberIn),
    MotionSetY(NumberIn),
    MotionIfOnEdgeBounce,
    MotionSetRotationStyle(RotationStyle)
}