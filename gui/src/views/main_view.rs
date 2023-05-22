use crate::{ModViewModel, TemplateApp};

impl TemplateApp {
    pub fn main_view(&mut self, ui: &mut egui::Ui) {
        // library folder path
        ui.horizontal(|ui| {
            if let Some(p) = self.mods_library.clone() {
                ui.label(p.display().to_string());
            } else {
                ui.label("Choose mod library path ...");
            }
            if ui.button("...").clicked() {
                if let Some(folder) = rfd::FileDialog::new().set_directory("/").pick_folder() {
                    self.mods_library = Some(folder);
                }
            }
        });

        ui.separator();

        // mods view
        // this is the main view
        // it holds a list of installed mods (states of them vary per profile)
        // a mod can be enabled or disabled
        // the installed mods info can be serialized centrally
        // we can add a health check on app start, rest is user fault
        if let Some(_library) = self.mods_library.clone() {
            let mut is_any_changed = false;
            let mut to_delete: Vec<usize> = vec![];

            let mut i = 0;
            // ui.add_sized(ui.available_size(), widget);
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let response = self.dnd_mods.ui::<ModViewModel>(
                        ui,
                        self.mods.iter_mut(),
                        |mod_info, ui, handle| {
                            // the list item view
                            ui.horizontal(|ui| {
                                let r = ui.push_id(&mod_info.full_name.clone(), |ui| {
                                    ui.horizontal(|ui| {
                                        // Anything in the handle can be used to drag the item
                                        handle.ui(ui, mod_info, |ui| {
                                            ui.label(" :: ");
                                        });
                                        if ui.checkbox(&mut mod_info.enabled, "").changed() {
                                            is_any_changed = true;
                                        }
                                        ui.label(
                                            mod_info
                                                .full_name
                                                .file_name()
                                                .unwrap()
                                                .to_string_lossy(),
                                        );
                                    })
                                });
                                // context menu
                                r.response.context_menu(|ui| {
                                    // uninstall mod
                                    if ui.button("Uninstall").clicked() {
                                        // delete the mod from the mod library
                                        if mod_info.full_name.exists() {
                                            match std::fs::remove_dir_all(&mod_info.full_name) {
                                                Ok(_) => {
                                                    self.toasts.success("Mod removed");
                                                    // remove the mod from the list
                                                    to_delete.push(i);
                                                    ui.close_menu();
                                                }
                                                Err(err) => {
                                                    log::error!(
                                                        "failed to remove mod {}: {}",
                                                        mod_info.full_name.display(),
                                                        err
                                                    );
                                                }
                                            }
                                        }
                                    }
                                });
                            });

                            i += 1;
                        },
                    );

                    // After the drag is complete, we get a response containing the old index of the
                    // dragged item, as well as the index it was moved to. You can use the
                    // shift_vec function as a helper if you store your items in a Vec.
                    if let Some(response) = response.completed {
                        egui_dnd::utils::shift_vec(response.from, response.to, &mut self.mods);
                    }
                });

            // delete mods
            for idx in to_delete {
                self.mods.remove(idx);
                is_any_changed = true;
            }

            // update cfg
            if is_any_changed {
                // update serialized mod list
                self.update_profile_mods();

                // update openmwcfg
                self.update_cfg();
            }
        }
    }
}
