#[cfg(feature = "gdnative")]
mod godot;

pub mod hotkey_listener;
pub use hotkey_listener::HotkeyListener;
