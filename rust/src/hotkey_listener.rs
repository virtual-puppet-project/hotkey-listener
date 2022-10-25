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

    // ActionAlreadyExists,
    ActionDoesNotExist,
    KeyNotMapped,

    MappedKeyMissingInReverseLookup,

    BadKeycodeName,
    CannotRegisterHotkey,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub struct ActionMapping {
    name: String,
    keys: HashMap<KeyCode, KeyCodeData>,
}

impl ActionMapping {
    fn new(name: &String) -> Self {
        ActionMapping {
            name: name.clone(),
            keys: HashMap::new(),
        }
    }

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

    fn add_key(&mut self, key: &KeyCode) {
        match self.keys.get_mut(key) {
            Some(kcd) => kcd.add_dependent(),
            None => {
                let mut kcd = KeyCodeData::new(key);
                kcd.add_dependent();

                self.keys.insert(key.clone(), kcd);
            }
        }
    }

    /// Removes a given key from the mapping.
    ///
    /// Returns `true` if there are still dependencies on the key
    fn remove_key(&mut self, key: &KeyCode) -> Result<bool> {
        match self.keys.get_mut(key) {
            Some(kcd) => {
                kcd.remove_dependent();
                Ok(kcd.has_dependents())
            }
            None => Err(Error::KeyNotMapped),
        }
    }
}

#[derive(Debug, Clone)]
struct KeyCodeData {
    key: KeyCode,
    timestamp: Instant,
    dependents: u16,
}

impl KeyCodeData {
    fn new(key: &KeyCode) -> Self {
        KeyCodeData {
            key: key.clone(),
            timestamp: Instant::now() - Duration::from_secs(60), // TODO this is kinda weird
            dependents: 0,
        }
    }

    fn press(&mut self) {
        self.timestamp = Instant::now();
    }

    fn is_pressed(&self, min_elapsed_time: Duration) -> bool {
        self.timestamp.elapsed() < min_elapsed_time
    }

    fn add_dependent(&mut self) {
        self.dependents += 1;
    }

    fn remove_dependent(&mut self) {
        self.dependents -= 1;
    }

    fn has_dependents(&self) -> bool {
        self.dependents > 0
    }
}

/// Based off the HotkeySystem implementation from https://github.com/LiveSplit/livesplit-core
pub struct HotkeyListener {
    hook: Hook,

    key_to_action_map: HashMap<KeyCode, Vec<String>>,
    action_map: HashMap<String, ActionMapping>,
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

            key_to_action_map: HashMap::default(),
            action_map: HashMap::default(),
            min_elapsed_time: Duration::from_secs_f32(0.2),

            callback_sender: sender,
            callback_receiver: receiver,

            listener_sender: listener_sender,
        })
    }

    pub fn register_action(&mut self, name: &String, keys: &[String]) -> Result<()> {
        // if self.action_map.contains_key(name) {
        //     return Err(Error::ActionAlreadyExists);
        // }

        let mut mapping = self
            .action_map
            .get(name)
            .unwrap_or(&ActionMapping::new(name))
            .clone();

        for key in keys {
            let keycode = match KeyCode::from_str(key.as_str()) {
                Ok(k) => k,
                Err(_) => return Err(Error::BadKeycodeName),
            };

            // mapping.keys.insert(keycode, KeyCodeData::new(&keycode));
            mapping.add_key(&keycode);

            match self.key_to_action_map.get_mut(&keycode) {
                Some(m) => m.push(name.clone()),
                None => {
                    self.key_to_action_map.insert(keycode, vec![name.clone()]);

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

        self.action_map.insert(name.clone(), mapping);

        Ok(())
    }

    pub fn unregister_action(&mut self, name: &String) -> Result<()> {
        if !self.action_map.contains_key(name) {
            return Err(Error::ActionDoesNotExist);
        }

        let mapping = self.action_map.get_mut(name).unwrap();
        let mut keys_to_remove = vec![];

        for (k, v) in mapping.keys.iter_mut() {
            v.remove_dependent();
            if !v.has_dependents() {
                keys_to_remove.push(k);
                let key_to_action_map_vec = match self.key_to_action_map.get_mut(k) {
                    Some(vec) => vec,
                    None => return Err(Error::MappedKeyMissingInReverseLookup),
                };

                // TODO this assumes the action name does exist, maybe the case where the reverse
                // lookup exists but the action name is not mapped should be handled?
                key_to_action_map_vec.retain(|action_name| action_name != name);
            }
        }

        todo!();

        Ok(())
    }

    pub fn poll(&mut self) {
        if self.callback_receiver.is_empty() {
            return;
        }

        match self.callback_receiver.recv() {
            Ok(key) => {
                if !self.key_to_action_map.contains_key(&key) {
                    return;
                }
                for hotkey_name in self.key_to_action_map.get(&key).unwrap() {
                    let mapping = match self.action_map.get_mut(hotkey_name) {
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
