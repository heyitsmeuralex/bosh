//! CLI interface to **bosh_compiler**.

extern crate bosh_compiler as bosh;

extern crate structopt;
#[macro_use] extern crate structopt_derive;

extern crate serde_json;

use std::process;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;
use bosh::compiler::s3::CompileResult::{Tree, Fail};

#[derive(StructOpt, Debug)]
#[structopt(name = "bosh", about = "Compiler for bosh, a Lisp that compiles to Scratch 3.0 JSON")]
struct Opt {
    #[structopt(help = "Input file")]
    input: String,
}

fn main() {
    let opt = Opt::from_args();

    // Load the input file
    let mut f = File::open(opt.input);
    if let Err(err) = f {
        println!("{}", err);
        process::exit(1);
    }

    // Read its contents
    let mut f_content = String::new();
    if let Err(err) = f.unwrap().read_to_string(&mut f_content) {
        println!("{}", err);
        process::exit(1);
    }

    // Compile the source
    match bosh::compiler::compile(f_content) {
        Tree(project) => println!("{}", serde_json::to_string(&project).unwrap()),
        Fail(error_msg) => {
            println!("{}", error_msg);
            process::exit(1);
        }
    }
}
