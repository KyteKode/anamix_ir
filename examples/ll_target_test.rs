use either::Either;
use id_arena::Arena;

use std::{
    collections::BTreeMap,
    path::PathBuf,
};

use anamix_ir::low_level::{
    block::{LLArrayBlock, LLBlock, LLInput, LLInputBlock, LLObjectBlock, LLField},
    target::LLTarget,
    asset::{LLAsset, LLCostume}
};

fn main() {
    let mut blocks: Arena<LLBlock> = Arena::new();

    let green_flag = blocks.alloc(LLBlock::Object(LLObjectBlock {
        opcode: String::from("event_whenflagclicked"),

        next: None,
        parent: None,

        inputs: BTreeMap::new(),
        fields: BTreeMap::new(),

        shadow: false,
        top_level: true,

        x: Some(0.0),
        y: Some(0.0),

        mutation: None,
    }));

    let broadcast = blocks.alloc(LLBlock::Object(LLObjectBlock {
        opcode: String::from("event_broadcast"),

        next: None,
        parent: Some(green_flag),

        inputs: BTreeMap::from([(
            String::from("BROADCAST_INPUT"),
            LLInput(
                1,
                LLInputBlock::Array(LLArrayBlock(
                    11,
                    Either::Left(String::from("broadcast0")),
                    Some(String::from("b0")),
                    None,
                    None,
                )),
                None,
            ),
        )]),
        fields: BTreeMap::new(),

        shadow: false,
        top_level: false,

        x: None,
        y: None,

        mutation: None,
    }));

    if let LLBlock::Object(obj) = &mut blocks[green_flag] {
        obj.next = Some(broadcast);
    }

    let when_recieved = blocks.alloc(LLBlock::Object(LLObjectBlock {
        opcode: String::from("event_whenbroadcastreceived"),

        next: None,
        parent: None,

        inputs: BTreeMap::from([(
            String::from("BROADCAST_INPUT"),
            LLInput(
                1,
                LLInputBlock::Array(LLArrayBlock(
                    11,
                    Either::Left(String::from("broadcast0")),
                    Some(String::from("b0")),
                    None,
                    None,
                )),
                None,
            ),
        )]),
        fields: BTreeMap::new(),

        shadow: false,
        top_level: true,

        x: Some(100.0),
        y: Some(0.0),

        mutation: None,
    }));

    let say = blocks.alloc(LLBlock::Object(LLObjectBlock {
        opcode: String::from("looks_say"),

        next: None,
        parent: Some(when_recieved),

        inputs: BTreeMap::from([(
            String::from("MESSAGE"),
            LLInput(
                3,
                LLInputBlock::Array(LLArrayBlock(
                    12,
                    Either::Left(String::from("var0 string")),
                    Some(String::from("v0")),
                    None,
                    None,
                )),
                Some(LLInputBlock::Array(LLArrayBlock(
                    8,
                    Either::Left(String::from("90")),
                    None,
                    None,
                    None,
                ))),
            ),
        )]),
        fields: BTreeMap::new(),

        shadow: false,
        top_level: false,

        x: None,
        y: None,

        mutation: None,
    }));

    if let LLBlock::Object(obj) = &mut blocks[when_recieved] {
        obj.next = Some(say);
    }

    let stop = blocks.alloc(LLBlock::Object(LLObjectBlock {
        opcode: String::from("control_stop"),

        next: None,
        parent: Some(say),

        inputs: BTreeMap::new(),
        fields: BTreeMap::from([
            (
                String::from("STOP_OPTION"),
                LLField(
                    String::from("other scripts in sprite"),
                    None
                )
            )
        ]),

        shadow: false,
        top_level: false,

        x: None,
        y: None,

        mutation: None,
    }));

    if let LLBlock::Object(obj) = &mut blocks[say] {
        obj.next = Some(stop);
    }





    let target = LLTarget {
        is_stage: false,

        name: String::from("LLTargetTest"),

        variables: BTreeMap::new(),/*BTreeMap::from([
            ("v0", ("var0 string", "Hello, world!", false)),
            ("v1", ("var1 number", "32768", true)),
            ("v2", ("var2 bool", "true", true)),
        ])
        .into_iter()
        .map(|(key, (name, value, cloud))| {
            (
                key.to_string(),
                (name.to_string(), value.to_string(), cloud),
            )
        })
        .collect(),*/

        lists: BTreeMap::new(), /*BTreeMap::from([
            ("l0", ("list0", vec!["Hello,", "world!"])),
            ("l1", ("list1", vec!["1", "2", "3", "4", "5"])),
        ])
        .into_iter()
        .map(|(key, (name, values))| {
            (
                key.to_string(),
                (
                    name.to_string(),
                    values.into_iter().map(|v| v.to_string()).collect(),
                ),
            )
        })
        .collect(),*/

        broadcasts: BTreeMap::new(), /*BTreeMap::from([("b0", "broadcast0"), ("l1", "broadcast1")])
            .into_iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect(),*/

        blocks,

        current_costume: 0,
        costumes: vec![
            LLCostume {
                asset_data: LLAsset {
                    name: String::from("Anamix IR"),
                    data_format: String::from("svg"),
                    asset: PathBuf::from("examples/anamix_ir.svg")
                },
                bitmap_resolution: Some(2.0),
                rotation_center_x: Some(55.35),
                rotation_center_y: Some(43.875)
            }
        ],
        sounds: Vec::new(),

        layer_order: None,

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
        rotation_style: Some(String::from("all around"))
    };

    target.compile(String::from("LL Target Test")).unwrap();
}
