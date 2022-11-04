use gdnative::prelude::*;

mod node;
use node::HotkeyListenerNode;

mod popup;
use popup::HotkeyListenerPopup;

/// Converts a `VariantArray` to a `Vec`.
fn varray_to_vec(keys: &VariantArray) -> Vec<String> {
    keys.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn init(handle: InitHandle) {
    handle.add_class::<HotkeyListenerNode>();
    handle.add_class::<HotkeyListenerPopup>();
}

godot_init!(init);
