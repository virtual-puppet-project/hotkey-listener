use std::{collections::HashMap, str::FromStr};

use crossbeam_channel::{bounded, Receiver, Sender};
use gdnative::prelude::*;
use livesplit_hotkey::{Hook, KeyCode};

#[derive(Debug)]
enum Error {
    HookCreate,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
struct Mapping {
    name: String,
    keys: HashMap<KeyCode, bool>,
}

/// Based off the HotkeySystem implementation from https://github.com/LiveSplit/livesplit-core
struct HotkeyListener {
    hook: Hook,
    key_to_hotkey_map: HashMap<KeyCode, Vec<String>>,
    hotkey_map: HashMap<String, Mapping>,
    receiver: Receiver<KeyCode>,
    callback_sender: Sender<KeyCode>,
}

impl HotkeyListener {
    fn new() -> Result<Self> {
        let hook = match Hook::new() {
            Ok(h) => h,
            Err(e) => {
                godot_error!("{:?}", e);
                return Err(Error::HookCreate);
            }
        };

        let (sender, receiver) = bounded::<KeyCode>(1);

        Ok(HotkeyListener {
            hook: hook,
            key_to_hotkey_map: HashMap::default(),
            hotkey_map: HashMap::default(),
            receiver: receiver,
            callback_sender: sender,
        })
    }
}

const KEY_RECEIVED_SIGNAL: &str = "key_received";

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
struct HotkeyListenerNode {
    is_valid: bool,
    hotkey_listener: Option<HotkeyListener>,
}

#[methods]
impl HotkeyListenerNode {
    fn new(_o: &Node) -> Self {
        match HotkeyListener::new() {
            Ok(hl) => {
                return HotkeyListenerNode {
                    is_valid: true,
                    hotkey_listener: Some(hl),
                }
            }
            Err(_) => {
                return HotkeyListenerNode {
                    is_valid: false,
                    hotkey_listener: None,
                }
            }
        }
    }

    fn register_signals(build: &ClassBuilder<Self>) {
        build.signal(KEY_RECEIVED_SIGNAL).done();
    }

    #[method]
    fn _process(&self, #[base] owner: &Node, _delta: f32) {
        if !self.is_valid {
            return;
        }

        let listener = self.hotkey_listener.as_ref().unwrap();
        if listener.receiver.is_empty() {
            return;
        }
        match listener.receiver.recv() {
            Ok(key) => {
                owner.emit_signal(
                    KEY_RECEIVED_SIGNAL,
                    &[GodotString::from_str(key.as_str()).to_variant()],
                );
            }
            Err(e) => {
                godot_error!("{:?}", e);
                return;
            }
        }
    }

    #[method]
    fn register_hotkey(&mut self, name: GodotString, keys: PoolArray<GodotString>) -> bool {
        let listener = self.hotkey_listener.as_mut().unwrap();

        if listener.hotkey_map.contains_key(&name.to_string()) {
            godot_error!("Hotkey {} is already registered.", name);
            return false;
        }

        let mut mapping = Mapping::default();
        mapping.name = name.to_string();
        for key in keys.read().iter() {
            let keycode = match KeyCode::from_str(key.to_string().as_str()) {
                Ok(k) => k,
                Err(_) => {
                    godot_error!("Invalid key {}", key);
                    return false;
                }
            };
            mapping.keys.insert(keycode, false);

            match listener.key_to_hotkey_map.get_mut(&keycode) {
                Some(m) => m.push(mapping.name.clone()),
                None => {
                    listener
                        .key_to_hotkey_map
                        .insert(keycode, vec![mapping.name.clone()]);

                    let hook = &listener.hook;
                    let sender = listener.callback_sender.clone();
                    match hook.register(keycode, move || match sender.send(keycode) {
                        Ok(_) => {}
                        Err(e) => godot_error!("{:?}", e),
                    }) {
                        Ok(_) => {}
                        Err(e) => {
                            godot_error!("{:?}", e);
                            return false;
                        }
                    }
                }
            }
        }

        listener.hotkey_map.insert(name.to_string(), mapping);

        true
    }

    #[method]
    fn register_hotkey_dict(&mut self, dict: Dictionary) -> bool {
        true
    }

    #[method]
    fn is_valid(&self) -> bool {
        self.is_valid
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HotkeyListenerNode>();
}

godot_init!(init);
