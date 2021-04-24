use std::fs;
use std::io::Write;
use std::path::PathBuf;
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
        #[structopt(long)]
        over: Option<String>,
    },
}

fn get_vscode_path() -> Option<PathBuf> {
    match os_info::get().os_type() {
        os_info::Type::Macos => {
            // Some("~/Library/Application Support/Code/User/snippets/typescript.json")
            if let Some(config_path) = dirs::config_dir() {
                Some(config_path.join("Code/User/snippets/typescript.json"))
                // config_path
            } else {
                None
            }
        }
        _ => None,
    }
}

fn main() {
    match Opt::from_args() {
        Opt::Gen {
            fpath,
            format,
            over,
        } => match &format {
            Some(s) if s != "vscode" => {
                println!("vscode is only supported, currently");
            }
            _ => {
                let config = Config::from_fpath(fpath);

                let res = config.to_json();

                match &over {
                    Some(s) if s == "vscode" => match get_vscode_path() {
                        Some(vscode_path) => {
                            let a = &vscode_path;
                            let mut f =
                                fs::File::create(&vscode_path).expect("failed to create file");
                            f.write_all(res.as_bytes()).expect("failed to write file");
                            println!(
                                "{} was overwritten",
                                vscode_path.into_os_string().into_string().expect("")
                            );
                        }
                        _ => {
                            println!("this os is not suppored");
                        }
                    },
                    _ => {
                        println!("{}", res);
                    }
                }
            }
        },
        _ => {}
    }
}
