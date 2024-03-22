mod movement;
mod ui;
mod gravity;
mod actions;

use backend::items::OreItem;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_xpbd_3d::components::RigidBody;
use bevy_xpbd_3d::plugins::collision::Collider;

use crate::camera::ThirdPersonCameraData;
use crate::player::gravity::FloorDetector;

use self::movement::player_movement;
use self::ui::tab_menu::{handle_inventory_input, inventory_popup, InventoryUIMarker};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, inventory_popup.after(spawn_player)))
            .add_systems(
                Update,
                (
                    player_movement,
                    action_input_handler,
                    handle_inventory_input,
                ),
            );
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
    pub inventory: backend::iams::Inventory,
    pub camera_data: ThirdPersonCameraData,
    pub mouse_sensitivity: f32,
    pub movement_enabled: bool,
}

impl Default for Player {
    fn default() -> Self {
        let mut inventory = backend::iams::Inventory::default();
        for i in 0..10 {
            inventory.add(OreItem::new_iron_ore(i as f32 * 0.5));
            inventory.add(OreItem::new_copper_ore(i as f32));
        }

        Self {
            speed: 250.0,
            camera_data: ThirdPersonCameraData::default(),
            mouse_sensitivity: 0.001,
            inventory,
            movement_enabled: true,
        }
    }
}

pub fn action_input_handler(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut query: Query<&mut Player>,
    mut tab_menu: Query<&mut Visibility, With<InventoryUIMarker>>,
) {
    let mut player = query.single_mut();
    let mut primary_window = window.single_mut();

    let free_cursor = |primary_window: &mut Window, player: &mut Player| {
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
        primary_window.cursor.visible = false;
        player.movement_enabled = true;
    };

    let capture_cursor = |primary_window: &mut Window, player: &mut Player| {
        primary_window.cursor.grab_mode = CursorGrabMode::Confined;
        primary_window.cursor.visible = true;
        player.movement_enabled = false;
    };

    if keys.just_pressed(KeyCode::Escape) {
        match primary_window.cursor.grab_mode {
            CursorGrabMode::Confined | CursorGrabMode::None => {
                capture_cursor(&mut primary_window, &mut player)
            }
            CursorGrabMode::Locked => free_cursor(&mut primary_window, &mut player),
        }
    }

    if keys.just_pressed(KeyCode::Tab) {
        let mut tab_menu = tab_menu.single_mut();
        match *tab_menu {
            Visibility::Visible => {
                free_cursor(&mut primary_window, &mut player);
                *tab_menu = Visibility::Hidden;
            }
            Visibility::Hidden => {
                capture_cursor(&mut primary_window, &mut player);
                *tab_menu = Visibility::Visible;
            }
            Visibility::Inherited => {
                free_cursor(&mut primary_window, &mut player);
                *tab_menu = Visibility::Hidden;
            }
        }
    }
}

/// Spawns a gman player model.
pub fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = window.single_mut();
    primary_window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
    primary_window.cursor.visible = false;

    let model = SceneBundle {
        scene: assets.load("gman.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(0.022),
            ..Default::default()
        },
        ..Default::default()
    };

    let camera = Camera3dBundle::default();

    commands.spawn((
        model,
        Player::default(),
        RigidBody::Kinematic,
        Collider::capsule(10.0, 1.0),
        FloorDetector::default(),
    ));
    commands.spawn(camera);


    log::debug!("Player spawned");
}
