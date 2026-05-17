use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_opacity: f32,
    pub always_on_top: bool,
    pub frameless: bool,
    pub enable_transparency: bool,
    pub start_url: String,
    pub cache_dir: PathBuf,
    pub window_width: u32,
    pub window_height: u32,
    pub shortcuts: Vec<ShortcutDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutDef {
    pub keys: String,
    pub action: String,
}

impl Default for Config {
    fn default() -> Self {
        let proj = ProjectDirs::from("com", "heliumgecko", "Helium").unwrap();
        Self {
            default_opacity: 0.92,
            always_on_top: true,
            frameless: true,
            enable_transparency: true,
            start_url: "https://www.youtube.com".to_string(),
            cache_dir: proj.cache_dir().to_path_buf(),
            window_width: 1200,
            window_height: 900,
            shortcuts: vec![
                ShortcutDef { keys: "Ctrl+Q".into(), action: "quit".into() },
                ShortcutDef { keys: "Ctrl+Shift+O".into(), action: "toggle_opacity_slider".into() },
            ],
        }
    }
}

impl Config {
    pub fn load(custom_path: Option<&str>) -> Self {
        let path = custom_path.map(PathBuf::from).unwrap_or_else(|| {
            let proj = ProjectDirs::from("com", "heliumgecko", "Helium").unwrap();
            proj.config_dir().join("config.toml")
        });
        if path.exists() {
            let content = fs::read_to_string(&path).expect("Cannot read config");
            toml::from_str(&content).unwrap_or_else(|e| {
                log::warn!("Config parse error: {e}, using defaults");
                Config::default()
            })
        } else {
            let default = Config::default();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).ok();
            }
            let toml_str = toml::to_string_pretty(&default).unwrap();
            fs::write(&path, toml_str).ok();
            default
        }
    }
}
