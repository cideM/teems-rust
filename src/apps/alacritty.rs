use crate::Replacer;
use crate::Theme;
use failure::Error;
use regex::Captures;
use regex::Regex;
use std::convert::AsRef;

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

fn alacritty_color_to_theme_color(c: &str, is_normal_block: bool) -> &str {
    match c {
        "black" if is_normal_block => "color0",
        "black" if !is_normal_block => "color8",
        "red" if is_normal_block => "color1",
        "red" if !is_normal_block => "color9",
        "green" if is_normal_block => "color2",
        "green" if !is_normal_block => "color10",
        "yellow" if is_normal_block => "color3",
        "yellow" if !is_normal_block => "color11",
        "blue" if is_normal_block => "color4",
        "blue" if !is_normal_block => "color12",
        "magenta" if is_normal_block => "color5",
        "magenta" if !is_normal_block => "color13",
        "cyan" if is_normal_block => "color6",
        "cyan" if !is_normal_block => "color14",
        "white" if is_normal_block => "color7",
        "white" if !is_normal_block => "color15",
        "foreground" => "foreground",
        "background" => "background",
        _ => "color0",
    }
}

impl Replacer for Alacritty {
    fn convert_colors(&self, theme: &Theme, app_config: &str) -> Result<String, Error> {
        let re_bright = Regex::new(r"^\s*bright:")?;
        let re_normal = Regex::new(r"^\s*normal:")?;
        let re_line_with_color = Regex::new(
            r"(?x)
               ^
               (?P<leading>\s*)
               (?P<color_name>black
                 |red
                 |green
                 |yellow
                 |blue
                 |magenta
                 |cyan
                 |white
                 |foreground
                 |background)
               (?P<middle>:\s*'0x)
               (?P<color_value>\w{6})
               (?P<trailing>'.*)
            ",
        )?;

        let mut is_normal_block = false;
        let mut results = vec![];

        for line in app_config.lines() {
            if re_bright.is_match(line) {
                is_normal_block = false;
            }

            if re_normal.is_match(line) {
                is_normal_block = true;
            }

            let after = re_line_with_color
                .replace_all(line, |caps: &Captures| {
                    let theme_color_name =
                        alacritty_color_to_theme_color(&caps["color_name"], is_normal_block);

                    format!(
                        "{}{}{}{}{}",
                        &caps["leading"],
                        &caps["color_name"],
                        &caps["middle"],
                        &theme
                            .colors
                            // Use existing color value if theme doesn't have a replacement
                            .get(theme_color_name)
                            .unwrap_or(&caps["color_value"].to_string())
                            .replace("#", ""),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn get_theme() -> Theme {
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

        Theme {
            name: String::from("theme"),
            colors: c,
        }
    }

    #[test]
    fn it_replaces_bright_colors_in_alacritty() {
        let a = Alacritty {
            name: String::from("Alacritty"),
            config_paths: vec![],
        };

        let theme = get_theme();

        let cfg = "
        background:     '0x2E3440'
        foreground:     '0xD8DEE9'
        
        normal:
            black:       '0x3B4252'
            red:         '0xBF616A'
            green:       '0xA3BE8C'
        #   green:       '0xA3BE8C'
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
        #   green:       '0xA3BE8C'
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
        ";

        let result = a.convert_colors(&theme, &cfg).unwrap();
        assert_eq!(result, cfg_expected);
    }

    #[test]
    fn it_keeps_formatting() {
        let a = Alacritty {
            name: String::from("Alacritty"),
            config_paths: vec![],
        };

        let theme = get_theme();

        let cfg = "
            normal:
                black: '0x123456'
                red:         '0x123456'
        ";

        let expected = "
            normal:
                black: '0x000000'
                red:         '0x111111'
        ";

        let result = a.convert_colors(&theme, &cfg).unwrap();
        assert_eq!(result, expected);
    }
}
