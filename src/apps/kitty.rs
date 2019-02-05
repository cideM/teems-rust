use crate::Theme;
use failure::Error;
use regex::Regex;

pub fn convert_colors(theme: &Theme, app_config: &str) -> Result<String, Error> {
    let mut results: Vec<String> = vec![];

    let re_line_with_color = Regex::new(
        r"(?x)
        ^\s*
        (?P<color_name>color\d+
            |foreground
            |background
            |cursor
            |url_color
            |active_border_color
            |inactive_border_color
            |active_tab_foreground
            |active_tab_background
            |inactive_tab_foreground
            |inactive_tab_background
            |selection_foreground
            |selection_background)
        \s*
        (?P<color_value>\#\w{6})
    ",
    )?;

    for line in app_config.lines() {
        if let Some(captures) = re_line_with_color.captures(line) {
            let new_value = &theme
                .colors
                // Use existing color value if theme doesn't have a replacement
                .get(&captures["color_name"])
                .and_then(|c| Some(c.to_hex()))
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
            (String::from("selection_foreground"), RGBA(70, 70, 70, 1.0)),
            (String::from("selection_background"), RGBA(70, 70, 70, 1.0)),
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
# The foreground for selections
selection_foreground #000000

# The background for selections
selection_background #FFFACD

# The 16 terminal colors. There are 8 basic colors, each color has a dull and
# bright version. You can also set the remaining colors from the 256 color table
# as color16 to color256.

# black
color0 #1d1f21
color8 #969896

color1 #cc6666
color9 #cc6666

color2 #b5bd68
color3 #f0c674
color4 #81a2be
color5 #b294bb
color6 #8abeb7
color7 #c5c8c6

color10 #b5bd68
color11 #f0c674
color12 #81a2be
color13 #b294bb
color14 #8abeb7
color15 #ffffff
        ";

        let cfg_expected = "
# The foreground for selections
selection_foreground #464646

# The background for selections
selection_background #464646

# The 16 terminal colors. There are 8 basic colors, each color has a dull and
# bright version. You can also set the remaining colors from the 256 color table
# as color16 to color256.

# black
color0 #000000
color8 #080808

color1 #010101
color9 #090909

color2 #020202
color3 #030303
color4 #040404
color5 #050505
color6 #060606
color7 #070707

color10 #0a0a0a
color11 #0b0b0b
color12 #0c0c0c
color13 #0d0d0d
color14 #0e0e0e
color15 #0f0f0f
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
