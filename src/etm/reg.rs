use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use super::controller::Device;

#[derive(Debug)]
pub struct ETM {
    cpu: u8,
}

impl Device {
    fn get(&self, reg: &str) -> Result<String, Box<dyn Error>> {
        let path = format!("{}/{}", self.sysfs, reg);
        let mut file = File::open(path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer.trim().to_string())
    }

    fn set(&self, reg: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/{}", self.sysfs, reg);
        let mut file = File::create(path)?;
        write!(file, "{}", content)?;
        Ok(())
    }
}

/// get detail info of an ETM device
pub fn get_device_info(d: &Device) -> Result<ETM, Box<dyn Error>> {
    let cpu: u8 = d.get("cpu")?.parse()?;
    Ok(ETM { cpu })
}

/// enable ETM device
pub fn enable_device(d: &Device) -> Result<(), Box<dyn Error>> {
    d.set("enable_source", "1")
}

/// disable ETM device
pub fn disable_device(d: &Device) -> Result<(), Box<dyn Error>> {
    d.set("enable_source", "0")
}
