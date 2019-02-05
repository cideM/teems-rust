use crate::Theme;
use crate::RGBA;
use failure::Error;
use regex::Regex;

pub fn convert_colors(theme: &Theme, app_config: &str) -> Result<String, Error> {
    let mut results: Vec<String> = vec![];

    let re_line_with_color = Regex::new(
        r"(?xi)
        ^
        (?P<color_name>color\d+
            |foreground
            |background
            |cursor
            |foreground_bold
            |cursor_foreground
            |highlight)
        \s*=\s*
        (?P<color_value>(\#\w{6}|rgba\(.*\)))
    ",
    )?;

    for line in app_config.lines() {
        if let Some(captures) = re_line_with_color.captures(line) {
            let new_value = &theme
                .colors
                .get(&captures["color_name"])
                .and_then(|RGBA(r, b, g, a)| Some(format!("rgba({},{},{},{})", r, g, b, a)))
                .unwrap_or_else(|| captures["color_value"].to_string());

            let after = line.replace(&captures["color_value"], new_value);

            results.push(after);
        } else {
            results.push(line.to_owned());
        }
    }

    Ok(results.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ColorName, RGBA};
    use std::collections::HashMap;

    fn get_theme() -> Theme {
        let c: HashMap<ColorName, RGBA> = vec![
            (String::from("color0"), RGBA(0, 0, 0, 1.0)),
            (String::from("color1"), RGBA(1, 1, 1, 1.0)),
            (String::from("color2"), RGBA(2, 2, 2, 1.0)),
            (String::from("color3"), RGBA(3, 3, 3, 1.0)),
            (String::from("color4"), RGBA(4, 4, 4, 1.0)),
            (String::from("color5"), RGBA(5, 5, 5, 1.0)),
            (String::from("color6"), RGBA(6, 6, 6, 1.0)),
            (String::from("color7"), RGBA(7, 7, 7, 1.0)),
            (String::from("color8"), RGBA(8, 8, 8, 1.0)),
            (String::from("color9"), RGBA(9, 9, 9, 1.0)),
            (String::from("color10"), RGBA(10, 10, 10, 1.0)),
            (String::from("color11"), RGBA(11, 11, 11, 1.0)),
            (String::from("color12"), RGBA(12, 12, 12, 1.0)),
            (String::from("color13"), RGBA(13, 13, 13, 1.0)),
            (String::from("color14"), RGBA(14, 14, 14, 1.0)),
            (String::from("color15"), RGBA(15, 15, 15, 1.0)),
            (String::from("foreground"), RGBA(255, 255, 255, 1.0)),
            (String::from("background"), RGBA(50, 50, 50, 1.0)),
            (String::from("cursor"), RGBA(60, 60, 60, 1.0)),
            (String::from("text"), RGBA(70, 70, 70, 1.0)),
        ]
        .into_iter()
        .collect();

        Theme {
            name: String::from("theme"),
            colors: c,
        }
    }

    #[test]
    fn it_replaces_colors() {
        let theme = get_theme();

        let cfg = "
[colors]
#foreground_bold = #ffffff
#cursor = #dcdccc
#cursor_foreground = #dcdccc
foreground = rgba(175,183,192,1)
background = rgba(44,45,48,1)
#highlight = #242424
color0 = rgba(44,45,48,1)
color1 = rgba(190,134,140,1)
color2 = rgba(127,157,119,1)
color3 = #ffffff
color4 = rgba(117,154,189,1)
color5 = rgba(168,140,179,1)
color6 = rgba(93,161,159,1)
color7 = rgba(175,183,192,1)
color8 = rgba(54,58,62,1)
color9 = rgba(190,134,140,1)
color10 = rgba(127,157,119,1)
color11 = rgba(171,145,109,1)
color12 = rgba(117,154,189,1)
color13 = rgba(168,140,179,1)
color14 = rgba(93,161,159,1)
color15 = rgba(203,210,217,1)
        ";

        let cfg_expected = "
[colors]
#foreground_bold = #ffffff
#cursor = #dcdccc
#cursor_foreground = #dcdccc
foreground = rgba(255,255,255,1)
background = rgba(50,50,50,1)
#highlight = #242424
color0 = rgba(0,0,0,1)
color1 = rgba(1,1,1,1)
color2 = rgba(2,2,2,1)
color3 = rgba(3,3,3,1)
color4 = rgba(4,4,4,1)
color5 = rgba(5,5,5,1)
color6 = rgba(6,6,6,1)
color7 = rgba(7,7,7,1)
color8 = rgba(8,8,8,1)
color9 = rgba(9,9,9,1)
color10 = rgba(10,10,10,1)
color11 = rgba(11,11,11,1)
color12 = rgba(12,12,12,1)
color13 = rgba(13,13,13,1)
color14 = rgba(14,14,14,1)
color15 = rgba(15,15,15,1)
        ";

        let result = convert_colors(&theme, &cfg).unwrap();
        assert_eq!(result, cfg_expected);
    }

    #[test]
    fn it_does_not_affect_other_apps() {
        let theme = get_theme();

        let cfg = "
URxvt.foreground: #afb7c0
URxvt.background: #2c2d30
        ";

        let cfg_expected = "
URxvt.foreground: #afb7c0
URxvt.background: #2c2d30
        ";

        let result = convert_colors(&theme, &cfg).unwrap();
        assert_eq!(result, cfg_expected);
    }
}
