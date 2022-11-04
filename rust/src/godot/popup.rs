use gdnative::{
    api::{
        control::{LayoutPreset, LayoutPresetMode, SizeFlags},
        label::{Align, VAlign},
        CheckBox, GlobalConstants, HBoxContainer, HFlowContainer, VBoxContainer, WindowDialog, OS,
    },
    core_types::Margin,
    prelude::*,
};
use livesplit_hotkey::KeyCode;

const DIALOG_COMPLETE_SIGNAL: &str = "dialog_complete";

#[derive(NativeClass)]
#[inherit(WindowDialog)]
#[register_with(Self::register_signals)]
pub struct HotkeyListenerPopup {
    keys_pressed: u16,
    key_names: Vec<GodotString>,
    modifier_names: Vec<GodotString>,
    values_label: Ref<Label, Shared>,
}

#[methods]
impl HotkeyListenerPopup {
    fn new(_o: &WindowDialog) -> Self {
        HotkeyListenerPopup {
            keys_pressed: 0,
            key_names: vec![],
            modifier_names: vec![],
            values_label: Label::new().into_shared(),
        }
    }

    fn register_signals(build: &ClassBuilder<Self>) {
        build.signal(DIALOG_COMPLETE_SIGNAL).done()
    }

    #[method]
    fn _ready(&self, #[base] owner: TRef<WindowDialog>) {
        owner.set_resizable(true);
        owner.set_title("HOTKEY_LISTENER_POPUP_WINDOW_TITLE");

        let vbox = VBoxContainer::new();
        vbox.set_anchors_and_margins_preset(LayoutPreset::WIDE.0, LayoutPresetMode::MINSIZE.0, 10);

        let values_label = unsafe { self.values_label.assume_unique() };
        values_label.set_align(Align::CENTER.into());
        values_label.set_valign(VAlign::CENTER.into());
        values_label.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        values_label.set_v_size_flags(SizeFlags::EXPAND_FILL.0);

        let modifiers_box = HFlowContainer::new();

        let control_l = CheckBox::new();
        setup_modifier(owner, &control_l, &KeyCode::ControlLeft);
        control_l.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        let control_r = CheckBox::new();
        setup_modifier(owner, &control_r, &KeyCode::ControlRight);
        control_r.set_h_size_flags(SizeFlags::EXPAND_FILL.0);

        let alt_l = CheckBox::new();
        setup_modifier(owner, &alt_l, &KeyCode::AltLeft);
        alt_l.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        let alt_r = CheckBox::new();
        setup_modifier(owner, &alt_r, &KeyCode::AltRight);
        alt_r.set_h_size_flags(SizeFlags::EXPAND_FILL.0);

        let shift_l = CheckBox::new();
        setup_modifier(owner, &shift_l, &KeyCode::ShiftLeft);
        shift_l.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        let shift_r = CheckBox::new();
        setup_modifier(owner, &shift_r, &KeyCode::ShiftRight);
        shift_r.set_h_size_flags(SizeFlags::EXPAND_FILL.0);

        let super_l = CheckBox::new();
        setup_modifier(owner, &super_l, &KeyCode::MetaLeft);
        super_l.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        let super_r = CheckBox::new();
        setup_modifier(owner, &super_r, &KeyCode::MetaRight);
        super_r.set_h_size_flags(SizeFlags::EXPAND_FILL.0);

        modifiers_box.add_child(control_l, false);
        modifiers_box.add_child(control_r, false);
        modifiers_box.add_child(alt_l, false);
        modifiers_box.add_child(alt_r, false);
        modifiers_box.add_child(super_l, false);
        modifiers_box.add_child(super_r, false);

        let confirm_cancel_box = HBoxContainer::new();

        let confirm = Button::new();
        confirm.set_h_size_flags(SizeFlags::EXPAND.0 + SizeFlags::SHRINK_CENTER.0);
        confirm.set_text("HOTKEY_LISTENER_POPUP_WINDOW_CONFIRM");
        confirm
            .connect(
                "pressed",
                owner,
                "_on_confirm",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();

        let cancel = Button::new();
        cancel.set_h_size_flags(SizeFlags::EXPAND.0 + SizeFlags::SHRINK_CENTER.0);
        cancel.set_text("HOTKEY_LISTENER_POPUP_WINDOW_CANCEL");
        cancel
            .connect("pressed", owner, "_on_hide", VariantArray::new_shared(), 0)
            .unwrap();

        confirm_cancel_box.add_child(confirm, false);
        confirm_cancel_box.add_child(cancel, false);

        vbox.add_child(values_label, false);
        vbox.add_child(modifiers_box, false);
        vbox.add_child(confirm_cancel_box, false);

        owner.add_child(vbox, false);

        owner
            .connect(
                "popup_hide",
                owner,
                "_on_hide",
                VariantArray::new_shared(),
                0,
            )
            .unwrap();
    }

    #[method]
    fn _input(&mut self, event: Ref<InputEvent>) {
        let event = unsafe { event.assume_safe() };

        let event = match event.cast::<InputEventKey>() {
            Some(e) => e,
            None => return,
        };

        if event.is_echo() {
            return;
        }

        if event.is_pressed() {
            let label = unsafe { self.values_label.assume_unique() };

            if self.keys_pressed == 0 {
                self.key_names.clear();
                label.set_text("");
            }

            self.keys_pressed += 1;

            match scancode_to_keycode(event.scancode()) {
                Some(k) => match keycode_to_godot_string(&k) {
                    Some(gs) => {
                        self.key_names.push(gs);
                        self.key_names.sort();
                        self.key_names.dedup();

                        label.set_text(GodotString::from_str(
                            &self
                                .key_names
                                .iter()
                                .collect::<VariantArray<Unique>>()
                                .owned_to_variant()
                                .to_string()
                                .as_str(),
                        ));
                    }
                    None => godot_error!("Unknown key {:?}", k.as_str()),
                },
                None => {}
            }
        } else {
            self.keys_pressed -= 1;
        }
    }

    #[method]
    fn _on_modifier_toggled(&mut self, state: bool, name: GodotString) {
        if state {
            self.modifier_names.push(name);
        } else {
            self.modifier_names.retain(|x| x != &name);
        }
    }

    #[method]
    fn _on_hide(&self, #[base] owner: TRef<WindowDialog>) {
        owner.queue_free();
    }

    #[method]
    fn _on_confirm(&self, #[base] owner: TRef<WindowDialog>) {
        let args = VariantArray::new();
        for i in self.key_names.iter() {
            args.push(i);
        }
        for i in self.modifier_names.iter() {
            args.push(i);
        }

        owner.emit_signal(DIALOG_COMPLETE_SIGNAL, &[args.owned_to_variant()]);

        self._on_hide(owner);
    }
}

fn setup_modifier(
    owner: TRef<WindowDialog>,
    check_box: &Ref<CheckBox, Unique>,
    key_code: &KeyCode,
) {
    check_box.set_text(match key_code {
        KeyCode::AltLeft => "Left alt",
        KeyCode::AltRight => "Right alt",
        KeyCode::ControlLeft => "Left control",
        KeyCode::ControlRight => "Right control",
        KeyCode::MetaLeft => "Left super",
        KeyCode::MetaRight => "Right super",
        KeyCode::ShiftLeft => "Left shift",
        KeyCode::ShiftRight => "Right shift",
        _ => unreachable!(),
    });

    let args = VariantArray::new();
    args.push(match keycode_to_godot_string(key_code) {
        Some(gs) => gs,
        None => {
            godot_error!("Unknown key {:?}", key_code.as_str());
            GodotString::from_str("Unknown key")
        }
    });
    check_box
        .connect(
            "toggled",
            owner,
            "_on_modifier_toggled",
            args.into_shared(),
            0,
        )
        .unwrap();
}

macro_rules! generate_kc_sc_mapping {
    ($( $godot:ident: $kc:ident ),+,) => {
        /// There are duplicate keys here since Godot does not distinguish
        /// between left/right alt/control/shift keys
        #[allow(unreachable_patterns)]
        fn scancode_to_keycode(scancode: i64) -> Option<KeyCode> {
            match scancode {
                $( GlobalConstants::$godot => Some(KeyCode::$kc) ),+,
                _ => None
            }
        }

        fn keycode_to_scancode(key_code: &KeyCode) -> Option<i64> {
            match key_code {
                $( KeyCode::$kc => Some(GlobalConstants::$godot) ),+,
                KeyCode::AltLeft => Some(GlobalConstants::KEY_ALT),
                KeyCode::AltRight => Some(GlobalConstants::KEY_ALT),
                KeyCode::ControlLeft => Some(GlobalConstants::KEY_CONTROL),
                KeyCode::ControlRight => Some(GlobalConstants::KEY_CONTROL),
                KeyCode::MetaLeft => Some(GlobalConstants::KEY_META),
                KeyCode::MetaRight => Some(GlobalConstants::KEY_META),
                KeyCode::ShiftLeft => Some(GlobalConstants::KEY_SHIFT),
                KeyCode::ShiftRight => Some(GlobalConstants::KEY_SHIFT),
                _ => None
            }
        }

        fn keycode_to_godot_string(key_code: &KeyCode) -> Option<GodotString> {
            match key_code {
                $( KeyCode::$kc => Some(GodotString::from_str(key_code.as_str())) ),+,
                KeyCode::AltLeft => Some(GodotString::from_str("AltLeft")),
                KeyCode::AltRight => Some(GodotString::from_str("AltRight")),
                KeyCode::ControlLeft => Some(GodotString::from_str("ControlLeft")),
                KeyCode::ControlRight => Some(GodotString::from_str("ControlRight")),
                KeyCode::MetaLeft => Some(GodotString::from_str("SuperLeft")),
                KeyCode::MetaRight => Some(GodotString::from_str("SuperRight")),
                KeyCode::ShiftLeft => Some(GodotString::from_str("ShiftLeft")),
                KeyCode::ShiftRight => Some(GodotString::from_str("ShiftRight")),
                _ => None
            }
        }
    };
}

generate_kc_sc_mapping!(
    // System keys
    KEY_QUOTELEFT: Backquote,
    KEY_BACKSLASH: Backslash,
    KEY_BACKSPACE: Backspace,
    KEY_BRACKETLEFT: BracketLeft,
    KEY_BRACKETRIGHT: BracketRight,
    KEY_COMMA: Comma,
    KEY_0: Digit0,
    KEY_1: Digit1,
    KEY_2: Digit2,
    KEY_3: Digit3,
    KEY_4: Digit4,
    KEY_5: Digit5,
    KEY_6: Digit6,
    KEY_7: Digit7,
    KEY_8: Digit8,
    KEY_9: Digit9,
    KEY_EQUAL: Equal,
    // No IntlBackslash
    // No IntlRo
    KEY_YEN: IntlYen,
    KEY_A: KeyA,
    KEY_B: KeyB,
    KEY_C: KeyC,
    KEY_D: KeyD,
    KEY_E: KeyE,
    KEY_F: KeyF,
    KEY_G: KeyG,
    KEY_H: KeyH,
    KEY_I: KeyI,
    KEY_J: KeyJ,
    KEY_K: KeyK,
    KEY_L: KeyL,
    KEY_M: KeyM,
    KEY_N: KeyN,
    KEY_O: KeyO,
    KEY_P: KeyP,
    KEY_Q: KeyQ,
    KEY_R: KeyR,
    KEY_S: KeyS,
    KEY_T: KeyT,
    KEY_U: KeyU,
    KEY_V: KeyV,
    KEY_W: KeyW,
    KEY_X: KeyX,
    KEY_Y: KeyY,
    KEY_Z: KeyZ,
    KEY_MINUS: Minus,
    KEY_PERIOD: Period,
    KEY_APOSTROPHE: Quote,
    KEY_SEMICOLON: Semicolon,
    KEY_SLASH: Slash,
    //Functional keys
    // KEY_ALT: AltLeft,
    // KEY_ALT: AltRight,
    KEY_CAPSLOCK: CapsLock,
    KEY_MENU: ContextMenu,
    // KEY_CONTROL: ControlLeft,
    // KEY_CONTROL: ControlRight,
    KEY_ENTER: Enter,
    // KEY_META: MetaLeft,
    // KEY_META: MetaRight,
    // KEY_SHIFT: ShiftLeft,
    // KEY_SHIFT: ShiftRight,
    KEY_TAB: Tab,
    KEY_SPACE: Space,
    // Control pad
    KEY_DELETE: Delete,
    KEY_END: End,
    KEY_HELP: Help, // What key is this? :thinking:
    KEY_HOME: Home,
    KEY_INSERT: Insert,
    KEY_PAGEDOWN: PageDown,
    KEY_PAGEUP: PageUp,
    // Arrows
    KEY_DOWN: ArrowDown,
    KEY_UP: ArrowUp,
    KEY_LEFT: ArrowLeft,
    KEY_RIGHT: ArrowRight,
    // Numpad
    KEY_NUMLOCK: NumLock,
    KEY_KP_0: Numpad0,
    KEY_KP_1: Numpad1,
    KEY_KP_2: Numpad2,
    KEY_KP_3: Numpad3,
    KEY_KP_4: Numpad4,
    KEY_KP_5: Numpad5,
    KEY_KP_6: Numpad6,
    KEY_KP_7: Numpad7,
    KEY_KP_8: Numpad8,
    KEY_KP_9: Numpad9,
    KEY_KP_ADD: NumpadAdd,
    KEY_KP_SUBTRACT: NumpadSubtract,
    KEY_KP_MULTIPLY: NumpadMultiply,
    KEY_KP_DIVIDE: NumpadDivide,
    KEY_KP_ENTER: NumpadEnter,
    KEY_KP_PERIOD: NumpadDecimal,
    // Function keys
    KEY_F1: F1,
    KEY_F2: F2,
    KEY_F3: F3,
    KEY_F4: F4,
    KEY_F5: F5,
    KEY_F6: F6,
    KEY_F7: F7,
    KEY_F8: F8,
    KEY_F9: F9,
    KEY_F10: F10,
    KEY_F11: F11,
    KEY_F12: F12,
    KEY_F13: F13,
    KEY_F14: F14,
    KEY_F15: F15,
    KEY_F16: F16,
    // Godot doesn't have F17 - F24
    // No Fn or FnLock
    KEY_PRINT: PrintScreen,
    KEY_SCROLLLOCK: ScrollLock,
    KEY_PAUSE: Pause,
    // Media keys
    KEY_BACK: BrowserBack,
    KEY_FAVORITES: BrowserFavorites,
    KEY_FORWARD: BrowserForward,
    KEY_HOMEPAGE: BrowserHome,
    KEY_REFRESH: BrowserRefresh,
    KEY_SEARCH: BrowserSearch,
    // No Eject
    KEY_LAUNCH1: LaunchApp1,
    KEY_LAUNCH2: LaunchApp2,
    // No KEY_LAUNCH 3 - 9
    // No KEY_LAUNCH A - F
    KEY_LAUNCHMAIL: LaunchMail,
    KEY_MEDIAPLAY: MediaPlayPause, // TODO might be MediaPlay?
    // No MediaSelect
    KEY_MEDIASTOP: MediaStop,
    KEY_MEDIANEXT: MediaTrackNext,
    KEY_MEDIAPREVIOUS: MediaTrackPrevious,
    // No Power, Sleep
    KEY_VOLUMEDOWN: AudioVolumeDown,
    KEY_VOLUMEMUTE: AudioVolumeMute,
    KEY_VOLUMEUP: AudioVolumeUp,
    // No WakeUp
);
