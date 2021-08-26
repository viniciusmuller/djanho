use crate::{
    colors, decoder, generator::ConfigGenerator, lua::LuaGenerator, vim,
    vimscript::VimscriptGenerator,
};
use std::collections::HashMap;

type UsedColors = HashMap<String, String>;

pub fn generate_vimscript_config(theme: decoder::VSCodeTheme) -> String {
    // TODO: Maybe find a way to have kinda a trait-generic variable to store
    // the generator struct
    let mut generator = VimscriptGenerator::new();
    generate_config(theme, &mut generator);
    generator.collect()
}

pub fn generate_lua_config(theme: decoder::VSCodeTheme) -> String {
    let mut generator = LuaGenerator::new();
    generate_config(theme, &mut generator);
    generator.collect()
}

fn generate_config<T: ConfigGenerator>(theme: decoder::VSCodeTheme, generator: &mut T) {
    let highlights = vim::highlights();
    let mut color_idx = 0;
    let mut used_colors: UsedColors = HashMap::new();

    // Parse process:
    // Identify matches between the specified vim groups and the colorscheme
    // and save them into a hashmap (1pass)
    // Create color variables in vimscript
    // For each group, check if it's in the matched colors, if it is,
    // add it to the vimscript stuff using the generated variable by step 1 (1pass)
    // Done

    for token in theme.tokens {
        if let decoder::VSCodeHighlight {
            scope: Some(scope),
            settings:
                decoder::VSCodeScopeSettings {
                    background: bg,
                    foreground: fg,
                    ..
                },
        } = token
        {
            let background = bg.unwrap_or_default();
            let foreground = fg.unwrap_or_default();

            if let decoder::VSCodeScope::Single(group) = scope {
                if let Some(_) = vim::map_groups(&group) {
                    add_to_hashmap(
                        &mut color_idx,
                        &mut used_colors,
                        background.to_string(),
                        foreground.to_string(),
                    );
                }
            } else if let decoder::VSCodeScope::Multiple(scopes) = scope {
                for group in scopes {
                    if let Some(_) = vim::map_groups(&group) {
                        add_to_hashmap(
                            &mut color_idx,
                            &mut used_colors,
                            background.to_string(),
                            foreground.to_string(),
                        );
                    }
                }
            }
        }
    }

    dbg!(used_colors);

    let combined_opts = vim::combined_options();
    let mut default_bg = colors::from_hex_string("#444444ff").unwrap();

    if let Some(theme_colors) = theme.colors {
        for combined in combined_opts {
            let mut foreground: String = theme_colors
                .get(&combined.combinator_foreground.to_owned())
                .cloned()
                .unwrap_or_default();

            let mut background: String = theme_colors
                .get(&combined.combinator_background.to_owned())
                .cloned()
                .unwrap_or_default();

            if combined.combinator_background == "editor.background" {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&foreground.to_string())
                {
                    default_bg = colors::RGBA { r, g, b, a }
                }
            }

            // If the color is RGBA, we blend it with the background
            if colors::is_rgba(&foreground.to_string()) {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&foreground.to_string())
                {
                    let mut color = colors::blend(default_bg, colors::RGBA { r, g, b, a });
                    color = colors::scale(&color, combined.color_scaler);
                    foreground = colors::to_rgb_hex_string(color);
                }
            }

            if colors::is_rgba(&background.to_string()) {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&background.to_string())
                {
                    let mbg = colors::RGBA { r, g, b, a };
                    let mut color = colors::blend(default_bg, mbg);
                    color = colors::scale(&color, combined.color_scaler);

                    background = colors::to_rgb_hex_string(color)
                }
            }

            let options = vim::VimHighlight {
                group: combined.vim_group,
                foreground,
                background,
                text_style: String::new(),
            };

            // target.push_str(&mapper(&options));
            generator.highlight(&options);
        }
    }

    // Linking highlight groups (this boi is ok)
    for (group, target_group) in highlights.links {
        // target.push_str(&linker(group, target_group));
        generator.link(group, target_group);
    }
}

fn append_highlight<T: ConfigGenerator>(
    group: &'static str,
    background: &str,
    foreground: &str,
    text_style: &str,
    generator: &mut T,
    // target: &mut String,
    // mapper: &dyn Fn(&vim::VimHighlight) -> String,
) {
    let options = vim::VimHighlight {
        group,
        background: background.to_string(),
        foreground: foreground.to_string(),
        text_style: text_style.to_string(),
    };
    generator.highlight(&options);
    // target.push_str(&mapper(&options))
}

fn add_to_hashmap(
    idx: &mut i32,
    used_colors: &mut UsedColors,
    background: String,
    foreground: String,
) {
    if !background.is_empty() {
        add_and_increase_idx(used_colors, idx, background.to_string())
    }
    if !foreground.is_empty() {
        add_and_increase_idx(used_colors, idx, foreground.to_string())
    }
}

fn add_and_increase_idx(used_colors: &mut UsedColors, idx: &mut i32, value: String) {
    used_colors.insert(format!("Color{}", idx), value);
    *idx += 1;
}
