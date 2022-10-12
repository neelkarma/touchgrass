use colored::Colorize;

use crate::{context::Context, provider::Weather};

use super::Formatter;

#[derive(Debug)]
pub struct DefaultFormatter;

impl Formatter for DefaultFormatter {
    fn format(&self, ctx: &Context, weather: &Weather) -> String {
        [
            format!(
                "{} {}",
                weather.location.bold(),
                weather.condition.color(weather.condition_color)
            ),
            format!(
                "  Temperature ({}/actual/{}/{}):    {}/{}/{}/{} {}",
                "feel".bold(),
                "min".cyan(),
                "max".red(),
                weather.feels_like.to_string().bold(),
                weather.temp.to_string(),
                weather.min_temp.to_string().cyan(),
                weather.max_temp.to_string().red(),
                ctx.units.temp()
            ),
            format!(
                "  Precipitation ({}):                   {} {}",
                "1h".blue(),
                weather.precipitation.to_string().blue(),
                "mm".blue()
            ),
            format!(
                "  Wind ({}/{}):                    {}/{} {}",
                "speed".green(),
                "gust".purple(),
                weather.wind_speed.to_string().green(),
                weather.wind_gust.to_string().purple(),
                ctx.units.speed()
            ),
        ]
        .join("\n")
    }
}
