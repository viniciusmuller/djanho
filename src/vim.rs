use std::default::Default;

use crate::colors;

// TODO: Maybe use traits to create config generators?
// TODO: Figure out how to properly use those strings here
pub fn map_groups(group: &str) -> Option<String> {
    // TODO: Will also need to parse an existing "colors" field
    match group {
        "comment" => mk_group("Comment"),
        "constant" => mk_group("Constant"),
        "keyword" => mk_group("Keyword"),
        "string" => mk_group("String"),
        "invalid" => mk_group("Error"),
        "variable" => mk_group("Variable"),
        "brace" => mk_group("parens"),
        "entity.name.function" => mk_group("Function"),
        "keyword.operator" => mk_group("Operator"),
        "keyword.control" => mk_group("Conditional"),

        "meta.type.name" => mk_group("Type"),
        "storage" => mk_group("Type"),
        _ => None,
    }
}

pub fn combined_options() -> Vec<VimOption> {
    vec![
        mk_combined(
            "StatusLine",
            "statusBar.foreground",
            "statusBar.background",
            1.0,
        ),
        mk_combined("Normal", "editor.foreground", "editor.background", 1.0),
        mk_combined("Visual", "VIM_NONE", "editor.selectionBackground", 0.5),
        mk_combined("CursorLine", "VIM_NONE", "editor.selectionBackground", 0.4),
        mk_combined("ColorColumn", "VIM_NONE", "editor.selectionBackground", 0.5),
        mk_combined("SignColumn", "VIM_NONE", "editor.background", 1.0),
        mk_combined(
            "LineNr",
            "editorLineNumber.foreground",
            "editorLineNumber.background",
            1.0,
        ),
        mk_combined(
            "TabLine",
            "tab.inactiveForeground",
            "tab.inactiveBackground",
            1.0,
        ),
        mk_combined(
            "TabLineSel",
            "tab.activeBackground",
            "tab.activeForeground",
            1.0,
        ),
        mk_combined(
            "TabLineFill",
            "tab.inactiveForeground",
            "tab.inactiveBackground",
            1.0,
        ),
    ]
}

#[macro_export]
macro_rules! vim_option {
    ( $( $x:expr ),* ) => {
        {
            // let mut temp_vec = Vec::new();
            VimOption { }
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn mk_combined(
    vim_group: &str,
    foreground: &str,
    background: &str,
    color_scaler: f32,
) -> VimOption {
    VimOption {
        vim_group: vim_group.to_owned(),
        combinator_foreground: foreground.to_owned(),
        combinator_background: background.to_owned(),
        color_scaler,
        color: None,
    }
}

pub fn mk_group(group: &str) -> Option<String> {
    Some(group.to_owned())
}

pub fn highlight(options: &Highlight) -> String {
    let guibg = mk_option("guibg", &options.background);
    let guifg = mk_option("guifg", &options.foreground);
    let gui = mk_option("gui", &map_font_styles(&options.text_style));

    if guibg.is_empty() && guifg.is_empty() && gui.is_empty() {
        return String::new();
    }

    format!("highlight {}{}{}{}\n", options.group, guibg, guifg, gui)
}

pub struct VimOption {
    pub combinator_foreground: String,
    pub combinator_background: String,
    pub vim_group: String,
    pub color_scaler: f32,
    pub color: Option<colors::RGBA>,
}

// impl Default for VimOption {
//     fn default() -> Self {
//         VimOption {
//             combinator_foreground: "VIM_NONE",
//             combinator_background: "VIM_NONE",
//             vim_group: "NO_GROUP",
//             color: colors::RGBA {
//                 r: 0,
//                 g: 0,
//                 b: 0,
//                 a: 1.0,
//             },
//             color_scaler: 1.0,
//         }
//     }
// }

fn map_font_styles(style: &str) -> String {
    match style {
        "italic" => "italic".to_owned(),
        "bold" => "bold".to_owned(),
        _ => String::new(),
    }
}

fn mk_option(option_type: &str, value: &str) -> String {
    if value.is_empty() {
        String::new()
    } else {
        format!(" {}={}", option_type, value)
    }
}

#[derive(Debug)]
pub struct CombinedOption {
    pub vim_group: String,
    pub combinator_foreground: String,
    pub combinator_background: String,
}

#[derive(Debug)]
pub struct Highlight {
    pub group: String,
    pub background: String,
    pub foreground: String,
    pub text_style: String,
}
