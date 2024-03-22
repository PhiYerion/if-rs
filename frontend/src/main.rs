#![feature(stmt_expr_attributes)]
mod camera;
mod player;
mod scene;
mod entities;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_3d::plugins::PhysicsPlugins;

use self::player::PlayerPlugin;
use self::scene::ScenePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            ScenePlugin,
            WorldInspectorPlugin::new(),
            PhysicsPlugins::default(),
        ))
        .run();
}
