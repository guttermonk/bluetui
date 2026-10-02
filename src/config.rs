use core::fmt;
use std::{path::PathBuf, process::exit};

use ratatui::layout::Flex;
use toml;

use dirs;
use serde::{
    Deserialize, Deserializer,
    de::{self, Unexpected, Visitor},
};
use ratatui::style::Color;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_layout", deserialize_with = "deserialize_layout")]
    pub layout: Flex,

    #[serde(default = "Width::default")]
    pub width: Width,

    #[serde(default = "default_toggle_scanning")]
    pub toggle_scanning: char,

    #[serde(default = "default_esc_quit")]
    pub esc_quit: bool,

    #[serde(default)]
    pub adapter: Adapter,

    #[serde(default)]
    pub paired_device: PairedDevice,

    #[serde(default)]
    pub navigation: Navigation,

    #[serde(default)]
    pub colors: Colors,
}

#[derive(Debug, Default)]
pub enum Width {
    #[default]
    Auto,
    Size(u16),
}

struct WidthVisitor;

impl<'de> Visitor<'de> for WidthVisitor {
    type Value = Width;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("the string \"auto\" or a positive integer (u16)")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            "auto" => Ok(Width::Auto),
            _ => value
                .parse::<u16>()
                .map(Width::Size)
                .map_err(|_| de::Error::invalid_value(Unexpected::Str(value), &self)),
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match u16::try_from(value) {
            Ok(v) => Ok(Width::Size(v)),
            Err(_) => Err(de::Error::invalid_value(Unexpected::Unsigned(value), &self)),
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match u16::try_from(value) {
            Ok(v) => Ok(Width::Size(v)),
            Err(_) => Err(de::Error::invalid_value(Unexpected::Signed(value), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for Width {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WidthVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct Adapter {
    #[serde(default = "default_toggle_adapter_pairing")]
    pub toggle_pairing: char,

    #[serde(default = "default_toggle_adapter_power")]
    pub toggle_power: char,

    #[serde(default = "default_toggle_adapter_discovery")]
    pub toggle_discovery: char,
}

impl Default for Adapter {
    fn default() -> Self {
        Self {
            toggle_pairing: 'p',
            toggle_power: 'o',
            toggle_discovery: 'd',
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PairedDevice {
    #[serde(default = "default_unpair_device")]
    pub unpair: char,

    #[serde(default = "default_toggle_device_trust")]
    pub toggle_trust: char,

    #[serde(default = "default_toggle_device_favorite")]
    pub toggle_favorite: char,

    #[serde(default = "default_set_new_name")]
    pub rename: char,
}

impl Default for PairedDevice {
    fn default() -> Self {
        Self {
            unpair: 'u',
            toggle_trust: 't',
            toggle_favorite: 'f',
            rename: 'e',
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Navigation {
    #[serde(default = "default_nav_up")]
    pub up: char,

    #[serde(default = "default_nav_down")]
    pub down: char,

    #[serde(default = "default_nav_left")]
    pub left: char,

    #[serde(default = "default_nav_right")]
    pub right: char,

    #[serde(default = "default_quit")]
    pub quit: char,

    #[serde(default = "default_select")]
    pub select: char,
}

impl Default for Navigation {
    fn default() -> Self {
        Self {
            up: 'k',
            down: 'j',
            left: 'h',
            right: 'l',
            quit: 'q',
            select: ' ',
        }
    }
}

fn deserialize_layout<'de, D>(deserializer: D) -> Result<Flex, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    match s.as_str() {
        "Legacy" => Ok(Flex::Legacy),
        "Start" => Ok(Flex::Start),
        "End" => Ok(Flex::End),
        "Center" => Ok(Flex::Center),
        "SpaceAround" => Ok(Flex::SpaceAround),
        "SpaceBetween" => Ok(Flex::SpaceBetween),
        _ => {
            eprintln!("Wrong config: unknown layout variant {}", s);
            eprintln!(
                "The possible values are: Legacy, Start, End, Center, SpaceAround, SpaceBetween"
            );
            std::process::exit(1);
        }
    }
}

fn default_layout() -> Flex {
    Flex::SpaceAround
}

fn default_set_new_name() -> char {
    'e'
}

fn default_toggle_scanning() -> char {
    's'
}

fn default_esc_quit() -> bool {
    false
}

fn default_toggle_adapter_pairing() -> char {
    'p'
}

fn default_toggle_adapter_power() -> char {
    'o'
}

fn default_toggle_adapter_discovery() -> char {
    'd'
}

fn default_unpair_device() -> char {
    'u'
}

fn default_toggle_device_trust() -> char {
    't'
}

fn default_toggle_device_favorite() -> char {
    'f'
}

fn default_nav_up() -> char {
    'k'
}

fn default_nav_down() -> char {
    'j'
}

fn default_nav_left() -> char {
    'h'
}

fn default_nav_right() -> char {
    'l'
}

fn default_quit() -> char {
    'q'
}

fn default_select() -> char {
    ' '
}

#[derive(Deserialize, Debug)]
pub struct Colors {
    #[serde(default = "default_focused_border", deserialize_with = "deserialize_color")]
    pub focused_border: Color,

    #[serde(default = "default_unfocused_border", deserialize_with = "deserialize_color")]
    pub unfocused_border: Color,

    #[serde(default = "default_focused_header", deserialize_with = "deserialize_color")]
    pub focused_header: Color,

    #[serde(default = "default_highlight_bg", deserialize_with = "deserialize_color")]
    pub highlight_bg: Color,

    #[serde(default = "default_highlight_fg", deserialize_with = "deserialize_color")]
    pub highlight_fg: Color,

    #[serde(default = "default_info", deserialize_with = "deserialize_color")]
    pub info: Color,

    #[serde(default = "default_warning", deserialize_with = "deserialize_color")]
    pub warning: Color,

    #[serde(default = "default_error", deserialize_with = "deserialize_color")]
    pub error: Color,

    #[serde(default = "default_spinner", deserialize_with = "deserialize_color")]
    pub spinner: Color,

    #[serde(default = "default_help_text", deserialize_with = "deserialize_color")]
    pub help_text: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            focused_border: Color::Green,
            unfocused_border: Color::Reset,
            focused_header: Color::Yellow,
            highlight_bg: Color::DarkGray,
            highlight_fg: Color::White,
            info: Color::Green,
            warning: Color::Yellow,
            error: Color::Red,
            spinner: Color::Blue,
            help_text: Color::Blue,
        }
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    
    match s.to_lowercase().as_str() {
        "reset" => Ok(Color::Reset),
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "gray" | "grey" => Ok(Color::Gray),
        "darkgray" | "darkgrey" | "dark_gray" | "dark_grey" => Ok(Color::DarkGray),
        "lightred" | "light_red" => Ok(Color::LightRed),
        "lightgreen" | "light_green" => Ok(Color::LightGreen),
        "lightyellow" | "light_yellow" => Ok(Color::LightYellow),
        "lightblue" | "light_blue" => Ok(Color::LightBlue),
        "lightmagenta" | "light_magenta" => Ok(Color::LightMagenta),
        "lightcyan" | "light_cyan" => Ok(Color::LightCyan),
        "white" => Ok(Color::White),
        _ => {
            // Try to parse as RGB hex color (#RRGGBB)
            if let Some(hex) = s.strip_prefix('#') {
                if hex.len() == 6 {
                    if let Ok(r) = u8::from_str_radix(&hex[0..2], 16) {
                        if let Ok(g) = u8::from_str_radix(&hex[2..4], 16) {
                            if let Ok(b) = u8::from_str_radix(&hex[4..6], 16) {
                                return Ok(Color::Rgb(r, g, b));
                            }
                        }
                    }
                }
            }
            Err(de::Error::custom(format!("Invalid color: {}. Use named colors (e.g., 'green', 'yellow') or hex format '#RRGGBB'", s)))
        }
    }
}

fn default_focused_border() -> Color {
    Color::Green
}

fn default_unfocused_border() -> Color {
    Color::Reset
}

fn default_focused_header() -> Color {
    Color::Yellow
}

fn default_highlight_bg() -> Color {
    Color::DarkGray
}

fn default_highlight_fg() -> Color {
    Color::White
}

fn default_info() -> Color {
    Color::Green
}

fn default_warning() -> Color {
    Color::Yellow
}

fn default_error() -> Color {
    Color::Red
}

fn default_spinner() -> Color {
    Color::Blue
}

fn default_help_text() -> Color {
    Color::Blue
}

/// The optional second colour file, layered over the `[colors]` table of the
/// config.
///
/// It is for tools that recolour a desktop from the current wallpaper --
/// wallust, pywal -- which re-extract a palette on every wallpaper change and
/// have no business owning a config file that also holds your keybinds and
/// layout. Such a tool renders the overlay; bluetui is a fresh process every
/// time it starts, so the next run already has the new colours.
///
/// Set BLUETUI_COLORS_OVERLAY to read it from somewhere else.
fn colors_overlay_path() -> PathBuf {
    if let Ok(path) = std::env::var("BLUETUI_COLORS_OVERLAY") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }

    let cache = match std::env::var_os("XDG_CACHE_HOME") {
        Some(dir) => PathBuf::from(dir),
        None => dirs::home_dir().unwrap_or_default().join(".cache"),
    };
    cache.join("wallust").join("colors-bluetui.toml")
}

/// Merge the overlay's `[colors]` entries over the config's, key by key.
///
/// Per key rather than per file, so an overlay naming only `error` leaves the
/// other nine as the config has them. A missing, unreadable or malformed
/// overlay is the normal state -- it means "use the config" -- and is silently
/// nothing, which is what every theme that is not wallpaper-derived wants.
fn merge_colors_overlay(config: &mut toml::Value) {
    let text = match std::fs::read_to_string(colors_overlay_path()) {
        Ok(text) => text,
        Err(_) => return,
    };
    // toml::from_str, not str::parse: FromStr for Value parses a single VALUE,
    // so a document starting with [colors] is read as an array and then errors
    // on the rest of the file. Caught by instrumenting this -- it fails
    // silently by design, which is right for a missing overlay and very wrong
    // for one that is simply being parsed the wrong way.
    let overlay: toml::Value = match toml::from_str(&text) {
        Ok(value) => value,
        Err(_) => return,
    };

    let overlay_colors = match overlay.get("colors").and_then(|v| v.as_table()) {
        Some(table) => table.clone(),
        None => return,
    };
    let table = match config.as_table_mut() {
        Some(table) => table,
        None => return,
    };

    let colors = table
        .entry("colors".to_string())
        .or_insert_with(|| toml::Value::Table(Default::default()));
    if let Some(colors) = colors.as_table_mut() {
        for (key, value) in overlay_colors {
            colors.insert(key, value);
        }
    }
}

impl Config {
    pub fn new(config_file_path: Option<PathBuf>) -> Self {
        let conf_path = config_file_path.unwrap_or(
            dirs::config_dir()
                .unwrap()
                .join("bluetui")
                .join("config.toml"),
        );

        let config = std::fs::read_to_string(conf_path).unwrap_or_default();
        // Parsed to a Value first so the overlay can be merged into it before
        // anything is deserialised. Appending the two files as text cannot
        // work: TOML rejects a second [colors] header as a duplicate key.
        let mut value: toml::Value = match toml::from_str(&config) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };
        merge_colors_overlay(&mut value);

        let app_config: Config = match value.try_into() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        };

        app_config
    }
}