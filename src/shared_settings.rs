pub struct OptionToggle {
    pub msaa: bool,
    pub resolution: bool,
    pub window_mode: bool,
    pub vsync: bool,
    pub title: bool,
    pub fps: bool,
    pub ft: bool,
}

pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

pub struct ToolbarSettings {
    pub current_window_size: WindowSize,
    pub window_sizes: Vec<WindowSize>,
    pub option: OptionToggle,
    pub title: String,
}

impl std::fmt::Display for WindowSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl Default for ToolbarSettings {
    fn default() -> Self {
        Self {
            title: "".to_string(),
            current_window_size: WindowSize {
                width: 800.0,
                height: 800.0,
            },
            // Make options disabled by default
            option: OptionToggle {
                msaa: false,
                resolution: false,
                window_mode: false,
                vsync: false,
                title: false,
                fps: false,
                ft: false,
            },
            window_sizes: vec![
                WindowSize {
                    width: 640.0,
                    height: 480.0,
                },
                WindowSize {
                    width: 800.0,
                    height: 800.0,
                },
                WindowSize {
                    width: 1024.0,
                    height: 768.0,
                },
                WindowSize {
                    width: 1280.0,
                    height: 720.0,
                },
                WindowSize {
                    width: 1920.0,
                    height: 1080.0,
                },
                WindowSize {
                    width: 2560.0,
                    height: 1440.0,
                },
                WindowSize {
                    width: 3840.0,
                    height: 2160.0,
                },
            ],
        }
    }
}
