use anyhow::Result;
use context::Context;

mod args;
mod config;
mod context;
mod formatter;
mod provider;

fn main() -> Result<()> {
    let ctx = Context::build()?;
    let weather = ctx.provider.into_provider().get(&ctx)?;
    let formatted = ctx.formatter.into_formatter().format(&ctx, &weather);
    println!("{}", formatted);

    Ok(())
}
