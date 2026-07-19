use serde::{
    Serialize, Serializer,
    ser::SerializeMap
};

use std::{path::PathBuf, fs};

use super::helper::serialize_entry_if_some;

#[derive(Clone, Debug)]
pub struct LLCostume {
    pub name: String,
    pub data_format: String,
    pub asset: PathBuf,

    pub bitmap_resolution: Option<f64>,
    pub rotation_center_x: Option<f64>,
    pub rotation_center_y: Option<f64>,
}

impl Serialize for LLCostume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("dataFormat", &self.data_format)?;

        let asset_data: Vec<u8> = fs::read(&self.asset).map_err(serde::ser::Error::custom)?;
        let md5 = md5::compute(asset_data);
        let md5_hex = hex::encode(md5.0);

        map.serialize_entry("assetId", &md5_hex)?;
        map.serialize_entry("md5ext", &format!("{}.{}", md5_hex, self.data_format))?;

        serialize_entry_if_some(&mut map, "bitmapResolution", &self.bitmap_resolution)?;
        serialize_entry_if_some(&mut map, "rotationCenterX", &self.rotation_center_x)?;
        serialize_entry_if_some(&mut map, "rotationCenterY", &self.rotation_center_y)?;

        map.end()
    }
}

#[derive(Clone, Debug)]
pub struct LLSound {
    pub name: String,
    pub data_format: String,
    pub asset: PathBuf,

    pub rate: f64,
    pub sample_count: f64,
}

impl Serialize for LLSound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("dataFormat", &self.data_format)?;

        let asset_data: Vec<u8> = fs::read(&self.asset).map_err(serde::ser::Error::custom)?;
        let md5 = md5::compute(asset_data);
        let md5_hex = hex::encode(md5.0);

        map.serialize_entry("assetId", &md5_hex)?;
        map.serialize_entry("md5ext", &format!("{}.{}", md5_hex, self.data_format))?;

        map.serialize_entry("rate", &self.rate)?;
        map.serialize_entry("sampleCount", &self.sample_count)?;

        map.end()
    }
}