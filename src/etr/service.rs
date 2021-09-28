use std::error::Error;

use crate::device::Device;

use super::find::find_availabe_etrs;

#[derive(Debug)]
pub struct ETR {
    device: Device,
    buffer_size: u32,
}

impl ETR {
    pub fn from_device(device: Device) -> Result<Self, Box<dyn Error>> {
        let buffer_size = device.get_from_hex("buffer_size")?;
        Ok(ETR {
            device,
            buffer_size,
        })
    }
}

impl ETR {
    pub fn enable(&self) -> Result<(), Box<dyn Error>> {
        let device = &self.device;
        device.set("enable_sink", "1")
    }

    pub fn disable(&self) -> Result<(), Box<dyn Error>> {
        let device = &self.device;
        device.set("enable_sink", "0")
    }

    pub fn set_buffer_size(&mut self, size: u32) -> Result<(), Box<dyn Error>> {
        let device = &self.device;
        device.set(
            "buffer_size",
            format!("{:#02X}", size as u64 * 1024 * 1024).as_str(),
        )?;
        self.buffer_size = size;
        Ok(())
    }
}

pub fn get_etrs() -> Result<Vec<ETR>, Box<dyn Error>> {
    find_availabe_etrs()
}
