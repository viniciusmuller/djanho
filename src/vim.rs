/// A tuple containing (VSCode Token, Vim target highlight, fallback group)
type VSCodeToken = (&'static str, &'static str, Option<&'static str>);
/// A tuple containing (Vim target, VSCode UI FG, VSCode UI BG, Opacity)
type VSCodeColor = (
    &'static str,
    Option<&'static str>,
    Option<&'static str>,
    f32,
);
/// A tuple containing (Vim target group, Vim source group)
type VimLink = (&'static str, &'static str);

#[derive(Debug)]
pub struct Highlight {
    pub tokens: Vec<VSCodeToken>,
    pub colors: Vec<VSCodeColor>,
    pub links: Vec<VimLink>,
}

pub fn highlights() -> Highlight {
    Highlight {
        tokens: vec![
            ("comment", "Comment", None),
            ("macro", "Macro", Some("Function")),
            ("comment", "Comment", None),
            ("constant", "Constant", None),
            ("keyword", "Keyword", None),
            ("string", "String", None),
            ("invalid", "Error", None),
            ("brace", "parens", None),
            ("macro", "Macro", None),
            ("entity.name.function.macro", "Macro", None),
            ("number", "Number", None),
            ("constant.numeric", "Number", None),
            ("brackethighlighter.tag", "MyTag", None),
            ("brackethighlighter.angle", "MyTag", None),
            ("brackethighlighter.round", "MyTag", None),
            ("brackethighlighter.square", "MyTag", None),
            ("entity.name.function", "Function", None),
            ("function", "Function", None),
            ("keyword.operator", "Operator", None),
            ("operator", "Operator", None),
            ("label", "Label", None),
            ("keyword.control", "Conditional", None),
            ("conditional", "Conditional", None),
            ("keyword.control.conditional", "Conditional", None),
            ("struct", "Structure", None),
            ("enum", "Structure", None),
            ("variable", "Identifier", None),
            // Type
            ("type", "Type", None),
            ("typeParameter", "Type", None),
            ("entity.type.name", "Type", None),
            ("entity.name.type", "Type", None),
            ("meta.type.name", "Type", None),
            ("storage", "Type", None),
            // -- TSAnnotation         { };    -- For C++/Dart attributes, annotations that can be attached to the code to denote some kind of meta information.
            // -- TSAttribute          { };    -- (unstable) TODO: docs
            // -- TSBoolean            { };    -- For booleans.
            ("constant.character", "TSCharacter", None),
            // -- TSConstructor        { };    -- For constructor calls and definitions: ` { }` in Lua, and Java constructors.
            // -- TSConstMacro         { };    -- For constants that are defined by macros: `NULL` in C.
            // -- TSError              { };    -- For syntax/parser errors.
            // -- TSException          { };    -- For exception related keywords.
            ("function.defaultLibrary", "TSFuncBuiltin", None),
            // -- TSInclude            { };    -- For includes: `#include` in C, `use` or `extern crate` in Rust, or `require` in Lua.
            ("keyword.declaration", "TSKeywordFunction", None),
            ("method", "TSMethod", None),
            ("namespace", "TSNamespace", None),
            // -- TSNone               { };    -- TODO: docs
            ("property", "TSField", None),
            ("parameter", "TSParameter", None),
            // -- TSRepeat             { };    -- For keywords related to loops.
            ("regex", "TSStringRegex", None),
            // -- TSStringEscape       { };    -- For escape characters within a string.
            // -- TSSymbol             { };    -- For identifiers referring to symbols or atoms.
            ("type.defaultLibrary", "TSTypeBuiltin", None),
            (
                "variable.readonly.defaultLibrary",
                "TSVariableBuiltin",
                None,
            ),
            // -- TSText               { };    -- For strings considered text in a markup language.
            // -- TSEmphasis           { };    -- For text to be represented with emphasis.
            // -- TSUnderline          { };    -- For text to be represented with an underline.
            // -- TSStrike             { };    -- For strikethrough text.
            // -- TSTitle              { };    -- Text that is part of a title.
            // -- TSLiteral            { };    -- Literal text.
            // -- TSURI                { };    -- Any URI like a link or email.
        ],
        colors: vec![
            (
                "StatusLine",
                Some("statusBar.foreground"),
                Some("statusBar.background"),
                1.0,
            ),
            (
                "WildMenu",
                Some("editor.foreground"),
                Some("editor.background"),
                0.7,
            ),
            // Popup menu
            (
                "Pmenu",
                Some("editor.foreground"),
                Some("editor.background"),
                0.8,
            ),
            (
                "PmenuSel",
                Some("tab.activeBackground"),
                Some("editor.foreground"),
                1.0,
            ),
            (
                "PmenuThumb",
                Some("editor.foreground"),
                Some("editor.background"),
                1.0,
            ),
            // Diffs
            (
                "DiffAdd",
                None,
                Some("diffEditor.insertedTextBackground"),
                0.8,
            ),
            (
                "DiffDelete",
                None,
                Some("diffEditor.removedTextBackground"),
                0.8,
            ),
            // Normal and visual modes
            (
                "Normal",
                Some("editor.foreground"),
                Some("editor.background"),
                1.0,
            ),
            ("Visual", None, Some("editor.selectionBackground"), 0.5),
            // Misc
            ("CursorLine", None, Some("editor.selectionBackground"), 0.4),
            ("ColorColumn", None, Some("editor.selectionBackground"), 0.5),
            ("SignColumn", None, Some("editor.background"), 1.0),
            (
                "LineNr",
                Some("editorLineNumber.foreground"),
                Some("editorLineNumber.background"),
                1.0,
            ),
            // Tabs
            (
                "TabLine",
                Some("tab.inactiveForeground"),
                Some("tab.inactiveBackground"),
                1.0,
            ),
            (
                "TabLineSel",
                Some("tab.activeBackground"),
                Some("tab.activeForeground"),
                1.0,
            ),
            (
                "TabLineFill",
                Some("tab.inactiveForeground"),
                Some("tab.inactiveBackground"),
                1.0,
            ),
            // Treesitter
            ("TSPunctDelimiter", Some("editor.foreground"), None, 1.0),
        ],
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

pub fn lua_highlight(options: &VimHighlight) -> String {
    let guibg = mk_lua_option(&options.background);
    let guifg = mk_lua_option(&options.foreground);
    let gui = mk_lua_option(&map_font_styles(&options.text_style));

    if guibg == "nil" && guifg == "nil" && gui == "nil" {
        return String::new();
    }

    format!(
        "highlight('{}', {}, {}, {})\n",
        options.group, guibg, guifg, gui
    )
}

pub fn vim_highlight(options: &VimHighlight) -> String {
    let guibg = mk_vim_option("guibg", &options.background);
    let guifg = mk_vim_option("guifg", &options.foreground);
    let gui = mk_vim_option("gui", &map_font_styles(&options.text_style));

    if guibg.is_empty() && guifg.is_empty() && gui.is_empty() {
        return String::new();
    }

    format!("highlight {}{}{}{}\n", options.group, guibg, guifg, gui)
}

pub fn vim_link(group: &str, target: &str) -> String {
    format!("highlight! link {} {}\n", group, target)
}

pub fn lua_link(group: &str, target: &str) -> String {
    format!("link('{}', '{}')\n", group, target)
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

fn mk_vim_option(option_type: &str, value: &str) -> String {
    if value.is_empty() {
        String::new()
    } else {
        format!(" {}={}", option_type, value)
    }
}

fn mk_lua_option(value: &str) -> String {
    if value.is_empty() {
        "nil".to_owned()
    } else {
        format!("'{}'", value)
    }
}

#[derive(Debug)]
pub struct CombinedOption {
    pub vim_group: &'static str,
    pub combinator_foreground: &'static str,
    pub combinator_background: &'static str,
}

#[derive(Debug)]
pub struct VimHighlight {
    pub group: &'static str,
    pub background: String,
    pub foreground: String,
    pub text_style: String,
}
