use backend::iams::Inventory;
use bevy::ecs::system::Commands;

pub fn drop_item(
    mut commands: Commands,
    inventory: &mut Inventory,
    item_id: usize,
) {
    let to_drop = inventory.remove_by_id(item_id);
    


    Ok(to_drop)
}
