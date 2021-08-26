use crate::utils::map_font_styles;
use crate::{generator::ConfigGenerator, vim::VimHighlight};

pub struct VimscriptGenerator {
    buffer: String,
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
    fn newline(&mut self) -> () {
        self.buffer += "\n";
    }
}

impl VimscriptGenerator {
    pub fn new() -> VimscriptGenerator {
        let mut _self = VimscriptGenerator {
            buffer: String::new(),
        };
        _self.buffer += "highlight clear\n";
        _self
    }
}

fn highlight(options: &VimHighlight) -> String {
    let guibg = mk_option("guibg", &options.background);
    let guifg = mk_option("guifg", &options.foreground);
    let gui = mk_option("gui", &map_font_styles(&options.text_style));

    if guibg.is_empty() && guifg.is_empty() && gui.is_empty() {
        return String::new();
    }

    format!("highlight {}{}{}{}\n", options.group, guibg, guifg, gui)
}

fn mk_option(option_type: &str, value: &str) -> String {
    if value.is_empty() {
        String::new()
    } else {
        format!(" {}={}", option_type, value)
    }
}

fn link(group: &str, target: &str) -> String {
    format!("highlight! link {} {}\n", group, target)
}
