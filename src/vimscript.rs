use indoc::indoc;

use crate::highlights::map_font_styles;
use crate::{generator::ConfigGenerator, highlights::VimHighlight};

pub struct VimscriptGenerator {
    buffer: String,
}

impl Default for VimscriptGenerator {
    fn default() -> VimscriptGenerator {
        let mut _self = VimscriptGenerator {
            buffer: String::new(),
        };
        _self.buffer += indoc! {
          "
            highlight clear

            function s:highlight(group, bg, fg, style)
              let gui = a:style == '' ? '' : 'gui=' . a:style
              let fg = a:fg == '' ? '' : 'guifg=' . a:fg
              let bg = a:bg == '' ? '' : 'guibg=' . a:bg
              exec 'hi ' . a:group . ' ' . bg . ' ' . fg  . ' ' . gui
            endfunction
          "
        };
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
    let guibg = helper(&options.background);
    let guifg = helper(&options.foreground);
    let gui = map_font_styles(&options.text_style).unwrap_or_default();

    if guibg == "''" && guifg == "''" && gui.is_empty() {
        return String::new();
    }

    format!(
        "call s:highlight('{}', {}, {}, '{}')\n",
        options.group, guibg, guifg, gui
    )
}

fn helper(value: &Option<String>) -> String {
    if let Some(color_variable) = value {
        format!("s:{}", color_variable)
    } else {
        "''".to_string()
    }
}

fn link(group: &str, target: &str) -> String {
    format!("highlight! link {} {}\n", group, target)
}

fn create_variable(name: String, color: String) -> String {
    format!("let s:{} = '{}'\n", name, color)
}
