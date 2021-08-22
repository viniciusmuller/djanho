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
        "brace" => mk_group("parens"),
        "entity.name.function" => mk_group("Function"),
        "keyword.operator" => mk_group("Function"),
        "meta.type.name" => mk_group("Type"),
        _ => None,
    }
}

pub fn combined_options() -> Vec<CombinedOption> {
    vec![
        mk_combined("StatusLine", "statusBar.foreground", "statusBar.background"),
        mk_combined(
            "LineNr",
            "editorLineNumber.foreground",
            "editorLineNumber.background",
        ),
        mk_combined("SignColumn", "VIM_NONE", "editor.background"),
        mk_combined("Normal", "editor.foreground", "editor.background"),
        mk_combined("Visual", "VIM_NONE", "editor.selectionBackground"),
        // mk_combined("CursorLine", "VIM_NONE", "editor.selectionBackground"),
        mk_combined("ColorColumn", "VIM_NONE", "editor.selectionBackground"),
        mk_combined("TabLineSel", "tab.activeBackground", "tab.activeForeground"),
    ]
}

pub fn mk_combined(vim_group: &str, foreground: &str, background: &str) -> CombinedOption {
    CombinedOption {
        vim_group: vim_group.to_owned(),
        combinator_foreground: foreground.to_owned(),
        combinator_background: background.to_owned(),
    }
}

pub fn mk_group(group: &str) -> Option<String> {
    Some(group.to_owned())
}

pub fn highlight(options: Highlight) -> String {
    format!(
        "highlight {}{}{}{}\n",
        options.group,
        mk_option("guibg", options.background),
        mk_option("guifg", options.foreground),
        mk_option("gui", options.text_style)
    )
}

fn mk_option(option_type: &str, value: String) -> String {
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
