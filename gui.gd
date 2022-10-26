extends PanelContainer

var logger := Logger.new("HotkeyListenGui")

func _init() -> void:
	var sc := ScrollContainer.new()
	ControlUtil.all_expand_fill(sc)

	add_child(sc)

	var vb := VBoxContainer.new()
	ControlUtil.h_expand_fill(vb)

	sc.add_child(vb)

	var usage := Label.new()
	ControlUtil.h_expand_fill(usage)
	usage.text = tr("HOTKEY_LISTENER_USAGE_TEXT")
	usage.autowrap = true

	vb.add_child(usage)

	var le := LineEdit.new()
	ControlUtil.h_expand_fill(le)
	le.editable = false

	AM.get_node(tr("HOTKEY_LISTENER_EXTENSION_NAME")).connect("key_received", self, "_on_key_received", [le])

	vb.add_child(le)

func _on_key_received(text: String, le: LineEdit) -> void:
	le.text = text
