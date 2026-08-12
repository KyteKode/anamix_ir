use serde::{Serialize, Serializer, ser::SerializeMap};

use std::{
    collections::{HashMap, BTreeSet},
    fs::File,
    path::PathBuf
};
use crate::low_level::compile::CompileData;
use super::{
    target::LLTarget,
    monitor::LLMonitor
};

pub struct LLProject {
    pub targets: Vec<LLTarget>,
    pub monitors: Vec<LLMonitor>,
    pub extensions: Vec<String>,
}
impl Serialize for LLProject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("targets", &self.targets)?;
        map.serialize_entry("monitors", &self.monitors)?;
        map.serialize_entry("extensions", &self.extensions)?;
        map.serialize_entry("meta", &HashMap::from([
            ("semver", "3.0.0")
        ]))?;
        
        map.end()
    }
}

impl LLProject {
    pub(crate) fn get_files(&self) -> Vec<PathBuf> {
        let mut files: BTreeSet<PathBuf> = BTreeSet::new();

        for target in &self.targets {
            let target_files = target.get_files();
            for file in target_files {
                files.insert(file);
            }
        }

        files.into_iter().collect()
    }

    pub fn compile(&self, name: String) -> anyhow::Result<File> {
        let json = serde_json::to_string_pretty(&self)?;
        let compile_data = CompileData {
            json,
            name: name.clone(),
            sprite: false,
            paths: self.get_files(),
        };

        compile_data.compile()
    }
}