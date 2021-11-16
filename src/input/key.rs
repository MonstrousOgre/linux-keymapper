use evdev_rs::enums::*;

pub struct Key {}

impl Key {
    pub fn int_to_key(code: u32) -> EV_KEY {
        int_to_ev_key(code).unwrap()
    }
}
