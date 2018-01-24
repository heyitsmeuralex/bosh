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
js_serializable!(CompileResult);

fn main() {
    stdweb::initialize();

    js! {
        Module.exports.compile = @{compiler::compile};
    }
}
