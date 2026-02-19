use crate::fields::*;
use crate::project::{Broadcast, Variable, List};

use rgb::RGB8;

pub type BlockRef = Box<Block>;
pub type Menu = Box<Block>;
pub type Boolean = Box<Block>;

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
    MotionGoto(Menu),
    MotionGotoXY(NumberIn, NumberIn),
    MotionGlideTo(NumberIn, Menu),
    MotionGlideSecsToXY(NumberIn, NumberIn, NumberIn),
    MotionPointInDirection(NumberIn),
    MotionPointTowards(Menu),
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
    LooksSwitchCostumeTo(Menu),
    LooksNextCostume(StringIn),
    LooksSwitchBackdropTo(Menu),
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
    SoundPlayUntilDone(Menu),
    SoundPlay(Menu),
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
    ControlWaitUntil(Boolean),
    ControlCreateCloneOf(Menu),

    ControlStop(StopOption),
    ControlDeleteThisClone,

    ControlRepeat(NumberIn, Vec<Block>),
    ControlForever(Vec<Block>),
    ControlIf(Boolean, Vec<Block>),
    ControlIfElse(Boolean, Vec<Block>, Vec<Block>),
    ControlRepeatUntil(Boolean, Vec<Block>),

    ControlStartAsClone,

    ControlCreateCloneOfMenu(CloneOf),

    // Sensing
    SensingAskAndWait(StringIn),
    SensingResetTimer,
    SensingSetDragMode(DragMode),

    SensingTouchingObject(Menu),
    SensingTouchingColor(RGB8),
    SensingColorIsTouchingColor(RGB8, RGB8),
    SensingKeyPressed(Menu),
    SensingMouseDown,

    SensingDistanceTo(Menu),
    SensingAnswer,
    SensingMouseX,
    SensingMouseY,
    SensingTimer,
    SensingOf(Menu, SensingOfProperty),
    SensingCurrent(CurrentMenu),
    SensingDaysSince2000,
    SensingUsername,

    SensingTouchingObjectMenu(TouchingObjectTarget),
    SensingDistanceToMenu(DistanceToTarget),
    SensingKeyOptions(Key),
    SensingOfObjectMenu(OfObjectMenu),

    // Operators
    OperatorGT(StringIn, StringIn),
    OperatorLT(StringIn, StringIn),
    OperatorEquals(StringIn, StringIn),
    OperatorAnd(Boolean, Boolean),
    OperatorOr(Boolean, Boolean),
    OperatorNot(Boolean),
    OperatorContains(StringIn, StringIn),

    OperatorAdd(NumberIn, NumberIn),
    OperatorSubtract(NumberIn, NumberIn),
    OperatorMultiply(NumberIn, NumberIn),
    OperatorDivide(NumberIn, NumberIn),
    OperatorRandom(NumberIn, NumberIn),
    OperatorJoin(StringIn, StringIn),
    OperatorLetterOf(NumberIn, StringIn),
    OperatorLength(StringIn),
    OperatorMod(NumberIn, NumberIn),
    OperatorRound(NumberIn),
    OperatorMathOp(NumberIn, MathOperator),

    // Variables
    DataSetVarialeTo(StringIn, Variable),
    DataChangeVariableBy(NumberIn, Variable),
    DataShowVariable(Variable),
    DataHideVariable(Variable),
    ArrayVariableBlock(Variable),

    // Lists
    DataAddToList(StringIn, List),
    DataDeleteOfList(NumberIn, List),
    DataDeleteAllOfList(List),
    DataInsertAtList(StringIn, NumberIn, List),
    DataReplaceItemOfList(NumberIn, StringIn, List),
    DataShowList(List),
    DataHideList(List),

    DataItemOfList(NumberIn, List),
    DataItemNumOfList(StringIn, List),
    DataLengthOfList(List),

    DataListContainItem(StringIn, List)
}
