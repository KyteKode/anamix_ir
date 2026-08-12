use super::block::LLBlock;

use either::Either;
use id_arena::Id;

use serde::{Serialize, Serializer};

pub(crate) fn id_string(id: &Id<LLBlock>) -> String {
    format!("b{:05}", id.index())
}

pub(crate) fn serialize_either<S>(
    either: &Either<String, f64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
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
