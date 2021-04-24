use std::fs;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::Config;

#[derive(StructOpt, Debug)]
struct Opt {
    /// input toml file
    #[structopt(parse(from_os_str,))]
    fpath: PathBuf,

    /// snippet format of output. [vscode]
    #[structopt(short = "-f", long = "--format", default_value = "vscode")]
    format: String,

    /// overwrite config of editor. [vscode, vscode-insiders]
    #[structopt(long)]
    over: Option<String>,
    //
    // #[structopt(short, long)]
    // out: Option<PathBuf>,
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

fn main() {
    match Opt::from_args() {
        Opt {
            fpath,
            format,
            // out,
            over,
        } => match format.as_str() {
            "vscode" => {
                let config = Config::from_fpath(fpath);

                let res = config.to_json();

                match &over {
                    Some(editor) if editor == "vscode" || editor == "vscode-insiders" => {
                        match get_vscode_path(&editor) {
                            Some(vscode_path) => {
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
                        }
                    }
                    Some(editor) => {
                        println!(
                            "{} is not a supported editor. Only [vscode, vscode-insiders]",
                            editor
                        );
                    }
                    _ => {
                        println!("{}", res);
                    }
                }
            }
            _ => {
                println!("vscode is only supported, currently");
            }
        },
        _ => {}
    }
}
