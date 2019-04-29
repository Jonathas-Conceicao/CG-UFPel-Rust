use failure::format_err;
use serde::Deserialize;
use serde_json;
use std::{fs, io::Read, path::Path};

const BASE_SPEED: f32 = 8.;
const ROTATION_SPEED: f32 = 30.;
const CIRCLE_SPEED: f32 = 60.;
const SCALE_SPEED: f32 = 2.;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Configuration {
    pub base_speed: f32,
    pub rotation_speed: f32,
    pub circle_speed: f32,
    pub scale_speed: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_speed: BASE_SPEED,
            rotation_speed: ROTATION_SPEED,
            circle_speed: CIRCLE_SPEED,
            scale_speed: SCALE_SPEED,
        }
    }
}

impl Configuration {
    pub fn from_path<P>(path: P) -> Result<Configuration, failure::Error>
    where
        P: AsRef<Path>,
    {
        let mut content = String::default();
        fs::File::open(path)?.read_to_string(&mut content)?;
        serde_json::from_str(&content)
            .map_err(|e| format_err!("Failed to read config file: {:#?}", e))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn deserialize() {
        assert_eq!(
            Configuration {
                base_speed: 4.,
                rotation_speed: 15.,
                circle_speed: 30.,
                scale_speed: 2.,
            },
            serde_json::from_value(json!({
                "base_speed": 4.0,
                "rotation_speed": 15.0,
                "circle_speed": 30.0,
                "scale_speed": 2.0,
            }))
            .unwrap()
        );
    }
}
