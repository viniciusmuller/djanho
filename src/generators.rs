use crate::{decoder, vim};

pub fn generate_vimscript_config(theme: decoder::VSCodeTheme) -> String {
    let mut result = String::new();

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

                if scope.is_none() {
                    let options = vim::Highlight {
                        group: "Normal".to_owned(),
                        background,
                        foreground,
                        text_style,
                    };

                    result.push_str(&vim::highlight(options));
                    continue;
                }

                if let Some(group) = match scope {
                    Some(decoder::VSCodeScope::Multiple(scopes)) => vim::map_groups(&scopes[0]),
                    Some(decoder::VSCodeScope::Single(scope)) => vim::map_groups(&scope),
                    None => None,
                } {
                    let options = vim::Highlight {
                        group: group.to_owned(),
                        background,
                        foreground,
                        text_style,
                    };

                    result.push_str(&vim::highlight(options))
                }
            }
        }
    }

    let combined_opts = vim::combined_options();

    if let Some(colors) = theme.colors {
        for combined in combined_opts {
            let foreground: String = colors
                .get(&combined.combinator_foreground)
                .cloned()
                .unwrap_or_default();
            let background: String = colors
                .get(&combined.combinator_background)
                .cloned()
                .unwrap_or_default();

            let options = vim::Highlight {
                group: combined.vim_group,
                foreground,
                background,
                text_style: String::new(), // TODO: Maybe use it here
            };

            let line = vim::highlight(options);
            result.push_str(&line)
        }
    }

    result
}

pub fn generate_lua_config(theme: decoder::VSCodeTheme) {}
