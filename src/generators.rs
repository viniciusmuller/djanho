use crate::{colors, decoder, vim};

pub fn generate_vimscript_config(target: &mut String, theme: decoder::VSCodeTheme) -> String {
    // TODO: Should this be cleared?
    target.push_str("highlight clear\n");
    generate_config(theme, target, &vim::vim_highlight, &vim::vim_link)
}

pub fn generate_lua_config(target: &mut String, theme: decoder::VSCodeTheme) -> String {
    target.push_str("local cmd = vim.cmd\n\n");
    target.push_str("cmd[[highlight clear]]\n");
    generate_config(theme, target, &vim::lua_highlight, &vim::lua_link)
}

fn generate_config(
    theme: decoder::VSCodeTheme,
    target: &mut String,
    mapper: &dyn Fn(&vim::Highlight) -> String,
    linker: &dyn Fn(&str, &str) -> String,
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
                                append_highlight(
                                    group,
                                    &background,
                                    &foreground,
                                    &text_style,
                                    target,
                                    mapper,
                                );
                            }
                        }
                    }
                    Some(decoder::VSCodeScope::Single(scope)) => {
                        if let Some(group) = vim::map_groups(&scope) {
                            append_highlight(
                                group,
                                &background,
                                &foreground,
                                &text_style,
                                target,
                                mapper,
                            );
                        }
                    }
                    _ => (),
                }
            }
        }
    }

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

    // Linking highlight groups
    let links = vim::links();
    for (group, target_group) in links {
        target.push_str(&linker(group, target_group))
    }

    target.to_owned()
}

fn append_highlight(
    group: &'static str,
    background: &str,
    foreground: &str,
    text_style: &str,
    target: &mut String,
    mapper: &dyn Fn(&vim::Highlight) -> String,
) {
    let options = vim::Highlight {
        group,
        background: background.to_string(),
        foreground: foreground.to_string(),
        text_style: text_style.to_string(),
    };
    target.push_str(&mapper(&options))
}
