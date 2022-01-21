mod top_panel;
mod bottom_panel;

use bevy::{
    app::*,
    diagnostic::*
};

use bevy_egui::*;

use crate::top_panel::*;
use crate::bottom_panel::*;

pub struct ToolbarPlugins;

impl PluginGroup for ToolbarPlugins {
	fn build(
		&mut self,
		group: &mut PluginGroupBuilder
	) {
		group
            .add(TopPanel)
            .add(BottomPanel)
            .add(EguiPlugin)
            .add(FrameTimeDiagnosticsPlugin);
	}
}
