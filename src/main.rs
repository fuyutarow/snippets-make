use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod lib;
use lib::Config;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "gen")]
    Gen {
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,
        #[structopt(short = "-f", long = "--format")]
        format: Option<String>,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::Gen { fpath, format } => match &format {
            Some(s) if s != "vscode" => {}
            _ => {
                let config = Config::from_fpath(fpath);

                let res = config.to_json();
                println!("{}", res);
            }
        },
        _ => {}
    }
}
