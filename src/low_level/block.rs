use either::Either;
use id_arena::Id;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

use std::collections::BTreeMap;

use super::helper::{id_string, serialize_entry_if_some, serialize_element_if_some};

#[derive(Clone, Debug)]
pub enum LLBlock {
    Object(LLObjectBlock),
    Array(LLArrayBlock),
}

impl Serialize for LLBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(match self {
            Self::Object(object) => object.serialize(serializer)?,
            Self::Array(array) => array.serialize(serializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct LLObjectBlock {
    pub opcode: String,
    pub next: Option<Id<LLBlock>>,
    pub parent: Option<Id<LLBlock>>,
    pub inputs: BTreeMap<String, LLInput>,
    pub fields: BTreeMap<String, LLField>,
    pub shadow: bool,
    pub top_level: bool,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub mutation: Option<BTreeMap<String, String>>,
}

impl Serialize for LLObjectBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("opcode", &self.opcode)?;

        let next: &Option<String> = &self.next.map(|id| id_string(&id));
        map.serialize_entry("next", next)?;

        let parent: &Option<String> = &self.parent.map(|id| id_string(&id));
        map.serialize_entry("parent", parent)?;

        map.serialize_entry("inputs", &self.inputs)?;
        map.serialize_entry("fields", &self.fields)?;

        map.serialize_entry("shadow", &self.shadow)?;
        map.serialize_entry("topLevel", &self.top_level)?;

        serialize_entry_if_some(&mut map, "x", &self.x)?;
        serialize_entry_if_some(&mut map, "y", &self.y)?;

        serialize_entry_if_some(&mut map, "mutations", &self.mutation)?;

        map.end()
    }
}

#[derive(Clone, Debug)]
pub struct LLArrayBlock(
    u8,
    Either<String, f64>,
    Option<String>,
    Option<f64>,
    Option<f64>,
);

impl Serialize for LLArrayBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.0)?;

        if let Either::Left(data) = &self.1 {
            seq.serialize_element(data)?;
        } else if let Either::Right(data) = &self.1 {
            seq.serialize_element(data)?;
        }

        serialize_element_if_some(&mut seq, &self.2)?;
        serialize_element_if_some(&mut seq, &self.3)?;
        serialize_element_if_some(&mut seq, &self.4)?;

        seq.end()
    }
}

#[derive(Clone, Debug)]
pub enum LLInputBlock {
    ID(Id<LLBlock>),
    Array(LLArrayBlock),
}

impl Serialize for LLInputBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(match self {
            Self::ID(id) => serializer.serialize_str(&id_string(id))?,
            Self::Array(array) => array.serialize(serializer)?,
        })
    }
}

#[derive(Clone, Debug)]
pub struct LLInput(u8, LLInputBlock, Option<LLInputBlock>);
impl Serialize for LLInput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.0)?;
        seq.serialize_element(&self.1)?;
        serialize_element_if_some(&mut seq, &self.2)?;

        seq.end()
    }
}

#[derive(Clone, Debug)]
pub struct LLField(String, Option<String>);
impl Serialize for LLField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;

        seq.serialize_element(&self.0)?;
        serialize_element_if_some(&mut seq, &self.1)?;

        seq.end()
    }
}


