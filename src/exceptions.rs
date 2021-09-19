//! Module for custom exceptions and error messages.

use specs::Entity;

/// Returns the `error message` for the `DamageSystem`, when the storing of
/// any damage `amount` for the `target` [Entity].
///
/// # Arguments
/// * `target`: The [Entity] for which the damage `amount` couldn't be added.
/// * `amount`: The `amount` of damage, that should have been added to the `target` [Entity].
///
pub fn get_add_damage_amount_error_message(target: &Entity, amount: i32) -> String {
    format!(
        "Damage amount {} couldn't be stored in the ecs for entity with id {}",
        amount,
        target.id()
    )
}

/// Returns the `error message`for the `ItemCollectionSystem`, when the picking up of
/// an item [Entity] has failed for the passed `collector` [Entity].
///
/// # Arguments
/// * `collector`: The [Entity] that wants to pick up the `item`.
/// * `item`: The item [Entity], which the `collector` is trying to pickup.
///
pub fn get_pick_up_item_error_message(collector: &Entity, item: &Entity) -> String {
    format!(
        "Unable to pick up item with id {} and add it to inventory of collector with id {}!",
        item.id(),
        collector.id()
    )
}

/// Returns the error message for the `DropItemSystem`, when the droping of an
/// item [Entity] fails for the passed `owner` entity.
///
/// # Arguments
/// * `owner`: The current owning [Entity], which is trying to drop the `item` [Entity].
/// * `item`: The item [Entity] the `owner` is trying to drop.
///
pub fn get_drop_item_error_message(owner: &Entity, item: &Entity) -> String {
    format!(
        "Unable to drop item with id {} from inventory of owner with id {}!",
        owner.id(),
        item.id()
    )
}

/// Returns the error message for the `UsePotion` systen, when the insertion
/// of a use potion request failes.
/// 
/// # Arguments
/// * `user`: The [Entity] that wants to drink the `potion`.
/// * `potion`: The `potion` [Entity] the `user` wants to drink.
/// 
pub fn get_drink_potion_error_message(user: &Entity, potion: &Entity) -> String {
    format!(
        "Unable to insert use potion request for user with id {} and potion with id {}",
        user.id(),
        potion.id()
    )
}

/// Returns the error message for `MeleeCombatSystem` system, used when the
/// adding of a melee attack from a monster against the player fails.
/// 
/// #A Arguments
/// * `monster`: The [Entity] attacking the player.
/// 
pub fn get_add_melee_damage_error_message(monster: &Entity) -> String {
    format!(
        "Adding melee attack from monster with id {} against player failed!",
        monster.id()
    )
}
