// use gdnative::prelude::*;

// use crate::hotkey_listener::*;

// const KEY_RECEIVED_SIGNAL: &str = "key_received";

// #[derive(NativeClass)]
// #[inherit(Node)]
// #[register_with(Self::register_signals)]
// pub struct HotkeyListenerNode {
//     is_valid: bool,
//     hotkey_listener: Option<HotkeyListener>,
// }

// #[methods]
// impl HotkeyListenerNode {
//     fn new(_o: &Node) -> Self {
//         match HotkeyListener::new() {
//             Ok(hl) => {
//                 return HotkeyListenerNode {
//                     is_valid: true,
//                     hotkey_listener: Some(hl),
//                 }
//             }
//             Err(_) => {
//                 return HotkeyListenerNode {
//                     is_valid: false,
//                     hotkey_listener: None,
//                 }
//             }
//         }
//     }

//     fn register_signals(build: &ClassBuilder<Self>) {
//         build.signal(KEY_RECEIVED_SIGNAL).done();
//     }

//     #[method]
//     fn _process(&self, #[base] owner: &Node, _delta: f32) {
//         if !self.is_valid {
//             return;
//         }

//         let listener = self.hotkey_listener.as_ref().unwrap();
//         if listener.callback_receiver.is_empty() {
//             return;
//         }
//         match listener.callback_receiver.recv() {
//             Ok(key) => {
//                 owner.emit_signal(
//                     KEY_RECEIVED_SIGNAL,
//                     &[GodotString::from_str(key.as_str()).to_variant()],
//                 );
//             }
//             Err(e) => {
//                 godot_error!("{:?}", e);
//                 return;
//             }
//         }
//     }

//     #[method]
//     fn register_hotkey(&mut self, name: GodotString, keys: VariantArray) -> bool {
//         let listener = self.hotkey_listener.as_mut().unwrap();

//         match listener.register_hotkey(
//             &name.to_string(),
//             keys.into_iter()
//                 .map(|x| x.to_string())
//                 .collect::<Vec<String>>()
//                 .as_slice(),
//         ) {
//             Ok(_) => true,
//             Err(e) => {
//                 godot_error!("{:?}", e);
//                 false
//             }
//         }
//     }

//     #[method]
//     fn is_valid(&self) -> bool {
//         self.is_valid
//     }
// }

// pub fn init(handle: InitHandle) {
//     handle.add_class::<HotkeyListenerNode>();
// }

// godot_init!(init);
