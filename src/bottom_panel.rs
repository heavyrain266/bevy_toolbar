use bevy::{
    prelude::*,
};

use bevy_egui::*;

pub(crate) struct BottomPanel;

impl Plugin for BottomPanel {
    fn build(
        &self,
        app: &mut App
    ) {
        app.add_system(bottom_panel);
    }
}

pub(self) fn bottom_panel(
    egui: Res<EguiContext>,
    mut windows: ResMut<Windows>
) {
    let prime = windows
    .get_primary_mut()
    .unwrap();

    egui::TopBottomPanel::bottom("Window state")
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Title: {}", prime.title()));
                ui.label(format!("Vsync: {}", prime.vsync()));
                ui.label(format!("Mode: {:?}", prime.mode()));
                ui.label(format!("Size: {}", prime.width()) + "x" + &format!("{}", prime.height()) + "px");
        });
    });
}
