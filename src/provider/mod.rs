use anyhow::Result;
use colored::Color;
use serde::Serialize;

use crate::context::Context;

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
pub enum Providers {
    OpenWeatherMap,
}

impl Providers {
    pub fn into_provider(&self) -> &dyn Provider {
        match self {
            Self::OpenWeatherMap => &OWMProvider,
        }
    }
}

pub trait Provider {
    fn get(&self, cx: &Context) -> Result<Weather>;
}
