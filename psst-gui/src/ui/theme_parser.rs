use std::collections::HashMap;
use druid::{Color, FontWeight};

use super::{
    theme_color::parse_color,
    theme_font::{parse_font_size, parse_font_weight},
};

#[derive(Default, Debug, Clone)]
pub struct ParsedTheme {
    pub colors: HashMap<String, Color>,
    pub font_family: Option<String>,
    pub font_family_mono: Option<String>,
    pub font_size_small: Option<f64>,
    pub font_size_normal: Option<f64>,
    pub font_size_large: Option<f64>,
    pub font_weight: Option<FontWeight>,
    pub font_weight_medium: Option<FontWeight>,
}

pub fn parse_theme_css(css: &str) -> ParsedTheme {
    let clean = strip_comments(css);
    let mut theme = ParsedTheme::default();
    let blocks = extract_blocks(&clean);

    for block in blocks {
        for chunk in block.split(';') {
            parse_declaration_into(chunk, &mut theme);
        }
    }
    theme
}

fn parse_declaration_into(line: &str, theme: &mut ParsedTheme) {
    let Some((raw_key, raw_val)) = line.split_once(':') else {
        return;
    };
    let key = normalize_key(raw_key);
    let val = raw_val.trim();
    if key.is_empty() || val.is_empty() {
        return;
    }

    if !handle_font_declaration(&key, val, theme) {
        if let Some(color) = parse_color(val) {
            theme.colors.insert(key, color);
        }
    }
}

fn handle_font_declaration(key: &str, val: &str, theme: &mut ParsedTheme) -> bool {
    match key {
        "font_family" | "ui_font_family" | "font" => {
            theme.font_family = Some(val.to_string());
            true
        }
        "font_family_mono" | "ui_font_mono_family" | "font_mono" => {
            theme.font_family_mono = Some(val.to_string());
            true
        }
        "font_size_small" | "text_size_small" => {
            theme.font_size_small = parse_font_size(val);
            true
        }
        "font_size" | "font_size_normal" | "text_size_normal" | "text_size" => {
            theme.font_size_normal = parse_font_size(val);
            true
        }
        "font_size_large" | "text_size_large" => {
            theme.font_size_large = parse_font_size(val);
            true
        }
        "font_weight" | "ui_font_weight" => {
            theme.font_weight = Some(parse_font_weight(val));
            true
        }
        "font_weight_medium" | "ui_font_weight_medium" => {
            theme.font_weight_medium = Some(parse_font_weight(val));
            true
        }
        _ => false,
    }
}

fn extract_blocks(css: &str) -> Vec<&str> {
    let mut blocks = Vec::new();
    let mut remaining = css;
    while let Some(start) = remaining.find('{') {
        let after_start = &remaining[start + 1..];
        if let Some(end) = after_start.find('}') {
            blocks.push(&after_start[..end]);
            remaining = &after_start[end + 1..];
        } else {
            blocks.push(after_start);
            break;
        }
    }
    if blocks.is_empty() {
        blocks.push(css);
    }
    blocks
}

fn normalize_key(key: &str) -> String {
    let trimmed = key.trim().to_ascii_lowercase();
    let stripped = trimmed.strip_prefix("--").unwrap_or(&trimmed);
    stripped.replace('-', "_")
}

fn strip_comments(input: &str) -> String {
    let without_block = strip_block_comments(input);
    strip_line_comments(&without_block)
}

fn strip_block_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut remaining = input;
    while let Some(start) = remaining.find("/*") {
        out.push_str(&remaining[..start]);
        let after_start = &remaining[start + 2..];
        if let Some(end) = after_start.find("*/") {
            remaining = &after_start[end + 2..];
        } else {
            return out;
        }
    }
    out.push_str(remaining);
    out
}

fn strip_line_comments(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            if let Some((code, _)) = line.split_once("//") {
                code
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_theme_css_variables() {
        let css = r#"
            /* Custom Theme */
            :root {
                --grey-000: #000000;
                --grey-700: #ffffff;
                --blue-100: rgb(0, 140, 255);
                --window-background-color: #121212; // dark bg
                --font-family: "JetBrains Mono", monospace;
                --font-size: 14px;
                --font-weight: 500;
            }
        "#;
        let theme = parse_theme_css(css);
        assert_eq!(theme.colors.get("grey_000"), Some(&Color::rgba8(0, 0, 0, 255)));
        assert_eq!(theme.colors.get("grey_700"), Some(&Color::rgba8(255, 255, 255, 255)));
        assert_eq!(theme.colors.get("blue_100"), Some(&Color::rgba(0.0, 140.0 / 255.0, 1.0, 1.0)));
        assert_eq!(theme.colors.get("window_background_color"), Some(&Color::rgba8(18, 18, 18, 255)));
        assert_eq!(theme.font_family.as_deref(), Some("\"JetBrains Mono\", monospace"));
        assert_eq!(theme.font_size_normal, Some(14.0));
        assert_eq!(theme.font_weight, Some(FontWeight::new(500)));
    }
}
