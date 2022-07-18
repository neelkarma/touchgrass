use std::collections::HashMap;

use super::Weather;
use crate::context::{Context, Location};
use colored::Color;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponse {
    #[serde(default)]
    pub coord: OWMResponseCoord,
    #[serde(default)]
    pub weather: Vec<OWMResponseWeather>,
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub main: OWMResponseMain,
    #[serde(default)]
    pub visibility: i64,
    #[serde(default)]
    pub wind: OWMResponseWind,
    #[serde(default)]
    pub rain: OWMResponseRain,
    #[serde(default)]
    pub snow: OWMResponseSnow,
    #[serde(default)]
    pub clouds: OWMResponseClouds,
    #[serde(default)]
    pub dt: i64,
    #[serde(default)]
    pub sys: OWMResponseSys,
    #[serde(default)]
    pub timezone: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub cod: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseCoord {
    #[serde(default)]
    pub lon: f64,
    #[serde(default)]
    pub lat: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseWeather {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub main: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub icon: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseMain {
    #[serde(default)]
    pub temp: f64,
    #[serde(default)]
    pub feels_like: f64,
    #[serde(default)]
    pub temp_min: f64,
    #[serde(default)]
    pub temp_max: f64,
    #[serde(default)]
    pub pressure: f64,
    #[serde(default)]
    pub humidity: f64,
    #[serde(default)]
    pub sea_level: f64,
    #[serde(default)]
    pub grnd_level: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseWind {
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub deg: i64,
    #[serde(default)]
    pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseRain {
    #[serde(rename = "1h")]
    #[serde(default)]
    pub n1h: f64,
    #[serde(rename = "3h")]
    #[serde(default)]
    pub n3h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseSnow {
    #[serde(rename = "1h")]
    #[serde(default)]
    pub n1h: f64,
    #[serde(rename = "3h")]
    #[serde(default)]
    pub n3h: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseClouds {
    #[serde(default)]
    pub all: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseSys {
    #[serde(rename = "type", default)]
    pub internal_type: i64,
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub message: f64,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub sunrise: i64,
    #[serde(default)]
    pub sunset: i64,
}

pub type OWMGeolocationResponse = Vec<OWMGeolocationItem>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMGeolocationItem {
    pub name: String,
    pub local_names: HashMap<String, String>,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
}

#[derive(Debug)]
pub struct OWMProvider;
impl OWMProvider {
    pub fn get(ctx: &Context) -> Result<Weather, reqwest::Error> {
        let client = Client::new();

        let coords = match &ctx.location {
            Location::Coords(lat, lon) => (lat.to_string(), lon.to_string()),
            Location::Name(name) => {
                let res = client
                    .get("http://api.openweathermap.org/geo/1.0/direct")
                    .query(&[
                        ("q", name),
                        ("limit", &"1".to_string()),
                        ("appid", &ctx.apikey.clone()),
                    ])
                    .send()?;

                let parsed = &res.json::<OWMGeolocationResponse>()?[0];
                (parsed.lon.to_string(), parsed.lat.to_string())
            }
        };

        let res = client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&[
                ("lat", coords.0),
                ("lon", coords.1),
                ("appid", ctx.apikey.clone()),
                ("units", ctx.units.to_str().to_string()),
            ])
            .send()?;

        let parsed: OWMResponse = res.json()?;

        Ok(Weather {
            location: parsed.name,
            condition_color: match parsed.weather[0].id {
                200..=232 => Color::BrightYellow,
                300..=321 => Color::Cyan,
                500..=531 => Color::Blue,
                600..=622 => Color::BrightWhite,
                800 => Color::BrightBlue,
                _ => Color::White,
            },
            condition: parsed.weather[0].description.clone(),
            feels_like: parsed.main.feels_like,
            temp: parsed.main.temp as f64,
            min_temp: parsed.main.temp_min as f64,
            max_temp: parsed.main.temp_max as f64,
            precipitation: parsed.rain.n1h,
            wind_speed: parsed.wind.speed,
            wind_gust: parsed.wind.gust,
        })
    }
}
