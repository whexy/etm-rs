use std::{
    error::Error,
    fmt::LowerHex,
    fs::File,
    io::{Read, Write},
};

use num_traits::Num;

use super::{controller::Device, etmerror::ETMError, mode::*};

#[derive(Debug)]
pub struct ETM {
    cpu: u8,
    mode: EtmMode,
}

impl Device {
    pub fn get(&self, reg: &str) -> Result<String, Box<dyn Error>> {
        let path = format!("{}/{}", self.sysfs, reg);
        let mut file = File::open(path.as_str())?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        buffer = buffer.trim().to_string();
        info!("read {}: {}", path, buffer);
        Ok(buffer)
    }

    pub fn set(&self, reg: &str, content: &str) -> Result<(), Box<dyn Error>> {
        let path = format!("{}/{}", self.sysfs, reg);
        let mut file = File::create(path.as_str())?;
        write!(file, "{}", content)?;
        info!("write {}: {}", path, content);
        Ok(())
    }
}

impl Device {
    pub fn get_from_hex<T>(&self, reg: &str) -> Result<T, Box<dyn Error>>
    where
        T: Num,
    {
        let str = self.get(reg)?;
        match T::from_str_radix(str.trim_start_matches("0x"), 16) {
            Ok(result) => Ok(result),
            Err(_) => Err(Box::new(ETMError::InvalidHex(str.to_string()))),
        }
    }

    pub fn set_from_hex<T>(
        &self,
        reg: &str,
        content: T,
    ) -> Result<(), Box<dyn Error>>
    where
        T: LowerHex,
    {
        self.set(reg, format!("{:#010x}", content).as_str())
    }
}

/// get detail info of an ETM device
pub fn get_device_info(d: &Device) -> Result<ETM, Box<dyn Error>> {
    let cpu: u8 = get_device_cpu(d)?;
    let mode: EtmMode = get_device_mode(d)?;
    Ok(ETM { cpu, mode })
}

/// enable ETM device
pub fn enable_device(d: &Device) -> Result<(), Box<dyn Error>> {
    d.set("enable_source", "1")
}

/// disable ETM device
pub fn disable_device(d: &Device) -> Result<(), Box<dyn Error>> {
    d.set("enable_source", "0")
}

/// get ETM CPU
pub fn get_device_cpu(d: &Device) -> Result<u8, Box<dyn Error>> {
    let cpu: u8 = d.get("cpu")?.parse()?;
    Ok(cpu)
}

/// reset ETM
pub fn reset_device(d: &Device) -> Result<(), Box<dyn Error>> {
    d.set("reset", "1")?;
    Ok(())
}