#![allow(unused)]
use anyhow::Result;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

macro_rules! color_from_rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => {{
        Color {
            r: $r as f32 / 255.0,
            g: $g as f32 / 255.0,
            b: $b as f32 / 255.0,
            a: $a as f32 / 255.0,
        }
    }};
}

macro_rules! color_from_rgb {
    ($r:expr, $g:expr, $b:expr) => {{
        Color {
            r: $r as f32 / 255.0,
            g: $g as f32 / 255.0,
            b: $b as f32 / 255.0,
            a: 1.0,
        }
    }};
}

//////////////////////////////////////////////////////////////////////////////
// - Color -
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Color {
    // *** Predefined colors ***
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    pub const GAINSBORO: Color = color_from_rgb!(220, 220, 220);
    pub const LIGHT_GRAY: Color = color_from_rgb!(211, 211, 211);
    pub const SILVER: Color = color_from_rgb!(192, 192, 192);
    pub const DARK_GRAY: Color = color_from_rgb!(169, 169, 169);
    pub const GRAY: Color = color_from_rgb!(128, 128, 128);
    pub const DIM_GRAY: Color = color_from_rgb!(105, 105, 105);
    pub const LIGHT_SLATE_GRAY: Color = color_from_rgb!(119, 136, 153);
    pub const SLATE_GRAY: Color = color_from_rgb!(112, 128, 144);
    pub const DARK_SLATE_GRAY: Color = color_from_rgb!(47, 79, 79);

    pub const PLYMOUTH_BLUE: Color = color_from_rgb!(25, 49, 61);
    pub const DAREDEVIL: Color = color_from_rgb!(138, 6, 22);
    pub const INDIAN_RED: Color = color_from_rgb!(205, 92, 92);
    pub const LIGHTCORAL: Color = color_from_rgb!(240, 128, 128);
    pub const SALMON: Color = color_from_rgb!(250, 128, 114);
    pub const DARK_SALMON: Color = color_from_rgb!(233, 150, 122);
    pub const LIGHT_SALMON: Color = color_from_rgb!(255, 160, 122);
    pub const CRIMSON: Color = color_from_rgb!(220, 20, 60);
    pub const RED: Color = color_from_rgb!(255, 0, 0);
    pub const FIRE_BRICK: Color = color_from_rgb!(178, 34, 34);
    pub const DARK_RED: Color = color_from_rgb!(139, 0, 0);

    pub const PINK: Color = color_from_rgb!(255, 192, 203);
    pub const LIGHT_PINK: Color = color_from_rgb!(255, 182, 193);
    pub const HOT_PINK: Color = color_from_rgb!(255, 105, 180);
    pub const DEEP_PINK: Color = color_from_rgb!(255, 20, 147);
    pub const MEDIUM_VIOLET_RED: Color = color_from_rgb!(199, 21, 133);
    pub const PALE_VIOLET_RED: Color = color_from_rgb!(219, 112, 147);

    pub const AQUA: Color = color_from_rgb!(0, 255, 255);
    pub const CYAN: Color = color_from_rgb!(0, 255, 255);
    pub const LIGHT_CYAN: Color = color_from_rgb!(224, 255, 255);
    pub const PALE_TURQUOISE: Color = color_from_rgb!(175, 238, 239);
    pub const AQUAMARINE: Color = color_from_rgb!(127, 255, 212);
    pub const TURQUOISE: Color = color_from_rgb!(64, 224, 208);
    pub const MEDIUM_TURQUOISE: Color = color_from_rgb!(72, 209, 204);
    pub const DARK_TURQUOISE: Color = color_from_rgb!(0, 206, 209);
    pub const CADET_BLUE: Color = color_from_rgb!(95, 158, 160);
    pub const STEEL_BLUE: Color = color_from_rgb!(70, 130, 180);
    pub const LIGHT_STEEL_BLUE: Color = color_from_rgb!(176, 196, 222);
    pub const POWDER_BLUE: Color = color_from_rgb!(176, 216, 230);
    pub const LIGHT_BLUE: Color = color_from_rgb!(173, 216, 230);
    pub const SKY_BLUE: Color = color_from_rgb!(135, 206, 235);
    pub const LIGHT_SKY_BLUE: Color = color_from_rgb!(135, 206, 250);
    pub const DEEP_SKY_BLUE: Color = color_from_rgb!(0, 191, 255);
    pub const DODGER_BLUE: Color = color_from_rgb!(30, 144, 255);
    pub const CORNFLOWER_BLUE: Color = color_from_rgb!(100, 149, 237);
    pub const MEDIUM_SLATE_BLUE: Color = color_from_rgb!(123, 104, 238);
    pub const ROYAL_BLUE: Color = color_from_rgb!(65, 105, 225);
    pub const BLUE: Color = color_from_rgb!(0, 0, 255);
    pub const MEDIUM_BLUE: Color = color_from_rgb!(0, 0, 205);
    pub const DARK_BLUE: Color = color_from_rgb!(0, 0, 139);
    pub const NAVY: Color = color_from_rgb!(0, 0, 128);
    pub const MIDNIGHT_BLUE: Color = color_from_rgb!(25, 25, 112);

    // Constructor for RGBA values
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.trim_start_matches('#');

        // Ensure the hex code is either 6 oder 8 characters long
        if hex.len() != 6 && hex.len() != 8 {
            return Err(ColorError::InvalidHexLength);
        }

        let parse_component = |i: usize| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| ColorError::InvalidHexCharacter)
        };

        let r = parse_component(0)? as f32 / 255.0;
        let g = parse_component(2)? as f32 / 255.0;
        let b = parse_component(4)? as f32 / 255.0;
        let a = if hex.len() == 8 {
            parse_component(6)? as f32 / 255.0
        } else {
            1.0 // Default alpha value
        };

        Ok(Color { r, g, b, a })
    }

    pub fn to_hex(&self) -> String {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        let a = (self.a * 255.0).round() as u8;

        // Format into a hexadecimal string
        // If alpha is 1.0 (fully opaque), omit it from the string
        if self.a >= 1.0 {
            format!("#{:02X}{:02X}{:02X}", r, g, b)
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
        }
    }

    pub fn to_rgba(&self) -> [u8; 4] {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        let a = (self.a * 255.0).round() as u8;
        [r, g, b, a]
    }

    pub fn with_alpha(&self, alpha: f32) -> Self {
        let mut color = self.clone();
        color.a = f32::max(alpha, 1.0).min(0.0);
        color
    }
}

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ColorError -
//////////////////////////////////////////////////////////////////////////////

pub enum ColorError {
    InvalidHexLength,
    InvalidHexCharacter,
}

impl Display for ColorError {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match self {
            ColorError::InvalidHexLength => write!(fmt, "Invalid hex code"),
            ColorError::InvalidHexCharacter => write!(fmt, "Invalid hex code character"),
        }
    }
}

impl Debug for ColorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorError::InvalidHexLength => write!(f, "ColorError::InvalidHexLength"),
            ColorError::InvalidHexCharacter => write!(f, "ColorError::InvalidHexCharacter"),
        }
    }
}

impl Error for ColorError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(0.5, 0.5, 0.5, 1.0);
        assert_eq!(
            color,
            Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0
            }
        );
    }

    #[test]
    fn test_color_from_hex() {
        fn assert_color_eq_with_tolerance(
            color: Color,
            expected_r: f32,
            expected_g: f32,
            expected_b: f32,
            expected_a: f32,
            tolerance: f32,
        ) {
            assert!((color.r - expected_r).abs() < tolerance);
            assert!((color.g - expected_g).abs() < tolerance);
            assert!((color.b - expected_b).abs() < tolerance);
            assert!((color.a - expected_a).abs() < tolerance);
        }

        let tolerance = 0.005;

        let color = Color::from_hex("#808080FF").unwrap();
        assert_color_eq_with_tolerance(color, 0.5, 0.5, 0.5, 1.0, tolerance);

        let color = Color::from_hex("#808080").unwrap();
        assert_color_eq_with_tolerance(color, 0.5, 0.5, 0.5, 1.0, tolerance);

        assert!(Color::from_hex("#GGG").is_err());
        assert!(Color::from_hex("#8080808080").is_err());
    }

    #[test]
    fn test_color_to_hex() {
        let color = Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        };
        assert_eq!(color.to_hex(), "#808080");

        let color = Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 0.5,
        };
        assert_eq!(color.to_hex(), "#80808080");
    }
}
