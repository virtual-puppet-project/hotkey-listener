use crossbeam_channel::Receiver;
use gdnative::prelude::*;

use crate::hotkey_listener::*;

const KEY_RECEIVED_SIGNAL: &str = "key_received";

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
            Err(_) => {
                return HotkeyListenerNode {
                    is_valid: false,
                    hotkey_listener: None,
                    receiver: r,
                }
            }
        }
    }

    fn register_signals(build: &ClassBuilder<Self>) {
        build.signal(KEY_RECEIVED_SIGNAL).done();
    }

    #[method]
    fn _process(&mut self, #[base] owner: &Node, _delta: f32) {
        if !self.is_valid {
            return;
        }

        let listener = match self.hotkey_listener.as_mut() {
            Some(l) => l,
            None => return,
        };
        listener.poll();

        if self.receiver.is_empty() {
            return;
        }
        match self.receiver.recv() {
            Ok(s) => {
                owner.emit_signal(
                    KEY_RECEIVED_SIGNAL,
                    &[GodotString::from_str(s).to_variant()],
                );
                return;
            }
            Err(e) => godot_error!("{:?}", e),
        }
    }

    #[method]
    fn register_hotkey(&mut self, name: GodotString, keys: VariantArray) -> bool {
        let listener = self.hotkey_listener.as_mut().unwrap();

        match listener.register_hotkey(
            &name.to_string(),
            keys.into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .as_slice(),
        ) {
            Ok(_) => true,
            Err(e) => {
                godot_error!("{:?}", e);
                false
            }
        }
    }

    #[method]
    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

pub fn init(handle: InitHandle) {
    handle.add_class::<HotkeyListenerNode>();
}

godot_init!(init);
