#![allow(non_snake_case)]

pub mod s3;
use self::s3::{CompileResult, Project, Target, Costume, ProjectMetadata};
use pest::{Position, Parser};
use pest::iterators::Pair;
use std::collections::HashMap;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../grammar.pest");

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct Grammar;

fn fmt_pos(pos: Position) -> String {
    let (line, col) = pos.line_col();
    format!("line {} col {}", line, col)
}

fn error_at_pos(pos: Position, msg: &'static str) -> String {
    msg.to_owned() + " at " + &fmt_pos(pos)
}

fn error_at(rule_pair: Pair<Rule>, msg: &'static str) -> String {
    error_at_pos(rule_pair.clone().into_span().start_pos(), msg)
}

/// Takes bosh sourcecode and attempts to parse it into a Project.
pub fn compile(source: String) -> CompileResult { // TODO: use Result and make CompileResult JS-specific
    let parse_result = Grammar::parse(Rule::file, &source);
    let mut project = Project {
        targets: Vec::new(),
        meta: ProjectMetadata {
            semver: "3.0.0".to_string(),
            vm: "0.1.0".to_string(),
            agent: "bosh (https://github.com/heyitsmeuralex/bosh)".to_string()
        }
    };

    match parse_result.clone().err() {
        Some(err) => return CompileResult::Fail(format!("{}", err)),

        None => {
            // Parse success
            for declaration in parse_result.unwrap() {
                let mut pairs = declaration.clone().into_inner();
                let declaration_type = pairs.nth(0); // consumes pairs.0

                if declaration_type.is_none() {
                    // No declaration type given (first arg) ie. ()
                    return CompileResult::Fail(error_at(declaration.clone(), "Illegal empty list, expected declaration"));
                }

                match declaration_type.unwrap().into_span().as_str() {
                    "sprite" | "stage" => {
                        // TODO
                        return CompileResult::Fail("Not yet implemented".to_string());
                    },
                    _ => return CompileResult::Fail(error_at(declaration.clone(), "Expected declaration (eg. `sprite` call)"))
                }

                /*
                for pair in expr.into_inner() {
                    match pair.as_rule() {
                        Rule::ident  => log(format!("ident {:?}", pair.clone().into_span().as_str())),
                        Rule::float  => log("float".to_string()),
                        Rule::int    => log("int".to_string()),
                        Rule::string => log("string".to_string()),
                        Rule::list   => log("list".to_string()),
                        _ => unreachable!()
                    }
                }
                */
            }
        },
    }

    /*
    CompileResult::Tree(Project {
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
    })
    */

    CompileResult::Tree(project)
}

// TODO: tests
