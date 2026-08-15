use druid::Color;

pub fn parse_color(input: &str) -> Option<Color> {
    let s = input.trim();
    if s.starts_with('#') {
        parse_hex(s)
    } else if s.starts_with("rgb") {
        parse_rgb(s)
    } else if s.starts_with("hsl") {
        parse_hsl(s)
    } else {
        parse_named(s)
    }
}

fn parse_hex(s: &str) -> Option<Color> {
    let hex = s.strip_prefix('#')?;
    match hex.len() {
        3 => parse_hex_short(hex, false),
        4 => parse_hex_short(hex, true),
        6 => parse_hex_long(hex, false),
        8 => parse_hex_long(hex, true),
        _ => None,
    }
}

fn parse_hex_short(s: &str, with_alpha: bool) -> Option<Color> {
    let r = u8::from_str_radix(&s[0..1], 16).ok()? * 17;
    let g = u8::from_str_radix(&s[1..2], 16).ok()? * 17;
    let b = u8::from_str_radix(&s[2..3], 16).ok()? * 17;
    let a = if with_alpha {
        u8::from_str_radix(&s[3..4], 16).ok()? * 17
    } else {
        255
    };
    Some(Color::rgba8(r, g, b, a))
}

fn parse_hex_long(s: &str, with_alpha: bool) -> Option<Color> {
    let r = u8::from_str_radix(&s[0..2], 16).ok()?;
    let g = u8::from_str_radix(&s[2..4], 16).ok()?;
    let b = u8::from_str_radix(&s[4..6], 16).ok()?;
    let a = if with_alpha {
        u8::from_str_radix(&s[6..8], 16).ok()?
    } else {
        255
    };
    Some(Color::rgba8(r, g, b, a))
}

fn parse_rgb(s: &str) -> Option<Color> {
    let inner = extract_parenthesized(s)?;
    let parts = split_components(inner);
    if parts.len() < 3 {
        return None;
    }
    let r = parse_channel(parts[0])?;
    let g = parse_channel(parts[1])?;
    let b = parse_channel(parts[2])?;
    let a = parts.get(3).and_then(|v| parse_alpha(v)).unwrap_or(1.0);
    Some(Color::rgba(
        r as f64 / 255.0,
        g as f64 / 255.0,
        b as f64 / 255.0,
        a,
    ))
}

fn parse_hsl(s: &str) -> Option<Color> {
    let inner = extract_parenthesized(s)?;
    let parts = split_components(inner);
    if parts.len() < 3 {
        return None;
    }
    let h = parse_number(parts[0])? % 360.0;
    let sat = parse_percent_or_float(parts[1])?;
    let lit = parse_percent_or_float(parts[2])?;
    let alpha = parts.get(3).and_then(|v| parse_alpha(v)).unwrap_or(1.0);
    let (r, g, b) = hsl_to_rgb(h, sat, lit);
    Some(Color::rgba(r, g, b, alpha))
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (rp, gp, bp) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (rp + m, gp + m, bp + m)
}

fn extract_parenthesized(s: &str) -> Option<&str> {
    let start = s.find('(')? + 1;
    let end = s.rfind(')')?;
    (start <= end).then(|| &s[start..end])
}

fn split_components(s: &str) -> Vec<&str> {
    s.split(|c: char| c == ',' || c == '/' || c.is_whitespace())
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect()
}

fn parse_channel(s: &str) -> Option<u8> {
    if let Some(pct) = s.strip_suffix('%') {
        let val = pct.parse::<f64>().ok()?;
        Some((val.clamp(0.0, 100.0) * 2.55).round() as u8)
    } else {
        let val = s.parse::<f64>().ok()?;
        Some(val.clamp(0.0, 255.0).round() as u8)
    }
}

fn parse_alpha(s: &str) -> Option<f64> {
    if let Some(pct) = s.strip_suffix('%') {
        pct.parse::<f64>().map(|v| (v / 100.0).clamp(0.0, 1.0)).ok()
    } else {
        s.parse::<f64>().map(|v| v.clamp(0.0, 1.0)).ok()
    }
}

fn parse_number(s: &str) -> Option<f64> {
    s.trim_end_matches("deg").parse::<f64>().ok()
}

fn parse_percent_or_float(s: &str) -> Option<f64> {
    if let Some(pct) = s.strip_suffix('%') {
        pct.parse::<f64>().map(|v| (v / 100.0).clamp(0.0, 1.0)).ok()
    } else {
        s.parse::<f64>().map(|v| v.clamp(0.0, 1.0)).ok()
    }
}

fn parse_named(s: &str) -> Option<Color> {
    match s.to_ascii_lowercase().as_str() {
        "black" => Some(Color::BLACK),
        "white" => Some(Color::WHITE),
        "transparent" => Some(Color::rgba8(0, 0, 0, 0)),
        "red" => Some(Color::rgb8(255, 0, 0)),
        "green" => Some(Color::rgb8(0, 128, 0)),
        "blue" => Some(Color::rgb8(0, 0, 255)),
        "gray" | "grey" => Some(Color::grey8(128)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_color("#fff"), Some(Color::rgba8(255, 255, 255, 255)));
        assert_eq!(parse_color("#000"), Some(Color::rgba8(0, 0, 0, 255)));
        assert_eq!(parse_color("#ff0000"), Some(Color::rgba8(255, 0, 0, 255)));
        assert_eq!(parse_color("#00ff0080"), Some(Color::rgba8(0, 255, 0, 128)));
    }

    #[test]
    fn test_parse_rgb() {
        assert_eq!(
            parse_color("rgb(255, 128, 0)"),
            Some(Color::rgba(1.0, 128.0 / 255.0, 0.0, 1.0))
        );
        assert_eq!(
            parse_color("rgba(0, 0, 0, 0.5)"),
            Some(Color::rgba(0.0, 0.0, 0.0, 0.5))
        );
    }

    #[test]
    fn test_parse_hsl() {
        assert!(parse_color("hsl(0, 100%, 50%)").is_some());
        assert!(parse_color("hsla(120, 100%, 50%, 0.8)").is_some());
    }

    #[test]
    fn test_parse_named() {
        assert_eq!(parse_color("white"), Some(Color::WHITE));
        assert_eq!(parse_color("red"), Some(Color::rgb8(255, 0, 0)));
    }
}
