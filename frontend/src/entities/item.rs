use backend::items::ItemTrait;
use bevy::prelude::*;
use bevy_xpbd_3d::components::RigidBody;

struct ItemTemplate {
    inner: Box<dyn ItemTrait>,
    model: Handle<Scene>,
    weight: f32,
}

#[derive(Component)]
struct ItemComponent {
    item: Box<dyn ItemTrait>,
}

#[derive(Bundle)]
struct ItemBundle {
    pub item: ItemComponent,
    pub rigid_body: RigidBody,
    pub model: Handle<Scene>,
}

pub fn create_bundle(
    item: Box<dyn ItemTrait>,
    model: &'static str,
    assets: &AssetServer,
) -> ItemBundle {
    ItemBundle {
        item: ItemComponent { item },
        rigid_body: RigidBody::default(),
        model: assets.load(model),
    }
}
