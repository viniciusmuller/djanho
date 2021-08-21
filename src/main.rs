use std::fs;
use std::fs::File;
use std::io::Write;

use clap::{App, Arg};
use json_comments::StripComments;
use serde::{Deserialize, Serialize};

fn main() {
    let matches = App::new("VSCode->Vim Colorscheme Converter")
        .version("0.1")
        .author("Vinícius Müller <vinigm.nho@gmail.com>")
        .about("Convert VSCode's JSON themes to Vimscript/Lua themes")
        .arg(
            Arg::with_name("FILENAME")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filepath = matches.value_of("FILENAME").unwrap();
    let theme = parse_file(filepath.to_string());
    let generated = generate_vimscript_config(theme);
    let mut f = File::create("generated.vim").expect("Unable to create file");
    f.write_all(generated.as_bytes())
        .expect("Unable to write data");
    // println!("{:?}", generated)
}

fn parse_file(filepath: String) -> VSCodeTheme {
    let body = fs::read_to_string(filepath).expect("Could not read the file.");
    let stripped = StripComments::new(body.as_bytes());
    let theme: VSCodeTheme = serde_json::from_reader(stripped).unwrap();
    theme
}

fn generate_vimscript_config(theme: VSCodeTheme) -> String {
    let mut result = String::new();

    for token in theme.token_colors {
        match token {
            VSCodeHighlight {
                scope,
                settings:
                    VSCodeScopeSettings {
                        background: bg,
                        foreground: fg,
                        font_style: fs,
                    },
            } => {
                // println!("{:?}", scope);
                let background = bg.unwrap_or_else(|| "".to_string());
                let foreground = fg.unwrap_or_else(|| "".to_string());
                let font_style = fs.unwrap_or_else(|| "".to_string());

                if scope.is_none() {
                    result.push_str(&highlight("Normal", background, foreground, font_style));
                    continue;
                }

                if let Some(group) = match scope {
                    Some(VSCodeScope::Multiple(scopes)) => map_groups(&scopes[0]),
                    Some(VSCodeScope::Single(scope)) => map_groups(&scope),
                    None => None,
                } {
                    result.push_str(&highlight(
                        &group.to_owned(),
                        background,
                        foreground,
                        font_style,
                    ))
                }
            }
        }
    }

    result
}

// TODO: Figure out how to properly use those strings here
fn map_groups(group: &str) -> Option<String> {
    // TODO: Will also need to parse an existing "colors" field
    match group {
        "comment" => Some("Comment".to_owned()),
        "constant" => Some("Constant".to_owned()),
        "keyword" => Some("Keyword".to_owned()),
        "string" => Some("String".to_owned()),
        "invalid" => Some("Error".to_owned()),
        "brace" => Some("parens".to_owned()),
        "entity.name.function" => Some("Function".to_owned()),
        "keyword.operator" => Some("Function".to_owned()),
        "meta.type.name" => Some("Type".to_owned()),
        _ => None,
    }
}

fn highlight(group: &str, bg: String, fg: String, fs: String) -> String {
    let gui = if fs.is_empty() {
        "".to_owned()
    } else {
        format!(" gui={}", fs)
    };
    format!("highlight {} guibg={} guifg={}{}\n", group, bg, fg, gui)
}

#[derive(Serialize, Deserialize, Debug)]
struct VSCodeTheme {
    name: Option<String>, // TODO: Use default names for those
    #[serde(rename = "type")]
    type_: Option<String>,
    #[serde(rename = "tokenColors")]
    token_colors: Vec<VSCodeHighlight>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VSCodeHighlight {
    scope: Option<VSCodeScope>,
    settings: VSCodeScopeSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum VSCodeScope {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
struct VSCodeScopeSettings {
    foreground: Option<String>,
    background: Option<String>,
    #[serde(rename = "fontStyle")]
    font_style: Option<String>,
}
