//! Module containing all UI functionality of the game

use rltk::{Point, Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{
    config, swatch, timestamp_formatted, GameLog, Map, Name, Player, Position, Statistics,
};
use crate::{pythagoras_distance, TargetSelectionResult, TargetSelectionState, FOV};

/// Draws the ui of the game in the given `ctx`.
///
/// # Arguments
/// * `ecs`: The `ecs` is needed to read values from the game for display.
/// * `ctx`: The console context in which the ui should be drawn.
///
/// # See also
/// * [draw_message_log]
/// * [draw_messages]
/// * [draw_player_health]
/// * [draw_mouse_cursor]
///
pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    draw_message_log(ctx);
    draw_messages(ecs, ctx);
    draw_player_health(ecs, ctx);
    draw_mouse_cursor(ctx);
}

/// Draws the games message log at the bottom of the
/// Screen.
///
/// # Arguments
/// * `ctx`: The [Rltk] context in which the message log
/// should be drawn.
///
fn draw_message_log(ctx: &mut Rltk) {
    let (x, y) = (0, config::MAP_HEIGHT);
    let (width, height) = (
        config::WINDOW_WIDTH - 1,
        config::WINDOW_WIDTH - config::MAP_HEIGHT - 1,
    );
    let (fg, bg) = swatch::MESSAGE_BOX.colors();

    ctx.draw_box(x, y, width, height, fg, bg);
}

/// Writes the messages which are stored in the [GameLog]
/// struct of the `ecs` inside the message log ui.
///
/// # Arguments
/// * `ecs`: THe [World] in which the [GameLog] is stored.
/// * `ctx`: The [Rltk] context in which the messages should
/// be written.
///
fn draw_messages(ecs: &World, ctx: &mut Rltk) {
    let mut game_log = ecs.fetch_mut::<GameLog>();

    let x = 2;
    let mut y = config::MAP_HEIGHT + 1;

    game_log.messages_for_each_rev(|message| {
        if y < config::WINDOW_HEIGHT - 2 {
            let timestamp = timestamp_formatted();
            ctx.print(x, y, &format!("{} > {}", timestamp, message));
            y += 1;
        }
    })
}

/// Draws the players healh information in form of status
/// text and a health bar on top of the message log ui.
///
/// # Arguments
/// * `ecs`: The [World] in which the player is stored.
/// * `ctx`: The [Rltk] context in which the ui should be drawn.
///
fn draw_player_health(ecs: &World, ctx: &mut Rltk) {
    let players = ecs.read_storage::<Player>();
    let statistics = ecs.read_storage::<Statistics>();

    for (_, statistic) in (&players, &statistics).join() {
        // Draw the players health bar

        let (fg, bg) = swatch::PLAYER_HEALTH_BAR.colors();
        let player_health_bar_width = (config::MAP_WIDTH / 2) - 1;

        ctx.draw_bar_horizontal(
            1,
            config::MAP_HEIGHT,
            player_health_bar_width,
            statistic.hp,
            statistic.hp_max,
            fg,
            bg,
        );

        // Draw the players health text

        let (fg, bg) = swatch::PLAYER_HEALTH_TEXT.colors();
        let health = format!("HP: {}/{}", statistic.hp, statistic.hp_max);
        let health_text_x = (player_health_bar_width / 2) - (health.len() / 2) as i32;

        ctx.print_color(health_text_x, config::MAP_HEIGHT, fg, bg, &health);

        // Draw player mana bar

        let (fg, bg) = swatch::PLAYER_MANA_BAR.colors();
        let player_mana_bar_x = (config::MAP_WIDTH / 2) + 1;
        let player_mana_bar_width = (config::MAP_WIDTH - 1) - player_mana_bar_x;

        ctx.draw_bar_horizontal(
            player_mana_bar_x,
            config::MAP_HEIGHT,
            player_mana_bar_width,
            10,
            10,
            fg,
            bg,
        );

        // Draw player mana text

        let mana = format!("MP: {}/{}", 10, 10);
        let (fg, bg) = swatch::PLAYER_MANA_TEXT.colors();
        let mana_text_x =
            (player_mana_bar_x + (player_mana_bar_width / 2)) - (mana.len() / 2) as i32;

        ctx.print_color(mana_text_x, config::MAP_HEIGHT, fg, bg, &mana);
    }
}

/// Sets the background color of the
/// tile currently focused by the mouse cursor.
///
/// # Arguments
/// * `ctx`: The [Rltk] context in which the mouse cursor
/// should be highlighted.
///
fn draw_mouse_cursor(ctx: &mut Rltk) {
    let (x, y) = ctx.mouse_pos();
    ctx.set_bg(x, y, swatch::MOUSE_CURSOR);
}

/// Draws a tooltip displaying the name of all entities
/// on a tile, when the mouse is hovered over it.
///
/// # Arguments
/// * `ecs`: The [World] struct, required to read the entities names and positions.
/// * `ctx`: The [Rltk] context in which the tooltips should be drawn.
///
pub fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Position>();

    let (x, y) = ctx.mouse_pos();

    if !map.check_idx(x, y) {
        return;
    }

    let mut tooltips: Vec<String> = Vec::new();

    for (name, position) in (&names, &positions).join() {
        if position.is_equal_to_tuple(&(x, y)) && map.is_tile_in_fov(x, y) {
            tooltips.push(name.name.to_string());
        }
    }

    if tooltips.is_empty() {
        return;
    }

    let mut max_width = tooltips
        .iter()
        .max_by_key(|&element| (element.len(), element))
        .unwrap()
        .len() as i32;

    max_width += 3;

    let mut y_position = y;
    let (fg, bg) = swatch::TOOLTIP.colors();

    if x > (config::MAP_WIDTH / 2) {
        let start_x = x - max_width + 1;
        let arrow_position = Point::new(x - 2, y);

        for tooltip in tooltips.iter() {
            ctx.print_color(start_x, y_position, fg, bg, tooltip);
            y_position += 1;
        }

        ctx.print_color(
            arrow_position.x,
            arrow_position.y,
            fg,
            bg,
            &"->".to_string(),
        )
    } else {
        let start_x = x + 3;
        let arrow_position = Point::new(x + 1, y);

        for tooltip in tooltips.iter() {
            ctx.print_color(start_x, y_position, fg, bg, &tooltip);
            y_position += 1;
        }

        ctx.print_color(
            arrow_position.x,
            arrow_position.y,
            fg,
            bg,
            &"<-".to_string(),
        );
    }
}

pub fn draw_player_ranged_targeting(
    range: i32,
    ecs: &World,
    ctx: &mut Rltk,
) -> TargetSelectionResult {
    let fovs = ecs.read_storage::<FOV>();
    let player_entity = ecs.fetch::<Entity>();
    let player_position = ecs.fetch::<Point>();

    let mut cells_in_range = Vec::new();

    if let Some(player_fov) = fovs.get(*player_entity) {
        for point in player_fov.content.iter() {
            let distance_to_point = pythagoras_distance(&*player_position, point);

            if distance_to_point <= range as f32 {
                ctx.set_bg(point.x, point.y, swatch::TILE_SELECTION);
                cells_in_range.push(point);
            }
        }
    } else {
        return TargetSelectionResult(TargetSelectionState::Cancel, None);
    }

    let mouse_position = ctx.mouse_point();

    let target = cells_in_range
        .iter()
        .find(|point| point.x == mouse_position.x && point.y == mouse_position.y);

    if let Some(target) = target {
        ctx.set_bg(
            mouse_position.x,
            mouse_position.y,
            swatch::TILE_SELECTION_SUCCESS,
        );

        if ctx.left_click && ctx.shift {
            return TargetSelectionResult(TargetSelectionState::Selected, Some(*target.clone()));
        }
    } else {
        ctx.set_bg(
            mouse_position.x,
            mouse_position.y,
            swatch::TILE_SELECTION_FAILED,
        );

        if ctx.left_click && ctx.shift {
            return TargetSelectionResult(TargetSelectionState::Cancel, None);
        }
    }

    if let Some(key) = ctx.key {
        if key == VirtualKeyCode::Escape {
            return TargetSelectionResult(TargetSelectionState::Cancel, None);
        }
    }

    TargetSelectionResult(TargetSelectionState::Selecting, None)
}
