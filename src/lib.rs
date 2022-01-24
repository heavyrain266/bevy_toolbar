mod bottom_panel;
mod top_panel;

use bevy::{app::*, diagnostic::*};

use bevy_egui::*;

use crate::bottom_panel::*;
use crate::top_panel::*;

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
