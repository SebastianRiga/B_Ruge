//! Module containing all UI functionality of the game

/// TODO: Finish documentation
use rltk::{Point, Rltk};
use specs::prelude::*;

use super::{
    config, swatch, timestamp_formatted, GameLog, Map, Name, Player, Position, Statistics,
};

/// Draws the ui of the game in the given `ctx`.
///
/// # Arguments
/// * `ecs`: The `ecs` is needed to read values from the game for display.
/// * `ctx`: The console context in which the ui should be drawn.
///
/// # See also
/// * [draw_message_log]: Draws the message log box at the bottom of the screen.
/// * [draw_player_health]: Draws the players health information and a corresponding health bar
/// on top of the
///
pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    draw_message_log(ctx);
    draw_messages(ecs, ctx);
    draw_player_health(ecs, ctx);
    draw_mouse_cursor(ctx);
    draw_tooltips(ecs, ctx);
}

fn draw_message_log(ctx: &mut Rltk) {
    let (x, y) = (0, config::MAP_HEIGHT);
    let (width, height) = (
        config::WINDOW_WIDTH - 1,
        config::WINDOW_WIDTH - config::MAP_HEIGHT - 1,
    );
    let (fg, bg) = swatch::MESSAGE_BOX.colors();

    ctx.draw_box(x, y, width, height, fg, bg);
}

fn draw_messages(ecs: &World, ctx: &mut Rltk) {
    let mut game_log = ecs.fetch_mut::<GameLog>();

    let x = 2;
    let mut y = config::MAP_HEIGHT + 1;

    game_log.messages_for_each_rev(|message| {
        if y < config::WINDOW_HEIGHT - 1 {
            let timestamp = timestamp_formatted();
            ctx.print(x, y, &format!("{} > {}", timestamp, message));
            y += 1;
        }
    })
}

fn draw_player_health(ecs: &World, ctx: &mut Rltk) {
    let players = ecs.read_storage::<Player>();
    let statistics = ecs.read_storage::<Statistics>();

    for (_, statistic) in (&players, &statistics).join() {
        let health = format!(" HP: {} / {} ", statistic.hp, statistic.hp_max);

        let (fg, bg) = swatch::PLAYER_HEALTH_TEXT.colors();

        ctx.print_color(12, config::MAP_HEIGHT, fg, bg, &health);

        let (fg, bg) = swatch::PLAYER_HEALTH_BAR.colors();

        ctx.draw_bar_horizontal(
            28,
            config::MAP_HEIGHT,
            50,
            statistic.hp,
            statistic.hp_max,
            fg,
            bg,
        );
    }
}

fn draw_mouse_cursor(ctx: &mut Rltk) {
    let (x, y) = ctx.mouse_pos();
    ctx.set_bg(x, y, swatch::MOUSE_CURSOR);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
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

    if x > 40 {
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
