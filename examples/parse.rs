use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawDotSnippet {
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct DotSnippet {
    prefix: String,
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct VSCodeSnippet {
    prefix: String,
    body: Vec<String>,
}

impl From<DotSnippet> for VSCodeSnippet {
    fn from(dot_snippet: DotSnippet) -> Self {
        Self {
            prefix: dot_snippet.prefix,
            body: dot_snippet
                .body
                .lines()
                .map(String::from)
                .collect::<Vec<_>>(),
        }
    }
}

fn main() {
    let fpath = "examples/typescript.toml";
    let mut f = File::open(fpath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let dotsnippet_map: HashMap<String, DotSnippet> =
        toml::from_str::<HashMap<String, RawDotSnippet>>(&contents)
            .expect("Failed to parse")
            .into_iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    DotSnippet {
                        prefix: k,
                        body: v.body,
                    },
                )
            })
            .collect();

    let vscode_snippet_map: HashMap<String, VSCodeSnippet> = dotsnippet_map
        .into_iter()
        .map(|(k, v)| (k.clone(), VSCodeSnippet::from(v)))
        .collect();

    let json = serde_json::to_string(&vscode_snippet_map).expect("failed to format it to json");

    println!("{}", json);
}
