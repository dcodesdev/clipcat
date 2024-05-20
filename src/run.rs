use clap::Parser;

use crate::{cli, clip::copy_to_clipboard, fs::read_directory_contents};

pub fn run() -> anyhow::Result<()> {
    let opts = cli::Opts::parse();
    let path = opts.path;

    let contents = read_directory_contents(&path)?;
    copy_to_clipboard(&contents)?;

    Ok(())
}
