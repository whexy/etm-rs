use std::error::Error;

use super::{find::*, reg::*};

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub sysfs: String,
}

impl Device {
    pub fn enable(&self) -> Result<(), Box<dyn Error>> {
        enable_device(self)
    }

    pub fn disable(&self) -> Result<(), Box<dyn Error>> {
        disable_device(self)
    }

    pub fn get_info(&self) {
        println!("{:?}", self);

        match get_device_info(self) {
            Ok(reg) => println!("{:?}", reg),
            Err(err) => {
                eprintln!("[ERROR] Cannot get device info {:?}, {}", self, err)
            }
        }
    }
}

pub fn get_devices() -> Result<Vec<Device>, Box<dyn Error>> {
    find_availabe_devices()
}
