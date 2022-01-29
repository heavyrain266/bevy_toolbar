mod panels;

use panels::{
    top::TopPanel,
    bottom::BottomPanel,
};

use bevy::{
    app::*,
    diagnostic::*
};

use bevy_egui::*;

pub struct ToolbarPlugins;

impl PluginGroup for ToolbarPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(TopPanel)
            .add(BottomPanel)
            .add(EguiPlugin)
            .add(FrameTimeDiagnosticsPlugin);
    }
}
