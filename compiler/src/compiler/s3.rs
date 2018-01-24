//! Direct representation of a Scratch 3.0 project using structs. This file format is
//! currently undocumented within the Scratch project itself so the structs here
//! may not be exhaustive.
//!
//! All structs implement **serde::Serialize**.

use std::collections::HashMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Serialize)]
pub struct Project {
    /// Vector of the stage and the project's sprites, if any.
    pub targets: Vec<Target>,

    /// Compile-time metadata such as Scratch version and user agent.
    pub meta: ProjectMetadata,
}

#[derive(Serialize)]
pub struct ProjectMetadata {
    /// Latest Scratch version when this project was compiled. Latest is 3.0.0.
    pub semver: String,

    /// scratch-vm version this project was compiled for. Latest is 0.1.0.
    pub vm: String,

    /// User-Agent string.
    pub agent: String,
}

/// A Target represents a Scratch 3.0 sprite or stage.
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

pub struct Variable {
    pub id: String,
    pub name: String,
    pub value: f64, // XXX: do variables sometimes contain Strings?
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

/// Lists and variables are essentially the same, however their 'type' field is "list"
/// and their 'value' field is type Vec<64> rather than type f64.
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

#[derive(Serialize)]
pub struct Block {
    /// Unique ID of this block. Can be anything unique.
    pub id: String,

    /// Scratch 3.0 block opcode, eg. "event_whengreenflag".
    pub opcode: String,

    pub inputs: HashMap<String, BlockInput>,
    pub fields: HashMap<String, BlockField>,

    /// If this block starts a stack, ie. it's a hat block
    pub topLevel: bool,

    /// Next block in the stack, if any
    pub next: Option<String>,

    /// Parent block ID if nested (such as within an if).
    pub parent: Option<String>,

    /// If this block represents a shadow/slot (such as a custom block argument)
    pub shadow: bool,

    pub x: i32,
    pub y: i32,
}

#[derive(Serialize)]
pub struct BlockInput {
     // Should be the same as the BlockInput's parent HashMap key.
    pub name: String,

    /// The ID of the block in this sprite's scripts HashMap. Essentially a pointer.
    pub block: String,

    pub shadow: bool,
}

/// Fields are typically used for static arguments to blocks, such as text inputs
/// and number inputs.
#[derive(Serialize)]
pub struct BlockField {
    pub name: String,
    pub value: String,
}

// TODO: move this to main.rs and return a Result from compile()
#[derive(Serialize)]
pub enum CompileResult {
    /// Represents a successful compilation.
    Tree(Project),

    /// Represents a parse/compilation failure, with an error message.
    Fail(String),
}
