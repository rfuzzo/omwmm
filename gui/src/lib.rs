use std::path::PathBuf;

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
    pub full_name: PathBuf,

    // if a mod is enabled or not depends on the current profile
    // do not serialize this centrally
    #[serde(skip)]
    pub enabled: bool,
    // TODO files?
}
