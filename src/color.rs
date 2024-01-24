use anyhow::{Result, anyhow};

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
    // Predfined colors
    pub const BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    // Constructor for RGBA values
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a } 
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');

        // Ensure the hex code is either 6 oder 8 characters long
        if hex.len() != 6 && hex.len() != 8 {
            return Err(anyhow!("Invalid hex code length"));
        }

        let parse_component = |i: usize| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| anyhow!("Invalid hex code"))
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
