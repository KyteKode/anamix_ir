#[derive(Debug)]
pub enum RotationStyle {
    LeftRight,
    DontRotate,
    AllAround,
}

#[derive(Debug)]
pub enum MotionTarget {
    Random,
    Mouse,
    Sprite(String),
}

#[derive(Debug)]
pub enum GraphicEffect {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}

#[derive(Debug)]
pub enum FrontBack {
    Front,
    Back,
}

#[derive(Debug)]
pub enum ForwardBackward {
    Forward,
    Backward,
}

#[derive(Debug)]
pub enum NumberName {
    Number,
    Name,
}

#[derive(Debug)]
pub enum Backdrop {
    NextBackdrop,
    PreviousBackdrop,
    RandomBackdrop,
    Backdrop(String),
}

#[derive(Debug)]
pub enum SoundEffect {
    Pitch,
    Pan,
}

#[derive(Debug)]
pub enum Key {
    Space,
    UpArrow, DownArrow, LeftArrow, RightArrow,
    Any,
    A, B, C, D, E, F, G,
    H, I, J, K, L, M, N,
    O, P, Q, R, S, T,
    U, V, W, X, Y, Z,
    One, Two, Three, Four,
    Five, Six, Seven,
    Eight, Nine, Ten,
    Enter
}

#[derive(Debug)]
pub enum WhenGreaterThanMenu {
    Timer, Loudness
}

#[derive(Debug)]
pub enum CloneOf {
    Myself,
    Sprite(String)
}


#[derive(Debug)]
pub enum StopOption {
    All,
    OtherScriptsInSprite,
    ThisSprite
}

#[derive(Debug)]
pub enum DragMode {
    Draggable,
    NotDraggable
}

#[derive(Debug)]
pub enum SensingOfProperty {
    BackdropNum,
    BackdropName,

    XPosition,
    YPosition,
    Direction,
    CostumeNum,
    CostumeName,
    Size,
    SpriteVar(String),

    Volume
}

#[derive(Debug)]
pub enum CurrentMenu {
    Year,
    Month,
    Date,
    DayOfWeek,
    Hour,
    Minute,
    Second
}

#[derive(Debug)]
pub enum TouchingObjectTarget {
    Mouse,
    Edge,
    Sprite(String)
}

#[derive(Debug)]
pub enum DistanceToTarget {
    Mouse,
    Sprite(String)
}

#[derive(Debug)]
pub enum OfObjectMenu {
    Backdrop,
    Sprite(String)
}