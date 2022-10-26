# Hotkey Listener
Uses [livesplit-hotkey](https://github.com/LiveSplit/livesplit-core) to listen for sequences of
that can then trigger an action.

Multiple actions can be assigned to the same sequence of keys. The same action can be assigned
to many sequences of keys.

## Building

By default, this library is built as a GDNative library.

In the `rust/` directory, run the following command:

```Bash
cargo build --release --lib
```

If building for [vpuppr](https://github.com/virtual-puppet-project/vpuppr),
take the resulting binary and place it in the `hotkey-listener/` directory.
Then take the entire project (except for the `rust/` directory) and place it under
vpuppr's `resources/extensions/` directory.
