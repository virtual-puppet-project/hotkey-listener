use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    str::FromStr,
    time::{Duration, Instant},
};

use crossbeam_channel::{unbounded, Receiver, Sender};
use livesplit_hotkey::{Hook, KeyCode};

#[derive(Debug)]
pub enum Error {
    HookCreate,

    ActionAlreadyExists,
    ActionDoesNotExist,
    KeyNotMapped,

    MappedKeyMissingInReverseLookup,

    BadKeyCodeName,
    CannotRegisterHotkey(Box<dyn std::error::Error>),
}

type Result<T> = std::result::Result<T, Error>;

// #[derive(Clone)]
// pub struct ActionMapping {
//     id: Uuid,
//     name: String,
//     keys: HashMap<KeyCode, KeyCodeData>,
// }

// impl ActionMapping {
//     fn new(name: &String) -> Self {
//         ActionMapping {
//             id: Uuid::new_v4(),
//             name: name.clone(),
//             keys: HashMap::new(),
//         }
//     }

//     fn press(&mut self, key: &KeyCode) {
//         self.keys.get_mut(key).unwrap().press();
//     }

//     fn is_pressed(&self, min_elapsed_time: Duration) -> bool {
//         for v in self.keys.values() {
//             if !v.is_pressed(min_elapsed_time) {
//                 return false;
//             }
//         }
//         true
//     }

//     fn add_key(&mut self, key: &KeyCode) {
//         match self.keys.get_mut(key) {
//             Some(kcd) => kcd.add_dependent(),
//             None => {
//                 let mut kcd = KeyCodeData::new(key);
//                 kcd.add_dependent();

//                 self.keys.insert(key.clone(), kcd);
//             }
//         }
//     }

//     /// Removes a given key from the mapping.
//     ///
//     /// Returns `true` if there are still dependencies on the key
//     fn remove_key(&mut self, key: &KeyCode) -> Result<bool> {
//         match self.keys.get_mut(key) {
//             Some(kcd) => {
//                 kcd.remove_dependent();
//                 Ok(kcd.has_dependents())
//             }
//             None => Err(Error::KeyNotMapped),
//         }
//     }
// }

// #[derive(Debug, Clone)]
// struct KeyCodeData {
//     key: KeyCode,
//     timestamp: Instant,
//     dependents: u16,
// }

// impl KeyCodeData {
//     fn new(key: &KeyCode) -> Self {
//         KeyCodeData {
//             key: key.clone(),
//             timestamp: Instant::now() - Duration::from_secs(60), // TODO this is kinda weird
//             dependents: 0,
//         }
//     }

//     fn press(&mut self) {
//         self.timestamp = Instant::now();
//     }

//     fn is_pressed(&self, min_elapsed_time: Duration) -> bool {
//         self.timestamp.elapsed() < min_elapsed_time
//     }

//     fn add_dependent(&mut self) {
//         self.dependents += 1;
//     }

//     fn remove_dependent(&mut self) {
//         self.dependents -= 1;
//     }

//     fn has_dependents(&self) -> bool {
//         self.dependents > 0
//     }
// }

// /// Based off the HotkeySystem implementation from https://github.com/LiveSplit/livesplit-core
// pub struct HotkeyListener {
//     hook: Hook,

//     action_map: HashMap<Uuid, ActionMapping>,
//     key_to_action_map: HashMap<KeyCode, Vec<Uuid>>,
//     name_to_action_uuid_map: HashMap<String, Uuid>,

//     min_elapsed_time: Duration,

//     callback_sender: Sender<KeyCode>,
//     callback_receiver: Receiver<KeyCode>,

//     listener_sender: Sender<String>,
// }

// impl HotkeyListener {
//     pub fn new(listener_sender: Sender<String>) -> Result<Self> {
//         let hook = match Hook::new() {
//             Ok(h) => h,
//             Err(e) => {
//                 eprintln!("{e}");
//                 return Err(Error::HookCreate);
//             }
//         };

//         let (sender, receiver) = unbounded::<KeyCode>();

//         Ok(HotkeyListener {
//             hook: hook,

//             key_to_action_map: HashMap::default(),
//             action_map: HashMap::default(),
//             min_elapsed_time: Duration::from_secs_f32(0.2),

//             callback_sender: sender,
//             callback_receiver: receiver,

//             listener_sender: listener_sender,
//         })
//     }

//     pub fn register_action(&mut self, name: &String, keys: &[String]) -> Result<()> {
//         // if self.action_map.contains_key(name) {
//         //     return Err(Error::ActionAlreadyExists);
//         // }

//         // let mut mapping = self
//         //     .action_map
//         //     .get(name)
//         //     .unwrap_or(&ActionMapping::new(name))
//         //     .clone();
//         let mut mapping = self
//             .action_map
//             .

//         for key in keys {
//             let keycode = match KeyCode::from_str(key.as_str()) {
//                 Ok(k) => k,
//                 Err(_) => return Err(Error::BadKeycodeName),
//             };

//             // mapping.keys.insert(keycode, KeyCodeData::new(&keycode));
//             mapping.add_key(&keycode);

//             match self.key_to_action_map.get_mut(&keycode) {
//                 Some(m) => m.push(mapping),
//                 None => {
//                     self.key_to_action_map.insert(keycode, vec![mapping]);

//                     let sender = self.callback_sender.clone();
//                     match self
//                         .hook
//                         .register(keycode, move || match sender.send(keycode) {
//                             Ok(_) => {}
//                             Err(e) => eprintln!("{e}"),
//                         }) {
//                         Ok(_) => {}
//                         Err(e) => {
//                             eprintln!("{e}");
//                             return Err(Error::CannotRegisterHotkey);
//                         }
//                     }
//                 }
//             }
//         }

//         self.action_map.insert(name.clone(), mapping);

//         Ok(())
//     }

//     pub fn unregister_action(&mut self, name: &String) -> Result<()> {
//         if !self.action_map.contains_key(name) {
//             return Err(Error::ActionDoesNotExist);
//         }

//         let mapping = self.action_map.get_mut(name).unwrap();
//         let mut keys_to_remove = vec![];

//         for (k, v) in mapping.keys.iter_mut() {
//             v.remove_dependent();
//             if !v.has_dependents() {
//                 keys_to_remove.push(k);
//                 let key_to_action_map_vec = match self.key_to_action_map.get_mut(k) {
//                     Some(vec) => vec,
//                     None => return Err(Error::MappedKeyMissingInReverseLookup),
//                 };

//                 // TODO this assumes the action name does exist, maybe the case where the reverse
//                 // lookup exists but the action name is not mapped should be handled?
//                 key_to_action_map_vec.retain(|action_name| &action_name.name != name);
//             }
//         }

//         todo!();

//         Ok(())
//     }

//     pub fn poll(&mut self) {
//         if self.callback_receiver.is_empty() {
//             return;
//         }

//         match self.callback_receiver.recv() {
//             Ok(key) => {
//                 if !self.key_to_action_map.contains_key(&key) {
//                     return;
//                 }
//                 for mapping in self.key_to_action_map.get(&key).unwrap() {
//                     mapping.press(&key);
//                     if mapping.is_pressed(self.min_elapsed_time) {
//                         if let Err(e) = self.listener_sender.send(mapping.name) {
//                             eprintln!("{e}");
//                         }
//                     }
//                 }
//             }
//             Err(e) => eprintln!("{e}"),
//         }
//     }

//     pub fn set_min_elapsed_time(&mut self, new_time: f32) {
//         self.min_elapsed_time = Duration::from_secs_f32(new_time);
//     }

//     pub fn get_min_elapsed_time(&self) -> f32 {
//         self.min_elapsed_time.as_secs_f32()
//     }
// }

struct Action {
    name: String,
    keys: HashMap<KeyCode, Instant>,
}

impl Action {
    fn new(name: &String, keys: &[KeyCode]) -> Self {
        let mut hash_map = HashMap::new();
        let offset = Duration::from_secs(60);
        for key in keys.iter() {
            hash_map.insert(key.clone(), Instant::now() - offset);
        }

        Action {
            name: name.clone(),
            keys: hash_map,
        }
    }

    /// Update the last pressed time for a given keycode.
    ///
    /// Does not check if the key is valid!
    fn press_key(&mut self, key: &KeyCode) {
        self.keys.insert(*key, Instant::now());
    }

    /// Iterates through every single key's timestamp and compares it to the passed
    /// `min_elapsed_time`. If all timestamps are less than the `min_elapsed_time`,
    /// then the `Action` is considered to be pressed.
    fn is_pressed(&self, min_elapsed_time: Duration) -> bool {
        for time in self.keys.values() {
            if time.elapsed() > min_elapsed_time {
                return false;
            }
        }

        true
    }
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.keys.keys().collect::<Vec<&KeyCode>>().hash(state);
    }
}

pub struct HotkeyListener {
    hook: Hook,
    hasher: DefaultHasher,

    actions: HashMap<u64, HashMap<u64, Action>>,
    reverse_lookup: HashMap<KeyCode, Vec<u64>>,

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
            hasher: DefaultHasher::new(),

            actions: HashMap::new(),
            reverse_lookup: HashMap::new(),

            min_elapsed_time: Duration::from_secs_f32(0.2), // TODO hardcoded value?

            callback_sender: sender,
            callback_receiver: receiver,

            listener_sender: listener_sender,
        })
    }

    pub fn register_action(&mut self, action_name: &String, keys: &[String]) -> Result<()> {
        let mut key_codes = vec![];
        for key in keys.iter() {
            match KeyCode::from_str(key) {
                Ok(k) => key_codes.push(k),
                Err(_) => return Err(Error::BadKeyCodeName),
            };
        }

        key_codes.hash(&mut self.hasher);
        let key_codes_hash = self.hasher.finish();

        let action = Action::new(action_name, key_codes.as_slice());
        action.hash(&mut self.hasher);
        let action_hash = self.hasher.finish();

        match self.actions.get_mut(&key_codes_hash) {
            Some(hm) => {
                if hm.contains_key(&action_hash) {
                    return Err(Error::ActionAlreadyExists);
                }
                hm.insert(action_hash, action);
            }
            None => {
                let mut hm = HashMap::new();
                hm.insert(action_hash, action);
                self.actions.insert(key_codes_hash, hm);
            }
        }

        for key in key_codes.iter() {
            match self.reverse_lookup.get_mut(key) {
                Some(v) => v.push(key_codes_hash),
                None => {
                    let sender = self.callback_sender.clone();
                    let key = key.clone();
                    match self.hook.register(key, move || match sender.send(key) {
                        Ok(_) => {}
                        Err(e) => eprintln!("{e}"),
                    }) {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(Error::CannotRegisterHotkey(Box::new(e)));
                        }
                    }
                    self.reverse_lookup.insert(key, vec![key_codes_hash]);
                }
            }
        }

        Ok(())
    }

    pub fn unregister_action(&mut self, action_name: &String, keys: &[String]) -> Result<()> {
        todo!()
    }

    // TODO maybe we should clear the channel? Clearing the channel might infinitely loop though
    pub fn poll(&mut self) {
        if self.callback_receiver.is_empty() {
            return;
        }

        match self.callback_receiver.recv() {
            Ok(key) => {
                if !self.reverse_lookup.contains_key(&key) {
                    return;
                }
                let vec = match self.reverse_lookup.get(&key) {
                    Some(v) => v,
                    None => {
                        return;
                    }
                };

                for hash in vec.iter() {
                    match self.actions.get_mut(&hash) {
                        Some(hm) => {
                            for action in hm.values_mut() {
                                action.press_key(&key);
                                if action.is_pressed(self.min_elapsed_time) {
                                    match self.listener_sender.send(action.name.clone()) {
                                        Ok(_) => {}
                                        Err(e) => eprintln!("{e}"),
                                    }
                                }
                            }
                        }
                        None => unreachable!(),
                    }
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    pub fn set_min_elapsed_time(&mut self, min_elapsed_time: f32) {
        self.min_elapsed_time = Duration::from_secs_f32(min_elapsed_time);
    }
}
