mod cli;
mod clip;
mod fs;
mod run;
mod tiktoken;

use run::run;

fn main() -> anyhow::Result<()> {
    run()
}
