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
