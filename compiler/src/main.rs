#![allow(non_snake_case)]

#[macro_use] extern crate stdweb;

extern crate pest;
#[macro_use] extern crate pest_derive;

extern crate serde;
#[macro_use] extern crate serde_derive;

mod s3;
use s3::{Project, Target, Costume, ProjectMetadata};
use pest::Parser;
use std::collections::HashMap;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

fn log(s: String) {
    js! {
        console.log(@{s})
    }
}

fn compile(source: String) -> Project {
    let pairs = Grammar::parse(Rule::file, &source).unwrap();

    for pair in pairs {
        log(format!("Rule:    {:?}", pair.as_rule()));
        log(format!("Span:    {:?}", pair.clone().into_span()));
        log(format!("Text:    {}", pair.clone().into_span().as_str()));
    }

    Project {
        targets: vec![
            Target {
                id: "ye xd".to_string(),
                name: "Stage".to_string(),
                isStage: true,
                x: 0,
                y: 0,
                size: 100,
                direction: 90,
                draggable: false,

                currentCostume: 0,
                costume: Costume {
                    name: "backdrop1".to_string(),
                    bitmapResolution: 1,
                    rotationCenterX: 240,
                    rotationCenterY: 180,
                    skinId: 2,
                    dataFormat: "svg".to_string(),
                    //assetId: "739b5e2a2435f6e1ec2993791b423146".to_string()
                    assetId: "3696356a03a8d938318876a593572843".to_string()
                },
                costumes: vec![
                    Costume {
                        name: "backdrop1".to_string(),
                        bitmapResolution: 1,
                        rotationCenterX: 240,
                        rotationCenterY: 180,
                        skinId: 2,
                        dataFormat: "svg".to_string(),
                        //assetId: "739b5e2a2435f6e1ec2993791b423146".to_string()
                        assetId: "3696356a03a8d938318876a593572843".to_string()
                    }
                ],
                costumeCount: 1,

                visible: true,
                rotationStyle: "all around".to_string(),

                variables: HashMap::new(),
                lists: HashMap::new(),

                sounds: vec![],

                blocks: HashMap::new(),
            },
        ],
        meta: ProjectMetadata {
            semver: String::from("3.0.0"),
            vm: String::from("0.1.0"),
            agent: String::from("bosh (https://github.com/heyitsmeuralex/bosh)")
        }
    }
}

// exports the compile() function for use in JS
fn main() {
    stdweb::initialize();

    js! {
        Module.exports.compile = @{compile};
    }
}

#[test]
fn it_works() {
    // TODO
}
