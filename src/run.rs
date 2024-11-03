use clap::Parser;
use std::path::Path;

use crate::{
    cli, clip::copy_to_clipboard, fs::read_contents, num::format_number, tiktoken::count_tokens,
};

pub fn run() -> anyhow::Result<()> {
    let opts = cli::Opts::parse();
    let (files, token) = (opts.path, opts.token);

    let mut contents = String::new();
    for path in files.iter() {
        let path = Path::new(&path);
        let content = read_contents(&path)?;
        contents.push_str(&content);
    }

    copy_to_clipboard(&contents)?;

    let char_count = format_number(contents.chars().count() as f64);

    println!("✅ {} characters copied to clipboard.", char_count);

    if token {
        let tokens = count_tokens(&contents)?;
        println!("{} GPT-4 tokens.", tokens);
    }

    if opts.print {
        println!("{}", contents);
    }

    Ok(())
}
