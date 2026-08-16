use druid::{FontFamily, FontWeight};

pub fn parse_font_family(input: &str) -> FontFamily {
    let first = input.split(',').next().unwrap_or(input).trim();
    let unquoted = first.trim_matches(|c| c == '\'' || c == '"').trim();

    match unquoted.to_ascii_lowercase().as_str() {
        "system-ui" | "system" => FontFamily::SYSTEM_UI,
        "monospace" | "mono" => FontFamily::MONOSPACE,
        "sans-serif" | "sans" => FontFamily::SANS_SERIF,
        "serif" => FontFamily::SERIF,
        _ => FontFamily::new_unchecked(unquoted),
    }
}

pub fn parse_font_weight(input: &str) -> FontWeight {
    let s = input.trim().to_ascii_lowercase();
    if let Ok(num) = s.parse::<u16>() {
        return FontWeight::new(num);
    }
    match_named_weight(&s)
}

fn match_named_weight(s: &str) -> FontWeight {
    match s {
        "thin" | "100" => FontWeight::THIN,
        "light" | "300" => FontWeight::LIGHT,
        "regular" | "normal" | "400" => FontWeight::REGULAR,
        "medium" | "500" => FontWeight::MEDIUM,
        "semi-bold" | "semibold" | "600" => FontWeight::SEMI_BOLD,
        "bold" | "700" => FontWeight::BOLD,
        "extra-bold" | "extrabold" | "800" => FontWeight::EXTRA_BOLD,
        "black" | "heavy" | "900" => FontWeight::BLACK,
        _ => FontWeight::REGULAR,
    }
}

pub fn parse_font_size(input: &str) -> Option<f64> {
    let s = input.trim().to_ascii_lowercase();
    let numeric_part = s
        .trim_end_matches("px")
        .trim_end_matches("pt")
        .trim_end_matches("em")
        .trim_end_matches("rem")
        .trim();
    numeric_part.parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_font_family() {
        assert_eq!(parse_font_family("system-ui"), FontFamily::SYSTEM_UI);
        assert_eq!(parse_font_family("monospace"), FontFamily::MONOSPACE);
        assert_eq!(
            parse_font_family("'Inter', sans-serif"),
            FontFamily::new_unchecked("Inter")
        );
        assert_eq!(
            parse_font_family("\"JetBrains Mono\", monospace"),
            FontFamily::new_unchecked("JetBrains Mono")
        );
    }

    #[test]
    fn test_parse_font_weight() {
        assert_eq!(parse_font_weight("bold"), FontWeight::BOLD);
        assert_eq!(parse_font_weight("medium"), FontWeight::MEDIUM);
        assert_eq!(parse_font_weight("500"), FontWeight::new(500));
        assert_eq!(parse_font_weight("700"), FontWeight::BOLD);
    }

    #[test]
    fn test_parse_font_size() {
        assert_eq!(parse_font_size("13px"), Some(13.0));
        assert_eq!(parse_font_size("11pt"), Some(11.0));
        assert_eq!(parse_font_size("16"), Some(16.0));
    }
}
