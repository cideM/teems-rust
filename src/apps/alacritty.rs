use crate::Theme;
use failure::Error;
use regex::Regex;

enum Mode {
    Bright,
    Normal,
}

fn get_theme_color<'a>(color: &'a str, mode: &Mode) -> &'a str {
    match mode {
        Mode::Normal => match color {
            "black" => "color0",
            "red" => "color1",
            "green" => "color2",
            "yellow" => "color3",
            "blue" => "color4",
            "magenta" => "color5",
            "cyan" => "color6",
            "white" => "color7",
            "foreground" => "foreground",
            "background" => "background",
            "text" => "text",
            "cursor" => "cursor",
            _ => "color0",
        },
        Mode::Bright => match color {
            "black" => "color8",
            "red" => "color9",
            "green" => "color10",
            "yellow" => "color11",
            "blue" => "color12",
            "magenta" => "color13",
            "cyan" => "color14",
            "white" => "color15",
            "foreground" => "foreground",
            "background" => "background",
            "text" => "text",
            "cursor" => "cursor",
            _ => "color0",
        },
    }
}

pub fn convert_colors(theme: &Theme, app_config: &str) -> Result<String, Error> {
    let re_line_with_color = Regex::new(
        r##"(?x)
            ^
            \s*
            (?P<color_name>black
                |red
                |green
                |yellow
                |blue
                |magenta
                |cyan
                |white
                |foreground
                |background
                |cursor
                |text)
            :
            \s*['"]0x
            (?P<color_value>\w{6})
            ['"].*
        "##,
    )?;

    let mut mode = Mode::Normal;
    let mut results: Vec<String> = vec![];

    for line in app_config.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("bright:") {
            mode = Mode::Bright;
        } else if trimmed.starts_with("normal:") {
            mode = Mode::Normal;
        }

        if let Some(captures) = re_line_with_color.captures(line) {
            let theme_color_name = get_theme_color(&captures["color_name"], &mode);

            let new_value = &theme
                .colors
                // Use existing color value if theme doesn't have a replacement
                .get(theme_color_name)
                .and_then(|c| Some(c.to_hex()))
                .unwrap_or_else(|| captures["color_value"].to_string())
                .replace("#", "");

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
        background:     '0x2E3440'
        foreground:     '0xD8DEE9'

        cursor:
            cursor:       '0x3B4252'
            text:         '0xBF616A'

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
        background:     '0x323232'
        foreground:     '0xffffff'

        cursor:
            cursor:       '0x3c3c3c'
            text:         '0x464646'

        normal:
            black:       '0x000000'
            red:         '0x010101'
            green:       '0x020202'
        #   green:       '0xA3BE8C'
            yellow:      '0x030303'
            blue:        '0x040404'
            magenta:     '0x050505'
            cyan:        '0x060606'
            white:       '0x070707'

        # Bright colors
        bright:
            black:       '0x080808'
            red:         '0x090909'
            green:       '0x0a0a0a'
            yellow:      '0x0b0b0b'
            blue:        '0x0c0c0c'
            magenta:     '0x0d0d0d'
            cyan:        '0x0e0e0e'
            white:       '0x0f0f0f'
        ";

        let result = convert_colors(&theme, &cfg).unwrap();
        assert_eq!(result, cfg_expected);
    }

    #[test]
    fn it_keeps_formatting() {
        let theme = get_theme();

        let cfg = "
            normal:
                black: '0x123456'
                red:         '0x123456'
        ";

        let expected = "
            normal:
                black: '0x000000'
                red:         '0x010101'
        ";

        let result = convert_colors(&theme, &cfg).unwrap();
        assert_eq!(result, expected);
    }
}
