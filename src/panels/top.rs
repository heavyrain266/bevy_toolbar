use {
	bevy::{
		prelude::{
			Windows, ResMut,
			Msaa, App, Plugin
		},
		window::WindowMode,
	},
	bevy_egui::{egui, EguiContext},
	strum_macros::EnumIter,
	strum::IntoEnumIterator,

	super::options::Options,
};

pub struct TopPanel;

impl Plugin for TopPanel {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<Options>()
			.add_system(top_panel)
			.add_startup_system(get_window_info);
	}
}

// WebGPU currently only exposes 1(Off) and 4x Samples
// Updates can be tracked here: https://github.com/gfx-rs/wgpu/issues/1832
#[derive(Debug, EnumIter)]
enum MsaaSettings {
    Off = 1,
    X4 = 4,
}

fn get_window_info(
	mut windows: ResMut<Windows>,
	mut options: ResMut<Options>,
) {
	let window = windows
		.get_primary_mut()
		.expect("Cannot get primary window");

	options.current_size.width = window.requested_width();
	options.current_size.height = window.requested_height();
}

pub fn top_panel(
	mut egui: ResMut<EguiContext>,
	mut msaa: ResMut<Msaa>,
	mut windows: ResMut<Windows>,
	mut options: ResMut<Options>,
) {
	let window = windows
		.get_primary_mut()
		.expect("Cannot get priamry window");

	egui::TopBottomPanel::top("Window Options")
		.show(egui.ctx_mut(), |ui| {
			ui.horizontal(|ui| {
				ui.menu_button("Settings", |ui| {
					ui.checkbox(&mut options.toggle.fps, "Show FPS");
					ui.checkbox(&mut options.toggle.msaa, "MSAA samples");
					ui.checkbox(&mut options.toggle.frame_time, "Show Frame Time");
					ui.checkbox(&mut options.toggle.window_size, "Resize window");
					ui.checkbox(&mut options.toggle.window_mode, "Window modes");
					ui.checkbox(&mut options.toggle.window_title, "Change window title");
				});

				if options.toggle.msaa {
					ui.menu_button(
						"Set MSAA", |ui| {
							for msaa_option in MsaaSettings::iter() {
								if ui.button(format!("{:?}", msaa_option)).clicked() {
                                	msaa.samples = msaa_option as u32;
                            	}
							}
						}
					);
				}

				if options.toggle.window_size {
                    ui.menu_button(
						"Set size", |ui| {
                            for window_size in options.window_sizes.iter() {
                                if ui.button(format!("{}", window_size)).clicked() {
                                    window.set_resolution(window_size.width, window_size.height);
                                }
                            }
                        },
                    );
                }

				if options.toggle.window_mode {
					ui.menu_button(
						"Set mode", |ui| {
							if ui.button("Decorated").clicked() {
                            	window.set_decorations(!window.decorations());
                        	} else if ui.button("Windowed").clicked() {
                            	window.set_mode(WindowMode::Windowed);
                        	} else if ui.button("Fullscreen").clicked() {
                            	window.set_mode(WindowMode::Fullscreen);
                        	} else if ui.button("Borderless").clicked() {
                            	window.set_mode(WindowMode::BorderlessFullscreen);
                        	}
                    	}
					);
				}
			});
		});
}
