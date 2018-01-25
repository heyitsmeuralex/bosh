// Compiled to WebAssembly for use by the playground.

extern crate pest;
#[macro_use] extern crate pest_derive;

extern crate serde;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate stdweb;

mod compiler;
use compiler::s3::*;

js_serializable!(Project);
js_serializable!(ProjectMetadata);
js_serializable!(Target);
js_serializable!(Costume);
js_serializable!(Sound);
js_serializable!(Variable);
js_serializable!(List);
js_serializable!(Block);
js_serializable!(BlockInput);
js_serializable!(BlockField);

/// JS-serializable form of std::result::Result. Should probably not exist.
#[derive(Serialize)]
pub enum CompileResult {
    /// Represents a successful compilation.
    Tree(Project),

    /// Represents a parse/compilation failure, with an error message.
    Fail(String),
}

js_serializable!(CompileResult);

/// Wraps compiler::compile and returns a js
fn compile_serializable(source: String) -> CompileResult {
    match compiler::compile(source) {
        Ok(project)  => CompileResult::Tree(project),
        Err(message) => CompileResult::Fail(message)
    }
}

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.compile = @{compile_serializable};
    }
}
