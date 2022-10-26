use crossbeam_channel::unbounded;
use hotkey_listener::hotkey_listener;

fn main() {
    let (s, r) = unbounded::<String>();

    let mut listener = hotkey_listener::HotkeyListener::new(s).unwrap();

    listener
        .register_action(
            &"APressed".to_string(),
            &["B".to_string(), "ControlLeft".to_string()],
        )
        .unwrap();
    listener
        .register_action(
            &"Test".to_string(),
            &["A".to_string(), "ControlLeft".to_string()],
        )
        .unwrap();
    listener
        .register_action(
            &"Test".to_string(),
            &["B".to_string(), "ControlLeft".to_string()],
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
