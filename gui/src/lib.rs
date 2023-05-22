use std::path::PathBuf;

pub use app::TemplateApp;

mod app;
mod appui;
mod views;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EScale {
    Native,
    Small,
    Medium,
    Large,
}
impl From<EScale> for f32 {
    fn from(val: EScale) -> Self {
        match val {
            EScale::Native => 1.2,
            EScale::Small => 2.2,
            EScale::Medium => 3.0,
            EScale::Large => 4.5,
        }
    }
}

#[derive(Default, PartialEq)]
pub struct PluginViewModel {
    pub name: String,
    pub enabled: bool,
}
// We need this to uniquely identify items. You can also implement the Hash trait.
impl egui_dnd::DragDropItem for PluginViewModel {
    fn id(&self) -> egui::Id {
        egui::Id::new(&self.name)
    }
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
    // TODO files
    // TODO vanilla tag
}
// We need this to uniquely identify items. You can also implement the Hash trait.
impl egui_dnd::DragDropItem for ModViewModel {
    fn id(&self) -> egui::Id {
        egui::Id::new(&self.full_name)
    }
}

#[derive(Default)]
pub struct ArchiveViewModel {
    pub file_name: String,
    //pub installed: bool,

    // nexus metadata
    pub meta_data: Option<ModNexusMetaData>,
}

#[derive(Default, serde::Serialize, serde::Deserialize, Debug)]
pub struct ModNexusMetaData {
    pub mod_id: Option<usize>,
    pub description: Option<String>,
    pub mod_name: Option<String>,
    pub version: Option<String>,
}
