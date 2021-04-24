use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
struct RawDotSnippet {
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawConfig {
    lang: String,
    snippets: HashMap<String, RawDotSnippet>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    lang: String,
    snippets: HashMap<String, DotSnippet>,
}

impl From<RawConfig> for Config {
    fn from(raw_config: RawConfig) -> Self {
        Self {
            lang: raw_config.lang,
            snippets: raw_config
                .snippets
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
                .collect(),
        }
    }
}

impl Config {
    pub fn from_fpath(fpath: PathBuf) -> Self {
        let mut f = File::open(fpath).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        Self::from_toml(&contents)
    }
    fn from_toml(toml_str: &str) -> Self {
        let rawConfig = toml::from_str::<RawConfig>(toml_str).expect("failed to parse config file");
        Self::from(rawConfig)
    }
    pub fn to_json(&self) -> String {
        let vscode_snippet_map: HashMap<String, VSCodeSnippet> = self
            .clone()
            .snippets
            .into_iter()
            .map(|(k, v)| (k.clone(), VSCodeSnippet::from(v)))
            .collect();

        serde_json::to_string(&vscode_snippet_map).expect("failed to format config to json")
    }
}
