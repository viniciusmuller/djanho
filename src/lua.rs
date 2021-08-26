use crate::utils::map_font_styles;
use crate::{generator::ConfigGenerator, vim::VimHighlight};

#[derive(Debug)]
pub struct LuaGenerator {
    // This should be a mutable reference, but the borrow checker
    // is too strong, I can't beat him :(
    buffer: String,
}

impl LuaGenerator {
    pub fn new() -> LuaGenerator {
        let mut _self = LuaGenerator {
            buffer: String::new(),
        };
        _self.buffer += "local highlight = function(group, fg, bg, attr, sp)
    fg = fg and 'guifg=' .. fg or ''
    bg = bg and 'guibg=' .. bg or ''
    attr = attr and 'gui=' .. attr or ''
    sp = sp and 'guisp=' .. sp or ''

    vim.api.nvim_command('highlight ' .. group .. ' '.. fg .. ' ' .. bg .. ' '.. attr .. ' ' .. sp)
end

local link = function(target, group)
    vim.api.nvim_command('highlight link ' .. target .. ' '.. group)
end

vim.cmd[[highlight clear]]\n";
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
    fn newline(&mut self) {
        self.buffer += "\n"
    }
}

fn highlight(options: &VimHighlight) -> String {
    let guibg = mk_option(&options.background);
    let guifg = mk_option(&options.foreground);
    let gui = mk_option(&map_font_styles(&options.text_style));

    if guibg == "nil" && guifg == "nil" && gui == "nil" {
        return String::new();
    }

    format!(
        "highlight('{}', {}, {}, {})\n",
        options.group, guibg, guifg, gui
    )
}

fn mk_option(value: &str) -> String {
    if value.is_empty() {
        "nil".to_owned()
    } else {
        format!("'{}'", value)
    }
}

fn link(group: &str, target: &str) -> String {
    format!("link('{}', '{}')\n", group, target)
}
