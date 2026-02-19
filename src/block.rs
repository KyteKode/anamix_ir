use crate::fields::*;
use crate::project::Broadcast;

use rgb::RGB8;

pub type BlockRef = Box<Block>;

#[derive(Debug)]
pub struct Thread(Vec<Block>);

#[derive(Debug)]
pub enum StringIn {
    String(String),
    Block(BlockRef),
}

#[derive(Debug)]
pub enum NumberIn {
    Number(f64),
    Block(BlockRef),
}

#[derive(Debug)]
pub enum Block {
    // Motion
    MotionMoveSteps(NumberIn),
    MotionTurnRight(NumberIn),
    MotionTurnLeft(NumberIn),
    MotionGoto(BlockRef),
    MotionGotoXY(NumberIn, NumberIn),
    MotionGlideTo(NumberIn, BlockRef),
    MotionGlideSecsToXY(NumberIn, NumberIn, NumberIn),
    MotionPointInDirection(NumberIn),
    MotionPointTowards(BlockRef),
    MotionChangeXBy(NumberIn),
    MotionSetX(NumberIn),
    MotionChangeYBy(NumberIn),
    MotionSetY(NumberIn),
    MotionIfOnEdgeBounce,
    MotionSetRotationStyle(RotationStyle),

    MotionXPosition,
    MotionYPosition,
    MotionDirection,

    MotionGotoMenu(MotionTarget),
    MotionGlidetoMenu(MotionTarget),
    MotionPointTowardsMenu(MotionTarget),

    // Looks
    LooksSayForSecs(StringIn, NumberIn),
    LooksSay(StringIn),
    LooksThinkForSecs(StringIn, NumberIn),
    LooksThink(StringIn),
    LooksSwitchCostumeTo(BlockRef),
    LooksNextCostume(StringIn),
    LooksSwitchBackdropTo(BlockRef),
    LooksNextBackdrop(StringIn),
    LooksChangeSizeBy(NumberIn),
    LooksSetSizeTo(NumberIn),
    LooksChangeEffectBy(GraphicEffect, NumberIn),
    LooksSetEffectTo(GraphicEffect, NumberIn),
    LooksClearGraphicEffects,
    LooksShow,
    LooksHide,
    LooksGotoFrontBack(FrontBack),
    LooksGoForwardBackwardLayers(ForwardBackward, NumberIn),

    LooksCostumeNumberName(NumberName),
    LooksBackdropNumberName(NumberName),

    LooksCostume(StringIn),
    LooksBackdrops(Backdrop),

    // Sound
    SoundPlayUntilDone(BlockRef),
    SoundPlay(BlockRef),
    SoundStopAllSounds,
    SoundChangeEffectBy(SoundEffect, NumberIn),
    SoundSetEffectTo(SoundEffect, NumberIn),
    SoundChangeVolumeBy(NumberIn),
    SoundSetVolumeTo(NumberIn),

    SoundVolume,

    SoundSoundsMenu(StringIn),

    // Events
    EventBroadcast(Broadcast),
    EventBroadcastAndWait(Broadcast),

    EventWhenFlagClicked,
    EventWhenKeyPressed(Key),
    EventWhenThisSpriteClicked,
    EventWhenBackdropSwitchesTo(Backdrop),
    EventWhenGreaterThan(NumberIn, WhenGreaterThanMenu),
    EventWhenBroadcastRecieved(Broadcast),

    // Control
    ControlWait(NumberIn),
    ControlWaitUntil(BlockRef),
    ControlCreateCloneOf(BlockRef),

    ControlStop(StopOption),
    ControlDeleteThisClone,

    ControlRepeat(NumberIn, Vec<Block>),
    ControlForever(Vec<Block>),
    ControlIf(BlockRef, Vec<Block>),
    ControlIfElse(BlockRef, Vec<Block>, Vec<Block>),
    ControlRepeatUntil(BlockRef, Vec<Block>),

    ControlStartAsClone,

    ControlCreateCloneOfMenu(CloneOf),

    // Sensing
    SensingAskAndWait(StringIn),
    SensingResetTimer,
    SensingSetDragMode(DragMode),

    SensingTouchingObject(BlockRef),
    SensingTouchingColor(RGB8),
    SensingColorIsTouchingColor(RGB8, RGB8),
    SensingKeyPressed(BlockRef),
    SensingMouseDown,

    SensingDistanceTo(BlockRef),
    SensingAnswer,
    SensingMouseX,
    SensingMouseY,
    SensingTimer,
    SensingOf(BlockRef, SensingOfProperty),
    SensingCurrent(CurrentMenu),
    SensingDaysSince2000,
    SensingUsername,

    SensingTouchingObjectMenu(TouchingObjectTarget),
    SensingDistanceToMenu(DistanceToTarget),
    SensingKeyOptions(Key),
    SensingOfObjectMenu(OfObjectMenu),
}
