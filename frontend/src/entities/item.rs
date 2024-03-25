use backend::items::Item;
use bevy::prelude::*;
use bevy_xpbd_3d::components::RigidBody;

struct ItemTemplate {
    inner: Box<dyn Item>,
    model: Handle<Scene>,
    weight: f32,
}

#[derive(Component)]
struct ItemComponent {
    item: Box<dyn Item>,
}

#[derive(Bundle)]
struct ItemBundle {
    pub item: ItemComponent,
    pub rigid_body: RigidBody,
    pub model: Handle<Scene>,
}

pub fn create_bundle(
    item: Box<dyn Item>,
    model: &'static str,
    assets: &AssetServer,
) -> ItemBundle {
    ItemBundle {
        item: ItemComponent { item },
        rigid_body: RigidBody::default(),
        model: assets.load(model),
    }
}
