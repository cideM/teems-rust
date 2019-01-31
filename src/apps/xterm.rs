use crate::Theme;
use failure::Error;
use regex::Captures;
use regex::Regex;

pub fn convert_colors(theme: &Theme, app_config: &str) -> Result<String, Error> {
    let mut results: Vec<String> = vec![];

    let re_line_with_color = Regex::new(
        r"(?xi)
        ^XTerm\*
        (?P<color_name>color\d+
            |foreground
            |background)
        (?P<middle>:\s*)
        (?P<color_value>\#\w{6})
    ",
    )?;

    for line in app_config.lines() {
        let after = re_line_with_color
            .replace_all(line, |caps: &Captures| {
                // This is a bit sloppy. I should really parse the xterm string case insensitively
                // and just insert it here. But then I'd probably have to do it in JS and Haskell
                // too... ~_~
                format!(
                    "XTerm*{}{}{}",
                    &caps["color_name"],
                    &caps["middle"],
                    &theme
                        .colors
                        // Use existing color value if theme doesn't have a replacement
                        .get(&caps["color_name"])
                        .and_then(|c| Some(c.to_hex()))
                        .unwrap_or_else(|| caps["color_value"].to_string())
                )
            })
            .to_string();

        results.push(after);
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
XTerm*foreground: #afb7c0
XTerm*background: #2c2d30
XTerm*color0: #2c2d30
XTerm*color8: #363a3e
XTerm*color1: #be868c
XTerm*color9: #be868c
XTerm*color2: #7f9d77
XTerm*color10: #7f9d77
XTerm*color3: #ab916d
XTerm*color11: #ab916d
XTerm*color4: #759abd
XTerm*color12: #759abd
XTerm*color5: #a88cb3
XTerm*color13: #a88cb3
XTerm*color6: #5da19f
XTerm*color14: #5da19f
XTerm*color7: #afb7c0
XTerm*color15: #cbd2d9
        ";

        let cfg_expected = "
XTerm*foreground: #ffffff
XTerm*background: #323232
XTerm*color0: #000000
XTerm*color8: #080808
XTerm*color1: #010101
XTerm*color9: #090909
XTerm*color2: #020202
XTerm*color10: #0a0a0a
XTerm*color3: #030303
XTerm*color11: #0b0b0b
XTerm*color4: #040404
XTerm*color12: #0c0c0c
XTerm*color5: #050505
XTerm*color13: #0d0d0d
XTerm*color6: #060606
XTerm*color14: #0e0e0e
XTerm*color7: #070707
XTerm*color15: #0f0f0f
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
