use bevy::prelude::*;
use bevy_xpbd_3d::components::RigidBody;
use bevy_xpbd_3d::plugins::collision::Collider;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_scene);
    }
}

pub fn create_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let platform = PbrBundle {
        mesh: meshes.add(Cuboid::new(10.0, 10.0, 1.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    };

    let collider = (RigidBody::Static, Collider::cuboid(10.0, 10.0, 1.0));

    let light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    };

    commands.spawn(( platform, collider ));
    commands.spawn(light);
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 0.01, 1.0)),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..Default::default()
    });
}
