pub mod apps;

use failure::Error;
use failure_derive::Fail;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

type Hex = String;

pub trait Hexable {
    fn to_hex(&self) -> Hex;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RGBA(u8, u8, u8, f32);

impl Hexable for RGBA {
    fn to_hex(&self) -> Hex {
        format!("#{:0>2x}{:0>2x}{:0>2x}", &self.0, &self.1, &self.2,)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ColorValue {
    RGBA(RGBA),
}

impl fmt::Display for ColorValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        match self {
            ColorValue::RGBA(r) => output.push_str(&format!("{:?}", r)),
        };

        write!(f, "{}", output)
    }
}

type ColorName = String;

type ThemeName = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub name: ThemeName,
    pub colors: HashMap<ColorName, ColorValue>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        output.push_str(&format!("Name: {}\n", &self.name));
        output.push_str("Colors:\n");

        for (color, value) in &self.colors {
            output.push_str(&format!("\t: {}: {}", color, value));
        }

        output.push_str("\n");
        output.push_str("\n");

        write!(f, "{}", output)
    }
}

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Error during color conversion: {}", msg)]
    ConversionError { msg: String },
}

type Config = Vec<Theme>;

pub struct App {
    pub config_paths: Vec<PathBuf>,
    pub name: String,
    pub mk_config: Box<Fn(&Theme, &str) -> Result<String, Error>>,
}

impl App {
    pub fn new(
        name: String,
        config_paths: Vec<PathBuf>,
        mk_config: Box<Fn(&Theme, &str) -> Result<String, Error>>,
    ) -> App {
        App {
            name,
            config_paths,
            mk_config,
        }
    }
}

pub fn list_themes(config: Config) -> () {
    for theme in config {
        println!("{}", theme.name);
    }
}
