use crate::TemplateApp;

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // general storage save
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(f32::from(self.scale));
        // TODO themes
        catppuccin_egui::set_theme(ctx, get_theme(&self.theme));

        self.init_profile();

        // Top Panel
        self.update_top_panel(ctx, frame);

        // bottom Panel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            // Status Bar
            ui.horizontal(|ui| {
                // VERSION
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.label(VERSION);
                    ui.label("Version: ");
                    ui.separator();
                    ui.hyperlink("https://github.com/rfuzzo/omwmm");

                    ui.separator();
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        // Side Panel
        self.update_right_side_panel(ctx);

        // Central Panel
        self.update_central_panel(ctx);

        // notifications
        self.toasts.show(ctx);
    }
}

fn get_theme(theme: &crate::app::ETheme) -> catppuccin_egui::Theme {
    match theme {
        crate::app::ETheme::Frappe => catppuccin_egui::FRAPPE,
        crate::app::ETheme::Latte => catppuccin_egui::LATTE,
        crate::app::ETheme::Macchiato => catppuccin_egui::MACCHIATO,
        crate::app::ETheme::Mocha => catppuccin_egui::MOCHA,
    }
}
