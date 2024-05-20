mod cli;
mod clip;
mod fs;
mod run;

use run::run;

fn main() -> anyhow::Result<()> {
    run()
}
