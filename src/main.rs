#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct ThemeColors {
    color0: String,
    color1: String,
    color2: String,
    color3: String,
    color4: String,
    color5: String,
    color6: String,
    color7: String,
    color8: String,
    color9: String,
    color10: String,
    color11: String,
    color12: String,
    color13: String,
    color14: String,
    color15: String,
    background: String,
    foreground: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Theme {
    name: String,
    colors: ThemeColors,
}

type Config = Vec<Theme>;

fn main() {
    let matches = App::new("Teems")
        .version("0.1")
        .author("Florian B. <yuuki@protonmail.com")
        .about("Easily switch themes for your terminal(s)")
        .subcommand(SubCommand::with_name("list").about("List all themes"))
        .args(&[
            Arg::from_usage("-c, --config <FILE>, 'a required json file containing the themes'"),
            // TODO: To stay consistent, maybe this should also be a subcommand?
            Arg::from_usage("-t, --theme [THEME], 'an optional name of a theme'"),
        ]).get_matches();
    // TODO: Add dry run flag

    let config_path = matches.value_of("config").unwrap();
    let config = fs::read_to_string(config_path);

    match config {
        Ok(content) => {
            // TODO: Exit with nice error message
            let themes: Config = serde_json::from_str(&content).unwrap();
            // TODO: Iterate over themes and print them nicely.
            println!("{:?}", themes);
        }
        // TODO: This doesn't really help now does it? Maybe add a --debug flag which prints the error. Or just print
        // the actual error after this message
        Err(_) => println!("Error reading config file"),
    }
}
