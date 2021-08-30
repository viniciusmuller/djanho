/// A tuple containing (VSCode Token, Vim target highlight, fallback group)
type VSCodeToken = (&'static str, &'static str, Option<&'static str>);
/// A tuple containing (Vim target, VSCode UI FG, VSCode UI BG, Opacity)
pub type VSCodeColor = (
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
            ("keyword.operator", "Operator", Some("Keyword")),
            ("operator", "Operator", Some("Keyword")),
            ("label", "Label", None),
            // ("keyword.control", "Conditional", None),
            ("conditional", "Conditional", Some("Operator")),
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
            ("namespace", "TSNamespace", Some("TSType")),
            // -- TSNone               { };    -- TODO: docs
            ("property", "TSField", Some("Constant")),
            ("parameter", "TSParameter", Some("Constant")),
            ("keyword.control", "Repeat", Some("Conditional")),
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
                Some("statusBar.background"),
                Some("statusBar.foreground"),
                1.0,
            ),
            (
                "WildMenu",
                Some("editor.background"),
                Some("editor.foreground"),
                0.7,
            ),
            // Popup menu
            (
                "Pmenu",
                Some("editor.background"),
                Some("editor.foreground"),
                0.8,
            ),
            (
                "PmenuSel",
                Some("editor.foreground"),
                Some("tab.activeBackground"),
                1.0,
            ),
            (
                "PmenuThumb",
                Some("editor.background"),
                Some("editor.foreground"),
                1.0,
            ),
            // Diffs
            (
                "DiffAdd",
                Some("diffEditor.insertedTextBackground"),
                None,
                0.8,
            ),
            (
                "DiffDelete",
                Some("diffEditor.removedTextBackground"),
                None,
                0.8,
            ),
            // Normal and visual modes
            (
                "Normal",
                Some("editor.background"),
                Some("editor.foreground"),
                1.0,
            ),
            ("Visual", Some("editor.selectionBackground"), None, 0.5),
            // Misc
            ("CursorLine", Some("editor.selectionBackground"), None, 0.4),
            ("ColorColumn", Some("editor.selectionBackground"), None, 0.5),
            ("SignColumn", Some("editor.background"), None, 1.0),
            (
                "LineNr",
                Some("editorLineNumber.background"),
                Some("editorLineNumber.foreground"),
                1.0,
            ),
            // Tabs
            (
                "TabLine",
                Some("tab.inactiveBackground"),
                Some("tab.inactiveForeground"),
                1.0,
            ),
            (
                "TabLineSel",
                Some("tab.activeForeground"),
                Some("tab.activeBackground"),
                1.0,
            ),
            (
                "TabLineFill",
                Some("tab.inactiveBackground"),
                Some("tab.inactiveForeground"),
                1.0,
            ),
            // Treesitter
            ("TSPunctDelimiter", None, Some("editor.foreground"), 1.0),
        ],
        links: vec![
            // Vim builtins
            ("Folded", "Comment"),
            ("Whitespace", "Comment"),
            ("NonText", "Comment"),
            ("CursorLineNr", "Identifier"),
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
            ("TSRepeat", "Repeat"),
            ("TSConstBuiltin", "TSVariableBuiltin"),
        ],
    }
}

#[derive(Debug)]
pub struct VimHighlight {
    pub group: String,
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub text_style: Option<String>,
}

pub fn map_font_styles(style: &Option<String>) -> Option<String> {
    if let Some(style) = style {
        let style = style.as_str();
        let result = match style {
            "italic" => Some("italic".to_string()),
            "bold" => Some("bold".to_string()),
            _ => None,
        };
        if result.is_some() {
            result
        } else {
            None
        }
    } else {
        None
    }
}
