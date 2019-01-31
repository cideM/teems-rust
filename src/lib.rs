pub mod apps;

use failure::Error;
use failure_derive::Fail;
use serde::de::{self, Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use std::path::PathBuf;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

type ColorName = String;

type ThemeName = String;

#[derive(Debug, Serialize, PartialEq)]
pub struct RGBA(u8, u8, u8, f32);

impl RGBA {
    fn to_hex(&self) -> String {
        format!("#{:0>2x}{:0>2x}{:0>2x}", &self.0, &self.1, &self.2,)
    }
}

impl<'d> de::Deserialize<'d> for RGBA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum RGBAHelper {
            Str(String),
            Array(u8, u8, u8, f32),
        }

        match Deserialize::deserialize(deserializer)? {
            RGBAHelper::Str(str) => {
                if str.len() != 7 {
                    Err(de::Error::custom(
                        "Hex color string must be of format #ABCDEF",
                    ))
                } else {
                    // This only works on ASCII
                    let rgb = &str[1..]
                        .as_bytes()
                        .chunks_exact(2)
                        .map(|c| {
                            let s = c.iter().map(|&byte| byte as char).collect::<String>();
                            u8::from_str_radix(&s, 16)
                        })
                        .collect::<Result<Vec<u8>, ParseIntError>>()
                        .map_err(de::Error::custom)?;

                    Ok(RGBA(rgb[0], rgb[1], rgb[2], 1.0))
                }
            }
            RGBAHelper::Array(r, g, b, alpha) => Ok(RGBA(r, g, b, alpha)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Theme {
    pub name: ThemeName,
    pub colors: HashMap<ColorName, RGBA>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        output.push_str(&format!("Name: {}\n", &self.name));
        output.push_str("Colors:\n");

        // TODO: Implement display for RGBA
        for (color, value) in &self.colors {
            output.push_str(&format!("\t: {:?}: {:?}", color, value));
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

pub fn list_themes(config: Config) {
    for theme in config {
        println!("{}", theme.name);
    }
}

pub fn activate_theme(apps: Vec<App>, theme: &Theme) -> Result<(), Error> {
    let home_dir = dirs::home_dir().unwrap();
    // config_dir is Library/Preferences on MacOS but I don't think anyone
    // really stores configuration for e.g., terminal emulators there.
    let config_dir_os = dirs::config_dir().unwrap();
    let config_dir_linux = home_dir.join(".config");

    for app in apps {
        let mut valid_paths: Vec<PathBuf> = app
            .config_paths
            .iter()
            .flat_map(|p| vec![config_dir_linux.join(p), config_dir_os.join(p)].into_iter())
            .filter(|p| p.exists())
            .collect();

        valid_paths.sort();
        valid_paths.dedup();

        for path in valid_paths {
            let config = fs::read_to_string(&path)?;

            let new_config = (app.mk_config)(&theme, &config)?;

            fs::write(&path, new_config)?;

            println!("{} \u{2713}", app.name);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_rgba_str() {
        let s = r##"
        {
          "name": "foo",
          "colors": {
            "color1": "#FFAABB"
          }
        }"##;

        let res: Theme = serde_json::from_str(&s).unwrap();
        let mut colors = HashMap::new();

        colors.insert(String::from("color1"), RGBA(255, 170, 187, 1.0));

        let expect = Theme {
            name: String::from("foo"),
            colors,
        };

        assert_eq!(res, expect);
    }

    #[test]
    fn it_parses_rgba_array() {
        let s = r##"
        {
          "name": "foo",
          "colors": {
            "color1": [255, 170, 187, 1.0]
          }
        }"##;

        let res: Theme = serde_json::from_str(&s).unwrap();
        let mut colors = HashMap::new();

        colors.insert(String::from("color1"), RGBA(255, 170, 187, 1.0));

        let expect = Theme {
            name: String::from("foo"),
            colors,
        };

        assert_eq!(res, expect);
    }
}
