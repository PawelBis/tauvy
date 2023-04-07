use bevy_window::WindowResolution;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct WindowUpdated {
    pub width: u32,
    pub height: u32,
}

impl From<WindowResolution> for WindowUpdated {
    fn from(value: WindowResolution) -> Self {
        Self {
            width: value.width() as u32,
            height: value.height() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
