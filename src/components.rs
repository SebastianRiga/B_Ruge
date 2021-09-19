//! List of all components used in the game.

use rltk::{FontCharType, Point, RGB};
use specs::prelude::*;
use specs_derive::*;

use super::{exceptions, GameLog};

/// Component to describe the position
/// of a game entity in the game.
#[derive(Component, Copy, Clone, PartialEq)]
pub struct Position {
    /// X coordinate of the entity.
    pub x: i32,

    /// Y coordinate of the entity.
    pub y: i32,
}

impl Position {
    /// Creates a new [Position] with the `x` and `y` coordinates
    /// of the `tuple`. Tuple format: (`x`, `y`).
    pub fn new_from_tuple(tuple: (i32, i32)) -> Self {
        Position {
            x: tuple.0,
            y: tuple.1,
        }
    }

    /// Crates a new [Point] from the calling
    /// coordinate
    pub fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }

    /// Updates the the fields of the position with the
    /// passed `x` and `y` coordinates.
    pub fn update(&mut self, x: i32, y: i32) -> &Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Updates the fields of the [Position] with the data from
    /// the tuple, where the tuple has the format `(x, y)`.
    pub fn update_with_tuple(&mut self, tuple: (i32, i32)) -> &Self {
        self.update(tuple.0, tuple.1)
    }

    /// Returns `true` if the `x` and `y` coordinates of the
    /// the calling [Position] and the passed [Position]
    /// are equal. `False` otherwise.
    ///
    /// # Arguments
    /// * `other`: The [Position], the calling [Position]
    /// should be compared to.
    ///
    pub fn is_equal(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }

    /// Returns `true` if the `x` and `y` coordinates of the
    /// calling [Position] and the passed [Point] are equal.
    /// `False` otherwise.
    ///
    /// # Arguments
    /// * `point`: The [Point] with which the [Position] should be compared.
    ///
    pub fn is_equal_to_point(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }

    /// Returns `true` if the `x` and `y` coordinates of the
    /// calling [Position] and passed `(i32, i32) tuple` are
    /// equal. `False` otherwise.
    ///
    /// # Arguments
    /// * `tuple`: The `(i32, i32) tuple` the [Position] should
    /// be compared to.
    ///
    /// # Notes
    /// * The `(i32, i32) tuple` are presumed to be of the format
    /// `(x, y)`.
    ///
    pub fn is_equal_to_tuple(&self, tuple: &(i32, i32)) -> bool {
        self.x == tuple.0 && self.y == tuple.1
    }
}

/// Component to describe the render
/// information of an entity.
#[derive(Component)]
pub struct Renderable {
    /// Font symbol of the entity.
    pub symbol: FontCharType,

    /// Foreground color of the entity.
    pub fg: RGB,

    /// Background color of the entity.
    pub bg: RGB,

    /// Place in the rendering order
    pub order: i32,
}

/// Component for the player entity.
#[derive(Component, Debug)]
pub struct Player {}

/// Component for the field of view implementation.
#[derive(Component)]
pub struct FOV {
    /// Positions in the FOV.
    pub content: Vec<rltk::Point>,

    /// Range of the FOV.
    pub range: i32,

    /// Flag indicating if the
    /// [FOV] should be updated.
    pub is_dirty: bool,
}

impl FOV {
    /// Set the [FOV] to dirty, which triggers
    /// system execution in the next processing
    /// cycle.
    pub fn mark_as_dirty(&mut self) -> &Self {
        self.is_dirty = true;
        self
    }

    /// Mark the [FOV] as clean, meaning no further
    /// view processing for the associated entity is
    /// needed.
    pub fn mark_as_clean(&mut self) -> &Self {
        self.is_dirty = false;
        self
    }

    /// Returns `true` if the passed [Point] is contained
    /// in the [FOV]. Otherwise returns `false`.
    pub fn contains(&self, point: &rltk::Point) -> bool {
        self.content.contains(point)
    }
}

/// Component for the monsters.
#[derive(Component, Debug)]
pub struct Monster {}

/// Component to name entities
#[derive(Component, Debug)]
pub struct Name {
    /// The name of the entity
    pub name: String,
}

/// Component that designates a an associated
/// entity as blocking, meaning it can't be walked
/// over.
#[derive(Component, Debug)]
pub struct Collision {}

/// Component describing the
/// combat stats of an entity.
#[derive(Component, Debug)]
pub struct Statistics {
    /// Maximum hp of the entity.
    pub hp_max: i32,

    /// Current hp of the entity.
    pub hp: i32,

    /// Attack power of the entity.
    pub power: i32,

    /// Defense capabilities of the entity.
    pub defense: i32,
}

/// Component designating an entity which
/// attacks in melee range.
#[derive(Component, Debug, Clone)]
pub struct MeleeAttack {
    /// The target entity of the attack.
    pub target: Entity,
}

/// Component keeping track of
/// the damage an entity receives
/// in a turn.
#[derive(Component, Debug)]
pub struct DamageCounter {
    /// The amount of damage the entity has taken
    /// this turn as a vector.
    pub damage_values: Vec<i32>,
}

impl DamageCounter {
    /// Adds the passed damage `amount` to the damage values of the `target` and writes them
    /// into the associated `ecs` `store`.
    ///
    /// # Arguments
    /// * `store`: The store in which the [DamageCounter] component should be saved.
    /// * `target`: The [Entity] taking the damage.
    /// * `amount`: The number of damage the [Entity] has taken.
    ///
    pub fn add_damage_taken(store: &mut WriteStorage<DamageCounter>, target: Entity, amount: i32) {
        if let Some(damage_counter) = store.get_mut(target) {
            damage_counter.damage_values.push(amount)
        } else {
            let damage_counter = DamageCounter {
                damage_values: vec![amount],
            };

            let on_error_message = exceptions::get_add_damage_amount_error_message(&target, amount);

            store
                .insert(target, damage_counter)
                .expect(&on_error_message);
        }
    }
}

/// Component marking an entity as an item
/// e.g. potions, equipment, scrolls, etc.
#[derive(Component, Debug)]
pub struct Item {}

impl Item {
    /// Picks up the first [Item] [Entity] at the [Position] of the `collector` [Entity]
    /// and adds it to the [Loot] of the `collector` and sends a corresponding message to the
    /// [GameLog]. If no [Item] is present at the current [Position] of the [Entity] a message
    /// indicating that nothing was found is send to the [GameLog].
    ///
    /// # Arguments
    /// * `ecs`: Ecs reference to read the corresponding [Entity] values.
    /// * `collector`: The [Entity] reference, that wants to pick up an item.
    ///
    pub fn pick_up(ecs: &World, collector: &Entity) {
        let entities = ecs.entities();
        let names = ecs.read_storage::<Name>();
        let items = ecs.read_storage::<Item>();
        let positions = ecs.read_storage::<Position>();

        let mut game_log = ecs.fetch_mut::<GameLog>();

        let collector_name = names.get(*collector);
        let collector_position = positions.get(*collector);

        let mut picked_item: Option<Entity> = None;

        if let Some(collector_position) = collector_position {
            for (item_entity, _, position) in (&entities, &items, &positions).join() {
                if collector_position.is_equal(position) {
                    picked_item = Some(item_entity);
                    break;
                }
            }
        }

        let out_name: String = match collector_name {
            None => "Some one".to_string(),
            Some(name_plate) => (*name_plate.name).to_string(),
        };

        match picked_item {
            None => {
                let message = format!(
                    "{} tried to pick up an item, but there is nothing on the ground.",
                    out_name
                );
                game_log.messages_push(&message);
            }
            Some(picked_item) => {
                let mut pickups = ecs.write_storage::<PickupItem>();
                let pickup = PickupItem {
                    collector: *collector,
                    item: picked_item,
                };

                let on_error_message =
                    exceptions::get_pick_up_item_error_message(collector, &picked_item);

                pickups.insert(*collector, pickup).expect(&on_error_message);
            }
        };
    }

    /// Drops an [Item] [Entity] from the inventory of the `owner`
    /// [Entity].
    ///
    /// # Arguments
    /// * `ecs`: The [World] in which both the `owner` and `item` are stored.
    /// * `owner`: The `owner` [Entity] of the `item`.
    /// * `item`: The [Item] that the `owner` wants to drop.
    ///
    pub fn drop_item(ecs: &World, owner: &Entity, item: &Entity) {
        let mut drop_intent = ecs.write_storage::<DropItem>();

        let drop_item = DropItem { item: *item };

        let on_error_message = exceptions::get_drop_item_error_message(owner, item);
        drop_intent
            .insert(*owner, drop_item)
            .expect(&on_error_message);
    }
}

/// Component describing a drinkable potion
/// that heals the players hp.
#[derive(Component, Debug)]
pub struct Potion {
    /// The amount of health, the [Potion]
    /// restores for the [Entity] that drinks it.
    pub healing_amount: i32,
}

impl Potion {
    /// Adds a request to the passed `ecs`, that the `user` [Entity] wants to
    /// drink the supplied `potion` [Entity].
    ///
    /// # Arguments
    /// * `ecs`: The overarching `ecs` to write to.
    /// * `user`: The [Entity] that wants to drink the `potion`.
    /// * `potion`: The `potion` [Entity] the `user` wants to drink.
    ///
    pub fn drink(ecs: &World, user: &Entity, potion: &Entity) {
        let mut usage_intent = ecs.write_storage::<UsePotion>();

        let usage = UsePotion { potion: *potion };

        usage_intent.insert(*user, usage).expect("");
    }
}

/// Component marking an [Entity] as collected,
/// meaning it is in the inventory of a owning [Entity].
#[derive(Component, Debug)]
pub struct Loot {
    /// The owner of the collected loot.
    pub owner: Entity,
}

/// Component used for communication with the ItemCollectionSystem
/// to indicate, that an [Entity] wants to pickup an [Item].
#[derive(Component, Debug)]
pub struct PickupItem {
    /// The [Entity] wanting to pick up the item.
    pub collector: Entity,

    /// The [Item] the `collector` wants to pick up.
    pub item: Entity,
}

/// Component used for communication with the
/// ItemDropSystem to indicate, that an [Entity]
/// wants to drop a collected [Item].
#[derive(Component, Debug)]
pub struct DropItem {
    /// Reference to the [Item] entity to drop.
    pub item: Entity,
}

/// Component used for communication with the
/// PotionDrinkSystem to indicate, that an
/// [Entity] wants to drink a [Potion].
#[derive(Component, Debug)]
pub struct UsePotion {
    /// The [Potion] the [Entity] wants to consume.
    pub potion: Entity,
}

/// Shorthand function to register all needed
/// [Component]s of the game with the passed `ecs`.
///
/// # Arguments
/// * `ecs`: Reference to the ECS in which the [Component]s should be registered.
///
pub fn register_components(ecs: &mut World) {
    ecs.register::<FOV>();
    ecs.register::<Name>();
    ecs.register::<Item>();
    ecs.register::<Loot>();
    ecs.register::<Player>();
    ecs.register::<Potion>();
    ecs.register::<Monster>();
    ecs.register::<Position>();
    ecs.register::<DropItem>();
    ecs.register::<Collision>();
    ecs.register::<UsePotion>();
    ecs.register::<Renderable>();
    ecs.register::<Statistics>();
    ecs.register::<PickupItem>();
    ecs.register::<MeleeAttack>();
    ecs.register::<DamageCounter>();
}
