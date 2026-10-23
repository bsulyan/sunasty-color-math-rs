//! # Sunasty Color Math
//!
//! Zero-dependency Rust library for sRGB, OKLCH, relative luminance, and WCAG 2.1 contrast calculation.
//!
//! Developed for the [Sunasty Color Picker](https://sunasty.com/color-picker/) client-side tool
//! and the [Sunasty Developer Utilities](https://sunasty.com/dev/).

/// Represents an 8-bit sRGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse a 3 or 6-digit hex string with or without leading '#'.
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let clean = hex.trim().trim_start_matches('#');
        let expanded = match clean.len() {
            3 => {
                let mut s = String::with_capacity(6);
                for c in clean.chars() {
                    s.push(c);
                    s.push(c);
                }
                s
            }
            6 => clean.to_string(),
            _ => return Err("Invalid hex length"),
        };

        let val = u32::from_str_radix(&expanded, 16).map_err(|_| "Invalid hex characters")?;
        Ok(Self {
            r: ((val >> 16) & 0xFF) as u8,
            g: ((val >> 8) & 0xFF) as u8,
            b: (val & 0xFF) as u8,
        })
    }

    /// Format as `#rrggbb`.
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Convert 8-bit sRGB channel to linear light.
    pub fn linearize(val: u8) -> f64 {
        let v = (val as f64) / 255.0;
        if v <= 0.04045 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    }

    /// Compute WCAG 2.1 relative luminance (0.0 to 1.0).
    pub fn relative_luminance(&self) -> f64 {
        0.2126 * Self::linearize(self.r)
            + 0.7152 * Self::linearize(self.g)
            + 0.0722 * Self::linearize(self.b)
    }

    /// Calculate WCAG 2.1 contrast ratio between two colors (1.0 to 21.0).
    pub fn contrast_ratio(&self, other: &Self) -> f64 {
        let l1 = self.relative_luminance();
        let l2 = other.relative_luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        let ratio = (lighter + 0.05) / (darker + 0.05);
        (ratio * 100.0).round() / 100.0
    }
}

/// Evaluation of WCAG 2.1 conformance levels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WcagConformance {
    pub ratio: f64,
    pub aa_normal: bool,
    pub aa_large: bool,
    pub aaa_normal: bool,
    pub aaa_large: bool,
}

impl WcagConformance {
    pub fn evaluate(ratio: f64) -> Self {
        Self {
            ratio,
            aa_normal: ratio >= 4.5,
            aa_large: ratio >= 3.0,
            aaa_normal: ratio >= 7.0,
            aaa_large: ratio >= 4.5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_parsing() {
        assert_eq!(Rgb::from_hex("#ffffff"), Ok(Rgb::new(255, 255, 255)));
        assert_eq!(Rgb::from_hex("000000"), Ok(Rgb::new(0, 0, 0)));
        assert_eq!(Rgb::from_hex("#0f766e"), Ok(Rgb::new(15, 118, 110)));
    }

    #[test]
    fn test_contrast() {
        let white = Rgb::new(255, 255, 255);
        let black = Rgb::new(0, 0, 0);
        let ratio = white.contrast_ratio(&black);
        assert!((ratio - 21.0).abs() < 0.1);

        let conf = WcagConformance::evaluate(ratio);
        assert!(conf.aa_normal);
        assert!(conf.aaa_normal);
    }
}
