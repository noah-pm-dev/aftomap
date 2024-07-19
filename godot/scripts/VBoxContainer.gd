extends VBoxContainer

func _can_drop_data(at_position, data):
	return true

func _drop_data(at_position, data):
	data.get_parent().remove_child(data)
	add_child(data)
