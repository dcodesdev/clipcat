mod cli;
mod clip;
mod fs;
mod num;
mod run;
mod tiktoken;

use run::run;

fn main() -> anyhow::Result<()> {
    run()
}
