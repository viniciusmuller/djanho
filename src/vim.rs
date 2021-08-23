// TODO: Maybe use traits to create config generators?
pub fn map_groups(group: &str) -> Option<String> {
    match group {
        "comment" => mk_group("Comment"),
        "constant" => mk_group("Constant"),
        "keyword" => mk_group("Keyword"),
        "string" => mk_group("String"),
        "invalid" => mk_group("Error"),
        "brace" => mk_group("parens"),
        "macro" => mk_group("Macro"),
        "number" => mk_group("Number"),
        "entity.name.function" => mk_group("Function"),
        "keyword.operator" => mk_group("Operator"),
        "keyword.control" => mk_group("Conditional"),

        "struct" => mk_group("Structure"),
        "enum" => mk_group("Structure"),

        "variable" => mk_group("Identifier"),

        // Type
        "type" => mk_group("Type"),
        "entity.type.name" => mk_group("Type"),
        "meta.type.name" => mk_group("Type"),
        "storage" => mk_group("Type"),

        // TODO: Treesitter support

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
        mk_combined("WildMenu", "editor.foreground", "editor.background", 0.7),
        // Popup menu
        mk_combined("Pmenu", "editor.foreground", "editor.background", 0.8),
        mk_combined("PmenuSel", "tab.activeBackground", "editor.foreground", 1.0),
        mk_combined("PmenuThumb", "editor.foreground", "editor.background", 1.0),
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
    ]
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
    }
}

pub fn mk_group(group: &str) -> Option<String> {
    Some(group.to_owned())
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

pub struct VimOption {
    pub combinator_foreground: String,
    pub combinator_background: String,
    pub vim_group: String,
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
