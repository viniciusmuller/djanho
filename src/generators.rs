use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

type UsedColors = HashMap<String, String>;

use crate::{colors, decoder, vim};

pub fn generate_vimscript_config(target: &mut String, theme: decoder::VSCodeTheme) {
    // TODO: Should this be cleared?
    target.push_str("highlight clear\n");
    generate_config(theme, target, &vim::vim_highlight, &vim::vim_link)
}

pub fn generate_lua_config(target: &mut String, theme: decoder::VSCodeTheme) {
    // TODO: Create lua function to highlight
    target.push_str("local highlight = function(group, fg, bg, attr, sp)
  fg = fg and 'guifg=' .. fg or ''
  bg = bg and 'guibg=' .. bg or ''
  attr = attr and 'gui=' .. attr or ''
	sp = sp and 'guisp=' .. sp or ''

  vim.api.nvim_command('highlight ' .. group .. ' '.. fg .. ' ' .. bg .. ' '.. attr .. ' ' .. sp)
end

local link = function(target, group)
  vim.api.nvim_command('highlight link ' .. target .. ' '.. group)
end

vim.cmd[[highlight clear]]\n");
    generate_config(theme, target, &vim::lua_highlight, &vim::lua_link)
}

fn generate_config(
    theme: decoder::VSCodeTheme,
    target: &mut String,
    mapper: &dyn Fn(&vim::VimHighlight) -> String,
    linker: &dyn Fn(&str, &str) -> String,
) {
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

            target.push_str(&mapper(&options))
        }
    }

    // Linking highlight groups (this boi is ok)
    for (group, target_group) in highlights.links {
        target.push_str(&linker(group, target_group))
    }
}

fn append_highlight(
    group: &'static str,
    background: &str,
    foreground: &str,
    text_style: &str,
    target: &mut String,
    mapper: &dyn Fn(&vim::VimHighlight) -> String,
) {
    let options = vim::VimHighlight {
        group,
        background: background.to_string(),
        foreground: foreground.to_string(),
        text_style: text_style.to_string(),
    };
    target.push_str(&mapper(&options))
}

fn add_to_hashmap(
    idx: &mut i32,
    used_colors: &mut UsedColors,
    background: String,
    foreground: String,
) {
    if !background.is_empty() {
        used_colors.insert(format!("Color{}", idx), background.to_string());
        *idx += 1;
    }
    if !foreground.is_empty() {
        used_colors.insert(format!("Color{}", idx), foreground.to_string());
        *idx += 1;
    }
}
