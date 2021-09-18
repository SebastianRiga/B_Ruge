//! Global game settings.

use rltk::console;

/// The current version of the game.
pub const GAME_VERSION: &'static str = "v0.2.7";

/// The name of the game, needed for display on the
/// window and in-game.
pub const GAME_NAME: &'static str = "B_Ruge";

/// The width of the game's window.
pub const WINDOW_WIDTH: i32 = 80;

/// The height of the game's window.
pub const WINDOW_HEIGHT: i32 = 50;

/// The width of the in-game map.
pub const MAP_WIDTH: i32 = 80;

/// The height of the in-game map.
pub const MAP_HEIGHT: i32 = 40;

/// The maximum amount of rooms the
/// map can display.
pub const MAX_ROOMS: i32 = 30;

/// The minimum size of a room on
/// the map.
pub const MIN_ROOM_SIZE: i32 = 6;

/// The maximum size of a room on
/// the map.
pub const MAX_ROOM_SIZE: i32 = 10;

/// The maximum amount of monsters, that can be spawned in a single room of the game.
pub const MAX_MONSTERS_PER_ROOM: i32 = 4;

/// The maximum amount of items, that can be spawned in a single room of the game.
pub const MAX_ITEMS_PER_ROOM: i32 = 2;

/// Prints the games logo, copyright notice and current
/// version to the console. 
/// 
/// # Notes
/// * If the game is running in a browser through web assembly, the
/// message is printed to the browsers debug console.
/// 
pub fn log_starting_message() {
    let message = format!(
        r#"
             :::::::::            :::::::::  :::    :::  ::::::::  :::::::::: 
             :+:    :+:           :+:    :+: :+:    :+: :+:    :+: :+:         
            +:+    +:+           +:+    +:+ +:+    +:+ +:+        +:+          
           +#++:++#+            +#++:++#:  +#+    +:+ :#:        +#++:++#      
          +#+    +#+           +#+    +#+ +#+    +#+ +#+   +#+# +#+            
         #+#    #+#           #+#    #+# #+#    #+# #+#    #+# #+#             
        ######### ########## ###    ###  ########   ########  ##########   
        
        by Sebastian Riga (c) 2021
        version: {}
        "#,
        GAME_VERSION
    );

    console::log(message);
}
