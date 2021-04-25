use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::Config;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input toml file
    #[structopt(parse(from_os_str,))]
    fpath: PathBuf,

    /// Snippet format of output. [possible values: vscode, neosnippet, ultisnips]
    #[structopt(short = "-t", long = "--to", default_value = "vscode")]
    to: String,

    /// Overwrite config of editor. [vscode, vscode-insiders]
    #[structopt(long)]
    over: Option<String>,
    //
    // #[structopt(short, long)]
    // out: Option<PathBuf>,
}

fn main() {
    match Opt::from_args() {
        Opt { fpath, to, over } => {
            let config = Config::from_fpath(fpath);

            match to.as_str() {
                "vscode" => {
                    if let Some(editor) = over {
                        let result = config.write_vscode(&editor);
                        if let Ok(vscode_path) = result {
                            println!(
                                "{} was overwritten",
                                vscode_path.into_os_string().into_string().expect("")
                            );
                        } else {
                            println!("{} is not a suppored editor. [possible values: vscode, vscode-insiders]", editor);
                        }
                    } else {
                        config.print_vscode();
                    }
                }
                "neosnippet" => config.print_neosnippet(),
                "ultisnips" => config.print_ultisnps(),
                _ => {
                    println!(
                        "Snippet format of output. [possible values: vscode, neosnippet, ultisnips]");
                }
            }
        }
    }
}
