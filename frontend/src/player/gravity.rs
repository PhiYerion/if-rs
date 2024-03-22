use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Component)]
pub struct FloorDetectorMarker;

#[derive(Bundle)]
pub struct FloorDetector {
    pub marker: FloorDetectorMarker,
    pub ray: RayCaster,
}

impl Default for FloorDetector {
    fn default() -> Self {
        Self {
            marker: FloorDetectorMarker,
            ray: RayCaster::new(Vec3::ZERO, Direction3d::NEG_Y),
        }
    }
}
