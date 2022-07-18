use crate::{args::Args, config::Config, formatter::Formatter, provider::Provider};
use clap::{clap_derive::ArgEnum, Parser};

#[derive(Debug)]
pub struct Context {
    pub location: Location,
    pub units: Units,
    pub apikey: String,
    pub provider: Provider,
    pub formatter: Formatter,
}

impl Context {
    pub fn build() -> Self {
        let args = Args::parse();
        let config = Config::read();
        let formatter = match args.json {
            true => Formatter::JSON,
            false => Formatter::Default,
        };
        let provider = Provider::OpenWeatherMap;
        let location = if let Some(loc_str) = args.location {
            Location::from_str(&loc_str)
        } else if let Some(ref config) = config {
            if let Some(loc_str) = &config.location {
                Location::from_str(&loc_str)
            } else {
                panic!("No location specified in either config file or arguments!")
            }
        } else {
            panic!("No location specified in either config file or arguments!")
        };

        let units = if let Some(unit) = args.units {
            unit
        } else if let Some(ref config) = config {
            if let Some(unit_str) = &config.units {
                if let Some(unit) = Units::from_str(&unit_str) {
                    unit
                } else {
                    Units::default()
                }
            } else {
                Units::default()
            }
        } else {
            Units::default()
        };

        let apikey = if let Some(apikey) = args.apikey {
            apikey
        } else if let Some(config) = config {
            if let Some(apikey) = config.apikey {
                apikey
            } else {
                panic!("No OpenWeatherMap API Key specified in either config file or arguments!")
            }
        } else {
            panic!("No OpenWeatherMap API Key specified in either config file or arguments!")
        };

        Self {
            apikey,
            location,
            formatter,
            provider,
            units,
        }
    }
}

#[derive(Debug)]
pub enum Location {
    Coords(f64, f64),
    Name(String),
}

impl Location {
    fn from_str(input: &str) -> Self {
        let coords: Vec<_> = input.split(",").collect();
        if coords.len() != 2 {
            return Self::Name(input.to_string());
        };

        let coords: Vec<_> = coords.iter().map(|val| val.parse::<f64>()).collect();
        for coord in &coords {
            if coord.is_err() {
                return Self::Name(input.to_string());
            };
        }

        Self::Coords(*coords[0].as_ref().unwrap(), *coords[1].as_ref().unwrap())
    }
}

#[derive(Debug, Clone, ArgEnum)]
pub enum Units {
    Standard,
    Imperial,
    Metric,
}

impl Default for Units {
    fn default() -> Self {
        Self::Metric
    }
}

impl Units {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "standard" => Some(Self::Standard),
            "imperial" => Some(Self::Imperial),
            "metric" => Some(Self::Metric),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Standard => "standard",
            Self::Imperial => "imperial",
            Self::Metric => "metric",
        }
    }

    pub fn speed(&self) -> &str {
        match self {
            Self::Standard | Self::Metric => "m/s",
            Self::Imperial => "mph",
        }
    }

    pub fn temp(&self) -> &str {
        match self {
            Self::Standard => "K",
            Self::Imperial => "°F",
            Self::Metric => "°C",
        }
    }
}
