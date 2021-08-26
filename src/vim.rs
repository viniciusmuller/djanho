// VSCode Token, Vim target highlight, fallback group
type VSCodeToken = (&'static str, &'static str, Option<&'static str>);
// Vim target, VSCode UI FG, VSCode UI BG, Opacity
type VSCodeColor = (
    &'static str,
    Option<&'static str>,
    Option<&'static str>,
    f32,
);
// Vim target group, Vim source group
type VimLink = (&'static str, &'static str);

#[derive(Debug)]
struct NewHighlight {
    tokens: Vec<VSCodeToken>,
    colors: Vec<VSCodeColor>,
    links: Vec<VimLink>,
}

fn highlights() -> NewHighlight {
    NewHighlight {
        tokens: vec![
            ("comment", "Comment", None),
            ("macro", "Macro", Some("Function")),
        ],
        colors: vec![("Statusline", Some("editor.background"), None, 1.0)],
        links: vec![
            // Vim builtins
            ("Folded", "Comment"),
            ("Whitespace", "Comment"),
            ("NonText", "Comment"),
            ("CursorLineNr", "Function"),
            // Treesitter
            ("TSFuncMacro", "Macro"),
            ("TSFunction", "Function"),
            ("TSType", "Type"),
            ("TSLabel", "Type"),
            ("TSVariable", "Variable"),
            ("TSComment", "Comment"),
            ("TSProperty", "TSField"),
            ("TSParameterReference", "TSParameter"),
            ("TSOperator", "Operator"),
            ("TSNumber", "Number"),
            ("TSFloat", "Number"),
            ("TSString", "String"),
            ("TSConditional", "Conditional"),
            ("TSConstant", "Constant"),
            ("TSTag", "MyTag"),
            ("TSPunctBracket", "MyTag"),
            ("TSPunctSpecial", "TSPunctDelimiter"),
            ("TSTagDelimiter", "Type"),
            ("TSKeyword", "Keyword"),
            ("TSConstBuiltin", "TSVariableBuiltin"),
        ],
    }
}

// TODO: Learn how to use rust macros and turn this into a beautiful DSL.
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
        "entity.name.function.macro" => "Macro",

        "number" => "Number",
        "constant.numeric" => "Number",

        "brackethighlighter.tag" => "MyTag",
        "brackethighlighter.angle" => "MyTag",
        "brackethighlighter.round" => "MyTag",
        "brackethighlighter.square" => "MyTag",

        "entity.name.function" => "Function",
        "function" => "Function",

        "keyword.operator" => "Operator",
        "operator" => "Operator",

        "label" => "Label",

        "keyword.control" => "Conditional",
        "conditional" => "Conditional",
        "keyword.control.conditional" => "Conditional",

        "struct" => "Structure",
        "enum" => "Structure",
        "variable" => "Identifier",
        // Type
        "type" => "Type",
        "typeParameter" => "Type",
        "entity.type.name" => "Type",
        "entity.name.type" => "Type",
        "meta.type.name" => "Type",
        "storage" => "Type",

        // -- TSAnnotation         { };    -- For C++/Dart attributes, annotations that can be attached to the code to denote some kind of meta information.
        // -- TSAttribute          { };    -- (unstable) TODO: docs
        // -- TSBoolean            { };    -- For booleans.
        "constant.character" => "TSCharacter",
        // -- TSConstructor        { };    -- For constructor calls and definitions: ` { }` in Lua, and Java constructors.
        // -- TSConstMacro         { };    -- For constants that are defined by macros: `NULL` in C.
        // -- TSError              { };    -- For syntax/parser errors.
        // -- TSException          { };    -- For exception related keywords.
        "function.defaultLibrary" => "TSFuncBuiltin",
        // -- TSInclude            { };    -- For includes: `#include` in C, `use` or `extern crate` in Rust, or `require` in Lua.
        "keyword.declaration" => "TSKeywordFunction",
        "method" => "TSMethod",
        "namespace" => "TSNamespace",
        // -- TSNone               { };    -- TODO: docs
        "property" => "TSField",

        "parameter" => "TSParameter",
        // -- TSRepeat             { };    -- For keywords related to loops.
        "regex" => "TSStringRegex",
        // -- TSStringEscape       { };    -- For escape characters within a string.
        // -- TSSymbol             { };    -- For identifiers referring to symbols or atoms.
        "type.defaultLibrary" => "TSTypeBuiltin",
        "variable.readonly.defaultLibrary" => "TSVariableBuiltin",
        // -- TSText               { };    -- For strings considered text in a markup language.
        // -- TSEmphasis           { };    -- For text to be represented with emphasis.
        // -- TSUnderline          { };    -- For text to be represented with an underline.
        // -- TSStrike             { };    -- For strikethrough text.
        // -- TSTitle              { };    -- Text that is part of a title.
        // -- TSLiteral            { };    -- Literal text.
        // -- TSURI                { };    -- Any URI like a link or email.

        // TODO: Add linking group fallback, eg: Keyword fallbacks to Conditional
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
        // Vim builtins
        ("Folded", "Comment"),
        ("Whitespace", "Comment"),
        ("NonText", "Comment"),
        ("CursorLineNr", "Function"),
        // Treesitter
        ("TSFuncMacro", "Macro"),
        ("TSFunction", "Function"),
        ("TSType", "Type"),
        ("TSLabel", "Type"),
        ("TSVariable", "Variable"),
        ("TSComment", "Comment"),
        ("TSProperty", "TSField"),
        ("TSParameterReference", "TSParameter"),
        ("TSOperator", "Operator"),
        ("TSNumber", "Number"),
        ("TSFloat", "Number"),
        ("TSString", "String"),
        ("TSConditional", "Conditional"),
        ("TSConstant", "Constant"),
        ("TSTag", "MyTag"),
        ("TSPunctBracket", "MyTag"),
        ("TSPunctSpecial", "TSPunctDelimiter"),
        ("TSTagDelimiter", "Type"),
        ("TSKeyword", "Keyword"),
        ("TSConstBuiltin", "TSVariableBuiltin"),
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
        // Treesitter
        mk_combined("TSPunctDelimiter", "editor.foreground", "VIM_NONE", 1.0),
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
