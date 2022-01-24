use bevy::{prelude::*, window::WindowMode};

use bevy_egui::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub(crate) struct TopPanel;

impl Plugin for TopPanel {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolbarSettings>();
        app.add_startup_system(get_startup_resolution);
        app.add_system(top_panel);
    }
}

pub struct ToolbarSettings {
    current_window_size: WindowSize,
    window_sizes: Vec<WindowSize>,
    pub setting_toggles: SettingToggles,
}

pub struct SettingToggles {
    msaa: bool,
    resolution: bool,
    window_mode: bool,
    vsync: bool,
    pub fps: bool,
    pub ft: bool,
}

pub struct WindowSize {
    width: f32,
    height: f32,
}

impl std::fmt::Display for WindowSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl Default for ToolbarSettings {
    fn default() -> Self {
        Self {
            setting_toggles: SettingToggles {
                msaa: true,
                resolution: true,
                window_mode: true,
                vsync: true,
                fps: true,
                ft: true,
            },
            current_window_size: WindowSize {
                width: 800.,
                height: 800.,
            },
            window_sizes: vec![
                WindowSize {
                    width: 640.,
                    height: 480.,
                },
                WindowSize {
                    width: 800.,
                    height: 800.,
                },
                WindowSize {
                    width: 1024.,
                    height: 768.,
                },
                WindowSize {
                    width: 1280.,
                    height: 720.,
                },
                WindowSize {
                    width: 1920.,
                    height: 1080.,
                },
                WindowSize {
                    width: 2560.,
                    height: 1440.,
                },
                WindowSize {
                    width: 3840.,
                    height: 2160.,
                },
            ],
        }
    }
}

//WebGPU currently only exposes 1(Off) and 4x Samples:
//Updates can be tracked here: https://github.com/gfx-rs/wgpu/issues/1832
#[derive(Debug, EnumIter)]
enum MsaaSetting {
    Off = 1,
    #[cfg(any())]
    X2 = 2,
    X4 = 4,
    #[cfg(any())]
    X8 = 8,
    #[cfg(any())]
    X16 = 16,
}

fn get_startup_resolution(mut windows: ResMut<Windows>, mut settings: ResMut<ToolbarSettings>) {
    let prime = windows.get_primary_mut().unwrap();
    settings.current_window_size.width = prime.requested_width();
    settings.current_window_size.height = prime.requested_height();
}

pub(self) fn top_panel(
    egui: Res<EguiContext>,
    mut msaa: ResMut<Msaa>,
    mut windows: ResMut<Windows>,
    mut settings: ResMut<ToolbarSettings>,
) {
    let prime = windows.get_primary_mut().unwrap();

    egui::TopBottomPanel::top("Window").show(egui.ctx(), |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("Settings", |ui| {
                let _res = ui.checkbox(&mut settings.setting_toggles.window_mode, "Window Mode");
                let _res = ui.checkbox(&mut settings.setting_toggles.resolution, "Resolution");
                let _res = ui.checkbox(&mut settings.setting_toggles.vsync, "VSync");
                let _res = ui.checkbox(&mut settings.setting_toggles.msaa, "Msaa");
                let _res = ui.checkbox(&mut settings.setting_toggles.fps, "Fps");
                let _res = ui.checkbox(&mut settings.setting_toggles.ft, "Frame Time");
            });

            if settings.setting_toggles.window_mode {
                ui.menu_button(format!("Mode: {:?}", prime.mode()), |ui| {
                    if ui.button("Toggle Decorations").clicked() {
                        prime.set_decorations(!prime.decorations());
                    } else if ui.button("Windowed").clicked() {
                        prime.set_mode(WindowMode::Windowed);
                    } else if ui.button("Fullscreen").clicked() {
                        prime.set_mode(WindowMode::Fullscreen);
                    } else if ui.button("Borderless").clicked() {
                        prime.set_mode(WindowMode::BorderlessFullscreen);
                    }
                });
            }

            if settings.setting_toggles.resolution {
                ui.menu_button(
                    format!(
                        "Size: {width}x{height}px",
                        width = prime.width(),
                        height = prime.height(),
                    ),
                    |ui| {
                        for window_size in settings.window_sizes.iter() {
                            if ui.button(format!("{}", window_size)).clicked() {
                                prime.set_resolution(window_size.width, window_size.height);
                            }
                        }
                    },
                );
            }

            if settings.setting_toggles.vsync {
                ui.menu_button(
                    format!("Vsync: {}", if prime.vsync() { "On" } else { "Off" }),
                    |ui| {
                        if ui.button("Off").clicked() {
                            prime.set_vsync(false);
                        } else if ui.button("On").clicked() {
                            prime.set_vsync(true);
                        }
                    },
                );
            }

            if settings.setting_toggles.msaa {
                ui.menu_button(
                    format!(
                        "Msaa: {}",
                        if msaa.samples.eq(&1) {
                            "Off".to_string()
                        } else {
                            msaa.samples.to_string() + "x"
                        }
                    ),
                    |ui| {
                        for mssa_setting in MsaaSetting::iter() {
                            if ui.button(format!("{:?}", mssa_setting)).clicked() {
                                msaa.samples = mssa_setting as u32;
                            }
                        }
                    },
                );
            }
        });
    });
}
