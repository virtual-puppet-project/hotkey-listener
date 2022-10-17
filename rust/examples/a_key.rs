use crossbeam_channel::bounded;
use hotkey_listener::hotkey_listener;

fn main() {
    let (s, r) = bounded::<String>(1);

    let mut listener = hotkey_listener::HotkeyListener::new(s).unwrap();

    listener
        .register_hotkey(
            &"APressed".to_string(),
            &["B".to_string(), "ControlLeft".to_string()],
        )
        .unwrap();
    listener
        .register_hotkey(
            &"Test".to_string(),
            &["A".to_string(), "ControlLeft".to_string()],
        )
        .unwrap();
    listener.set_min_elapsed_time(0.2);

    loop {
        listener.poll();

        if r.is_empty() {
            continue;
        }
        match r.recv() {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}
