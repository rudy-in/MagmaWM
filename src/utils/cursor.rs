extern crate xcursor;

use std::fs::File;
use std::io::Read;
use xcursor::{Theme, ThemeManager};

pub fn lookup_cursor_theme(theme_name: &str) -> Option<Theme> {
    let theme_manager = ThemeManager::new().unwrap();
    theme_manager.get_theme(theme_name)
}

pub fn load_cursor_image(theme: &Theme, cursor_name: &str) -> Option<Vec<u8>> {
    let cursor = theme.load_cursor(cursor_name).ok()?;
    let mut file = File::open(cursor.path()).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    Some(buffer)
}

pub fn parse_image_data(image_data: &[u8]) -> Option<(usize, usize, (i32, i32))> {
    let cursor_image = xcursor::parser::parse_image(image_data).ok()?;
    let width = cursor_image.header.width as usize;
    let height = cursor_image.header.height as usize;
    let hotspot = (cursor_image.header.xhot as i32, cursor_image.header.yhot as i32);
    Some((width, height, hotspot))
}