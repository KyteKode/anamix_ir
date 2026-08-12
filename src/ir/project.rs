use indexmap::IndexMap;

use super::{
    extension::Extension,
    target::{Sprite, Stage},
};

pub struct Project {
    pub name: String,
    pub stage: Stage,
    pub sprites: IndexMap<String, Sprite>,
    pub extensions: Vec<Extension>,
}
