use godot::prelude::*;
// use godot::classes::Sprite2D;
// use godot::classes::ISprite2D;
use mouse_rs::{types::keys::Keys, Mouse};
use godot::classes::Button;
use godot::classes::IButton;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

fn move_and_press() {
    let mouse = Mouse::new();
    mouse.move_to(500, 500).expect("Unable to move mouse");
    mouse.press(&Keys::RIGHT).expect("Unable to press button");
    mouse.release(&Keys::RIGHT).expect("Unable to release button");
}

#[derive(GodotClass)]
#[class(base=Button)]
struct MacroButton {
    base: Base<Button>
}

#[godot_api]
impl IButton for MacroButton {
    fn init(base: Base<Button>) -> Self {
        godot_print!("Hello, world!");

        Self {
            base
        }
    }
}

#[godot_api]
impl MacroButton {
    #[func]
    fn run_macro() {
        move_and_press()
        
    }
}

// #[derive(GodotClass)]
// #[class(base=Sprite2D)]
// struct Player {
//     speed: f64,
//     angular_speed: f64,

//     base: Base<Sprite2D>
// }

// #[godot_api]
// impl ISprite2D for Player {
//     fn init(base: Base<Sprite2D>) -> Self {
//         godot_print!("Hello, world!"); // Prints to the Godot console
        
//         Self {
//             speed: 400.0,
//             angular_speed: std::f64::consts::PI,
//             base,
//         }
//     }

//     fn physics_process(&mut self, delta: f64) {
//         // In GDScript, this would be: 
//         // rotation += angular_speed * delta
        
//         let radians = (self.angular_speed * delta) as f32;
//         self.base_mut().rotate(radians);
//         // The 'rotate' method requires a f32, 
//         // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
//     }
// }