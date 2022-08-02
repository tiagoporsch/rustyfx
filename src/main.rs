use std::time::Duration;
use rusb::{DeviceHandle, Error, GlobalContext, Result};

struct Controller {
    device: DeviceHandle<GlobalContext>,
}

impl Controller {
    pub fn new() -> Result<Self> {
        let mut device = match rusb::open_device_with_vid_pid(0x187c, 0x0550) {
            Some(device) => device,
            None => return Err(Error::NoDevice),
        };
        device.detach_kernel_driver(0)?;
        device.claim_interface(0)?;
        Ok(Self { device })
    }

    pub fn set_brightness(&mut self, brightness: u8) -> Result<usize> {
        match brightness {
            0..=100 => {
                let payload = vec![3, 0x26, 100 - brightness, 0, 4, 0, 1, 2, 3];
                self.device.write_control(0x21, 9, 0x202, 0, &payload, Duration::ZERO)
            }
            _ => Err(Error::InvalidParam)
        }
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.device.release_interface(0).unwrap();
        self.device.attach_kernel_driver(0).unwrap();
    }
}

fn main() {
    let mut controller = Controller::new().unwrap();
    controller.set_brightness(100).unwrap();
}