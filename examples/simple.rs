use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example_system)
        .run();
}

fn ui_example_system(egui_ctx: Query<&EguiContext>) {
    egui::Window::new("Hello").show(egui_ctx.iter().next().unwrap(), |ui| {
        ui.label("world");
    });
}
