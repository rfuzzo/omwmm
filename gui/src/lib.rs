use std::{
    fs::{self},
    path::{Path, PathBuf},
};

pub use app::TemplateApp;

mod app;
mod appui;
mod views;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EScale {
    Small,
    Medium,
    Large,
}
impl From<EScale> for f32 {
    fn from(val: EScale) -> Self {
        match val {
            EScale::Small => 2.2,
            EScale::Medium => 3.0,
            EScale::Large => 4.5,
        }
    }
}

#[derive(Default)]
pub struct PluginViewModel {
    pub name: String,
    pub enabled: bool,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ModViewModel {
    /// Mod name, to get the full path join this with the mod library
    pub name: String,

    // if a mod is enabled or not depends on the current profile
    // do not serialize this centrally
    #[serde(skip)]
    pub enabled: bool,
    // TODO files?
}
impl ModViewModel {
    pub fn get_full_path<P>(&self, library: &P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        library.as_ref().join(self.name.clone())
    }
}

/// Returns the default openmw.cfg path if it exists, and None if not
///
/// # Panics
///
/// Panics if Home dir is not found in the OS
fn get_openmwcfg() -> Option<PathBuf> {
    let os_str = std::env::consts::OS;
    match os_str {
        "linux" => {
            // default cfg for linux is at $HOME/.config/openmw
            let preference_dir = dirs::config_dir().unwrap();
            let cfg = preference_dir.join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        "macos" => {
            // default cfg for mac is at /Users/Username/Library/Preferences/openmw
            let preference_dir = dirs::preference_dir().unwrap();
            let cfg = preference_dir.join("openmw").join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        "windows" => {
            // default cfg for windows is at C:\Users\Username\Documents\my games\openmw
            let preference_dir = dirs::document_dir().unwrap();
            let cfg = preference_dir
                .join("my games")
                .join("openmw")
                .join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Get all plugins (esp, omwaddon, omwscripts) in a folder
fn get_plugins_in_folder<P>(path: &P) -> Vec<PathBuf>
where
    P: AsRef<Path>,
{
    // get all plugins
    let mut results: Vec<PathBuf> = vec![];
    if let Ok(plugins) = fs::read_dir(path) {
        plugins.for_each(|p| {
            if let Ok(file) = p {
                let file_path = file.path();
                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "esm" || ext == "esp" || ext == "omwaddon" || ext == "omwscripts"
                        {
                            results.push(file_path);
                        }
                    }
                }
            }
        });
    }
    results
}
