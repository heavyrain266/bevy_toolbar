pub struct Options {
	pub title: String,
	pub toggle: Toggle,
	pub window_sizes: Vec<WindowSize>,
	pub current_size: WindowSize,
}

pub struct Toggle {
	pub fps: bool,
	pub msaa: bool,
	pub frame_time: bool,
	pub window_size: bool,
	pub window_mode: bool,
	pub window_title: bool,
}

pub struct WindowSize {
	pub width: f32,
	pub height: f32,
}

impl std::fmt::Display for WindowSize {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter<'_>
	) -> std::fmt::Result {
		writeln!(fmt, "{}x{}", self.width, self.height)
	}
}

impl Default for Options {
	fn default() -> Self {
		Self {
			title: String::from("bevy"),
			toggle: Toggle {
				fps: false,
				msaa: false,
				frame_time: false,
				window_size: false,
				window_mode: false,
				window_title: false,
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
			current_size: WindowSize {
				width: 800.0,
				height: 600.0,
			},
		}
	}
}
