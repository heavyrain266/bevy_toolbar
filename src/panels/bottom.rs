use {
	bevy::{
		prelude::{
			Res, ResMut, App, Plugin
		},
		window::Windows,
		diagnostic::{
			Diagnostics, FrameTimeDiagnosticsPlugin
		},
	},
	bevy_egui::{egui, EguiContext},

	super::options::Options,
};

pub struct BottomPanel;

impl Plugin for BottomPanel {
	fn build(&self, app: &mut App) {
		app.add_system(bottom_panel);
	}
}

fn bottom_panel(
	diag: Res<Diagnostics>,
	mut egui: ResMut<EguiContext>,
	mut windows: ResMut<Windows>,
	mut options: ResMut<Options>,
) {
	let window = windows
		.get_primary_mut()
		.expect("Cannot get primary window");

	    egui::TopBottomPanel::bottom("Window state")
        .show(egui.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if options.toggle.window_title {
                    ui.label("Title:");
                    ui.spacing_mut().text_edit_width = 160.0;
                    ui.text_edit_singleline(&mut options.title);

                    if window.title() != (&options.title) {
                        window.set_title(options.title.clone());
                    }
                }

                if options.toggle.frame_time {
                    if let Some(ft) = diag.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
                        if let Some(average) = ft.average() {
                            ui.label(format!("FT: {:.2}ms", average * 1000.0));
                        }
                    }
                }

                if options.toggle.fps {
                    if let Some(fps) = diag.get(FrameTimeDiagnosticsPlugin::FPS) {
                        if let Some(average) = fps.average() {
                            ui.label(format!("FPS: {:.2}", average));
                        }
                    }
                }
            });
        });
}
