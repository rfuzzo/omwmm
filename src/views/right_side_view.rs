use std::time::Duration;

use crate::TemplateApp;

impl TemplateApp {
    /// this view holds the downloads and esps
    pub fn combined_side_view(&mut self, ui: &mut egui::Ui) {
        // downloads
        // library folder path
        ui.horizontal(|ui| {
            if let Some(p) = self.downloads_library.clone() {
                ui.label(p.as_str());
            } else {
                ui.label("Choose library path ...");
            }
            if ui.button("...").clicked() {
                // TODO pick folder
                self.downloads_library = Some("/Users/ghost/Documents/omwmm/downloads".into());
            }
        });

        ui.separator();

        // downloads list
        if let Some(library_path) = self.downloads_library.clone() {
            // refresh downloads list
            if self.downloads.is_empty() {
                self.refresh_downloads(library_path);
            }
            // populate list
            egui::ScrollArea::vertical().show(ui, |ui| {
                // TODO use table
                for path in self.downloads.iter() {
                    // create viewmodel
                    if let Some(filename) = path.file_name() {
                        if ui
                            .add(
                                egui::Label::new(filename.to_string_lossy())
                                    .sense(egui::Sense::click()),
                            )
                            .double_clicked()
                        {
                            // TODO install mod
                            self.toasts
                                .success("Mod installed")
                                .set_duration(Some(Duration::from_secs(5)));
                        }
                    }
                }
            });
        }
    }
}
