use evdev_rs::UInputDevice;
use std::path::Path;
use std::{error::Error, fs::File};

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

    fn new_from_devnode(devnode: &Path) -> Result<Device, Box<dyn Error>> {
        let f = File::open(devnode)?;
        let d = evdev_rs::Device::new_from_fd(f)?;
        Ok(Device {
            evdev_device: d,
            grabbed: false,
            uinput_device: None,
        })
    }

    pub fn get_all_devices() -> Result<Vec<Device>, Box<dyn Error>> {
        let context = libudev::Context::new()?;
        let mut enumerator = libudev::Enumerator::new(&context)?;
        enumerator.match_property("ID_INPUT_KEYBOARD", "1").ok();

        //enumerator.match_subsystem("tty")?;

        let mut devices = Vec::new();

        for ud in enumerator.scan_devices()? {
            if let Some(devnode) = ud.devnode() {
                devices.push(Self::new_from_devnode(devnode)?);
            }
        }

        Ok(devices)
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
        let result = Device::get_all_devices();
        if let Ok(devices) = result {
            assert_ne!(devices.len(), 0)
        }
    }

    #[test]
    fn grab() {
        let result = Device::new_from_devnode(Path::new("/dev/input/event26"));

        if let Ok(mut device) = result {
            device.grab();

            assert!(device.is_grabbed());
            assert!(device.uinput_device.is_some());
        }
    }
}
