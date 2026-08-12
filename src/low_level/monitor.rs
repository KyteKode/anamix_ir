use serde::Serialize;

use std::collections::BTreeMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LLMonitor {
    pub id: String,
    pub mode: String,
    pub opcode: String,
    pub params: BTreeMap<String, String>,
    pub sprite_name: Option<String>,
    pub value: MonitorData,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub visible: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slider_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slider_max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_discrete: Option<bool>,
}


#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum MonitorData {
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<MonitorData>)
}