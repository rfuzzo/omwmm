use std::path::{Path, PathBuf};

use egui_notify::Toasts;

use crate::EScale;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // ui
    pub light_mode: bool,
    pub scale: EScale,
    #[serde(skip)]
    pub toasts: Toasts,
    // app
    pub downloads_library: Option<String>,
    #[serde(skip)]
    pub downloads: Vec<PathBuf>,
    pub mods_library: Option<String>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            light_mode: false,
            scale: EScale::Small,
            toasts: Toasts::default(),
            // TODO fix this with env vars
            downloads_library: None,
            downloads: vec![],
            mods_library: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //cc.egui_ctx.set_pixels_per_point(2.0_f32);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    // UI methods

    pub fn update_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            self.menu_bar_view(ui, frame);
        });
    }

    pub fn update_right_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("side_panel")
            //.min_width(250_f32)
            .show(ctx, |ui| {
                self.combined_side_view(ui);
            });
    }

    pub fn update_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_view(ui);
        });
    }

    // Logic

    /// refreshes the downloads list by walking the downloads library
    pub fn refresh_downloads(&mut self, library_path: String) {
        // TODO get files
        // TODO remove dbg
        self.downloads
            .push(Path::new(&library_path.as_str()).join("dbg1"));
        self.downloads
            .push(Path::new(&library_path.as_str()).join("dbg2"));
    }
}
