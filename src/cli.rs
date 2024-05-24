use clap::Parser;

/// CLI options
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[clap(default_value = "./")]
    pub path: String,

    #[clap(short, long)]
    pub token: bool,
}
