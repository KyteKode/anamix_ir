use id_arena::Arena;

use std::{collections::BTreeMap, path::PathBuf};

use anamix_ir::low_level::{
    asset::{LLAsset, LLCostume},
    project::LLProject,
    target::{LLTarget, LLVariable},
    monitor::{LLMonitor, MonitorData}
};

fn main() {
    let target = LLTarget {
        is_stage: false,

        name: String::from("Target"),

        variables: BTreeMap::new(),
        lists: BTreeMap::new(),
        broadcasts: BTreeMap::new(),

        blocks: Arena::new(),

        current_costume: 0,
        costumes: vec![LLCostume {
            asset_data: LLAsset {
                name: String::from("Anamix IR"),
                data_format: String::from("svg"),
                asset: PathBuf::from("examples/anamix_ir.svg"),
            },
            bitmap_resolution: Some(2.0),
            rotation_center_x: Some(55.35),
            rotation_center_y: Some(43.875),
        }],
        sounds: Vec::new(),

        layer_order: Some(1),

        volume: 100.0,

        tempo: None,
        video_state: None,
        video_transparency: None,
        text_to_speech_language: None,

        visible: Some(true),
        x: Some(24.0),
        y: Some(36.0),
        size: Some(100.0),
        direction: Some(90.0),
        draggable: Some(false),
        rotation_style: Some(String::from("all around")),
    };

    let stage = LLTarget {
        is_stage: true,

        name: String::from("Stage"),

        variables: BTreeMap::from([(
            String::from("v01"),
            LLVariable(
                String::from("stage var"),
                String::from("Hello, world!"),
                false,
            ),
        )]),
        lists: BTreeMap::new(),
        broadcasts: BTreeMap::new(),

        blocks: Arena::new(),

        current_costume: 0,
        costumes: vec![LLCostume {
            asset_data: LLAsset {
                name: String::from("Anamix IR"),
                data_format: String::from("svg"),
                asset: PathBuf::from("examples/anamix_ir.svg"),
            },
            bitmap_resolution: Some(2.0),
            rotation_center_x: Some(55.35),
            rotation_center_y: Some(43.875),
        }],
        sounds: Vec::new(),

        layer_order: Some(0),

        volume: 100.0,

        tempo: Some(100.0),
        video_state: Some(String::from("on")),
        video_transparency: Some(50.0),
        text_to_speech_language: Some(None),

        visible: None,
        x: None,
        y: None,
        size: None,
        direction: None,
        draggable: None,
        rotation_style: None,
    };

    let project = LLProject {
        targets: vec![stage, target],
        monitors: vec![LLMonitor {
            id: String::from("v01"),
            mode: String::from("large"),
            opcode: String::from("data_variable"),
            params: BTreeMap::from([
                (String::from("VARIABLE"), String::from("stage var"))
            ]),
            sprite_name: None,
            value: MonitorData::String(String::from("Hello, world!")),
            width: 0.0,
            height: 0.0,
            x: 240.0,
            y: 180.0,
            visible: true,
            slider_min: Some(0.0),
            slider_max: Some(0.0),
            is_discrete: Some(true)
        }],
        extensions: vec![String::from("pen")],
    };

    project.compile(String::from("LL Project Test")).unwrap();
}
