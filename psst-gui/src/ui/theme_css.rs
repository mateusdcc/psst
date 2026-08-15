use druid::{Color, Env, Key};
use std::{collections::HashMap, fs, path::PathBuf};

use super::{theme, theme_parser::parse_theme_css};
use crate::data::Config;

pub fn find_theme_css_path() -> Option<PathBuf> {
    if let Some(user_dirs) = directories::UserDirs::new() {
        let xdg_path = user_dirs.home_dir().join(".config/psst/theme.css");
        if xdg_path.is_file() {
            return Some(xdg_path);
        }
    }
    if let Some(config_dir) = Config::config_dir() {
        let app_path = config_dir.join("theme.css");
        if app_path.is_file() {
            return Some(app_path);
        }
    }
    None
}

pub fn load_theme_css() -> Option<HashMap<String, Color>> {
    let path = find_theme_css_path()?;
    let content = fs::read_to_string(&path).ok()?;
    log::info!("Loaded theme from CSS: {:?}", path);
    Some(parse_theme_css(&content))
}

pub fn apply_palette_overrides(env: &mut Env, colors: &HashMap<String, Color>) {
    set_if_present(env, colors, "grey_000", theme::GREY_000);
    set_if_present(env, colors, "grey_100", theme::GREY_100);
    set_if_present(env, colors, "grey_200", theme::GREY_200);
    set_if_present(env, colors, "grey_300", theme::GREY_300);
    set_if_present(env, colors, "grey_400", theme::GREY_400);
    set_if_present(env, colors, "grey_500", theme::GREY_500);
    set_if_present(env, colors, "grey_600", theme::GREY_600);
    set_if_present(env, colors, "grey_700", theme::GREY_700);
    set_if_present(env, colors, "blue_100", theme::BLUE_100);
    set_if_present(env, colors, "blue_200", theme::BLUE_200);
    set_if_present(env, colors, "red", theme::RED);
    set_if_present(env, colors, "link_hot_color", theme::LINK_HOT_COLOR);
    set_if_present(env, colors, "link_active_color", theme::LINK_ACTIVE_COLOR);
    set_if_present(env, colors, "link_cold_color", theme::LINK_COLD_COLOR);
}

pub fn apply_direct_overrides(env: &mut Env, colors: &HashMap<String, Color>) {
    apply_element_overrides(env, colors);
    apply_button_overrides(env, colors);
    apply_menu_overrides(env, colors);
}

fn apply_element_overrides(env: &mut Env, colors: &HashMap<String, Color>) {
    set_if_present(
        env,
        colors,
        "window_background_color",
        theme::WINDOW_BACKGROUND_COLOR,
    );
    set_if_present(env, colors, "text_color", theme::TEXT_COLOR);
    set_if_present(env, colors, "icon_color", theme::ICON_COLOR);
    set_if_present(env, colors, "placeholder_color", theme::PLACEHOLDER_COLOR);
    set_if_present(env, colors, "primary_light", theme::PRIMARY_LIGHT);
    set_if_present(env, colors, "primary_dark", theme::PRIMARY_DARK);
    set_if_present(env, colors, "background_light", theme::BACKGROUND_LIGHT);
    set_if_present(env, colors, "background_dark", theme::BACKGROUND_DARK);
    set_if_present(env, colors, "foreground_light", theme::FOREGROUND_LIGHT);
    set_if_present(env, colors, "foreground_dark", theme::FOREGROUND_DARK);
}

fn apply_button_overrides(env: &mut Env, colors: &HashMap<String, Color>) {
    set_if_present(env, colors, "button_light", theme::BUTTON_LIGHT);
    set_if_present(env, colors, "button_dark", theme::BUTTON_DARK);
    set_if_present(env, colors, "border_light", theme::BORDER_LIGHT);
    set_if_present(env, colors, "border_dark", theme::BORDER_DARK);
    set_if_present(
        env,
        colors,
        "selected_text_background_color",
        theme::SELECTED_TEXT_BACKGROUND_COLOR,
    );
    set_if_present(
        env,
        colors,
        "selection_text_color",
        theme::SELECTION_TEXT_COLOR,
    );
    set_if_present(env, colors, "cursor_color", theme::CURSOR_COLOR);
    set_if_present(env, colors, "scrollbar_color", theme::SCROLLBAR_COLOR);
    set_if_present(
        env,
        colors,
        "scrollbar_border_color",
        theme::SCROLLBAR_BORDER_COLOR,
    );
}

fn apply_menu_overrides(env: &mut Env, colors: &HashMap<String, Color>) {
    set_if_present(
        env,
        colors,
        "menu_button_bg_active",
        theme::MENU_BUTTON_BG_ACTIVE,
    );
    set_if_present(
        env,
        colors,
        "menu_button_bg_inactive",
        theme::MENU_BUTTON_BG_INACTIVE,
    );
    set_if_present(
        env,
        colors,
        "menu_button_fg_active",
        theme::MENU_BUTTON_FG_ACTIVE,
    );
    set_if_present(
        env,
        colors,
        "menu_button_fg_inactive",
        theme::MENU_BUTTON_FG_INACTIVE,
    );
}

fn set_if_present(env: &mut Env, map: &HashMap<String, Color>, key: &str, target: Key<Color>) {
    if let Some(&color) = map.get(key) {
        env.set(target, color);
    }
}
