use serde::{Serialize, Serializer, ser::SerializeMap};

use std::{fs, path::PathBuf};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LLCostume {
    #[serde(flatten)]
    pub asset_data: LLAsset,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitmap_resolution: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_center_x: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_center_y: Option<f64>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LLSound {
    #[serde(flatten)]
    pub asset_data: LLAsset,

    pub rate: f64,
    pub sample_count: f64,
}

#[derive(Clone, Debug)]
pub struct LLAsset {
    pub name: String,
    pub data_format: String,
    pub asset: PathBuf,
}

impl Serialize for LLAsset {
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

        map.end()
    }
}
