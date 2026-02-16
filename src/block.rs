use crate::fields::*;
use crate::project::Broadcast;

#[derive(Debug)]
pub struct Thread(Vec<Block>);

#[derive(Debug)]
pub enum Block {
    // Motion
    MotionMoveSteps(f64),
    MotionTurnRight(f64),
    MotionTurnLeft(f64),
    MotionGoto(Box<Block>),
    MotionGotoXY(f64, f64),
    MotionGlideTo(f64, Box<Block>),
    MotionGlideSecsToXY(f64, f64, f64),
    MotionPointInDirection(f64),
    MotionPointTowards(Box<Block>),
    MotionChangeXBy(f64),
    MotionSetX(f64),
    MotionChangeYBy(f64),
    MotionSetY(f64),
    MotionIfOnEdgeBounce,
    MotionSetRotationStyle(RotationStyle),

    MotionXPosition,
    MotionYPosition,
    MotionDirection,

    MotionGotoMenu(MotionTarget),
    MotionGlidetoMenu(MotionTarget),
    MotionPointTowardsMenu(MotionTarget),

    // Looks
    LooksSayForSecs(String, f64),
    LooksSay(String),
    LooksThinkForSecs(String, f64),
    LooksThink(String),
    LooksSwitchCostumeTo(Box<Block>),
    LooksNextCostume(String),
    LooksSwitchBackdropTo(Box<Block>),
    LooksNextBackdrop(String),
    LooksChangeSizeBy(f64),
    LooksSetSizeTo(f64),
    LooksChangeEffectBy(GraphicEffect, f64),
    LooksSetEffectTo(GraphicEffect, f64),
    LooksClearGraphicEffects,
    LooksShow,
    LooksHide,
    LooksGotoFrontBack(FrontBack),
    LooksGoForwardBackwardLayers(ForwardBackward, i64),

    LooksCostumeNumberName(NumberName),
    LooksBackdropNumberName(NumberName),

    LooksCostume(String),
    LooksBackdrops(Backdrop),

    // Sound
    SoundPlayUntilDone(Box<Block>),
    SoundPlay(Box<Block>),
    SoundStopAllSounds,
    SoundChangeEffectBy(SoundEffect, f64),
    SoundSetEffectTo(SoundEffect, f64),
    SoundChangeVolumeBy(f64),
    SoundSetVolumeTo(f64),

    SoundVolume,

    SoundSoundsMenu(String),

    EventBroadcast(Broadcast),
    EventBroadcastAndWait(Broadcast),
}
