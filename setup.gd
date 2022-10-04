func execute() -> int:
	print("Starting setup for HotkeyListener")

	var res: Result = Safely.wrap(
		AM.em.load_gdnative_resource(
			"HotkeyListener", "InputForwarderLib", "InputForwarder"))
	if res.is_err():
		return ERR_BUG
	
	var lib: Node = res.unwrap()
	lib.name = tr("HOTKEY_LISTENER_EXTENSION_NAME")
	AM.add_child(lib)

	print("Finished setup for HotkeyListener")
	return OK
