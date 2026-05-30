use crate::input::Device;
use crate::input::Key;
use std::collections::HashMap;

pub struct Keymap {
    device: Device,
    map: HashMap<Key, Key>,
}
