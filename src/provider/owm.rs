use super::{Provider, Weather};
use crate::context::{Context, Location};
use anyhow::Result;
use colored::Color;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponse {
    #[serde(default)]
    pub weather: Vec<OWMResponseWeather>,
    #[serde(default)]
    pub main: OWMResponseMain,
    #[serde(default)]
    pub wind: OWMResponseWind,
    #[serde(default)]
    pub rain: OWMResponseRain,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseWeather {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub description: String,
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
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseWind {
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub gust: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMResponseRain {
    #[serde(rename = "1h")]
    #[serde(default)]
    pub n1h: f64,
}

pub type OWMGeolocationResponse = Vec<OWMGeolocationItem>;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OWMGeolocationItem {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug)]
pub struct OWMProvider;

impl OWMProvider {
    fn get_geolocation(&self, ctx: &Context, name: &str) -> Result<(String, String)> {
        let res = ctx
            .client
            .get("http://api.openweathermap.org/geo/1.0/direct")
            .query(&[
                ("q", name),
                ("limit", &"1".to_string()),
                ("appid", &ctx.apikey.clone()),
            ])
            .send()?;

        let parsed = &res.json::<OWMGeolocationResponse>()?[0];
        Ok((parsed.lat.to_string(), parsed.lon.to_string()))
    }
}

impl Provider for OWMProvider {
    fn get(&self, ctx: &Context) -> Result<Weather> {
        let coords = match &ctx.location {
            Location::Coords(lat, lon) => (lat.to_string(), lon.to_string()),
            Location::Name(name) => self.get_geolocation(ctx, name)?,
        };

        let res = ctx
            .client
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
