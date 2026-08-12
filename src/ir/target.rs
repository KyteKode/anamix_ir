use indexmap::IndexMap;

use std::collections::HashMap;

use super::{
    asset::{Costume, Sound},
    data::{Broadcast, List, Variable},
    stack::BlockStack
};

pub struct Stage {
    pub variables: IndexMap<String, Variable>,
    pub lists: IndexMap<String, List>,
    pub broadcasts: HashMap<String, Broadcast>,

    pub blocks: Vec<BlockStack>,

    pub current_costume: usize,
    pub costumes: IndexMap<String, Costume>,

    pub sounds: IndexMap<String, Sound>,
    pub volume: f64,

    pub tempo: f64,
    pub video_state: VideoState,
    pub video_transparency: f64,
    pub text_to_speech_language: Option<String>,
}

pub struct Sprite {
    pub variables: IndexMap<String, Variable>,
    pub lists: IndexMap<String, List>,

    pub blocks: Vec<BlockStack>,

    pub current_costume: usize,
    pub costumes: IndexMap<String, Costume>,

    pub sounds: IndexMap<String, Sound>,
    pub volume: f64,

    pub tempo: f64,
    pub video_state: VideoState,
    pub video_transparency: f64,
    pub text_to_speech_language: Option<String>,
}

pub enum VideoState {
    On,
    Off,
    OnFlipped,
}
