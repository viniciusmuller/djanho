use crate::vim::map_font_styles;
use crate::{generator::ConfigGenerator, vim::VimHighlight};

pub struct VimscriptGenerator {
    buffer: String,
}

impl Default for VimscriptGenerator {
    fn default() -> VimscriptGenerator {
        let mut _self = VimscriptGenerator {
            buffer: String::new(),
        };
        _self.buffer += "highlight clear\n";
        _self
    }
}

impl ConfigGenerator for VimscriptGenerator {
    fn collect(&self) -> String {
        self.buffer.to_string()
    }
    fn link(&mut self, group: &str, target: &str) {
        self.buffer += link(group, target).as_str()
    }
    fn highlight(&mut self, options: &VimHighlight) {
        self.buffer += highlight(options).as_str()
    }
    fn variable(&mut self, name: String, color: String) {
        self.buffer += create_variable(name, color).as_str()
    }
    fn newline(&mut self) {
        self.buffer += "\n";
    }
}

fn highlight(options: &VimHighlight) -> String {
    let guibg = mk_option("guibg", &options.background);
    let guifg = mk_option("guifg", &options.foreground);
    let gui = mk_option("gui", &map_font_styles(&options.text_style));

    if guibg.is_empty() && guifg.is_empty() && gui.is_empty() {
        return String::new();
    }

    // TODO: Will need to use execute here (maybe create a vimscript function)
    format!("highlight {}{}{}{}\n", options.group, guibg, guifg, gui)
}

fn mk_option(option_type: &str, value: &Option<String>) -> String {
    if let Some(option) = value {
        format!(" {}={}", option_type, option)
    } else {
        String::new()
    }
}

fn link(group: &str, target: &str) -> String {
    format!("highlight! link {} {}\n", group, target)
}

fn create_variable(name: String, color: String) -> String {
    format!("let s:{} = '{}'\n", name, color)
}
