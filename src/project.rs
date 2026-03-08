use md5::{Digest, Md5};

use crate::block_thread::Thread;

pub struct Project<'a> {
    pub stage: Stage<'a>,
    pub sprites: Vec<Sprite<'a>>,
    pub broadcasts: Vec<Broadcast>,
}

pub struct Broadcast(String);

pub enum Graphic {
    Vector { data: String, name: String },
    Bitmap { data: Vec<u8>, name: String },
}

impl Graphic {
    pub fn md5(&self) -> String {
        let mut hasher = Md5::new();
        match self {
            Graphic::Vector { data, name: _ } => hasher.update(data),
            Graphic::Bitmap { data, name: _ } => hasher.update(data),
        }
        format!("{:x}", hasher.finalize())
    }
}

pub struct Variable {
    pub name: String,
    pub value: String,
    pub cloud: bool,
}

pub struct List {
    pub name: String,
    pub value: Vec<String>,
}

pub struct Sound {
    name: String,
    data: Vec<u8>,
}

impl Sound {
    pub fn md5(&self) -> String {
        let mut hasher = Md5::new();
        hasher.update(self.data.clone());
        format!("{:x}", hasher.finalize())
    }
}

pub struct Costume(pub Graphic);

impl Costume {
    pub fn md5(&self) -> String {
        self.0.md5()
    }
}

pub enum RotationStyle {
    AllAround,
    DontRotate,
    LeftRight,
}

pub struct Sprite<'a> {
    pub name: String,

    pub variables: Vec<Variable>,
    pub lists: Vec<List>,

    pub costumes: Vec<Costume>,
    pub current_costume: usize,
    pub layer_order: f64,

    pub sounds: Vec<Sound>,
    pub volume: f64,

    pub visible: bool,

    pub x: f64,
    pub y: f64,

    pub size: f64,

    pub direction: f64,
    pub draggable: bool,
    pub rotation_style: RotationStyle,

    pub threads: Vec<Thread<'a>>
}

pub struct Backdrop(pub Graphic);

impl Backdrop {
    pub fn md5(&self) -> String {
        self.0.md5()
    }
}

pub enum VideoState {
    On,
    Off,
    OnFlipped,
}

pub struct Stage<'a> {
    pub name: String,

    pub variables: Vec<Variable>,
    pub lists: Vec<List>,

    pub backdrops: Vec<Backdrop>,
    pub current_backdrop: usize,

    pub sounds: Vec<Sound>,
    pub volume: f64,

    pub tempo: f64,
    pub video_state: VideoState,
    pub video_transparency: f64,
    pub text_to_speech_languge: String,

    pub threads: Vec<Thread<'a>>
}
