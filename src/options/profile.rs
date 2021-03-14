use super::keymap::Keymap;
use crate::input::Device;

pub struct Profile {
    device: Device,
    key_map: Keymap,
}
