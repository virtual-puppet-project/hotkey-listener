use gdnative::{
    api::{
        control::{LayoutPreset, SizeFlags},
        label::{Align, VAlign},
        GlobalConstants, HBoxContainer, VBoxContainer, WindowDialog,
    },
    core_types::Margin,
    prelude::*,
};
use livesplit_hotkey::KeyCode;

const DIALOG_COMPLETE_SIGNAL: &str = "dialog_complete";

const UNKNOWN_SCANCODE: &str = "Unknown";

#[derive(NativeClass, Default)]
#[inherit(WindowDialog)]
#[register_with(Self::register_signals)]
pub struct HotkeyListenerPopup {
    keys_pressed: u16,
    key_names: Vec<String>,
}

#[methods]
impl HotkeyListenerPopup {
    fn new(_o: &WindowDialog) -> Self {
        HotkeyListenerPopup::default()
    }

    fn register_signals(build: &ClassBuilder<Self>) {
        build.signal(DIALOG_COMPLETE_SIGNAL).done()
    }

    #[method]
    fn _ready(&self, #[base] owner: TRef<WindowDialog>) {
        owner.set_resizable(true);
        owner.set_title("HOTKEY_LISTENER_POPUP_WINDOW_TITLE");

        let vbox = VBoxContainer::new();
        vbox.set_margin(Margin::Left.into(), 10.0);
        vbox.set_margin(Margin::Top.into(), 10.0);
        vbox.set_margin(Margin::Right.into(), -10.0);
        vbox.set_margin(Margin::Bottom.into(), -10.0);
        vbox.set_anchors_preset(LayoutPreset::WIDE.0, false);

        let values_label = Label::new();
        values_label.set_align(Align::CENTER.into());
        values_label.set_valign(VAlign::CENTER.into());
        values_label.set_h_size_flags(SizeFlags::EXPAND_FILL.0);
        values_label.set_v_size_flags(SizeFlags::EXPAND_FILL.0);

        let hbox = HBoxContainer::new();

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

        hbox.add_child(confirm, false);
        hbox.add_child(cancel, false);

        vbox.add_child(values_label, false);
        vbox.add_child(hbox, false);

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

        if event.is_pressed() {
            if self.keys_pressed == 0 {
                self.key_names.clear();
            }

            self.keys_pressed += 1;

            self.key_names
                .push(scancode_to_keycode_string(event.scancode()));
        } else {
            self.keys_pressed -= 1;
        }
    }

    #[method]
    fn _on_hide(&self, #[base] owner: TRef<WindowDialog>) {
        owner.queue_free();
    }

    #[method]
    fn _on_confirm(&self, #[base] owner: TRef<WindowDialog>) {
        owner.emit_signal(
            DIALOG_COMPLETE_SIGNAL,
            &[self
                .key_names
                .clone()
                .into_iter()
                .map(|x| GodotString::from_str(x.as_str()))
                .collect::<VariantArray<Unique>>()
                .owned_to_variant()],
        );
        self._on_hide(owner);
    }
}

macro_rules! sc_kc {
    ($val:ident{ $( $godot:ident : $kc:ident ),*}) => {
        match $val {
            $( GlobalConstants::$godot => KeyCode::$kc.as_str().to_string() ),*,
            _ => UNKNOWN_SCANCODE.to_string()
        }
    };
}

fn scancode_to_keycode_string(scancode: i64) -> String {
    sc_kc!(scancode {
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
        // No IntlBackslash in Godot
        // No IntlRo in Godot
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
        KEY_ALT: AltLeft
    })
}
