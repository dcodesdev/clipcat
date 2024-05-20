mod cli;
mod clip;
mod run;

use run::run;

fn main() -> anyhow::Result<()> {
    run()
}
