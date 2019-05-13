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
use serde::Deserialize;

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
pub enum Movement {
    ForwardX,
    BackwardX,
    ForwardY,
    BackwardY,
    ForwardZ,
    BackwardZ,
}

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum Command {
    ScaleU,
    ScaleD,

    SlideXF,
    SlideXB,
    SlideYF,
    SlideYB,
    SlideZF,
    SlideZB,

    CurveXF,
    CurveXB,
    CurveYF,
    CurveYB,
    CurveZF,
    CurveZB,

    RotateXF,
    RotateXB,
    RotateYF,
    RotateYB,
    RotateZF,
    RotateZB,
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

    pub fn scale_up(&mut self, delta_time: f32) {
        self.scale += self.config.scale_speed * delta_time;
    }

    pub fn scale_down(&mut self, delta_time: f32) {
        self.scale -= self.config.scale_speed * delta_time;
    }

    pub fn slide(&mut self, direction: Movement, delta_time: f32) {
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

    pub fn rotate(&mut self, direction: Movement, delta_time: f32) {
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

    pub fn rotate_around(&mut self, direction: Movement, p: Vector3<f32>, delta_time: f32) {
        let step = Deg(self.config.circle_speed * delta_time);
        let rot = match direction {
            Movement::ForwardX => Quaternion::from_angle_x(step),
            Movement::BackwardX => Quaternion::from_angle_x(-step),
            Movement::ForwardY => Quaternion::from_angle_y(step),
            Movement::BackwardY => Quaternion::from_angle_y(-step),
            Movement::ForwardZ => Quaternion::from_angle_z(step),
            Movement::BackwardZ => Quaternion::from_angle_z(-step),
        };
        self.translation = rot * (self.translation - p) + p;
        self.curve.reset();
    }

    pub fn look_at(&mut self, p: Vector3<f32>, up: Vector3<f32>, delta_time: f32) {
        // Quaternion::look_at currenty used rh system;
        // https://github.com/rustgd/cgmath/issues/448
        // so we need to correct the orientation with two steps
        //   (1) invert the look_at direction;
        //   (2) invert the resulting quaternion vector;
        let dir = p - self.translation;
        let rot = Quaternion::look_at(-dir, up);
        let rot = Quaternion::from_sv(rot.s, -rot.v);

        // delta_time is not currently used because we want the look at to be abrupt
        let _ = delta_time;

        self.orientation = rot;
    }

    pub fn slide_curve(&mut self, direction: Movement, delta_time: f32) {
        self.translation = self.curve.slide(self.translation, direction, delta_time);
    }

    pub fn run_command(&mut self, c: Command, delta_time: f32) {
        match c {
            Command::ScaleU => self.scale_up(delta_time),
            Command::ScaleD => self.scale_down(delta_time),

            Command::SlideXF => self.slide(Movement::ForwardX, delta_time),
            Command::SlideXB => self.slide(Movement::BackwardX, delta_time),
            Command::SlideYF => self.slide(Movement::ForwardY, delta_time),
            Command::SlideYB => self.slide(Movement::BackwardY, delta_time),
            Command::SlideZF => self.slide(Movement::ForwardZ, delta_time),
            Command::SlideZB => self.slide(Movement::BackwardZ, delta_time),

            Command::CurveXF => self.slide_curve(Movement::ForwardX, delta_time),
            Command::CurveXB => self.slide_curve(Movement::BackwardX, delta_time),
            Command::CurveYF => self.slide_curve(Movement::ForwardY, delta_time),
            Command::CurveYB => self.slide_curve(Movement::BackwardY, delta_time),
            Command::CurveZF => self.slide_curve(Movement::ForwardZ, delta_time),
            Command::CurveZB => self.slide_curve(Movement::BackwardZ, delta_time),

            Command::RotateXF => self.rotate(Movement::ForwardX, delta_time),
            Command::RotateXB => self.rotate(Movement::BackwardX, delta_time),
            Command::RotateYF => self.rotate(Movement::ForwardY, delta_time),
            Command::RotateYB => self.rotate(Movement::BackwardY, delta_time),
            Command::RotateZF => self.rotate(Movement::ForwardZ, delta_time),
            Command::RotateZB => self.rotate(Movement::BackwardZ, delta_time),
        };
    }
}

impl SceneObject for ModelPosition {
    fn process_input(&mut self, window: &glfw::Window, delta_time: f32) {
        if self.animation.is_running {
            for (c, t) in self.animation.step(delta_time) {
                self.run_command(c, t);
            }
            return;
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
        glfw::Key::G, glfw::Action::Press =>
                self.look_at(vec3(0., 0., 0.), vec3(0., 1., 0.), delta_time),
                self.look_at(vec3(0., 2., 0.), vec3(0., 1., 0.), delta_time),
        glfw::Key::V, glfw::Action::Press =>
                self.rotate_around(Movement::ForwardY, vec3(0., 0., 0.), delta_time),
                self.rotate_around(Movement::BackwardY, vec3(0., 0., 0.), delta_time),
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
        glfw::Key::H, glfw::Action::Press => self.animation.start(self.config.command_list.clone()),
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
