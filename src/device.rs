use std::{error::Error, fmt::LowerHex, fs::File, io::Read, io::Write};

use num_traits::Num;

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub sysfs: String,
}

impl Device {
    pub fn new(name: String, sysfs: String) -> Self {
        Device { name, sysfs }
    }

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
            Err(_) => Err(Box::new(DeviceError::InvalidHex(str.to_string()))),
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

pub enum DeviceError {
    InvalidHex(String),
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceError::InvalidHex(hex) => {
                write!(f, "Invalid hex string {}", hex)
            }
        }
    }
}

impl std::fmt::Debug for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DeviceError::InvalidHex(hex) => {
                write!(f, "Invalid hex string {}", hex)
            }
        }
    }
}

impl std::error::Error for DeviceError {
    fn description(&self) -> &str {
        match self {
            DeviceError::InvalidHex(_) => "Invalid hex string",
        }
    }
}
