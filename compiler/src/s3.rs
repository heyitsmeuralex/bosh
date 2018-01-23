// Scratch 3.0 pub struct definitions. The ST haven't actually documented
// this format anywhere and it's likely to change a bunch before release,
// but bosh compiles to it anyway.

use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Serialize)]
pub struct Project {
    pub targets: Vec<Target>,
    pub meta: ProjectMetadata,
}

js_serializable!(Project);

#[derive(Serialize)]
pub struct ProjectMetadata {
    pub semver: String,
    pub vm: String,
    pub agent: String,
}

js_serializable!(ProjectMetadata);

#[derive(Serialize)]
pub struct Target {
    pub id: String,
    pub name: String,
    pub isStage: bool,

    pub x: i32,
    pub y: i32,
    pub size: i32,

    pub direction: i32,
    pub rotationStyle: String,

    pub draggable: bool,
    pub visible: bool,

    pub blocks: HashMap<String, Block>,
    pub variables: HashMap<String, Variable>,
    pub lists: HashMap<String, List>,

    pub currentCostume: i32,
    pub costume: Costume,
    pub costumes: Vec<Costume>,
    pub costumeCount: i32,

    pub sounds: Vec<Sound>,
}

js_serializable!(Target);

#[derive(Serialize)]
pub struct Costume {
    pub name: String,
    pub assetId: String,

    pub dataFormat: String,
    pub bitmapResolution: i32,

    pub rotationCenterX: i32,
    pub rotationCenterY: i32,

    pub skinId: i32,
}

js_serializable!(Costume);

#[derive(Serialize)]
pub struct Sound {
    pub name: String,
    pub assetId: String,
    pub soundId: String,

    //data: null,
    pub dataFormat: String,
    pub bitmapResolution: i32,

    pub rotationCenterX: i32,
    pub rotationCenterY: i32,

    pub md5: String,
    pub soundID: i32,
}

js_serializable!(Sound);

pub struct Variable {
    pub id: String,
    pub name: String,
    pub value: f64,
}

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Variable", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        let _ = state.serialize_field("type", &"");
        state.end()
    }
}

js_serializable!(Variable);

// Lists and variables are essentially the same, however their 'type' field is "list"
// and their 'value' field is type Vec<64> rather than type f64.
pub struct List {
    pub id: String,
    pub name: String,
    pub value: Vec<f64>,
}

impl Serialize for List {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("List", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("value", &self.value)?;
        let _ = state.serialize_field("type", &"list");
        state.end()
    }
}

js_serializable!(List);

#[derive(Serialize)]
pub struct Block {
    pub id: String,
    pub opcode: String, // e.g. "event_whengreenflag"

    pub inputs: HashMap<String, BlockInput>, // inputs to this block and the blocks they point to
    pub fields: HashMap<String, BlockField>, // fields on this block and their values

    pub topLevel: bool, // if this block starts a stack
    pub next: Option<String>, // next block in the stack
    pub parent: Option<String>, // parent block id

    pub shadow: bool, // if this represents a shadow/slot (e.g. custom block var)

    pub x: i32, // if top-level
    pub y: i32, // if top-level
}

js_serializable!(Block);

#[derive(Serialize)]
pub struct BlockInput {
    pub name: String, // same as HashMap key
    pub block: String, // id
    pub shadow: bool,
}

js_serializable!(BlockInput);

// Fields are typically used for 'text' blocks (strings) to store their value.
#[derive(Serialize)]
pub struct BlockField {
    pub name: String, // same as HashMap key
    pub value: String, // XXX: is this *always* a string or can it be a num too?
}

js_serializable!(BlockField);
