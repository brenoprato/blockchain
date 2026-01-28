mod block;
mod blockchain;
mod cli;

use cli::Cli;

fn main() -> anyhow::Result<()> {
    let mut cli = Cli::new()?;
    cli.run()?;
    Ok(())
}
