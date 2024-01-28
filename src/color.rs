use anyhow::Result;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

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

impl Color {
    // Predefined colors
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    // Constructor for RGBA values
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
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

    fn to_hex(&self) -> String {
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
