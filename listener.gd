extends Node

var logger := Logger.new(tr("HOTKEY_EXTENSION_NAME"))

# Binary pid
var pid: int = 0

#-----------------------------------------------------------------------------#
# Builtin functions                                                           #
#-----------------------------------------------------------------------------#

func _init(p_port: int) -> void:
	# TODO change this once translations are in place
	var res: Result = Safely.wrap(AM.em.get_extension("HotkeyListener"))
	if res.is_err():
		logger.error("Failed to get extension resource")
		return

	var extension: Extension = res.unwrap()
	
	pid = OS.execute(
		"%s/bin/input-forwarder.exe" % extension.context,
		["websocket", "9999"],
		false,
		[],
		false,
		true
	)
	if pid < 0:
		logger.error("Failed to execute input-forwarder")
		return

func _exit_tree() -> void:
	OS.kill(pid)

#-----------------------------------------------------------------------------#
# Connections                                                                 #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Private functions                                                           #
#-----------------------------------------------------------------------------#

#-----------------------------------------------------------------------------#
# Public functions                                                            #
#-----------------------------------------------------------------------------#
