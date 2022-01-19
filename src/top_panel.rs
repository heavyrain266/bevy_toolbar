use bevy::{
    prelude::*,
    window::WindowMode
};

use bevy_egui::*;

pub(crate) struct TopPanel;

impl Plugin for TopPanel {
    fn build(
        &self,
        app: &mut App
    ) {
        app.add_system(top_panel);
    }
}

pub(self) fn top_panel(
    egui: Res<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut windows: ResMut<Windows>
) {
    let prime = windows
        .get_primary_mut()
        .unwrap();

    egui::TopBottomPanel::top("Window")
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Window", |ui| {
                    ui.menu_button("Mode", |ui| {
                        if ui.button("windowed").clicked() {
                            prime.set_mode(WindowMode::Windowed);
                        } else if ui.button("fullscreen").clicked() {
                            prime.set_mode(WindowMode::Fullscreen);
                        } else if ui.button("borderless").clicked() {
                            prime.set_mode(WindowMode::BorderlessFullscreen);
                        }
                    });
                    ui.menu_button("Settings", |ui| {
                        ui.menu_button("Vsync", |ui| {
                            if ui.button("on").clicked() {
                                prime.set_vsync(true);
                            } else if ui.button("off").clicked() {
                                prime.set_vsync(false);
                            }
                        });
                        ui.menu_button("MSAA", |ui|{
                            if ui.button("2x").clicked() {
                                msaa.samples = 2;
                            } else if ui.button("4x").clicked() {
                                msaa.samples = 4;
                            } else if ui.button("8x").clicked() {
                                msaa.samples = 8;
                            } else if ui.button("16x").clicked() {
                                msaa.samples = 16;
                            }
                        });
                    });
                });
            });
        });
}
