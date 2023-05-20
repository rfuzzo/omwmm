use crate::{app::ETheme, EScale, TemplateApp};

impl TemplateApp {
    #[allow(unused_variables)] // for wasm
    pub fn menu_bar_view(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        // Menu Bar
        egui::menu::bar(ui, |ui| {
            // FILE Menu
            ui.menu_button("File", |ui| {
                // TODO install mod

                ui.separator();
                // Quit button
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });

            // GAME menu
            ui.menu_button("Game", |ui| {
                if ui.button("Open config").clicked() {
                    if let Some(cfg_path) = common::get_openmwcfg() {
                        if open::that(cfg_path).is_err() {
                            self.toasts.error("Could not open openmw.cfg");
                        }
                    }
                }
            });

            // theme button on right
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                // theme
                theme_switch(ui, &mut self.theme);
                // scale
                egui::ComboBox::from_label("Scale: ")
                    .selected_text(format!("{:?}", self.scale))
                    .show_ui(ui, |ui| {
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
            ui.selectable_value(theme, ETheme::LATTE, "LATTE");
            ui.selectable_value(theme, ETheme::FRAPPE, "FRAPPE");
            ui.selectable_value(theme, ETheme::MACCHIATO, "MACCHIATO");
            ui.selectable_value(theme, ETheme::MOCHA, "MOCHA");
        });
}

// taken from egui
fn global_dark_light_mode_switch(ui: &mut egui::Ui, light_mode: &mut bool) {
    let style: egui::Style = (*ui.ctx().style()).clone();
    let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
    if let Some(visuals) = new_visuals {
        *light_mode = !visuals.dark_mode;
        ui.ctx().set_visuals(visuals);
    }
}
