//! Module for custom exceptions and exception messages

/// TODO: Finish documentation

use specs::Entity;

pub fn get_add_damage_amount_error_message(target: &Entity, amount: i32) -> String {
    format!(
        "Damage amount {} couldn't be stored in the ecs for entity with id {}",
        amount,
        target.id()
    )
}

pub fn get_pick_up_item_error_message(collector: &Entity, item: &Entity) -> String {
    format!(
        "Unable to pick up item with id {} and add it to inventory of collector with id {}!",
        item.id(),
        collector.id()
    )
}

pub fn get_drop_item_error_message(owner: &Entity, item: &Entity) -> String {
    format!(
        "Unable to drop item with id {} from inventory of owner with id {}!",
        owner.id(),
        item.id()
    )
}
