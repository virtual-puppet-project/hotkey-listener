use crossbeam_channel::Receiver;
use gdnative::prelude::*;

use crate::godot::varray_to_vec;
use crate::hotkey_listener::*;

const ACTION_RECEIVED_SIGNAL: &str = "action_pressed";

/// Godot wrapper for interacting with the base Rust library.
///
/// In the interest of having Godot handle errors, this struct can potentially fail to initialize.
/// The `is_valid` func should be checked before doing anything with the object. If the object is not valid,
/// then Godot should clean up the object.
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
pub struct HotkeyListenerNode {
    is_valid: bool,
    hotkey_listener: Option<HotkeyListener>,
    receiver: Receiver<String>,
}

#[methods]
impl HotkeyListenerNode {
    fn new(_o: &Node) -> Self {
        let (s, r) = crossbeam_channel::unbounded();
        match HotkeyListener::new(s) {
            Ok(hl) => {
                return HotkeyListenerNode {
                    is_valid: true,
                    hotkey_listener: Some(hl),
                    receiver: r,
                }
            }
            Err(e) => {
                godot_error!("An error occurred while setting up HotkeyListener: {:?}", e);
                return HotkeyListenerNode {
                    is_valid: false,
                    hotkey_listener: None,
                    receiver: r,
                };
            }
        }
    }

    fn register_signals(build: &ClassBuilder<Self>) {
        build.signal(ACTION_RECEIVED_SIGNAL).done();
    }

    #[method]
    fn _ready(&self, #[base] o: &Node) {
        o.set_process(if self.is_valid { true } else { false });
    }

    #[method]
    fn _process(&mut self, #[base] owner: &Node, _delta: f32) {
        let listener = self.hotkey_listener.as_mut().unwrap();
        listener.poll();

        if self.receiver.is_empty() {
            return;
        }
        match self.receiver.recv() {
            Ok(s) => {
                owner.emit_signal(
                    ACTION_RECEIVED_SIGNAL,
                    &[GodotString::from_str(s).to_variant()],
                );
                return;
            }
            Err(e) => godot_error!("{:?}", e),
        }
    }

    /// Setting up the initial OS hook can fail. If initial setup fails, then this class is no longer valid.
    ///
    /// This **MUST** be checked first since all other functions assume the setup succeeded.
    #[method]
    fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Godot -> Rust wrapper
    #[method]
    fn register_action(&mut self, name: GodotString, keys: VariantArray) -> bool {
        let listener = self.hotkey_listener.as_mut().unwrap();

        match listener.register_action(&name.to_string(), varray_to_vec(&keys).as_slice()) {
            Ok(_) => true,
            Err(e) => {
                godot_error!("{:?}", e);
                false
            }
        }
    }

    /// Godot -> Rust wrapper
    #[method]
    fn unregister_action(&mut self, name: GodotString, keys: VariantArray) -> bool {
        let listener = self.hotkey_listener.as_mut().unwrap();

        match listener.unregister_action(&name.to_string(), varray_to_vec(&keys).as_slice()) {
            Ok(_) => true,
            Err(e) => {
                godot_error!("{:?}", e);
                false
            }
        }
    }

    /// Godot -> Rust wrapper
    #[method]
    fn get_min_elapsed_time(&self) -> f32 {
        self.hotkey_listener
            .as_ref()
            .unwrap()
            .get_min_elapsed_time()
    }

    /// Godot -> Rust wrapper
    #[method]
    fn set_min_elapsed_time(&mut self, min_elapsed_time: f32) {
        self.hotkey_listener
            .as_mut()
            .unwrap()
            .set_min_elapsed_time(min_elapsed_time);
    }

    /// Godot -> Rust wrapper
    #[method]
    fn get_action_names(&self) -> VariantArray {
        let r = VariantArray::new();

        for n in self
            .hotkey_listener
            .as_ref()
            .unwrap()
            .get_action_names()
            .iter()
        {
            r.push(n);
        }

        r.into_shared()
    }

    /// Godot -> Rust wrapper
    #[method]
    fn get_key_names(&self) -> VariantArray {
        let r = VariantArray::new();

        for n in self
            .hotkey_listener
            .as_ref()
            .unwrap()
            .get_key_names()
            .iter()
        {
            r.push(n);
        }

        r.into_shared()
    }
}
