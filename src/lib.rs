mod panels;

use {
	bevy::app::{
		PluginGroup, PluginGroupBuilder
	},

	crate::panels::{
		top::TopPanel,
		bottom::BottomPanel,
	}
};

pub struct ToolbarPlugins;

impl PluginGroup for ToolbarPlugins {
	fn build(
		&mut self,
		group: &mut PluginGroupBuilder
	) {
		group.add(TopPanel);
		group.add(BottomPanel);
	}
}
