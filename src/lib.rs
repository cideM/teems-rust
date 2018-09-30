#[macro_use]
extern crate serde_derive;
extern crate regex;

use regex::Captures;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::convert::AsRef;
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
            output.push_str(&format!("\t0: {}: {}", color, value));
        }

        output.push_str("\n");
        output.push_str("\n");

        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone)]
pub struct TeemsError<'a> {
    pub msg: Cow<'a, str>,
}

impl<'a> TeemsError<'a> {
    fn new<T>(msg: T) -> TeemsError<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        TeemsError { msg: msg.into() }
    }
}

impl<'a> fmt::Display for TeemsError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

type Config = Vec<Theme>;

pub struct Dispatcher {
    pub apps: Vec<Box<Replacer>>,
}

impl Dispatcher {
    pub fn run(&self, theme: &Theme) -> Vec<Result<String, TeemsError>> {
        let mut results = Vec::new();

        for a in &self.apps {
            for path in a.config_paths() {
                match fs::read_to_string(path) {
                    Ok(app_config) => results.push(a.convert_colors(theme, &app_config)),
                    Err(e) => results.push(Err(TeemsError::new(format!("{}", e)))),
                }
            }
        }

        results
    }
}

pub trait Replacer {
    fn convert_colors(&self, theme: &Theme, app_config: &str) -> Result<String, TeemsError>;

    fn name(&self) -> &str;

    fn config_paths(&self) -> Vec<&str>;
}

pub struct Alacritty {
    name: String,
    config_paths: Vec<String>,
}

impl Alacritty {
    pub fn new(name: &str, config_paths: Vec<String>) -> Alacritty {
        Alacritty {
            name: name.to_owned(),
            config_paths,
        }
    }
}

fn alacritty_color_to_theme_color(c: &str, normal_colors: bool) -> &str {
    match c {
        "black" if normal_colors == true => "color0",
        "black" if normal_colors == false => "color8",
        "red" if normal_colors == true => "color1",
        "red" if normal_colors == false => "color9",
        "green" if normal_colors == true => "color2",
        "green" if normal_colors == false => "color10",
        "yellow" if normal_colors == true => "color3",
        "yellow" if normal_colors == false => "color11",
        "blue" if normal_colors == true => "color4",
        "blue" if normal_colors == false => "color12",
        "magenta" if normal_colors == true => "color5",
        "magenta" if normal_colors == false => "color13",
        "cyan" if normal_colors == true => "color6",
        "cyan" if normal_colors == false => "color14",
        "white" if normal_colors == true => "color7",
        "white" if normal_colors == false => "color15",
        "foreground" => "foreground",
        "background" => "background",
        _ => "color0",
    }
}

impl Replacer for Alacritty {
    fn convert_colors(&self, theme: &Theme, app_config: &str) -> Result<String, TeemsError> {
        let re_bright = Regex::new(r"^\s*bright:").unwrap();
        let re_normal = Regex::new(r"^\s*normal:").unwrap();
        let re_line_with_color = Regex::new(
            r"(?x)
               ^
               (?P<leading>\s*)
               (?P<color_name>black|red|green|yellow|blue|magenta|cyan|white|foreground|background)
               (?P<middle>:\s*'0x)
               (?P<color_value>\w{6})
               (?P<trailing>'.*)
            ",
        )
        .unwrap();

        let mut normal_colors = false;
        let mut results = vec![];

        for line in app_config.lines() {
            if re_bright.is_match(line) {
                normal_colors = false;
            }

            if re_normal.is_match(line) {
                normal_colors = true;
            }

            let after = re_line_with_color
                .replace_all(line, |caps: &Captures| {
                    let theme_color_name =
                        alacritty_color_to_theme_color(&caps["color_name"], normal_colors);

                    format!(
                        "{}{}{}{}{}",
                        &caps["leading"],
                        &caps["color_name"],
                        &caps["middle"],
                        // Remove # from color
                        &theme.colors.get(theme_color_name).expect(&format!(
                            "Could not find color {} in theme.",
                            theme_color_name
                        ))[1..],
                        &caps["trailing"]
                    )
                })
                .to_string();
            results.push(after);
        }

        Ok(results.join("\n"))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn config_paths(&self) -> Vec<&str> {
        self.config_paths.iter().map(AsRef::as_ref).collect()
    }
}

pub fn list_themes(config: &Config) -> () {
    for theme in config {
        println!("{}", theme);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_replaces_bright_colors_in_alacritty() {
        let a = Alacritty {
            name: String::from("Alacritty"),
            config_paths: vec![],
        };

        let c: HashMap<String, String> = [
            (String::from("color0"), String::from("#000000")),
            (String::from("color1"), String::from("#111111")),
            (String::from("color2"), String::from("#222222")),
            (String::from("color3"), String::from("#333333")),
            (String::from("color4"), String::from("#444444")),
            (String::from("color5"), String::from("#555555")),
            (String::from("color6"), String::from("#666666")),
            (String::from("color7"), String::from("#777777")),
            (String::from("color8"), String::from("#888888")),
            (String::from("color9"), String::from("#999999")),
            (String::from("color10"), String::from("#101010")),
            (String::from("color11"), String::from("#111111")),
            (String::from("color12"), String::from("#121212")),
            (String::from("color13"), String::from("#131313")),
            (String::from("color14"), String::from("#141414")),
            (String::from("color15"), String::from("#151515")),
            (String::from("foreground"), String::from("#FFFFFF")),
            (String::from("background"), String::from("#BBBBBB")),
        ]
            .iter()
            .cloned()
            .collect();

        let t = Theme {
            name: String::from("theme"),
            colors: c,
        };

        let cfg = "
        background:     '0x2E3440'
        foreground:     '0xD8DEE9'
        
        normal:
            black:       '0x3B4252'
            red:         '0xBF616A'
            green:       '0xA3BE8C'
            yellow:      '0xEBCB8B'
            blue:        '0x81A1C1'
            magenta:     '0xB48EAD'
            cyan:        '0x88C0D0'
            white:       '0xE5E9F0'

        # Bright colors
        bright:
            black:       '0x4C566A'
            red:         '0xBF616A'
            green:       '0xA3BE8C'
            yellow:      '0xEBCB8B'
            blue:        '0x81A1C1'
            magenta:     '0xB48EAD'
            cyan:        '0xA3BE8C'
            white:       '0xECEFF4'
        ";

        let cfg_expected = "
        background:     '0xBBBBBB'
        foreground:     '0xFFFFFF'
        
        normal:
            black:       '0x000000'
            red:         '0x111111'
            green:       '0x222222'
            yellow:      '0x333333'
            blue:        '0x444444'
            magenta:     '0x555555'
            cyan:        '0x666666'
            white:       '0x777777'

        # Bright colors
        bright:
            black:       '0x888888'
            red:         '0x999999'
            green:       '0x101010'
            yellow:      '0x111111'
            blue:        '0x121212'
            magenta:     '0x131313'
            cyan:        '0x141414'
            white:       '0x151515'
        ";

        let result = a.convert_colors(&t, &cfg).unwrap();
        assert_eq!(result, cfg_expected);
    }
}