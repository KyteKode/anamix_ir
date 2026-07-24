use zip::{ZipWriter, write::SimpleFileOptions};
use anyhow::{Context, Result};

use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Write,
    collections::HashSet,
};

const ALLOWED_EXTENSIONS: [&str; 9] = [
    "png", "svg", "jpeg", "jpg", "bmp", "gif", "wav", "wave", "mp3",
];

pub(crate) struct CompileData {
    pub json: String,
    pub name: String,
    pub sprite: bool,
    pub paths: Vec<PathBuf>,
}

impl CompileData {
    pub fn compile(&self) -> Result<File> {
        let json_name = if self.sprite {
            "sprite.json"
        } else {
            "project.json"
        };

        let sb3 = File::create(self.name.clone())?;
        let mut sb3_writer = ZipWriter::new(sb3);

        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        sb3_writer.start_file(json_name, options)?;
        sb3_writer.write_all(self.json.as_bytes())?;

        let mut written: HashSet<String> = HashSet::new();

        for path in self.paths.iter() {
            let Some(os_ext) = path.extension() else {
                continue;
            };

            let ext = os_ext.to_string_lossy().to_lowercase();
            if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
                continue;
            }

            let file = fs::read(path)?;
            let md5 = md5::compute(&file);
            let md5_hex = hex::encode(md5.0);

            let asset_name = format!("{}.{}", md5_hex, ext);

            if !written.insert(asset_name.clone()) {
                continue;
            }

            sb3_writer.start_file(asset_name, options)?;
            sb3_writer.write_all(file.as_slice())?;
        }

        sb3_writer.finish().context("Failed to finish writing file")
    }
}
