use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use bevy_egui::*;

use crate::top_panel::ToolbarSettings;

pub(crate) struct BottomPanel;

impl Plugin for BottomPanel {
    fn build(&self, app: &mut App) {
        app.add_system(bottom_panel);
    }
}

pub(self) fn bottom_panel(
    egui: Res<EguiContext>,
    diag: Res<Diagnostics>,
    mut windows: ResMut<Windows>,
    mut settings: ResMut<ToolbarSettings>,
) {
    let prime = windows.get_primary_mut().unwrap();

    egui::TopBottomPanel::bottom("Window state").show(egui.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Title:");
            ui.spacing_mut().text_edit_width = 160.;
            ui.text_edit_singleline(&mut settings.title);

            if prime.title().ne(&settings.title){
                prime.set_title(settings.title.clone());
            }

            if settings.setting_toggles.fps {
                if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        ui.label(format!("FPS: {:.2}", average));
                    }
                }
            }
            if settings.setting_toggles.ft {
                if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                    if let Some(average) = fps.average() {
                        ui.label(format!("Frame Time: {:.2}ms", average * 1000.));
                    }
                }
            }
        });
    });
}
