use super::theme_color::parse_color;
use druid::Color;
use std::collections::HashMap;

pub fn parse_theme_css(css: &str) -> HashMap<String, Color> {
    let clean = strip_comments(css);
    let mut theme_map = HashMap::new();
    let blocks = extract_blocks(&clean);

    for block in blocks {
        for chunk in block.split(';') {
            if let Some((key, val)) = parse_declaration(chunk) {
                theme_map.insert(key, val);
            }
        }
    }
    theme_map
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

fn parse_declaration(line: &str) -> Option<(String, Color)> {
    let (raw_key, raw_val) = line.split_once(':')?;
    let key = normalize_key(raw_key);
    if key.is_empty() {
        return None;
    }
    let color = parse_color(raw_val.trim())?;
    Some((key, color))
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
            }
        "#;
        let theme = parse_theme_css(css);
        assert_eq!(theme.get("grey_000"), Some(&Color::rgba8(0, 0, 0, 255)));
        assert_eq!(
            theme.get("grey_700"),
            Some(&Color::rgba8(255, 255, 255, 255))
        );
        assert_eq!(
            theme.get("blue_100"),
            Some(&Color::rgba(0.0, 140.0 / 255.0, 1.0, 1.0))
        );
        assert_eq!(
            theme.get("window_background_color"),
            Some(&Color::rgba8(18, 18, 18, 255))
        );
    }

    #[test]
    fn test_parse_flat_css() {
        let css = "grey-000: #111; window-background-color: #222;";
        let theme = parse_theme_css(css);
        assert_eq!(theme.get("grey_000"), Some(&Color::rgba8(17, 17, 17, 255)));
        assert_eq!(
            theme.get("window_background_color"),
            Some(&Color::rgba8(34, 34, 34, 255))
        );
    }

    #[test]
    fn test_parse_multi_block_css() {
        let css = r#"
            :root {
                --primary-light: #5cc4ff;
            }
            .theme-dark {
                --primary-dark: #008ddd;
            }
        "#;
        let theme = parse_theme_css(css);
        assert_eq!(
            theme.get("primary_light"),
            Some(&Color::rgba8(92, 196, 255, 255))
        );
        assert_eq!(
            theme.get("primary_dark"),
            Some(&Color::rgba8(0, 141, 221, 255))
        );
    }
}
