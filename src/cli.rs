use clap::Parser;

/// CLI options
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[clap(default_value = "./")]
    pub path: Vec<String>,

    #[clap(short, long)]
    pub token: bool,

    #[clap(short)]
    pub print: bool,
}
