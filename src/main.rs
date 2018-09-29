#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::convert::AsRef;
use std::fmt;
use std::fs;
use std::process;

#[derive(Serialize, Deserialize, Debug)]
struct Theme {
    name: String,
    colors: HashMap<String, String>,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        output.push_str(&format!("Name: {}\n", &self.name));
        output.push_str("Colors:\n");

        for (color, value) in &self.colors {
            output.push_str(&format!("\t0: {}: {}", color, value));
        }

        output.push_str("\n");
        output.push_str("\n");

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
struct TeemsError {
    msg: String,
}

impl TeemsError {
    fn new(msg: &str) -> TeemsError {
        TeemsError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for TeemsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

type Config = Vec<Theme>;

struct Dispatcher {
    apps: Vec<Box<Replacer>>,
}

impl Dispatcher {
    fn run(&self, app_config: &str, theme: &Theme) {
        for x in &self.apps {
            x.replace(app_config, theme);
        }
    }
}

trait Replacer {
    fn replace(&self, app_config: &str, theme: &Theme) -> Result<Vec<String>, TeemsError>;

    fn name(&self) -> &str;

    fn config_paths(&self) -> Vec<&str>;
}

struct Alacritty {
    name: String,
    config_paths: Vec<String>,
}

impl Alacritty {
    fn new(name: &str, config_paths: Vec<String>) -> Alacritty {
        Alacritty {
            name: name.to_owned(),
            config_paths,
        }
    }
}

impl Replacer for Alacritty {
    fn replace(&self, app_config: &str, theme: &Theme) -> Result<Vec<String>, TeemsError> {
        let mut result = Vec::new();

        for path in &self.config_paths {
            let content = match fs::read_to_string(path) {
                Ok(x) => {
                    result.push(x);
                }
                Err(e) => return Err(TeemsError::new(&format!("{}", e))),
            };
        }

        Ok(result)
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn config_paths(&self) -> Vec<&str> {
        self.config_paths.iter().map(AsRef::as_ref).collect()
    }
}

fn list_themes(config: &Config) -> () {
    for theme in config {
        println!("{}", theme);
    }
}

fn main() {
    // TODO: Initialize dispatcher and call run() in 'activate' branch
    let alacritty = Alacritty::new("alacritty", vec![]);

    let app = App::new("Teems")
        .version("0.1")
        .author("Florian B. <yuuki@protonmail.com")
        .about("Easily switch themes for your terminal(s)")
        .subcommand(SubCommand::with_name("list").about("List all themes"))
        .subcommand(
            SubCommand::with_name("activate")
                .about("Activate a theme")
                .arg(Arg::from_usage(
                    "-t, --theme <THEME> 'a required name of a theme'",
                )),
        )
        .args(&[Arg::from_usage(
            "-c, --config <FILE> 'a required json file containing the themes'",
        )]);

    let matches = app.get_matches();
    // TODO: Add dry run flag

    let config_path = matches
        .value_of("config")
        .expect("Couldn't read '--config' value");

    let config_serialized = fs::read_to_string(config_path).expect("Error reading config file");
    let config_deserialized = serde_json::from_str(&config_serialized);

    match config_deserialized {
        Ok(cfg) => match matches.subcommand() {
            ("list", _) => {
                list_themes(&cfg);
            }
            ("activate", Some(sub)) => {
                let theme_name = sub
                    .value_of("theme")
                    .expect("Could not read 'theme' argument");

                let theme = match cfg.into_iter().find(|x: &Theme| x.name == theme_name) {
                    Some(t) => t,
                    None => {
                        eprintln!("Theme {} not found in config file", theme_name);
                        process::exit(1);
                    }
                };

                let result = match alacritty.replace("foo", &theme) {
                    Ok(r) => r,
                    Err(_) => {
                        eprintln!("Error generating new app config");
                        process::exit(1);
                    }
                };

                println!("{:?}", result);
            }
            _ => {
                // Default if no subcommand matched
                list_themes(&cfg);
            }
        },
        Err(_) => println!("Could not deserialize config file"),
    }
}
