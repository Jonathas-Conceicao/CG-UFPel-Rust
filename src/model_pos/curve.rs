use super::Movement;

use cgmath::{vec3, Vector3};
use splines::{Interpolation, Key, Spline};

const TIME: f32 = 2.;
const MAIN_DEVIATION: f32 = 8.;
const AUX_DEVIATION: f32 = 4.;

#[derive(Clone, Debug)]
pub(super) struct CurveControl {
    step: f32,
    should_reset: bool,
    direction: Movement,
    spline: Spline<Vector3<f32>>,
}

impl Default for CurveControl {
    fn default() -> Self {
        Self {
            step: TIME,
            should_reset: true,
            direction: Movement::ForwardX,
            spline: Spline::from_vec(Vec::default()),
        }
    }
}

impl CurveControl {
    pub(super) fn slide(
        &mut self,
        p0: Vector3<f32>,
        direction: Movement,
        delta_time: f32,
    ) -> Vector3<f32> {
        if self.should_reset || self.step >= TIME || self.direction != direction {
            self.new_spline(&p0, direction);
        }

        self.step += delta_time;
        self.spline.sample(self.step).unwrap_or(p0)
    }

    pub(super) fn reset(&mut self) {
        self.should_reset = true;
    }

    fn new_spline(&mut self, p0: &Vector3<f32>, direction: Movement) {
        let p1 = match direction {
            Movement::ForwardX => vec3(p0.x + MAIN_DEVIATION * 0.333, p0.y + AUX_DEVIATION, p0.z),
            Movement::BackwardX => vec3(p0.x - MAIN_DEVIATION * 0.333, p0.y + AUX_DEVIATION, p0.z),
            Movement::ForwardY => vec3(p0.x + AUX_DEVIATION, p0.y + MAIN_DEVIATION * 0.333, p0.z),
            Movement::BackwardY => vec3(p0.x + AUX_DEVIATION, p0.y - MAIN_DEVIATION * 0.333, p0.z),
            Movement::ForwardZ => vec3(p0.x, p0.y + AUX_DEVIATION, p0.z + MAIN_DEVIATION * 0.333),
            Movement::BackwardZ => vec3(p0.x, p0.y + AUX_DEVIATION, p0.z - MAIN_DEVIATION * 0.333),
        };
        let p2 = match direction {
            Movement::ForwardX => vec3(p0.x + MAIN_DEVIATION * 0.666, p0.y - AUX_DEVIATION, p0.z),
            Movement::BackwardX => vec3(p0.x - MAIN_DEVIATION * 0.666, p0.y - AUX_DEVIATION, p0.z),
            Movement::ForwardY => vec3(p0.x - AUX_DEVIATION, p0.y + MAIN_DEVIATION * 0.666, p0.z),
            Movement::BackwardY => vec3(p0.x - AUX_DEVIATION, p0.y - MAIN_DEVIATION * 0.666, p0.z),
            Movement::ForwardZ => vec3(p0.x, p0.y - AUX_DEVIATION, p0.z + MAIN_DEVIATION * 0.666),
            Movement::BackwardZ => vec3(p0.x, p0.y - AUX_DEVIATION, p0.z - MAIN_DEVIATION * 0.666),
        };
        let p3 = match direction {
            Movement::ForwardX => vec3(p0.x + MAIN_DEVIATION, p0.y, p0.z),
            Movement::BackwardX => vec3(p0.x - MAIN_DEVIATION, p0.y, p0.z),
            Movement::ForwardY => vec3(p0.x, p0.y + MAIN_DEVIATION, p0.z),
            Movement::BackwardY => vec3(p0.x, p0.y - MAIN_DEVIATION, p0.z),
            Movement::ForwardZ => vec3(p0.x, p0.y, p0.z + MAIN_DEVIATION),
            Movement::BackwardZ => vec3(p0.x, p0.y, p0.z - MAIN_DEVIATION),
        };

        self.step = 0.;
        self.should_reset = false;
        self.direction = direction;
        self.spline = Spline::from_vec(vec![
            Key::new(-99.9, *p0, Interpolation::CatmullRom),
            Key::new(0., *p0, Interpolation::CatmullRom),
            Key::new(TIME * 0.333, p1, Interpolation::CatmullRom),
            Key::new(TIME * 0.666, p2, Interpolation::CatmullRom),
            Key::new(TIME, p3, Interpolation::CatmullRom),
            Key::new(99.9, p3, Interpolation::CatmullRom),
        ]);
    }
}
