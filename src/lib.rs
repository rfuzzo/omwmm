use std::{
    fs::File,
    io::{self, BufRead},
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

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ModInfo {
    pub path: PathBuf,
    pub enabled: bool,
    // TODO files?
}

/// Returns an Iterator to the Reader of the lines of the file.
pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
            todo!();
        }
        "macos" => {
            // default cfg for mac is at $HOME/Library/Preferences/openmw
            let home = dirs::home_dir().unwrap();
            let cfg = home
                .join("Library")
                .join("Preferences")
                .join("openmw")
                .join("openmw.cfg");
            if cfg.exists() {
                Some(cfg)
            } else {
                None
            }
        }
        "windows" => {
            todo!()
        }
        _ => None,
    }
}
