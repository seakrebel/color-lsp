use tower_lsp::lsp_types::Color;

/// Convert lsp_types::Color to markdown to list other color formats (HSLA, HEX, RGBA, OKLCH, OKLAB)
/// e.g.
///
/// Colorspace Formats:
///
/// - #EECC00
/// - #EECC00FF
/// - oklch(72.3% 0.19 95.2)
/// - oklab(0.847 0.065 -0.103)
/// - hsla(51.4, 100%, 46.7%, 100%)
/// - rgba(238, 204, 0, 100%)
#[allow(unused)]
pub(crate) fn color_summary(color: Color) -> String {
    let r = (color.red * 255.0).round() as u8;
    let g = (color.green * 255.0).round() as u8;
    let b = (color.blue * 255.0).round() as u8;
    let a = (color.alpha * 255.0).round() as u8;

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    let hex_alpha = format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        r,
        g,
        b,
        (color.alpha * 255.0).round() as u8
    );
    let hsla = rgba_to_hsla(r, g, b, a);
    let hsla_percent = format!(
        "hsla({}, {}%, {}%, {}%)",
        format_trimmed(hsla.0 * 360., 1, false),
        format_trimmed(hsla.1 * 100., 1, true),
        format_trimmed(hsla.2 * 100., 1, true),
        format_trimmed(hsla.3 * 100., 1, true),
    );
    let hsla_float = format!(
        "hsla({}, {}, {}, {})",
        format_trimmed(hsla.0, 3, false),
        format_trimmed(hsla.1, 3, false),
        format_trimmed(hsla.2, 3, false),
        format_trimmed(hsla.3, 3, false)
    );

    let rgba = format!("rgba({}, {}, {}, {}%)", r, g, b, a / 255 * 100);
    let rgba_float = format!(
        "rgba({}, {}, {}, {})",
        format_trimmed(color.red, 3, false),
        format_trimmed(color.green, 3, false),
        format_trimmed(color.blue, 3, false),
        format_trimmed(color.alpha, 3, false)
    );

    // OKLCH / OKLAB conversions
    let (okl, oka, okb) = srgb_to_oklab(color.red, color.green, color.blue);
    let oklch_l = okl * 100.0;
    let oklch_c = (oka * oka + okb * okb).sqrt();
    let oklch_h = oklch_c.atan2(okb).to_degrees();
    let oklch_h = if oklch_h < 0.0 { oklch_h + 360.0 } else { oklch_h };
    let oklch = format!(
        "oklch({}, {}{}, {})",
        format_trimmed(oklch_l, 1, true),
        format_trimmed(oklch_c, 3, false),
        if oklch_c > 0.001 {
            format!(" {}", format_trimmed(oklch_h, 1, true))
        } else {
            String::new()
        },
        if (color.alpha - 1.0).abs() > 0.01 {
            format!(" / {}%", format_trimmed(color.alpha * 100.0, 1, true))
        } else {
            String::new()
        }
    );
    let oklab = format!(
        "oklab({}, {}{}, {})",
        format_trimmed(okl, 3, false),
        format_trimmed(oka, 3, false),
        if okb >= 0.0 {
            format!(" +{}", format_trimmed(okb, 3, false))
        } else {
            format!(" {}", format_trimmed(okb, 3, false))
        },
        if (color.alpha - 1.0).abs() > 0.01 {
            format!(" / {}%", format_trimmed(color.alpha * 100.0, 1, true))
        } else {
            String::new()
        }
    );

    let color_link = format!("\n[Color Picker](https://colorpicker.dev/{})", &hex);

    format!(
        "Colorspace Formats:\n\n```\n{}\n```\n{}",
        vec![hex, hex_alpha, oklch, oklab, hsla_percent, hsla_float, rgba, rgba_float].join("\n"),
        color_link
    )
}

pub(crate) fn format_trimmed(x: f32, precision: usize, trim_end_dot: bool) -> String {
    let mut s = format!("{:.1$}", x, precision)
        .trim_end_matches('0')
        .to_string();

    if trim_end_dot {
        s = s.trim_end_matches(".").to_string();
    }

    s
}

/// Convert sRGB (linear) to OKLab.
/// Input: linear RGB values (0..1)
/// Output: (L, a, b) where L is 0..1, a and b are roughly -0.4..0.4
pub(crate) fn srgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    // sRGB to linear
    let linear = |x: f32| -> f32 {
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    };
    let r_lin = linear(r);
    let g_lin = linear(g);
    let b_lin = linear(b);

    // Linear sRGB to LMS (using M1 matrix)
    let l = 0.4122214708 * r_lin + 0.5363325363 * g_lin + 0.0514459929 * b_lin;
    let m = 0.2119034982 * r_lin + 0.6806995451 * g_lin + 0.1073969566 * b_lin;
    let s = 0.0883024619 * r_lin + 0.2817188376 * g_lin + 0.6299787005 * b_lin;

    // LMS to LMS (cube root)
    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    // LMS to OKLab (using M2 matrix)
    let okl = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
    let oka = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
    let okb = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

    (okl, oka, okb)
}

pub(crate) fn rgba_to_hsla(r: u8, g: u8, b: u8, a: u8) -> (f32, f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;
    let a = a as f32 / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let l = (max + min) / 2.0;

    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    (h / 360., s, l, a)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use tower_lsp::lsp_types::Color;

    #[test]
    fn test_color_summary() {
        let color = Color {
            red: 0.933,
            green: 0.8,
            blue: 0.0,
            alpha: 1.0,
        };

        let summary = super::color_summary(color);
        assert!(summary.contains("#EECC00"));
        assert!(summary.contains("oklch("));
        assert!(summary.contains("oklab("));
        assert!(summary.contains("hsla("));
        assert!(summary.contains("rgba("));
        assert!(summary.contains("[Color Picker]"));
    }

    #[test]
    fn test_rgba_to_hsla() {
        let (h, s, l, a) = super::rgba_to_hsla(238, 204, 0, 255);
        assert!((h - 0.143).abs() < 0.001);
        assert!((s - 1.0).abs() < 0.001);
        assert!((l - 0.467).abs() < 0.001);
        assert!((a - 1.0).abs() < 0.001);
    }
}
