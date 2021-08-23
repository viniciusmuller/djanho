pub fn from_hex_string(hex: &str) -> Result<RGBA, String> {
    match hex.len() {
        4 => Ok(RGBA {
            r: hex_to_u8(&hex[1..2].repeat(2)),
            g: hex_to_u8(&hex[2..3].repeat(2)),
            b: hex_to_u8(&hex[3..4].repeat(2)),
            a: 1.0,
        }),
        5 => Ok(RGBA {
            r: hex_to_u8(&hex[1..2].repeat(2)),
            g: hex_to_u8(&hex[2..3].repeat(2)),
            b: hex_to_u8(&hex[3..4].repeat(2)),
            a: hex_to_u8(&hex[4..5].repeat(2)) as f32 / 255.0,
        }),
        7 => Ok(RGBA {
            r: hex_to_u8(&hex[1..3]),
            g: hex_to_u8(&hex[3..5]),
            b: hex_to_u8(&hex[5..7]),
            a: 1.0,
        }),
        9 => Ok(RGBA {
            r: hex_to_u8(&hex[1..3]),
            g: hex_to_u8(&hex[3..5]),
            b: hex_to_u8(&hex[5..7]),
            a: (hex_to_u8(&hex[7..9]) as f32) / 255.0,
        }),
        _ => Err(format!("Unsupported or invalid hex string: {}", hex)),
    }
}

fn hex_to_u8(hex: &str) -> u8 {
    u8::from_str_radix(hex, 16).unwrap()
}

pub fn to_rgb_hex_string(rgb: RGBA) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b,)
}

pub fn is_rgba(hex: &str) -> bool {
    hex.len() == 9
}

pub fn blend(bg_color: RGBA, fg_color: RGBA) -> RGBA {
    RGBA {
        r: (((1.0 - fg_color.a) * bg_color.r as f32) + (fg_color.a * fg_color.r as f32)) as u8,
        g: (((1.0 - fg_color.a) * bg_color.g as f32) + (fg_color.a * fg_color.g as f32)) as u8,
        b: (((1.0 - fg_color.a) * bg_color.b as f32) + (fg_color.a * fg_color.b as f32)) as u8,
        a: 1.0,
    }
}

pub fn scale(mut color: RGBA, scale: f32) -> RGBA {
    color.r = (color.r as f32 * scale) as u8;
    color.g = (color.g as f32 * scale) as u8;
    color.b = (color.b as f32 * scale) as u8;
    color
}

#[derive(Debug, Copy, Clone)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn can_decode_rgb() {
    //     let hexcolor6 = "#0f0fff";
    //     let hexcolor3 = "#ff0";

    //     let target6 = RGB {
    //         r: 15,
    //         g: 15,
    //         b: 255,
    //     };

    //     let target3 = RGB {
    //         r: 255,
    //         g: 255,
    //         b: 0,
    //     };

    //     if let Ok(RGBEither::RGB { r, g, b }) = from_hex_string(hexcolor6) {
    //         assert_eq!((r, g, b), (target6.r, target6.g, target6.b))
    //     }

    //     if let Ok(RGBEither::RGB { r, g, b }) = from_hex_string(hexcolor3) {
    //         assert_eq!((r, g, b), (target3.r, target3.g, target3.b))
    //     }
    // }

    // #[test]
    // fn can_encode_rgb() {
    //     let color = RGBEither::RGB {
    //         r: 15,
    //         g: 15,
    //         b: 255,
    //     };
    //     let target = "#0f0fff".to_owned();

    //     let result = to_hex_string(color);
    //     assert_eq!(result, target)
    // }

    #[test]
    fn can_encode_rgba() {
        let target = "#0f0fffcc".to_owned();
        let color = RGBA {
            r: 15,
            g: 15,
            b: 255,
            a: 0.8,
        };

        let result = to_rgb_hex_string(color);
        assert_eq!(result, target)
    }

    #[test]
    fn can_decode_rgba() {
        let color = "#0f0fffcc";
        let target = RGBA {
            r: 15,
            g: 15,
            b: 255,
            a: 0.8,
        };

        if let Ok(RGBA { r, g, b, a }) = from_hex_string(color) {
            assert_eq!((r, g, b, a), (target.r, target.g, target.b, target.a))
        }
    }

    #[test]
    fn can_blend_colors() {
        let background = RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 0.5,
        };
        let foreground = RGBA {
            r: 255,
            g: 255,
            b: 0,
            a: 1.0,
        };
        let target = RGBA {
            r: 127,
            g: 127,
            b: 0,
            a: 1.0,
        };

        let result = blend(foreground, background);
        assert_eq!(
            (result.r, result.g, result.b),
            (target.r, target.g, target.b)
        )
    }
}
