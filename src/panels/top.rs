use super::settings::ToolbarSettings;

use bevy::{
    prelude::*,
    window::WindowMode
};

use bevy_egui::*;

use strum_macros::EnumIter;
use strum::IntoEnumIterator;

// WebGPU currently only exposes 1(Off) and 4x Samples
// Updates can be tracked here: https://github.com/gfx-rs/wgpu/issues/1832
#[derive(Debug, EnumIter)]
enum MsaaSettings {
    Off = 1,
    X4 = 4,
}

pub struct TopPanel;

impl Plugin for TopPanel {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ToolbarSettings>()
            .add_startup_system(get_startup_resolution)
            .add_system(top_panel);
    }
}

fn get_startup_resolution(
    mut windows: ResMut<Windows>,
    mut settings: ResMut<ToolbarSettings>
) {
    let window = windows
        .get_primary_mut()
        .unwrap();

    settings.current_window_size.width = window.requested_width();
    settings.current_window_size.height = window.requested_height();
    settings.title = window.title().into();
}

pub fn top_panel(
    egui: Res<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut windows: ResMut<Windows>,
    mut settings: ResMut<ToolbarSettings>,
) {
    let window = windows
        .get_primary_mut()
        .unwrap();

    egui::TopBottomPanel::top("Window")
        .show(egui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Settings", |ui| {
                    ui.checkbox(&mut settings.option.window_title, "Window title");
                    ui.checkbox(&mut settings.option.window_size, "Window size");
                    ui.checkbox(&mut settings.option.window_mode, "Window mode");
                    ui.checkbox(&mut settings.option.vsync, "V-Sync");
                    ui.checkbox(&mut settings.option.msaa, "MSAA");
                    ui.checkbox(&mut settings.option.ft, "Frame Time");
                    ui.checkbox(&mut settings.option.fps, "Frames Per Second");
                });

                if settings.option.window_size {
                    ui.menu_button(
                        format!(
                            "Size: {width}x{height}px",
                            width = window.width(),
                            height = window.height(),
                        ),
                        |ui| {
                            for window_size in settings.window_sizes.iter() {
                                if ui.button(format!("{}", window_size)).clicked() {
                                    window.set_resolution(window_size.width, window_size.height);
                                }
                            }
                        },
                    );
                }

                if settings.option.window_mode {
                    ui.menu_button(format!("Mode: {:?}", window.mode()), |ui| {
                        if ui.button("Toggle Decorations").clicked() {
                            window.set_decorations(!window.decorations());
                        } else if ui.button("Windowed").clicked() {
                            window.set_mode(WindowMode::Windowed);
                        } else if ui.button("Fullscreen").clicked() {
                            window.set_mode(WindowMode::Fullscreen);
                        } else if ui.button("Borderless").clicked() {
                            window.set_mode(WindowMode::BorderlessFullscreen);
                        }
                    });
                }

                if settings.option.vsync {
                    ui.menu_button(
                        format!("VSync: {}", if window.vsync() { "On" } else { "Off" }),
                        |ui| {
                            if ui.button("Off").clicked() {
                                window.set_vsync(false);
                            } else if ui.button("On").clicked() {
                                window.set_vsync(true);
                            }
                        },
                    );
                }

                if settings.option.msaa {
                    ui.menu_button(
                        format!("MSAA: {}",
                            if msaa.samples.eq(&1) {
                                "Off".to_string()
                            } else {
                                msaa.samples.to_string() + "x"
                            }
                        ),
                    |ui| {
                        for msaa_option in MsaaSettings::iter() {
                            if ui.button(format!("{:?}", msaa_option)).clicked() {
                                msaa.samples = msaa_option as u32;
                            }
                        }
                    },
                );
            }
        });
    });
}
