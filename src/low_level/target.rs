use id_arena::Arena;
use serde::{Serialize, Serializer};

use std::collections::BTreeMap;

use super::{
    asset::{LLCostume, LLSound},
    block::LLBlock,
    helper::id_string,
};

type Variable = (String, String, bool);
type List = (String, Vec<String>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LLTarget {
    pub is_stage: bool,

    pub name: String,

    pub variables: BTreeMap<String, Variable>,
    pub lists: BTreeMap<String, List>,
    pub broadcasts: BTreeMap<String, String>,

    #[serde(serialize_with = "serialize_blocks")]
    pub blocks: Arena<LLBlock>,

    pub current_costume: usize,
    pub costumes: Vec<LLCostume>,

    pub sounds: Vec<LLSound>,

    pub layer_order: usize,

    pub volume: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tempo: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_transparency: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_to_speech_language: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draggable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_style: Option<String>,
}

fn serialize_blocks<S>(blocks: &Arena<LLBlock>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ser_blocks: BTreeMap<String, LLBlock> = blocks
        .iter()
        .map(|(id, block)| (id_string(&id), block.clone()))
        .collect();

    ser_blocks.serialize(serializer)
}
