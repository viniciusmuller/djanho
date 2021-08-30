use indoc::indoc;

use crate::vim::map_font_styles;
use crate::{generator::ConfigGenerator, vim::VimHighlight};

#[derive(Debug)]
pub struct LuaGenerator {
    // This should be a mutable reference, but the borrow checker
    // is too strong, I can't beat him :(
    buffer: String,
}

impl Default for LuaGenerator {
    fn default() -> LuaGenerator {
        let mut _self = LuaGenerator {
            buffer: String::new(),
        };
        _self.buffer += indoc! {"
            vim.cmd[[highlight clear]]

            local highlight = function(group, bg, fg, attr, sp)
                fg = fg and 'guifg=' .. fg or ''
                bg = bg and 'guibg=' .. bg or ''
                attr = attr and 'gui=' .. attr or ''
                sp = sp and 'guisp=' .. sp or ''

                vim.api.nvim_command('highlight ' .. group .. ' '.. fg .. ' ' .. bg .. ' '.. attr .. ' ' .. sp)
            end

            local link = function(target, group)
                vim.api.nvim_command('highlight link ' .. target .. ' '.. group)
            end\n"
        };
        _self
    }
}

impl ConfigGenerator for LuaGenerator {
    fn collect(&self) -> String {
        self.buffer.to_string()
    }
    fn link(&mut self, group: &str, target: &str) {
        self.buffer += link(group, target).as_str()
    }
    fn highlight(&mut self, options: &VimHighlight) {
        self.buffer += &highlight(options)
    }
    fn variable(&mut self, name: String, color: String) {
        self.buffer += &create_variable(name, color)
    }
    fn newline(&mut self) {
        self.buffer += "\n"
    }
}

fn highlight(options: &VimHighlight) -> String {
    let guibg = mk_option(&options.background);
    let guifg = mk_option(&options.foreground);

    let text_style = &map_font_styles(&options.text_style).unwrap_or_else(|| "nil".to_string());
    let text_style = if text_style == "nil" {
        text_style.clone()
    } else {
        format!("'{}'", text_style)
    };

    if guibg == "nil" && guifg == "nil" && text_style == "nil" {
        return String::new();
    }

    format!(
        "highlight('{}', {}, {}, {})\n",
        options.group, guibg, guifg, text_style
    )
}

fn mk_option(value: &Option<String>) -> String {
    if let Some(option) = value {
        option.to_string()
    } else {
        "nil".to_owned()
    }
}

fn link(group: &str, target: &str) -> String {
    format!("link('{}', '{}')\n", group, target)
}

fn create_variable(name: String, color: String) -> String {
    format!("local {} = '{}'\n", name, color)
}
