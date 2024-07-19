extends Button

@onready var macro_creator = $"../MacroCreator"


func _on_pressed():
	macro_creator.show()
