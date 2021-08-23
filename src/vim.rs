pub fn map_groups(group: &str) -> Option<&'static str> {
    let result = match group {
        // https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide
        // :help highlight-groups
        "comment" => "Comment",
        "constant" => "Constant",
        "keyword" => "Keyword",
        "string" => "String",
        "invalid" => "Error",
        "brace" => "parens",
        "macro" => "Macro",
        "number" => "Number",
        "entity.name.function" => "Function",
        "keyword.operator" => "Operator",
        "keyword.control" => "Conditional",
        "struct" => "Structure",
        "enum" => "Structure",
        "variable" => "Identifier",
        // Type
        "type" => "Type",
        "entity.type.name" => "Type",
        "meta.type.name" => "Type",
        "storage" => "Type",

        // TODO: Treesitter support
        _ => "NO_MATCH",
    };

    if result != "NO_MATCH" {
        Some(result)
    } else {
        None
    }
}

pub fn links() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Folded", "Comment"),
        ("Whitespace", "Comment"),
        ("NonText", "Comment"),
        ("CursorLineNr", "Function"),
    ]
}

pub fn combined_options() -> Vec<VimOption> {
    vec![
        mk_combined(
            "StatusLine",
            "statusBar.foreground",
            "statusBar.background",
            1.0,
        ),
        mk_combined("WildMenu", "editor.foreground", "editor.background", 0.7),
        // Popup menu
        mk_combined("Pmenu", "editor.foreground", "editor.background", 0.8),
        mk_combined("PmenuSel", "tab.activeBackground", "editor.foreground", 1.0),
        mk_combined("PmenuThumb", "editor.foreground", "editor.background", 1.0),
        // Diffs
        mk_combined(
            "DiffAdd",
            "VIM_NONE",
            "diffEditor.insertedTextBackground",
            0.8,
        ),
        mk_combined(
            "DiffDelete",
            "VIM_NONE",
            "diffEditor.removedTextBackground",
            0.8,
        ),
        // Normal and visual modes
        mk_combined("Normal", "editor.foreground", "editor.background", 1.0),
        mk_combined("Visual", "VIM_NONE", "editor.selectionBackground", 0.5),
        // Misc
        mk_combined("CursorLine", "VIM_NONE", "editor.selectionBackground", 0.4),
        mk_combined("ColorColumn", "VIM_NONE", "editor.selectionBackground", 0.5),
        mk_combined("SignColumn", "VIM_NONE", "editor.background", 1.0),
        mk_combined(
            "LineNr",
            "editorLineNumber.foreground",
            "editorLineNumber.background",
            1.0,
        ),
        // Tabs
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
        // GitSigns
        // mk_combined(
        //     "GitSignsAdd",
        //     "editor.insertedTextForeground",
        //     "editor.background",
        //     1.0,
        // ),
        // mk_combined(
        //     "GitSignsDelete",
        //     "editor.deletedTextForeground",
        //     "editor.background",
        //     1.0,
        // ),
    ]
}

pub fn mk_combined(
    vim_group: &'static str,
    foreground: &'static str,
    background: &'static str,
    color_scaler: f32,
) -> VimOption {
    VimOption {
        vim_group,
        combinator_foreground: foreground,
        combinator_background: background,
        color_scaler,
    }
}

pub fn lua_highlight(options: &Highlight) -> String {
    let guibg = mk_option("guibg", &options.background);
    let guifg = mk_option("guifg", &options.foreground);
    let gui = mk_option("gui", &map_font_styles(&options.text_style));

    if guibg.is_empty() && guifg.is_empty() && gui.is_empty() {
        return String::new();
    }

    format!(
        "cmd[[highlight {}{}{}{}]]\n",
        options.group, guibg, guifg, gui
    )
}

pub fn vim_highlight(options: &Highlight) -> String {
    let guibg = mk_option("guibg", &options.background);
    let guifg = mk_option("guifg", &options.foreground);
    let gui = mk_option("gui", &map_font_styles(&options.text_style));

    if guibg.is_empty() && guifg.is_empty() && gui.is_empty() {
        return String::new();
    }

    format!("highlight {}{}{}{}\n", options.group, guibg, guifg, gui)
}

pub fn vim_link(group: &str, target: &str) -> String {
    format!("highlight! link {} {}\n", group, target)
}

pub fn lua_link(group: &str, target: &str) -> String {
    format!("cmd[[highlight! link {} {}]]\n", group, target)
}

pub struct VimOption {
    pub combinator_foreground: &'static str,
    pub combinator_background: &'static str,
    pub vim_group: &'static str,
    pub color_scaler: f32,
}

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
    pub vim_group: &'static str,
    pub combinator_foreground: &'static str,
    pub combinator_background: &'static str,
}

#[derive(Debug)]
pub struct Highlight {
    pub group: &'static str,
    pub background: String,
    pub foreground: String,
    pub text_style: String,
}
