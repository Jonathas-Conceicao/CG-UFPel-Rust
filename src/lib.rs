#[macro_use]
mod macros;

pub mod camera;
pub mod common;
pub mod mesh;
pub mod model;
pub mod scene;
pub mod shader;

pub use scene::run;
