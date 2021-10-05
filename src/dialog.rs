//! Module to create and manage in-game dialogs.

use std::any::Any;

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;

use super::{config, swatch, virtual_key_code_to_string};
use crate::ProcessingState;

/// Enum describing all the results
/// a [DialogInterface] can return when it is shown.
#[derive(PartialEq)]
pub enum DialogResult {
    /// Player has selected an option
    /// and the [DialogInterface] can be removed.
    Consumed {
        /// The processing state, the game will change into after
        /// dialog has been consumed.
        next_state: ProcessingState,
    },

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

    /// List of optional arguments to pass to the callback
    /// function.
    pub args: Vec<Box<dyn Any + Send + Sync>>,

    /// The callback function which is invoked when
    /// the player selects the option.
    pub callback:
        Box<fn(&World, &mut Rltk, args: &Vec<Box<dyn Any + Send + Sync>>) -> ProcessingState>,
}

impl DialogOption {
    /// Creates the standardized cancel [DialogOption] for
    /// [DialogInterface]s that are cancelable.
    pub fn create_cancel_option() -> DialogOption {
        DialogOption {
            description: "Dismiss".to_string(),
            key: VirtualKeyCode::Escape,
            args: Vec::new(),
            callback: Box::new(|_, _, _| ProcessingState::Internal),
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

    /// Optional message body of the [DialogInterface].
    pub message: Option<String>,

    /// Vector of options the player can
    /// select through the [DialogInterface].
    pub options: Vec<DialogOption>,

    /// Flag indicating whether or not the
    /// dialog can be closed by the user.
    pub cancelable: bool,

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
    /// * `message`: An optional message body of the dialog.
    /// * `options`: List of options the player can select.
    ///
    pub fn register_dialog(
        ecs: &mut World,
        title: String,
        message: Option<String>,
        options: Vec<DialogOption>,
        cancelable: bool,
    ) {
        // Create the new dialog
        let dialog = DialogInterface {
            title,
            message,
            options,
            cancelable,
            _private: (),
        };

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
    pub fn show(&mut self, ecs: &World, ctx: &mut Rltk) -> DialogResult {
        // Calculate the width and height for the dialog
        let message_length = match &self.message {
            None => 1 as f32,
            Some(message) => message.len() as f32,
        };

        let width = (config::MAP_WIDTH as f32 / 2.5) as i32;
        let mut height = (message_length / width as f32).ceil() as i32;
        height += (self.options.len() * 2) as i32 + 3;

        // Calculate the x and y coordinate for the dialog
        let x = (config::MAP_WIDTH / 2) - (width / 2);
        let y = (config::MAP_HEIGHT / 2) - (height / 2);

        let (fg, bg) = swatch::DIALOG_FRAME.colors();

        // Draw the dialog's box
        ctx.draw_box(x, y, width, height, fg, bg);

        let (fg, bg) = swatch::DIALOG_TITLE.colors();

        // Draw the dialog's title
        ctx.print_color(x + 2, y, fg, bg, &format!("{}", self.title));

        let mut y_position = y + 2;

        // Draw the message if present
        self.print_dialog_message(x, &mut y_position, width, ctx);

        y_position += 1;

        // Draw the dialog's options
        self.print_dialog_options(x, &mut y_position, ctx);

        // If the dialog is cancelable, print the `dismiss` option
        // at the bottom.
        self.print_dialog_dismiss_option(x, y, height, ctx);

        // Listen for key press event
        if let Some(key) = ctx.key {
            let selection = self.options.iter_mut().find(|element| element.key == key);

            if let Some(option) = selection {
                return DialogResult::Consumed {
                    next_state: (option.callback)(ecs, ctx, &option.args),
                };
            }

            // If the dialog is cancelable, check if the `escape` key
            // was pressed.
            if self.cancelable {
                if key == VirtualKeyCode::Escape {
                    return DialogResult::Consumed {
                        next_state: ProcessingState::Internal,
                    };
                }
            }
        }

        // If no key was pressed by the user, return the waiting state to try again in
        // the next frame
        DialogResult::Waiting
    }

    fn print_dialog_message(&mut self, x: i32, y: &mut i32, width: i32, ctx: &mut Rltk) {
        if let Some(message) = &self.message {
            // Split the message into chunks that fit into the dialogs frame
            let message_chunks = message
                .as_bytes()
                .chunks((width - 3) as usize)
                .map(|buffer| unsafe { String::from_utf8_unchecked(buffer.to_vec()) })
                .collect::<Vec<String>>();

            for chunk in message_chunks {
                ctx.print(x + 2, *y, chunk);
                *y += 1;
            }
        }
    }

    fn print_dialog_options(&mut self, x: i32, y: &mut i32, ctx: &mut Rltk) {
        let (fg, bg) = swatch::DIALOG_OPTION.colors();

        for option in self.options.iter() {
            let key_string = virtual_key_code_to_string(option.key);
            let option_label = self.make_dialog_options_label(key_string, &option.description);

            ctx.print_color(x + 2, *y, fg, bg, &option_label);

            *y += 2;
        }
    }

    fn print_dialog_dismiss_option(&mut self, x: i32, y: i32, height: i32, ctx: &mut Rltk) {
        if self.cancelable {
            let (fg, bg) = swatch::DIALOG_DISMISS_BUTTON.colors();

            let dismiss_label = self.make_dialog_options_label("Escape", "Dismiss");

            ctx.print_color(x + 2, y + height, fg, bg, &dismiss_label);
        }
    }

    fn make_dialog_options_label(&self, key: &str, description: &str) -> String {
        format!("({}) {}", key, description)
    }
}
