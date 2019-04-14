#![allow(dead_code)]

#[macro_use]
mod macros;

pub(crate) mod camera;
pub(crate) mod mesh;
pub(crate) mod model;
pub(crate) mod model_pos;
pub(crate) mod scene;
pub(crate) mod shader;

pub use scene::Scene;
