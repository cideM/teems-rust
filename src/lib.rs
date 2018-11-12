pub mod apps;

use failure::Error;
use failure_derive::Fail;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub name: String,
    pub colors: HashMap<String, String>,
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

pub struct Dispatcher {
    pub apps: Vec<Box<Replacer>>,
}

impl Dispatcher {
    pub fn run(&self, theme: &Theme) -> () {
        for app in &self.apps {
            let name = app.name();

            println!("App: {}", name);

            for path in app.config_paths() {
                let config = match fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(e) => {
                        println!(
                            "Error reading configuration file {} for app {}: {}",
                            path, name, e
                        );
                        break;
                    }
                };

                let new_config = match app.convert_colors(theme, &config) {
                    Ok(c) => c,
                    Err(e) => {
                        println!("Error converting colors for app {}: {}", name, e);
                        break;
                    }
                };

                match fs::write(path, new_config) {
                    Err(e) => println!("Error in app {}: {}", name, e),
                    Ok(_) => println!("Converted colors for {} in {}", name, path),
                };
            }
        }
    }
}

pub trait Replacer {
    fn convert_colors(&self, theme: &Theme, app_config: &str) -> Result<String, Error>;

    fn name(&self) -> &str;

    fn config_paths(&self) -> Vec<&str>;
}

pub fn list_themes(config: Config) -> () {
    for theme in config {
        println!("{}", theme.name);
    }
}
