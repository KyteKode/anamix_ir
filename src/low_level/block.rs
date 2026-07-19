use either::Either;
use id_arena::Id;
use serde::{Serialize, Serializer};

use super::helper::id_string;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum LLBlock {
    Object(LLObjectBlock),
    Array(LLArrayBlock),
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LLObjectBlock {
    pub opcode: String,

    #[serde(serialize_with = "serialize_optional_id")]
    pub next: Option<Id<LLBlock>>,

    #[serde(serialize_with = "serialize_optional_id")]
    pub parent: Option<Id<LLBlock>>,

    pub inputs: BTreeMap<String, LLInput>,
    pub fields: BTreeMap<String, LLField>,

    pub shadow: bool,
    pub top_level: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutation: Option<BTreeMap<String, String>>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LLArrayBlock(
    u8,
    #[serde(serialize_with = "serialize_either")] Either<String, f64>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")] Option<f64>,
);

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum LLInputBlock {
    #[serde(serialize_with = "serialize_id")]
    ID(Id<LLBlock>),

    Array(LLArrayBlock),
}

#[derive(Clone, Debug, Serialize)]
pub struct LLInput(
    u8,
    LLInputBlock,
    #[serde(skip_serializing_if = "Option::is_none")] Option<LLInputBlock>,
);

#[derive(Clone, Debug, Serialize)]
pub struct LLField(
    String,
    #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
);

fn serialize_id<S>(id: &Id<LLBlock>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    id_string(&id).serialize(serializer)
}

fn serialize_optional_id<S>(id: &Option<Id<LLBlock>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    id.map(|id| id_string(&id)).serialize(serializer)
}

fn serialize_either<S>(either: &Either<String, f64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Either::Left(data) = either {
        return data.serialize(serializer);
    } else if let Either::Right(data) = either {
        return data.serialize(serializer);
    }
    unreachable!();
}
