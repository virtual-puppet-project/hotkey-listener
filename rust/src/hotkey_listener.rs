use std::{
    collections::HashMap,
    str::FromStr,
    time::{Duration, Instant},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use livesplit_hotkey::{Hook, KeyCode};

#[derive(Debug)]
pub enum Error {
    HookCreate,

    HotkeyAlreadyExists,
    BadKeycodeName,
    CannotRegisterHotkey,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Mapping {
    name: String,
    keys: HashMap<KeyCode, KeyCodeData>,
}

impl Mapping {
    fn press(&mut self, key: &KeyCode) {
        self.keys.get_mut(key).unwrap().press();
    }

    fn is_pressed(&self, min_elapsed_time: Duration) -> bool {
        for v in self.keys.values() {
            if !v.is_pressed(min_elapsed_time) {
                return false;
            }
        }
        true
    }
}

struct KeyCodeData {
    key: KeyCode,
    timestamp: Instant,
}

impl KeyCodeData {
    fn new(key: &KeyCode) -> Self {
        KeyCodeData {
            key: key.clone(),
            timestamp: Instant::now() - Duration::from_secs(60), // TODO this is kinda weird
        }
    }

    fn press(&mut self) {
        self.timestamp = Instant::now();
    }

    fn is_pressed(&self, min_elapsed_time: Duration) -> bool {
        self.timestamp.elapsed() < min_elapsed_time
    }
}

/// Based off the HotkeySystem implementation from https://github.com/LiveSplit/livesplit-core
pub struct HotkeyListener {
    hook: Hook,

    key_to_hotkey_map: HashMap<KeyCode, Vec<String>>,
    hotkey_map: HashMap<String, Mapping>,
    min_elapsed_time: Duration,

    callback_sender: Sender<KeyCode>,
    callback_receiver: Receiver<KeyCode>,

    listener_sender: Sender<String>,
}

impl HotkeyListener {
    pub fn new(listener_sender: Sender<String>) -> Result<Self> {
        let hook = match Hook::new() {
            Ok(h) => h,
            Err(e) => {
                eprintln!("{e}");
                return Err(Error::HookCreate);
            }
        };

        let (sender, receiver) = unbounded::<KeyCode>();

        Ok(HotkeyListener {
            hook: hook,

            key_to_hotkey_map: HashMap::default(),
            hotkey_map: HashMap::default(),
            min_elapsed_time: Duration::from_secs_f32(0.2),

            callback_sender: sender,
            callback_receiver: receiver,

            listener_sender: listener_sender,
        })
    }

    pub fn register_hotkey<'a>(&mut self, name: &String, keys: &[String]) -> Result<()> {
        if self.hotkey_map.contains_key(name) {
            return Err(Error::HotkeyAlreadyExists);
        }

        let mut mapping = Mapping::default();
        mapping.name = name.clone();

        for key in keys {
            let keycode = match KeyCode::from_str(key.as_str()) {
                Ok(k) => k,
                Err(_) => return Err(Error::BadKeycodeName),
            };

            mapping.keys.insert(keycode, KeyCodeData::new(&keycode));

            match self.key_to_hotkey_map.get_mut(&keycode) {
                Some(m) => m.push(name.clone()),
                None => {
                    self.key_to_hotkey_map.insert(keycode, vec![name.clone()]);

                    let sender = self.callback_sender.clone();
                    match self
                        .hook
                        .register(keycode, move || match sender.send(keycode) {
                            Ok(_) => {}
                            Err(e) => eprintln!("{e}"),
                        }) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{e}");
                            return Err(Error::CannotRegisterHotkey);
                        }
                    }
                }
            }
        }

        self.hotkey_map.insert(name.clone(), mapping);

        Ok(())
    }

    pub fn poll(&mut self) {
        if self.callback_receiver.is_empty() {
            return;
        }

        match self.callback_receiver.recv() {
            Ok(key) => {
                if !self.key_to_hotkey_map.contains_key(&key) {
                    return;
                }
                for hotkey_name in self.key_to_hotkey_map.get(&key).unwrap() {
                    let mapping = match self.hotkey_map.get_mut(hotkey_name) {
                        Some(m) => m,
                        None => continue,
                    };

                    mapping.press(&key);
                    if mapping.is_pressed(self.min_elapsed_time) {
                        if let Err(e) = self.listener_sender.send(hotkey_name.clone()) {
                            eprintln!("{e}");
                        }
                    }
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    pub fn set_min_elapsed_time(&mut self, new_time: f32) {
        self.min_elapsed_time = Duration::from_secs_f32(new_time);
    }

    pub fn get_min_elapsed_time(&self) -> f32 {
        self.min_elapsed_time.as_secs_f32()
    }
}
