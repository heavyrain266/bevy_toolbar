use super::settings::ToolbarSettings;

use bevy::{
    prelude::*,
    diagnostic::*
};
use bevy_egui::*;
pub struct BottomPanel;

impl Plugin for BottomPanel {
    fn build(&self, app: &mut App) {
        app.add_system(bottom_panel);
    }
}

pub fn bottom_panel(
    egui: Res<EguiContext>,
    diag: Res<Diagnostics>,
    mut windows: ResMut<Windows>,
    mut settings: ResMut<ToolbarSettings>,
) {
    let window = windows
        .get_primary_mut()
        .unwrap();

    egui::TopBottomPanel::bottom("Window state")
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                if settings.option.window_title {
                    ui.label("Title:");
                    ui.spacing_mut().text_edit_width = 160.0;
                    ui.text_edit_singleline(&mut settings.title);

                    if window.title().ne(&settings.title) {
                        window.set_title(settings.title.clone());
                    }
                }

                if settings.option.ft {
                    if let Some(ft) = diag.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                        if let Some(average) = ft.average() {
                            ui.label(format!("FT: {:.2}ms", average * 1000.0));
                        }
                    }
                }

                if settings.option.fps {
                    if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(average) = fps.average() {
                            ui.label(format!("FPS: {:.2}", average));
                        }
                    }
                }
            });
        });
    }
