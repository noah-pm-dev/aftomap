extends Button


func _get_drag_data(at_position):
	var data = 1
	var prev = TextureRect.new()
	prev.texture = icon
	set_drag_preview(prev)
	return self
