use std::path::PathBuf;

use super::target::LLTarget;

pub struct LLProject {
    pub targets: Vec<LLTarget>,
    pub monitors: Vec<()>,
    pub extensions: Vec<String>,
    pub files: Vec<PathBuf>
}
