pub struct Device {
    evdev_device: evdev_rs::Device,
    pub udev_device: libudev::Device,
    grabbed: bool,
}

impl Device {
    fn grab(mut self) {
        let grab = self.evdev_device.grab(evdev_rs::GrabMode::Grab);

        match grab {
            Ok(k) => (self.grabbed = true),
            Err(e) => (),
        }
    }

    fn ungrab(mut self) {
        let grab = self.evdev_device.grab(evdev_rs::GrabMode::Ungrab);

        match grab {
            Ok(k) => (self.grabbed = false),
            Err(e) => (),
        }
    }
}
