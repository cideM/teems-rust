use clap::{App, Arg, SubCommand};
use std::fs;
use std::path::PathBuf;
use teems_rust::{apps, App as TermEmu, Theme};

fn main() {
    let alacritty = TermEmu::new(
        String::from("alacritty"),
        vec![PathBuf::from(r"alacritty/alacritty.yml")],
        Box::new(apps::alacritty::convert_colors),
    );

    let apps = vec![alacritty];

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
                // TODO: Do it like above eith list_themes
                let home_dir = dirs::home_dir().unwrap();
                // config_dir is Library/Preferences on MacOS but I don't think anyone
                // really stores configuration for e.g., terminal emulators there.
                let config_dir_os = dirs::config_dir().unwrap();
                let config_dir_linux = home_dir.join(".config");

                let theme_name = sub
                    .value_of("theme")
                    .expect("Could not read 'theme' argument");

                let theme = cfg
                    .into_iter()
                    .find(|x: &Theme| x.name == theme_name)
                    .unwrap_or_else(|| panic!("Theme {} not found in config file", theme_name));

                for app in apps {
                    println!("App: {}", app.name);

                    let mut valid_paths: Vec<PathBuf> = app
                        .config_paths
                        .iter()
                        .flat_map(|p| {
                            vec![config_dir_linux.join(p), config_dir_os.join(p)].into_iter()
                        })
                        .filter(|p| p.exists())
                        .collect();

                    valid_paths.sort();
                    valid_paths.dedup();

                    for path in valid_paths {
                        let config = fs::read_to_string(&path).unwrap_or_else(|_| {
                            panic!(
                                "Error reading configuration file {} for app {}.",
                                path.display(),
                                app.name
                            )
                        });

                        let new_config = (app.mk_config)(&theme, &config).unwrap_or_else(|_| {
                            panic!("Error converting colors for app {}.", app.name)
                        });

                        match fs::write(&path, new_config) {
                            Err(e) => println!("Error in app {}: {}", app.name, e),
                            Ok(_) => {
                                println!("Converted colors for {} in {}", app.name, path.display())
                            }
                        };
                    }
                }
            }
            _ => {
                // Default if no subcommand matched
                teems_rust::list_themes(cfg);
            }
        },
        Err(_) => println!("Could not deserialize config file"),
    }
}
