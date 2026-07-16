use id_arena::{Arena, Id};
use std::collections::HashMap;

use super::{
    block::LLBlock,
    asset::{LLCostume, LLSound}
};

pub struct LLTarget {
    pub is_stage: bool,
    pub name: String,
    pub variables: HashMap<String, (String, String, bool)>,
    pub lists: HashMap<String, (String, Vec<String>)>,
    pub broadcasts: Option<HashMap<String, String>>,
    pub block: Arena<LLBlock>,
    pub current_costume: usize,
    pub costumes: Vec<LLCostume>,
    pub sounds: Vec<LLSound>,
    pub layer_order: usize,
    pub volume: f64,

    pub tempo: Option<f64>,
    pub video_state: Option<String>,
    pub video_transparency: Option<f64>,
    pub text_to_speech_language: Option<String>,

    pub visible: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub size: Option<f64>,
    pub direction: Option<f64>,
    pub draggable: Option<bool>,
    pub rotation_style: Option<String>,
}
