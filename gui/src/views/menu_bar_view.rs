use crate::{app::ETheme, EScale, TemplateApp};

impl TemplateApp {
    #[allow(unused_variables)] // for wasm
    pub fn menu_bar_view(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Menu Bar
        egui::menu::bar(ui, |ui| {
            // FILE Menu
            ui.menu_button("File", |ui| {
                if ui.button("Install Mod").clicked() {
                    // TODO menu: install mod
                }

                ui.separator();

                // Quit button
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });

            // GAME menu
            ui.menu_button("Game", |ui| {
                // star game
                if ui.button("Start game").clicked() {
                    // TODO menu: start game/launcher
                }

                ui.separator();

                // open game folder
                if ui.button("Open game directory").clicked() {
                    // TODO menu: open game folder
                }
                // open cfg
                if ui.button("Open config").clicked() {
                    if let Some(cfg_path) = common::get_openmwcfg() {
                        if open::that(cfg_path).is_err() {
                            self.toasts.error("Could not open openmw.cfg");
                        }
                    }
                }
            });

            // MODS Menu
            ui.menu_button("Mods", |ui| {
                // open mods folder
                if ui.button("Open mods library").clicked() {
                    // TODO menu: open mods folder
                }
                // open downloads folder
                if ui.button("Open downloads library").clicked() {
                    // TODO menu: open downloads folder
                }
            });

            // right settings //TODO move to settings?
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                // theme
                theme_switch(ui, &mut self.theme);
                // scale
                egui::ComboBox::from_label("Scale: ")
                    .selected_text(format!("{:?}", self.scale))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.scale, EScale::Native, "Native");
                        ui.selectable_value(&mut self.scale, EScale::Small, "Small");
                        ui.selectable_value(&mut self.scale, EScale::Medium, "Medium");
                        ui.selectable_value(&mut self.scale, EScale::Large, "Large");
                    });
            });
        });
    }
}

fn theme_switch(ui: &mut egui::Ui, theme: &mut crate::app::ETheme) {
    egui::ComboBox::from_label("Theme")
        .selected_text(format!("{:?}", theme))
        .show_ui(ui, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.set_min_width(60.0);
            ui.selectable_value(theme, ETheme::Latte, "LATTE");
            ui.selectable_value(theme, ETheme::Frappe, "FRAPPE");
            ui.selectable_value(theme, ETheme::Macchiato, "MACCHIATO");
            ui.selectable_value(theme, ETheme::Mocha, "MOCHA");
        });
}
