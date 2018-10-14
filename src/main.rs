extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate teems_rust;

use clap::{App, Arg, SubCommand};
use std::fs;
use teems_rust::Alacritty;
use teems_rust::Dispatcher;
use teems_rust::Theme;

fn main() {
    // Does anyone really use MacOS actual config dir in Library/Preferences?
    let home_dir = dirs::home_dir().unwrap();
    let config_dir_linux = format!("{}/.config", home_dir.to_str().unwrap());

    let alacritty = Alacritty::new(
        "alacritty",
        vec![format!("{}/alacritty/alacritty.yml", config_dir_linux)],
    );

    let dispatcher = Dispatcher {
        apps: vec![Box::new(alacritty)],
    };

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
                teems_rust::list_themes(cfg);
            }
            ("activate", Some(sub)) => {
                let theme_name = sub
                    .value_of("theme")
                    .expect("Could not read 'theme' argument");

                let theme = cfg
                    .into_iter()
                    .find(|x: &Theme| x.name == theme_name)
                    .unwrap_or_else(|| panic!("Theme {} not found in config file", theme_name));

                dispatcher.run(&theme);
            }
            _ => {
                // Default if no subcommand matched
                teems_rust::list_themes(cfg);
            }
        },
        Err(_) => println!("Could not deserialize config file"),
    }
}
