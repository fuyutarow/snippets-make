use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

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
        let raw_config =
            toml::from_str::<RawConfig>(toml_str).expect("failed to parse config file");
        Self::from(raw_config)
    }

    pub fn to_vscode(&self) -> String {
        let vscode_snippet_map: HashMap<String, VSCodeSnippet> = self
            .clone()
            .snippets
            .into_iter()
            .map(|(k, v)| (k.clone(), VSCodeSnippet::from(v)))
            .collect();

        serde_json::to_string(&vscode_snippet_map).expect("failed to format config to json")
    }

    pub fn print_vscode(&self) {
        println!("{}", self.to_vscode());
    }

    pub fn write_vscode(&self, editor: &str) -> anyhow::Result<PathBuf> {
        match get_vscode_path(editor) {
            Some(vscode_path) => {
                let mut f = fs::File::create(&vscode_path).expect("failed to create file");
                f.write_all(self.to_vscode().as_bytes())
                    .expect("failed to write file");
                Ok(vscode_path)
                // );
            }
            _ => Err(anyhow::anyhow!("")),
        }
    }

    pub fn print_neosnippet(&self) {
        for (name, dot_snippet) in self.clone().snippets.into_iter() {
            println!("snippet {}", name);
            for line in dot_snippet.body.lines() {
                println!("    {}", line);
            }
            println!("");
        }
    }

    pub fn print_ultisnps(&self) {
        for (name, dot_snippet) in self.clone().snippets.into_iter() {
            println!("snippet {}", name);
            for line in dot_snippet.body.lines() {
                println!("{}", line);
            }
            println!("endsnippet");
            println!("");
        }
    }
}

fn get_vscode_path(vscode: &str) -> Option<PathBuf> {
    if let Some(config_path) = dirs::config_dir() {
        match os_info::get().os_type() {
            os_info::Type::Macos if vscode == "vscode" => {
                Some(config_path.join("Code/User/snippets/typescript.json"))
            }
            os_info::Type::Macos if vscode == "vscode-insiders" => {
                Some(config_path.join("Code - Insiders/User/snippets/typescript.json"))
            }
            _ => None,
        }
    } else {
        None
    }
}
