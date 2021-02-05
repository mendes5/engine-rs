use crate::events::VirtualKeyCode;
use crate::input::keyboard::KeyboardState;
use renderer::Camera;

pub struct FPSControls {
    pub camera: Camera,
}

impl FPSControls {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
        }
    }

    pub fn read_last_keyboard(&mut self, keyboard_state: &KeyboardState, speed: f32) {
        if keyboard_state.get_key_pressed(&VirtualKeyCode::W) {
            self.camera.position.z -= self.camera.forward.z * speed * self.camera.speed;
            self.camera.position.x -= self.camera.forward.x * speed * self.camera.speed;
        }
        if keyboard_state.get_key_pressed(&VirtualKeyCode::S) {
            self.camera.position.z += self.camera.forward.z * speed * self.camera.speed;
            self.camera.position.x += self.camera.forward.x * speed * self.camera.speed;
        }
        if keyboard_state.get_key_pressed(&VirtualKeyCode::A) {
            self.camera.position.z += self.camera.right.z * speed * self.camera.speed;
            self.camera.position.x += self.camera.right.x * speed * self.camera.speed;
        }
        if keyboard_state.get_key_pressed(&VirtualKeyCode::D) {
            self.camera.position.z -= self.camera.right.z * speed * self.camera.speed;
            self.camera.position.x -= self.camera.right.x * speed * self.camera.speed;
        }
        if keyboard_state.get_key_pressed(&VirtualKeyCode::R) {
            self.camera.position.y += speed * self.camera.speed;
        }
        if keyboard_state.get_key_pressed(&VirtualKeyCode::F) {
            self.camera.position.y -= speed * self.camera.speed;
        }

        self.camera
            .look_at
            .add_vectors(&self.camera.pointing, &self.camera.position);
    }
}
