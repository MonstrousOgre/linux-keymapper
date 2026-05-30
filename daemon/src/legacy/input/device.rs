use std::collections::HashMap;
use std::ffi::OsStr;

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
    fn new_from_devnode(devnode: &Path) -> Result<Device, Box<dyn Error>> {
        let f = File::open(devnode)?;
        let d = evdev_rs::Device::new_from_file(f)?;
        Ok(Device {
            evdev_device: d,
            grabbed: false,
            uinput_device: None,
        })
    }

    pub fn get_all_devices(
        context: &libudev::Context,
    ) -> Result<HashMap<String, Device>, Box<dyn Error>> {
        let mut enumerator = libudev::Enumerator::new(context)?;
        enumerator.match_property("ID_INPUT_KEYBOARD", "1").ok();

        let mut devices = HashMap::new();

        for ud in enumerator.scan_devices()? {
            if let Some(devnode) = ud.devnode() {
                devices.insert(
                    String::from(devnode.to_str().unwrap()),
                    Device::new_from_devnode(devnode)?,
                );
            }
        }

        Ok(devices)
    }

    pub fn listen_for_devices(
        context: &libudev::Context,
        devices: &mut HashMap<String, Device>,
    ) -> Result<(), Box<dyn Error>> {
        let mut monitor = libudev::Monitor::new(context)?;
        monitor.match_subsystem("input")?;
        let mut socket = monitor.listen()?;

        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));
            if let Some(event) = socket.receive_event() {
                let keyboard_prop = event
                    .device()
                    .property_value("ID_INPUT_KEYBOARD")
                    .unwrap_or(OsStr::new("0"));

                if keyboard_prop == "1" {
                    if let Some(devnode) = event.device().devnode() {
                        match event.event_type().to_string().as_str() {
                            "add" => {
                                Box::new(devices.remove(&String::from(devnode.to_str().unwrap())))
                            }
                            _ => Box::new(None),
                        };
                    }
                }
            }
        }
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
        let context = libudev::Context::new().unwrap();

        let result = Device::get_all_devices(&context);
        if let Ok(devices) = result {
            assert_ne!(devices.len(), 0)
        }
    }

    #[test]
    fn grab() {
        let result = Device::new_from_devnode(Path::new("/dev/input/event7"));

        if let Ok(mut device) = result {
            device.grab();

            assert!(device.is_grabbed());
            assert!(device.uinput_device.is_some());
        }
    }
}
