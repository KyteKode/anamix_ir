use id_arena::Id;
use serde::{
    Serialize,
    ser::{SerializeMap, SerializeSeq},
};

use super::block::LLBlock;

pub(crate) fn id_string(id: &Id<LLBlock>) -> String {
    format!("b{:05}", id.index())
}

pub(crate) fn serialize_element_if_some<S, T>(seq: &mut S, option: &Option<T>) -> Result<(), S::Error>
where
    S: SerializeSeq,
    T: Serialize,
{
    if let Some(data) = option {
        seq.serialize_element(data)?;
    }
    Ok(())
}

pub(crate) fn serialize_entry_if_some<S, T>(
    map: &mut S,
    key: &str,
    option: &Option<T>,
) -> Result<(), S::Error>
where
    S: SerializeMap,
    T: Serialize,
{
    if let Some(data) = option {
        map.serialize_entry(key, data)?;
    }
    Ok(())
}
