use cgmath::{vec3, Deg, Matrix4, Quaternion, Rotation, Rotation3, Vector3};
use glfw;
use std::path::Path;

mod animation;
mod config;
mod curve;

use crate::scene::SceneObject;
use animation::Animation;
pub use config::Configuration;
use curve::CurveControl;

#[derive(Clone, Debug)]
pub struct ModelPosition {
    pub orientation: Quaternion<f32>,
    pub translation: Vector3<f32>,
    pub scale: f32,
    pub is_selected: bool,
    pub config: Configuration,

    curve: CurveControl,
    animation: Animation,
    debug_pressed: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum Movement {
    ForwardX,
    BackwardX,
    ForwardY,
    BackwardY,
    ForwardZ,
    BackwardZ,
}

impl Default for ModelPosition {
    fn default() -> Self {
        ModelPosition {
            orientation: Quaternion::from_sv(1., vec3(0., 0., 0.)),
            translation: vec3(0., 0., 0.),
            scale: 1.,

            is_selected: false,
            config: Configuration::default(),

            debug_pressed: false,
            curve: CurveControl::default(),
            animation: Animation::default(),
        }
    }
}

impl ModelPosition {
    pub fn with_config<P>(path: P) -> Result<ModelPosition, failure::Error>
    where
        P: AsRef<Path>,
    {
        let mut m = ModelPosition::default();
        m.config = Configuration::from_path(path)?;
        Ok(m)
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        let tmat = Matrix4::from_translation(self.translation);
        let omat = Matrix4::from(self.orientation);
        let smat = Matrix4::from_scale(self.scale);
        tmat * omat * smat
    }

    fn scale_up(&mut self, delta_time: f32) {
        self.scale += self.config.scale_speed * delta_time;
    }

    fn scale_down(&mut self, delta_time: f32) {
        self.scale -= self.config.scale_speed * delta_time;
    }

    fn slide(&mut self, direction: Movement, delta_time: f32) {
        let step = self.config.base_speed * delta_time;
        match direction {
            Movement::ForwardX => self.translation.x += step,
            Movement::BackwardX => self.translation.x -= step,
            Movement::ForwardY => self.translation.y += step,
            Movement::BackwardY => self.translation.y -= step,
            Movement::ForwardZ => self.translation.z += step,
            Movement::BackwardZ => self.translation.z -= step,
        }
        self.curve.reset();
    }

    fn rotate(&mut self, direction: Movement, delta_time: f32) {
        let step = Deg(self.config.rotation_speed * delta_time);
        let rot = match direction {
            Movement::ForwardX => Quaternion::from_angle_x(step),
            Movement::BackwardX => Quaternion::from_angle_x(-step),
            Movement::ForwardY => Quaternion::from_angle_y(step),
            Movement::BackwardY => Quaternion::from_angle_y(-step),
            Movement::ForwardZ => Quaternion::from_angle_z(step),
            Movement::BackwardZ => Quaternion::from_angle_z(-step),
        };
        self.orientation = self.orientation * rot;
    }

    fn rotate_around(&mut self, p: Vector3<f32>, delta_time: f32) {
        let rot = Quaternion::from_angle_y(Deg(self.config.circle_speed * delta_time));
        self.translation = rot * (self.translation - p) + p;
    }

    fn look_at(&mut self, p: Vector3<f32>, up: Vector3<f32>, delta_time: f32) {
        // Base case to avoid NaN
        if p.x == 0. && p.y == 0. && p.z == 0. {
            return;
        }
        let rot = Quaternion::look_at(p, up);
        self.orientation = self.orientation.nlerp(rot, delta_time);
    }

    fn slide_curve(&mut self, direction: Movement, delta_time: f32) {
        self.translation = self.curve.slide(self.translation, direction, delta_time);
    }
}

impl SceneObject for ModelPosition {
    fn process_input(&mut self, window: &glfw::Window, delta_time: f32) {
        if self.animation.is_running {
            unimplemented!("TODO: FIXME: Handle animations");
        }

        if !self.is_selected {
            return;
        }

        process_keys!(
        window;
        glfw::Key::W, glfw::Action::Press =>
                self.slide(Movement::ForwardZ, delta_time),
                self.slide_curve(Movement::ForwardZ, delta_time),
        glfw::Key::A, glfw::Action::Press =>
                self.slide(Movement::BackwardX, delta_time),
                self.slide_curve(Movement::BackwardX, delta_time),
        glfw::Key::S, glfw::Action::Press =>
                self.slide(Movement::BackwardZ, delta_time),
                self.slide_curve(Movement::BackwardZ, delta_time),
        glfw::Key::D, glfw::Action::Press =>
                self.slide(Movement::ForwardX, delta_time),
                self.slide_curve(Movement::ForwardX, delta_time),
        glfw::Key::Q, glfw::Action::Press =>
                self.slide(Movement::ForwardY, delta_time),
                self.slide_curve(Movement::ForwardY, delta_time),
        glfw::Key::E, glfw::Action::Press =>
                self.slide(Movement::BackwardY, delta_time),
                self.slide_curve(Movement::BackwardY, delta_time),
        glfw::Key::R, glfw::Action::Press =>
                self.scale_up(delta_time),
                self.scale_down(delta_time),
        glfw::Key::V, glfw::Action::Press =>
                self.rotate_around(vec3(0.,0.,0.), delta_time),
                self.rotate_around(vec3(5.,0.,0.), delta_time),
        glfw::Key::Z, glfw::Action::Press =>
                self.rotate(Movement::ForwardZ, delta_time),
                self.rotate(Movement::BackwardZ, delta_time),
        glfw::Key::X, glfw::Action::Press =>
                self.rotate(Movement::ForwardX, delta_time),
                self.rotate(Movement::BackwardX, delta_time),
        glfw::Key::C, glfw::Action::Press =>
                self.rotate(Movement::ForwardY, delta_time),
                self.rotate(Movement::BackwardY, delta_time)
        );

        process_keys!(
        window;
        glfw::Key::F, glfw::Action::Release => self.debug_pressed = false,
        glfw::Key::F, glfw::Action::Press => {
            if self.debug_pressed == false {
                self.debug_pressed = true;
                println!("Model_pos: {:#?}", self);
                println!("Delta time: {:#?}", delta_time);
            }
        });
    }
}
