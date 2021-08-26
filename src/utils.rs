pub fn map_font_styles(style: &str) -> String {
    match style {
        "italic" => "italic".to_owned(),
        "bold" => "bold".to_owned(),
        _ => String::new(),
    }
}
