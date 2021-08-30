// TODO: Create functions for some repeated blocks of code here

use crate::{
    colors,
    decoder::{self, VSCodeScope},
    generator::ConfigGenerator,
    lua::LuaGenerator,
    vim::{self, VimHighlight},
    vimscript::VimscriptGenerator,
};
use std::collections::HashMap;

/// HashMap containing hex colors as keys, generated highlight groups as values
type UsedColors = HashMap<String, String>;

pub fn generate_vimscript_config(theme: decoder::VSCodeTheme) -> String {
    // TODO: Maybe find a way to have kind of a trait-generic variable to store
    // the generator struct
    let mut generator = VimscriptGenerator::default();
    generate_config(theme, &mut generator);
    generator.collect()
}

pub fn generate_lua_config(theme: decoder::VSCodeTheme) -> String {
    let mut generator = LuaGenerator::default();
    generate_config(theme, &mut generator);
    generator.collect()
}

fn create_group(idx: i32) -> String {
    format!("Color{}", idx)
}

fn generate_config<T>(theme: decoder::VSCodeTheme, generator: &mut T)
where
    T: ConfigGenerator,
{
    let highlights = vim::highlights();
    let mut used_colors: UsedColors = HashMap::new();
    let mut parsed_highlights: Vec<VimHighlight> = Vec::new();
    let mut links: HashMap<&str, &str> = highlights.links.into_iter().collect();
    let mut color_index = 0;
    let mut background_color = colors::RGBA {
        r: 0,
        g: 0,
        b: 0,
        a: 1.0,
    };

    if let Some(colors) = &theme.colors {
        for (option, color) in colors {
            if option == "editor.background" {
                if let Ok(rgba) = colors::from_hex_string(color) {
                    background_color = rgba;
                }
            }
        }
    }

    for theme_token in &theme.tokens {
        for highlight_token in &highlights.tokens {
            if let Some(VSCodeScope::Single(scope)) = &theme_token.scope {
                if highlight_token.0 == scope {
                    let (bg_group, fg_group) = parse_differences_and_add_to_hashmap(
                        &mut used_colors,
                        &mut color_index,
                        &theme_token.settings.background,
                        &theme_token.settings.foreground,
                        background_color,
                    );
                    parsed_highlights.push(VimHighlight {
                        group: highlight_token.1.to_string(),
                        background: bg_group,
                        foreground: fg_group,
                        text_style: theme_token.settings.font_style.clone(),
                    });
                }
                if let Some(fallback) = highlight_token.2 {
                    links.insert(highlight_token.1, fallback);
                }
            } else if let Some(VSCodeScope::Multiple(scopes)) = &theme_token.scope {
                for scope in scopes {
                    if highlight_token.0 == scope {
                        let (bg_group, fg_group) = parse_differences_and_add_to_hashmap(
                            &mut used_colors,
                            &mut color_index,
                            &theme_token.settings.background,
                            &theme_token.settings.foreground,
                            background_color,
                        );
                        parsed_highlights.push(VimHighlight {
                            group: highlight_token.1.to_string(),
                            background: bg_group,
                            foreground: fg_group,
                            text_style: theme_token.settings.font_style.clone(),
                        });
                    }
                    if let Some(fallback) = highlight_token.2 {
                        links.insert(highlight_token.1, fallback);
                    }
                }
            }
        }
    }

    if let Some(colors) = &theme.colors {
        for highlight_color in &highlights.colors {
            let (mut background, mut foreground) = (None, None);

            for (option, color) in colors {
                if let Some(background_option) = highlight_color.1 {
                    if background_option == option {
                        let (bg_group, _) = parse_differences_and_add_to_hashmap(
                            &mut used_colors,
                            &mut color_index,
                            &Some(color.to_string()),
                            &None,
                            background_color,
                        );
                        background = bg_group;
                    }
                }

                if let Some(foreground_option) = highlight_color.2 {
                    if foreground_option == option {
                        let (_, fg_group) = parse_differences_and_add_to_hashmap(
                            &mut used_colors,
                            &mut color_index,
                            &None,
                            &Some(color.to_string()),
                            background_color,
                        );
                        foreground = fg_group;
                    }
                }
            }

            if let (None, None) = (&foreground, &background) {
                continue;
            } else {
                parsed_highlights.push(VimHighlight {
                    group: highlight_color.0.to_string(),
                    background,
                    foreground,
                    text_style: None,
                })
            }
        }
    }

    generator.newline();
    for (color_hex, highlight_group) in &used_colors {
        generator.variable(highlight_group.to_string(), color_hex.to_string())
    }

    generator.newline();
    for highlight in parsed_highlights {
        generator.highlight(&highlight)
    }

    generator.newline();
    for (group, target_group) in links {
        generator.link(group, target_group);
    }
}

fn parse_differences_and_add_to_hashmap(
    used_colors: &mut HashMap<String, String>,
    color_idx: &mut i32,
    background: &Option<String>,
    foreground: &Option<String>,
    background_color: colors::RGBA,
) -> (Option<String>, Option<String>) {
    let mut result: (Option<String>, Option<String>) = (None, None);

    if let Some(background) = background {
        let background = parse_color(background.clone(), background_color, 1.0);
        if let Some(color_group) = used_colors.get(&background) {
            result.0 = Some(color_group.clone());
        } else {
            let group = create_group(*color_idx);
            used_colors.insert(background, group.clone());
            result.0 = Some(group);
            *color_idx += 1;
        }
    }

    if let Some(foreground) = foreground {
        let foreground = parse_color(foreground.clone(), background_color, 1.0);
        if let Some(color_group) = used_colors.get(&foreground) {
            result.1 = Some(color_group.clone());
        } else {
            let group = create_group(*color_idx);
            used_colors.insert(foreground, group.clone());
            result.1 = Some(group);
            *color_idx += 1;
        }
    }

    result
}

fn parse_color(color: String, bg_color: colors::RGBA, scaler: f32) -> String {
    if !colors::is_rgba(color.as_str()) {
        return color;
    }

    // If the color is RGBA, we blend it with the background
    if let Ok(colors::RGBA { r, g, b, a }) = colors::from_hex_string(color.as_str()) {
        // TODO: Use opacity here
        let mut color = colors::blend(bg_color, colors::RGBA { r, g, b, a });
        colors::scale(&mut color, scaler);
        colors::to_rgb_hex_string(color)
    } else {
        color.to_string()
    }
}
