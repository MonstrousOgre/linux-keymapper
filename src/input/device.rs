use evdev_rs::UInputDevice;
use std::fs::File;
use std::path::Path;

pub struct Device {
    evdev_device: evdev_rs::Device,
    //udev_device: libudev::Device,
    grabbed: bool,
    uinput_device: Option<evdev_rs::UInputDevice>,
}

impl Device {
    pub fn new() {
        unimplemented!();
    }

    fn new_from_devnode(devnode: &Path) -> Device {
        let f = File::open(devnode).unwrap();
        let d = evdev_rs::Device::new_from_fd(f).unwrap();
        Device {
            evdev_device: d,
            grabbed: false,
            uinput_device: None,
        }
    }

    pub fn get_all_devices() -> Vec<Device> {
        let context = libudev::Context::new().unwrap();
        let mut enumerator = libudev::Enumerator::new(&context).unwrap();
        enumerator.match_property("ID_INPUT_KEYBOARD", "1").ok();

        //enumerator.match_subsystem("tty").unwrap();

        let mut devices = Vec::new();

        for ud in enumerator.scan_devices().unwrap() {
            if let Some(devnode) = ud.devnode() {
                devices.push(Self::new_from_devnode(devnode));
            }
        }

        devices
    }
}

impl Device {
    pub fn is_grabbed(&self) -> bool {
        return self.grabbed;
    }

    fn grab(&mut self) {
        let grab_result = self.evdev_device.grab(evdev_rs::GrabMode::Grab);

        match grab_result {
            Ok(_k) => {
                self.grabbed = true;
                self.uinput_device =
                    Some(UInputDevice::create_from_device(&self.evdev_device).unwrap());
            }
            Err(_e) => {
                self.grabbed = false;
                self.uinput_device = None
            }
        }
    }

    fn ungrab(&mut self) {
        let ungrab_result = self.evdev_device.grab(evdev_rs::GrabMode::Ungrab);

        match ungrab_result {
            Ok(_k) => {
                self.grabbed = false;
                self.uinput_device = None;
            }
            Err(_e) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_all_devices() {
        let devices = Device::get_all_devices();
        assert_ne!(devices.len(), 0)
    }

    #[test]
    fn grab() {
        let mut device = Device::new_from_devnode(Path::new("/dev/input/event26"));

        device.grab();

        assert!(device.is_grabbed());
        assert!(device.uinput_device.is_some());
    }
}
