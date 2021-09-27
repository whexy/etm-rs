use super::{addr::*, find::*, mode::*, reg::*};
use std::error::Error;

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
            Ok(etm) => println!("{:?}", etm),
            Err(err) => {
                error!("[ERROR] Cannot get device info {:?}, {}", self, err)
            }
        }
    }

    pub fn get_mode(&self) {
        match get_device_mode(self) {
            Ok(mode) => println!("{:?}", mode),
            Err(err) => {
                error!("[ERROR] Cannot get device mode {:?}, {}", self, err)
            }
        }
    }

    pub fn set_mode(&self, mode: &EtmMode) {
        match set_device_mode(self, mode) {
            Ok(_) => info!("Successfully set mode {:?}", mode),
            Err(err) => {
                error!("Cannot set device mode {:?}, {:?}, {}", mode, self, err)
            }
        }
    }

    pub fn get_addr_range(&self) {
        match get_all_addr_range(self) {
            Ok(f) => f.into_iter().for_each(|(s, e)| {
                println!("{:#2X}-{:#2X}", s, e);
            }),
            Err(err) => {
                error!("Cannot get address range {:?}, {}", self, err)
            }
        }
    }

    pub fn set_addr_range(&self, range: &Vec<(u64, u64)>) {
        match set_all_addr_range(self, range) {
            Ok(_) => info!("Successfully set range {:?}", range),
            Err(err) => {
                error!(
                    "Cannot set address range {:?}, {:?}, {}",
                    range, self, err
                )
            }
        }
    }
}

pub fn get_devices() -> Result<Vec<Device>, Box<dyn Error>> {
    find_availabe_devices()
}
