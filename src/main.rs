use serde_derive::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml::Value as Toml;



#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "show")]
    Show {
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,
        #[structopt(long = "core")]
        core: bool,
    },
    #[structopt(name = "up")]
    Up {
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,
        #[structopt(short = "x", long = "major")]
        major: bool,
        #[structopt(short = "y", long = "minor")]
        minor: bool,
        #[structopt(short = "z", long = "patch")]
        patch: bool,
        #[structopt(short = "p", long = "pre")]
        pre: Option<String>,
        #[structopt(short = "b", long = "build")]
        build: Option<String>,
        #[structopt(short = "r", long = "replace")]
        replace: bool,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Show { fpath, core } => {
            println!("show")
        }
        Opt::Up {
            fpath,
            major,
            minor,
            patch,
            pre,
            build,
            replace,
        } => {
            println!("up")
        }
        _ => {}
    }
}
