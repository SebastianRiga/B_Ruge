//! Module to create and manage in-game dialogs.

use rltk::{Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

use super::{config, virtual_key_code_to_string};
use std::any::Any;

/// Enum describing all the results
/// a [DialogInterface] can return when it is shown.
#[derive(PartialEq)]
pub enum DialogResult {
    /// Player has selected an option
    /// and the [DialogInterface] can be removed.
    Consumed,

    /// Dialog is awaiting player
    /// input
    Waiting,
}

/// An option the player can select
/// on a [DialogInterface].
pub struct DialogOption {
    /// Description of the option, e.g. 'Yes', 'Leave', etc.
    pub description: String,

    /// The [VirtualKeyCode] the player needs to press to
    /// select the option.
    pub key: VirtualKeyCode,

    pub args: Vec<Box<dyn Any + Send + Sync>>,

    /// The callback function which is invoked when
    /// the player selects the option.
    pub callback: Box<fn(&World, &mut Rltk, args: &Vec<Box<dyn Any + Send + Sync>>)>,
}

impl DialogOption {
    pub fn create_cancel_option() -> DialogOption {
        DialogOption {
            description: "Dismiss".to_string(),
            key: VirtualKeyCode::Escape,
            args: Vec::new(),
            callback: Box::new(|_, _, _| ()),
        }
    }
}

/// A generic interface providing access to
/// dialog functionality, which can be displayed
/// at any part of the game for selection purposes
/// or menuing.
pub struct DialogInterface {
    /// Title of the [DialogInterface].
    pub title: String,

    /// The message body of the [DialogInterface].
    pub message: String,

    /// Vector of options the player can
    /// select through the [DialogInterface].
    pub options: Vec<DialogOption>,

    /// Restrict access for creation to member
    /// functions.
    _private: (),
}

impl DialogInterface {
    /// Registers a new dialog with the ecs, which
    /// will be shown during the next tick of the
    /// game.
    ///
    /// # Arguments
    ///
    /// * `ecs`: Reference to the `ecs` in which the dialog should be registered.
    /// * `title`: The title of the dialog.
    /// * `message`: The message body of the dialog.
    /// * `options`: List of options the player can select.
    ///
    pub fn register_dialog(
        ecs: &mut World,
        title: String,
        message: String,
        options: Vec<DialogOption>,
        cancelable: bool,
    ) {
        // Create the new dialog
        let mut dialog = DialogInterface {
            title,
            message,
            options,
            _private: (),
        };

        if cancelable {
            let cancel_option = DialogOption::create_cancel_option();
            dialog.options.push(cancel_option);
        }

        // If a dialog is already stored in the
        // ecs, remove it.
        if ecs.has_value::<DialogInterface>() {
            ecs.remove::<DialogInterface>();
        }

        // Add the new dialog into the ecs
        ecs.insert(dialog);
    }

    /// Displays the dialog on the screen.
    ///
    /// # Arguments
    /// * `terminal`: Reference to the terminal on which the dialog should be drawn.
    ///
    pub fn show(&mut self, ecs: &World, terminal: &mut Rltk) -> DialogResult {
        // Calculate the width and height for the dialog

        let width = (config::MAP_WIDTH as f32 / 2.5) as i32;
        let mut height = (self.message.len() as f32 / width as f32).ceil() as i32;
        height += (self.options.len() * 2) as i32 + 3;

        // Calculate the x and y coordinate for the dialog

        let x = (config::MAP_WIDTH / 2) - (width / 2);
        let y = (config::MAP_HEIGHT / 2) - (height / 2);

        // Split the message into chunks that fit into the dialogs frame

        let message_chunks = self
            .message
            .as_bytes()
            .chunks((width - 3) as usize)
            .map(|buffer| unsafe { String::from_utf8_unchecked(buffer.to_vec()) })
            .collect::<Vec<String>>();

        // Draw the dialog's box

        terminal.draw_box(
            x,
            y,
            width,
            height,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::DARK_GOLDENROD),
        );

        // Draw the dialog's title

        terminal.print(x + 2, y, &self.title);

        // Draw the message

        let mut y_position = y + 2;

        for chunk in message_chunks {
            terminal.print(x + 2, y_position, chunk);
            y_position += 1;
        }

        y_position += 1;

        // Draw the dialog's options

        for option in self.options.iter() {
            let key_string = virtual_key_code_to_string(option.key);
            terminal.print_color(
                x + 2,
                y_position,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                &format!("{} - {}", key_string, option.description),
            );
            y_position += 2;
        }

        // Listen for key press event

        if let Some(key) = terminal.key {
            let selection = self.options.iter_mut().find(|element| element.key == key);

            if let Some(option) = selection {
                (option.callback)(ecs, terminal, &option.args);
                return DialogResult::Consumed;
            }
        }

        DialogResult::Waiting
    }
}
