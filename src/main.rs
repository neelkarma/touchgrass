use context::Context;

use crate::{
    formatter::{default::DefaultFormatter, json::JSONFormatter, Formatter},
    provider::{owm::OWMProvider, Provider},
};

mod args;
mod config;
mod context;
mod formatter;
mod provider;

fn main() {
    let ctx = Context::build();
    let weather = match ctx.provider {
        Provider::OpenWeatherMap => OWMProvider::get(&ctx),
    };

    let weather = match weather {
        Ok(weather) => weather,
        Err(error) => {
            println!("{:?}", error);
            panic!();
        }
    };

    let formatted = match ctx.formatter {
        Formatter::Default => DefaultFormatter::format(&ctx, &weather),
        Formatter::JSON => JSONFormatter::format(&ctx, &weather),
    };

    println!("{}", formatted);
}
