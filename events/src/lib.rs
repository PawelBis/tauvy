use bevy_window::WindowResolution;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum WindowUpdated {
    Moved { x: i32, y: i32 },
    Resized { width: u32, height: u32 },
    Closed,
}

impl From<WindowResolution> for WindowUpdated {
    fn from(value: WindowResolution) -> Self {
        Self::Resized {
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
