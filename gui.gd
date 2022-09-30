extends PanelContainer

var logger := Logger.new("HotkeyListenGui")

func _init() -> void:
	var sc := ScrollContainer.new()
	ControlUtil.all_expand_fill(sc)

	add_child(sc)

	var vb := VBoxContainer.new()
	ControlUtil.h_expand_fill(vb)

	sc.add_child(vb)

	var button := Button.new()
	button.text = "start"

	button.connect("pressed", self, "_on_pressed")

	vb.add_child(button)

func _on_pressed() -> void:
	var res: Result = Safely.wrap(AM.em.load_resource("HotkeyListener", "listener.gd"))
	if res.is_err():
		logger.error("Unable to load Listener")
		return

	var listener: Node = res.unwrap().new(9999)

	get_tree().root.add_child(listener)
