use crate::{colors, decoder, vim};

pub fn generate_vimscript_config(target: &mut String, theme: decoder::VSCodeTheme) -> String {
    generate_config(theme, target, &vim::vim_highlight)
}

pub fn generate_lua_config(target: &mut String, theme: decoder::VSCodeTheme) -> String {
    target.push_str("local cmd = vim.cmd\n\n");
    generate_config(theme, target, &vim::lua_highlight)
}

fn generate_config(
    theme: decoder::VSCodeTheme,
    target: &mut String,
    mapper: &dyn Fn(&vim::Highlight) -> String,
) -> String {
    for token in theme.token_colors {
        match token {
            decoder::VSCodeHighlight {
                scope,
                settings:
                    decoder::VSCodeScopeSettings {
                        background: bg,
                        foreground: fg,
                        font_style: fs,
                    },
            } => {
                let background = bg.unwrap_or_default();
                let foreground = fg.unwrap_or_default();
                let text_style = fs.unwrap_or_default();

                match scope {
                    Some(decoder::VSCodeScope::Multiple(scopes)) => {
                        for group in scopes {
                            if let Some(group) = vim::map_groups(&group) {
                                let options = vim::Highlight {
                                    group: group.to_owned(),
                                    background: background.clone(),
                                    foreground: foreground.clone(),
                                    text_style: text_style.clone(),
                                };
                                target.push_str(&mapper(&options))
                            }
                        }
                    }
                    Some(decoder::VSCodeScope::Single(scope)) => {
                        if let Some(group) = vim::map_groups(&scope) {
                            let options = vim::Highlight {
                                group: group.to_owned(),
                                background: background.clone(),
                                foreground: foreground.clone(),
                                text_style: text_style.clone(),
                            };
                            target.push_str(&mapper(&options))
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    let combined_opts = vim::combined_options();
    let mut default_bg = colors::from_hex_string("#000000ff").unwrap();

    if let Some(theme_colors) = theme.colors {
        for combined in combined_opts {
            let mut foreground: String = theme_colors
                .get(&combined.combinator_foreground)
                .cloned()
                .unwrap_or_default();

            let mut background: String = theme_colors
                .get(&combined.combinator_background)
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
                    color = colors::scale(color, combined.color_scaler);
                    foreground = colors::to_rgb_hex_string(color);
                }
            }

            if colors::is_rgba(&background.to_string()) {
                if let Ok(colors::RGBA { r, g, b, a }) =
                    colors::from_hex_string(&background.to_string())
                {
                    let mbg = colors::RGBA { r, g, b, a };
                    let mut color = colors::blend(default_bg, mbg);
                    color = colors::scale(color, combined.color_scaler);

                    background = colors::to_rgb_hex_string(color)
                }
            }

            let options = vim::Highlight {
                group: combined.vim_group,
                foreground,
                background,
                text_style: String::new(),
            };

            target.push_str(&mapper(&options))
        }
    }

    target.to_owned()
}
