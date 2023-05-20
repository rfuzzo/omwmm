use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use egui_notify::Toasts;
use log::{error, info, warn};

use crate::{EScale, ModViewModel, PluginViewModel};
use common::{get_openmwcfg, get_plugins_in_folder};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    // ui
    pub light_mode: bool,
    pub scale: EScale,
    #[serde(skip)]
    pub toasts: Toasts,

    // app
    /// the folder where mod archives are stored
    pub downloads_library: Option<String>,
    /// runtime cache of mod archive paths
    #[serde(skip)]
    pub downloads: Vec<PathBuf>,

    /// the folder where mods are extracted to
    pub mods_library: Option<String>,
    /// info which mods are available
    pub mods: Vec<ModViewModel>,
    /// all plugins. should be populated on start
    #[serde(skip)]
    pub plugins: Vec<PluginViewModel>,
    #[serde(skip)]
    pub init: bool,
    ///
    pub current_profile: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            light_mode: false,
            scale: EScale::Small,
            toasts: Toasts::default(),
            downloads_library: None,
            downloads: vec![],
            mods_library: None,
            mods: vec![],
            plugins: vec![],
            init: false,
            current_profile: "default".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //cc.egui_ctx.set_pixels_per_point(2.0_f32);

        // simple_logger::init().unwrap();

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
        // TODO make proper viewmodels
        // TODO remove dbg
        self.downloads
            .push(Path::new(&library_path.as_str()).join("dbg1.zip"));
        self.downloads
            .push(Path::new(&library_path.as_str()).join("dbg2.zip"));
    }

    /// updates the openmw.cfg's content= and data= entries
    /// goes through the enabled mods and adds data= entries for each
    /// goes through the enbaled plugins and adds content= entries for each
    ///
    pub fn update_cfg(&mut self) -> bool {
        // TODO use per-profile configs?

        // find omw cfg
        if let Some(cfg_path) = get_openmwcfg() {
            // get everything that is not a content line
            info!("Parsing cfg {} ...", cfg_path.display());
            let mut original_cfg: Vec<String> = vec![];
            if let Ok(lines) = common::read_lines(&cfg_path) {
                for line in lines.flatten() {
                    // parse each line
                    if line.starts_with("data=") {
                        // TODO can we check this better?
                        if line.ends_with("Data Files\"") {
                            original_cfg.push(line);
                        }
                    } else if line.starts_with("content=") {
                        // ignore vanilla esms
                        if line.ends_with("Morrowind.esm")
                            || line.ends_with("Bloodmoon.esm")
                            || line.ends_with("Tribunal.esm")
                        {
                            original_cfg.push(line);
                        }
                    } else {
                        original_cfg.push(line);
                    }
                }
            } else {
                error!("Could not parse cfg file {}", cfg_path.display());
                return false;
            }
            // reassemble cfg
            if let Ok(mut file) = File::create(&cfg_path) {
                // write original lines
                for line in original_cfg {
                    // TODO proper eol
                    let line_with_eol = format!("{}\n", line);
                    match file.write(line_with_eol.as_bytes()) {
                        Ok(_) => {}
                        Err(err) => warn!("Error writing line {}: {}", line, err),
                    }
                }

                // write data
                if let Some(_library) = self.mods_library.clone() {
                    for m in &self.mods.iter().filter(|p| p.enabled).collect::<Vec<_>>() {
                        // TODO proper eol
                        // get full path
                        let data_line = format!("data=\"{}\"\n", m.full_name.to_string_lossy());
                        match file.write(data_line.as_bytes()) {
                            Ok(_) => {}
                            Err(err) => {
                                warn!("Error writing plugin {}: {}", m.full_name.display(), err)
                            }
                        }
                    }
                }

                // write plugins
                for p in &self
                    .plugins
                    .iter()
                    .filter(|p| p.enabled)
                    .collect::<Vec<_>>()
                {
                    // TODO proper eol
                    let content_line = format!("content={}\n", p.name);
                    match file.write(content_line.as_bytes()) {
                        Ok(_) => {}
                        Err(err) => warn!("Error writing plugin {}: {}", p.name, err),
                    }
                }
            } else {
                error!("Could not write cfg file {}", cfg_path.display());
                return false;
            }
        }

        false
    }

    /// Gets a path to the current profile dir and creates it if it doesn't exist
    pub fn get_current_profile_dir(&self) -> PathBuf {
        let current_profile_dir = dirs::config_dir()
            .unwrap()
            .join("omwmm")
            .join("profiles")
            .join(self.current_profile.as_str());
        if !current_profile_dir.exists() {
            fs::create_dir_all(&current_profile_dir).expect("Failed to create current profile dir");
        }
        current_profile_dir
    }

    /// initialize enabled mods and plugins from the current profile
    /// this executes once on the first frame
    /// TODO make this safer?
    pub(crate) fn init_profile(&mut self) {
        if self.init {
            return;
        }

        // if the app mods are empty, we import the openmw.cfg
        if self.mods.is_empty() {
            if let Some(cfg_path) = common::get_openmwcfg() {
                if let Some(info) = common::parse_cfg(cfg_path) {
                    for data_path in info.data {
                        // TODO handle vanilla dirs`
                        self.mods.push(ModViewModel {
                            full_name: data_path,
                            enabled: false,
                        })
                    }
                }
            }
        }

        // load the mod list and plugin list from the profiles folder
        let current_profile_dir = self.get_current_profile_dir();
        let mods_list_path = current_profile_dir.join("mods.txt");
        if mods_list_path.exists() {
            // load
            if let Ok(lines) = common::read_lines(&mods_list_path) {
                for mod_name in lines.flatten() {
                    // update mods enabled state
                    if let Some(info) = self
                        .mods
                        .iter_mut()
                        .find(|p| p.full_name.to_string_lossy() == mod_name)
                    {
                        info.enabled = true;
                    }
                }
            }
        }

        // crawl mods and populate plugins list
        self.plugins.clear();
        for mod_info in self.mods.iter() {
            if let Some(_library) = self.mods_library.clone() {
                let plugins = get_plugins_in_folder(&mod_info.full_name);
                for p in plugins {
                    if let Some(plugin_name) = p.file_name() {
                        self.plugins.push(PluginViewModel {
                            name: plugin_name.to_string_lossy().into(),
                            enabled: false,
                        });
                    } else {
                        // TODO logging
                    }
                }
            }
        }
        // update plugins enabled state
        let plugins_list_path = current_profile_dir.join("plugins.txt");
        if plugins_list_path.exists() {
            // load
            if let Ok(lines) = common::read_lines(&plugins_list_path) {
                for plugin_name in lines.flatten() {
                    if let Some(info) = self.plugins.iter_mut().find(|p| p.name == plugin_name) {
                        info.enabled = true;
                    }
                }
            }
        }

        self.init = true;
    }

    /// serializes the mods to the profile
    pub(crate) fn update_profile_mods(&self) {
        let mods_paths_list: Vec<_> = self
            .mods
            .iter()
            .filter(|f| f.enabled)
            .map(|e| format!("{}\n", e.full_name.to_string_lossy()))
            .collect();
        // save the list to file
        let current_profile_dir = self.get_current_profile_dir();
        let mods_list_path = current_profile_dir.join("mods.txt");
        if let Ok(mut f) = fs::File::create(mods_list_path) {
            for mod_name in mods_paths_list {
                match f.write(mod_name.as_bytes()) {
                    Ok(_) => {}
                    Err(_err) => {
                        // TODO logging
                    }
                }
            }
        } else {
            // TODO logging
        }
    }
}
