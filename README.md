# sunasty-color-math

Zero-dependency Rust crate for sRGB, OKLCH, relative luminance, and WCAG 2.1 accessibility contrast calculation.

Originally created for the [Sunasty Color Picker](https://sunasty.com/color-picker/) client-side tool and the [Sunasty Developer Tools](https://sunasty.com/dev/).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sunasty-color-math = "1.0.0"
```

## Quick Start

```rust
use sunasty_color_math::{Rgb, WcagConformance};

fn main() {
    let teal = Rgb::from_hex("#0F766E").unwrap();
    let white = Rgb::new(255, 255, 255);

    let ratio = teal.contrast_ratio(&white);
    let conformance = WcagConformance::evaluate(ratio);

    println!("Contrast ratio: {}:1", ratio);
    println!("Passes WCAG AA: {}", conformance.aa_normal);
}
```

## Related Tools

- [Sunasty Color Picker & Palette Extractor](https://sunasty.com/color-picker/)
- [Sunasty Developer Utilities](https://sunasty.com/dev/)
- [Ulysan](https://ulysan.fr)

## License

MIT © [Bastien Sulyan](https://ulysan.fr)
