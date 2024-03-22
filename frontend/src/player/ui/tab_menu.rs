use backend::items::Items;
use bevy::prelude::*;

use crate::player::Player;

#[derive(Component)]
pub struct InventoryUIItem {
    id: usize,
}

#[derive(Component)]
pub struct InventoryUIMarker;

pub fn inventory_popup(mut commands: Commands, player: Query<&Player>) {
    let player = player.single();
    let inventory = &player.inventory;

    let tab_ui = NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            position_type: PositionType::Relative,
            width: Val::Vw(50.0),
            height: Val::Vh(50.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            flex_wrap: FlexWrap::Wrap,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..Default::default()
        },
        visibility: Visibility::Hidden,
        background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
        ..Default::default()
    };

    let item_box = move |parent: &mut ChildBuilder, item: &Items| {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        position_type: PositionType::Relative,
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        margin: UiRect {
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                        },
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.1, 0.1, 0.1, 0.5).into(),
                    ..Default::default()
                },
                InventoryUIItem { id: item.id() },
            ))
            .with_children(|subparent| {
                subparent.spawn(TextBundle::from_section(
                    item.type_name(),
                    Default::default(),
                ));
                subparent.spawn(TextBundle::from_section(
                    item.quantity().to_string(),
                    Default::default(),
                ));
            });
    };

    commands
        .spawn((tab_ui, InventoryUIMarker))
        .with_children(|parent: &mut ChildBuilder| {
            for item in &inventory.items {
                item_box(parent, item);
            }
        });
}

pub fn handle_inventory_input(
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mut interaction: Query<
        (&Interaction, &InventoryUIItem, &mut Visibility),
        (Changed<Interaction>, With<Button>),
    >,
) {
    let mut player = player.single_mut();
    for (interaction, item, mut vis) in interaction.iter_mut() {
        if *interaction == Interaction::Pressed {
            player.inventory.remove_by_id(item.id);
            *vis = Visibility::Hidden;
        }
    }
}
