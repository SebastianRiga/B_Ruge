//! Factory for producing standardized dialogs

#[cfg_attr(target_arch = "wasm32", allow(unused_imports))]
use rltk::VirtualKeyCode;
#[cfg_attr(target_arch = "wasm32", allow(unused_imports))]
use specs::World;

#[cfg_attr(target_arch = "wasm32", allow(unused_imports))]
use super::{DialogInterface, DialogOption, ProcessingState};

#[cfg(not(target_arch = "wasm32"))]
pub fn register_pause_dialog(ecs: &mut World) {
    DialogInterface::register_dialog(
        ecs,
        "Pause".to_string(),
        Some("What would you like to do in this moment of respite?".to_string()),
        vec![
            DialogOption {
                description: "Save".to_string(),
                key: VirtualKeyCode::S,
                args: vec![],
                callback: Box::new(|_, ctx, _| {
                    ctx.quit();
                    ProcessingState::Internal
                }),
            },
            DialogOption {
                description: "Load".to_string(),
                key: VirtualKeyCode::L,
                args: vec![],
                callback: Box::new(|_, ctx, _| {
                    ctx.quit();
                    ProcessingState::Internal
                }),
            },
            DialogOption {
                description: "Quit".to_string(),
                key: VirtualKeyCode::Q,
                args: vec![],
                callback: Box::new(|_, ctx, _| {
                    ctx.quit();
                    ProcessingState::Internal
                }),
            },
        ],
        true,
    );
}

#[cfg(target_arch = "wasm32")]
pub fn register_pause_dialog(_ecs: &mut World) {}
