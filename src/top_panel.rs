use bevy::{
    prelude::*,
    window::WindowMode
};

use bevy_egui::*;

pub(crate) struct Panels;

impl Plugin for Panels {
    fn build(
        &self,
        app: &mut App
    ) {
        app.add_system(top_panel);
    }
}

pub(self) fn top_panel(
    egui: Res<EguiContext>,
    mut windows: ResMut<Windows>
) {
    let prime = windows
        .get_primary_mut()
        .unwrap();

    egui::TopBottomPanel::top("Window mode")
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Descriptor", |ui| {
                    ui.menu_button("Mode", |ui| {
                        if ui.button("Windowed").clicked() {
                            prime.set_mode(WindowMode::Windowed);
                        } else if ui.button("Fullscreen").clicked() {
                            prime.set_mode(WindowMode::Fullscreen);
                        } else if ui.button("Borderless").clicked() {
                            prime.set_mode(WindowMode::BorderlessFullscreen);
                        }
                    });
                });
            });
        });
}
