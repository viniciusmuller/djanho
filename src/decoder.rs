use std::collections::HashMap;
use std::fs;

use json_comments::StripComments;
use serde::{Deserialize, Serialize};

pub fn parse_file(filepath: String) -> VSCodeTheme {
    let body = fs::read_to_string(filepath).expect("Could not read the file.");
    let stripped = StripComments::new(body.as_bytes());
    let theme: VSCodeTheme = serde_json::from_reader(stripped).unwrap();
    theme
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VSCodeTheme {
    pub name: Option<String>, // TODO: Use default names for those
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "tokenColors")]
    pub token_colors: Vec<VSCodeHighlight>,
    pub colors: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VSCodeHighlight {
    pub scope: Option<VSCodeScope>,
    pub settings: VSCodeScopeSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum VSCodeScope {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VSCodeScopeSettings {
    pub foreground: Option<String>,
    pub background: Option<String>,
    #[serde(rename = "fontStyle")]
    pub font_style: Option<String>,
}
