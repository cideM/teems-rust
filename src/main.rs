use clap::{crate_version, App, Arg, SubCommand};
use std::fs;
use std::path::PathBuf;
use teems_rust::{activate_theme, apps, list_themes, App as TermEmu, Theme};

fn main() {
    let alacritty = TermEmu::new(
        String::from("alacritty"),
        vec![PathBuf::from(r"alacritty/alacritty.yml")],
        Box::new(apps::alacritty::convert_colors),
    );

    let apps = vec![alacritty];

    let app = App::new("Teems")
        .version(crate_version!())
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

    let config = fs::read_to_string(config_path).expect("Error reading config file");
    let config = serde_json::from_str(&config);

    match config {
        Ok(cfg) => match matches.subcommand() {
            ("list", _) => {
                list_themes(cfg);
            }
            ("activate", Some(sub)) => {
                let theme_name = sub
                    .value_of("theme")
                    .expect("Could not read 'theme' argument");

                let theme = cfg
                    .into_iter()
                    .find(|x: &Theme| x.name == theme_name)
                    .unwrap_or_else(|| {
                        eprintln!("Theme {} not found in config file", theme_name);
                        ::std::process::exit(0);
                    });

                match activate_theme(apps, &theme) {
                    Ok(_) => println!("Done!"),
                    Err(e) => {
                        eprintln!("{}", e);
                        ::std::process::exit(0);
                    }
                }
            }
            _ => {
                // Default if no subcommand matched
                list_themes(cfg);
            }
        },
        Err(_) => println!("Could not deserialize config file"),
    }
}
