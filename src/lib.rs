mod top_panel;
mod bottom_panel;

use bevy::app::*;
use bevy_egui::*;

use crate::top_panel::*;
use crate::bottom_panel::*;

pub struct ToolbarPlugin;

impl PluginGroup for ToolbarPlugin {
	fn build(
		&mut self,
		group: &mut PluginGroupBuilder
	) {
		group
            .add(Panels)
            .add(BottomPanel)
            .add(EguiPlugin);
	}
}
