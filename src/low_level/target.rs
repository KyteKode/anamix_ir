use id_arena::Arena;
use serde::ser::{Serialize, SerializeMap, Serializer};

use std::collections::BTreeMap;

use super::{
    asset::{LLCostume, LLSound},
    block::LLBlock,
    helper::{id_string, serialize_entry_if_some},
};

type Variable = (String, String, bool);
type List = (String, Vec<String>);

#[derive(Debug)]
pub struct LLTarget {
    pub is_stage: bool,
    pub name: String,
    pub variables: BTreeMap<String, Variable>,
    pub lists: BTreeMap<String, List>,
    pub broadcasts: Option<BTreeMap<String, String>>,
    pub blocks: Arena<LLBlock>,
    pub current_costume: usize,
    pub costumes: Vec<LLCostume>,
    pub sounds: Vec<LLSound>,
    pub layer_order: usize,
    pub volume: f64,

    pub tempo: Option<f64>,
    pub video_state: Option<String>,
    pub video_transparency: Option<f64>,
    pub text_to_speech_language: Option<Option<String>>, // None is excluded, Some(None) is null, Some(...) is string data

    pub visible: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub size: Option<f64>,
    pub direction: Option<f64>,
    pub draggable: Option<bool>,
    pub rotation_style: Option<String>,
}

impl Serialize for LLTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("isStage", &self.is_stage)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("variables", &self.variables)?;
        map.serialize_entry("lists", &self.lists)?;
        serialize_entry_if_some(&mut map, "broadcasts", &self.broadcasts)?;

        let mut block_map: BTreeMap<String, LLBlock> = BTreeMap::new();
        for (id, block) in self.blocks.iter() {
            block_map.insert(id_string(&id), block.clone());
        }
        map.serialize_entry("blocks", &block_map)?;

        map.serialize_entry("currentCostume", &self.current_costume)?;
        map.serialize_entry("costumes", &self.costumes)?;

        map.serialize_entry("sound", &self.sounds)?;

        map.serialize_entry("layerOrder", &self.layer_order)?;

        map.serialize_entry("volume", &self.volume)?;

        serialize_entry_if_some(&mut map, "tempo", &self.tempo)?;
        serialize_entry_if_some(&mut map, "videoState", &self.video_state)?;
        serialize_entry_if_some(&mut map, "videoTransparency", &self.video_transparency)?;
        serialize_entry_if_some(&mut map, "textToSpeechLanguage", &self.text_to_speech_language)?;

        serialize_entry_if_some(&mut map, "visible", &self.visible)?;
        serialize_entry_if_some(&mut map, "x", &self.x)?;
        serialize_entry_if_some(&mut map, "y", &self.y)?;
        serialize_entry_if_some(&mut map, "size", &self.size)?;
        serialize_entry_if_some(&mut map, "direction", &self.direction)?;
        serialize_entry_if_some(&mut map, "draggable", &self.draggable)?;
        serialize_entry_if_some(&mut map, "rotationStyle", &self.rotation_style)?;


        map.end()
    }
}
