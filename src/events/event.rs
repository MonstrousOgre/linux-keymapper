use crate::input;

use input::Device;
use input::Key;

pub struct Event {
    pub device: Device,
    pub key: Key,
    evdev_result: (evdev_rs::ReadStatus, evdev_rs::InputEvent),
}
