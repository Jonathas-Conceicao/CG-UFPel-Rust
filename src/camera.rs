use crate::{
    model_pos::{ModelPosition, Movement},
    scene::SceneObject,
};

use cgmath::{self, vec3};

type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;
type Quaternion = cgmath::Quaternion<f32>;

const SENSITIVITY: f32 = 0.005;
const ZOOM: f32 = 45.;
const FRONT_BASE: Vector3 = Vector3 {
    x: 0.,
    y: 0.,
    z: 1.,
};
const WORLD_UP: Vector3 = Vector3 {
    x: 0.,
    y: 1.,
    z: 0.,
};

#[derive(Debug)]
pub struct Camera {
    pub zoom: f32,
    pub sensitivity: f32,
    pub model_pos: ModelPosition,
    pub debug_pressed: bool,
}

impl Default for Camera {
    fn default() -> Camera {
        let camera = Camera {
            zoom: ZOOM,
            sensitivity: SENSITIVITY,
            model_pos: ModelPosition::default(),
            debug_pressed: false,
        };
        camera
    }
}

impl SceneObject for Camera {
    fn process_input(&mut self, window: &glfw::Window, delta_time: f32) {
        let point = vec3(0., 0., 0.);

        // // Control camera as model;
        // self.model_pos.is_selected = true;
        // self.model_pos.process_input(window, delta_time);

        process_keys!(
        window;
        glfw::Key::Up, glfw::Action::Press =>
            self.model_pos.rotate_around(Movement::BackwardX, point, delta_time),
            self.model_pos.slide(Movement::BackwardZ, delta_time),
        glfw::Key::Down, glfw::Action::Press =>
            self.model_pos.rotate_around(Movement::ForwardX, point, delta_time),
            self.model_pos.slide(Movement::ForwardZ, delta_time),
        glfw::Key::Left, glfw::Action::Press =>
            self.model_pos.rotate_around(Movement::BackwardY, point, delta_time),
            self.model_pos.slide(Movement::BackwardX, delta_time),
        glfw::Key::Right, glfw::Action::Press =>
            self.model_pos.rotate_around(Movement::ForwardY, point, delta_time),
            self.model_pos.slide(Movement::ForwardX, delta_time)
        );

        process_keys!(
        window;
        glfw::Key::K, glfw::Action::Press =>
            self.model_pos.look_at(point, WORLD_UP, delta_time),
        glfw::Key::J, glfw::Action::Release => self.debug_pressed = false,
        glfw::Key::J, glfw::Action::Press => {
            if self.debug_pressed == false {
                self.debug_pressed = true;
                println!("Model_pos: {:#?}", self);
                println!("Delta time: {:#?}", delta_time);
            }
        });
    }
}

impl Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt
    /// Matrix
    pub fn get_view_matrix(&self) -> Matrix4 {
        let rmat = Matrix4::from(self.model_pos.orientation);
        let tmat = Matrix4::from_translation(-self.model_pos.translation);
        rmat * tmat
    }

    pub fn process_mouse_movement(&mut self, xoffset: f32, yoffset: f32, constrain_pitch: bool) {
        self.model_pos
            .rotate(Movement::ForwardX, -yoffset * self.sensitivity);
        self.model_pos
            .rotate(Movement::ForwardY, xoffset * self.sensitivity);

        // Ensure z orientation dones't get messedup by normalization error;
        self.model_pos.orientation.v.z = 0.;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrain_pitch {
            // FIXME constrain quaternion to 90º
            // if self.pitch > 89. {
            //     self.pitch = 89.;
            // }
            // if self.pitch < -89. {
            //     self.pitch = -89.;
            // }
        }
    }

    // Processes input received from a mouse scroll-wheel event. Only requires input
    // on the vertical wheel-axis
    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1. && self.zoom <= 45. {
            self.zoom -= yoffset;
        }
        if self.zoom <= 1. {
            self.zoom = 1.;
        }
        if self.zoom >= 45. {
            self.zoom = 45.;
        }
    }
}
