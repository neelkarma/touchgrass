use clap::Parser;

use crate::context::Units;

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Args {
    #[clap(
        value_parser,
        help = "Location to get the weather for. Use a city name or comma-separated latitude and longitude."
    )]
    pub location: Option<String>,
    #[clap(short = 'k', long, help = "OpenWeatherMap API Key to use.")]
    pub apikey: Option<String>,
    #[clap(long, help = "Format output as JSON.")]
    pub json: bool,
    #[clap(
        short,
        long,
        arg_enum,
        help = "Units to use. Standard uses K and m/s, imperial uses °F and mph and metric uses °C and km/h."
    )]
    pub units: Option<Units>,
}
