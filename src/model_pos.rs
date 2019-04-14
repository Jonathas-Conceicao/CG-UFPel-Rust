use crate::scene::SceneObject;

use cgmath::{vec3, Matrix4, Vector3};
use glfw;

const BASE_SPEED: f32 = 1.0;

#[derive(Clone)]
pub struct ModelPosition {
    pub pos: Vector3<f32>,
    pub is_selected: bool,
}

impl ModelPosition {
    pub fn matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.pos)
    }
}

impl Default for ModelPosition {
    fn default() -> Self {
        ModelPosition {
            pos: vec3(0.0, 0.0, 0.0),
            is_selected: false,
        }
    }
}

impl SceneObject for ModelPosition {
    fn process_input(&mut self, window: &glfw::Window, delta_time: f32) {
        if !self.is_selected {
            return;
        }

        process_keys!(window;
        glfw::Key::W, glfw::Action::Press => {
            self.pos.y += BASE_SPEED * delta_time;
        },
        glfw::Key::S, glfw::Action::Press => {
            self.pos.y -= BASE_SPEED * delta_time;
        },
        glfw::Key::A, glfw::Action::Press => {
            self.pos.x -= BASE_SPEED * delta_time;
        },
        glfw::Key::D, glfw::Action::Press => {
            self.pos.x += BASE_SPEED * delta_time;
        });
    }
}
