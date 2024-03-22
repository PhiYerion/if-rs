use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ThirdPersonCameraData {
    pub offset: Vec3,
    pub y_rotation: f32,
}

impl Default for ThirdPersonCameraData {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 2.0, -10.0),
            y_rotation: 0.0,
        }
    }
}

/// Orbits about the y axis according to [Player]'s x rotation and about the x
/// axis according to [ThirdPersonCameraData]'s y rotation. This function
/// updates [ThirdPersonCameraData]'s y rotation based on camera_y_rotation
/// input.
///
/// # Arguments
/// * `player_transform` - [Player]'s transform. Rotation & translation are
/// followed by the camera.
/// * `camera_transform` - The transform to update.
/// * `camera_data` - The camera data to be updated and used. Holds y rotation
/// state and offset to use.
/// * `camera_y_rotation` - How much more to rotate the camera about the x axis
pub fn third_person_camera_update(
    target_pos: &Vec3,
    target_rotation: &Quat,
    sensitivity: f32,
    camera_transform: &mut Transform,
    camera_data: &mut ThirdPersonCameraData,
    camera_y_rotation: f32,
) {
    camera_data.y_rotation =
        (camera_y_rotation * sensitivity / 1.5 + camera_data.y_rotation).clamp(-1.0, 1.0);

    let player_head_pos = target_pos;
    let player_rotation_mod = target_rotation;
    let camera_rotation_mod = Quat::from_axis_angle(Vec3::X, camera_data.y_rotation);

    let total_offset = *player_head_pos + *player_rotation_mod * camera_rotation_mod * camera_data.offset;

    camera_transform.translation = total_offset;
    camera_transform.look_at(*player_head_pos, Vec3::Y);
}
