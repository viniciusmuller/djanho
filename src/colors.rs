pub fn from_hex_string(hex: &str) -> RGBEither {
    // TODO: Maybe handle 4 characters RGBA strings? Don't really know if they exist..
    match hex.len() {
        4 => RGBEither::RGB {
            r: hex_to_u8(&hex[1..2]),
            g: hex_to_u8(&hex[2..3]),
            b: hex_to_u8(&hex[3..4]),
        },
        7 => RGBEither::RGB {
            r: hex_to_u8(&hex[1..3]),
            g: hex_to_u8(&hex[3..5]),
            b: hex_to_u8(&hex[5..7]),
        },
        9 => RGBEither::RGBA {
            r: hex_to_u8(&hex[1..3]),
            g: hex_to_u8(&hex[3..5]),
            b: hex_to_u8(&hex[5..7]),
            a: hex_to_u8(&hex[7..9]),
        },
        _ => panic!("Unsupported or invalid hex string"),
    }
}

fn hex_to_u8(hex: &str) -> u8 {
    u8::from_str_radix(hex, 16).unwrap()
}

pub fn to_hex_string(rgb: RGBEither) -> String {
    // TODO:
    String::new()
}

pub fn is_rgba(hex: &str) -> bool {
    hex.len() == 9
}

pub fn blend_rgba(a: RGB, b: RGBA) {}

#[derive(Debug)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[derive(Debug)]
pub enum RGBEither {
    RGB { r: u8, g: u8, b: u8 },
    RGBA { r: u8, g: u8, b: u8, a: u8 },
}
