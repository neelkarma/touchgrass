use colored::Color;
use serde::Serialize;

use self::owm::OWMProvider;

pub mod owm;

#[derive(Debug, Serialize)]
pub struct Weather {
    pub location: String,
    #[serde(skip_serializing)]
    pub condition_color: Color,
    pub condition: String,
    pub feels_like: f64,
    pub temp: f64,
    pub min_temp: f64,
    pub max_temp: f64,
    pub precipitation: f64,
    pub wind_speed: f64,
    pub wind_gust: f64,
}

#[derive(Debug)]
pub enum Provider {
    OpenWeatherMap,
}
