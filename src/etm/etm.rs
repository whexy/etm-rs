use super::{addr::*, find::*, mode::*};
use crate::device::Device;
use std::error::Error;

#[derive(Debug)]
pub struct ETM {
    device: Device,
    cpu: u8,
    mode: EtmMode,
}

/// Get ETM info from Device
impl ETM {
    pub fn from_device(device: Device) -> Result<Self, Box<dyn Error>> {
        let cpu: u8 = device.get("cpu")?.parse()?;
        let mode: EtmMode = get_device_mode(&device)?;
        Ok(ETM { device, cpu, mode })
    }
}

impl ETM {
    pub fn enable(&self) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        d.set("enable_source", "1")
    }

    pub fn disable(&self) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        d.set("enable_source", "0")
    }

    pub fn reset(&self) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        d.set("reset", "1")
    }

    pub fn get_mode(&self) -> Result<EtmMode, Box<dyn Error>> {
        let d = &self.device;
        get_device_mode(d)
    }

    pub fn set_mode(&self, mode: &EtmMode) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        set_device_mode(d, mode)
    }

    pub fn get_addr_range(&self) -> Result<Vec<(u64, u64)>, Box<dyn Error>> {
        get_all_addr_range(&self.device)
    }

    pub fn set_addr_range(
        &self,
        range: &Vec<(u64, u64)>,
    ) -> Result<(), Box<dyn Error>> {
        set_all_addr_range(&self.device, range)
    }
}

pub fn get_etms() -> Result<Vec<ETM>, Box<dyn Error>> {
    find_availabe_etms()
}

pub enum ETMError {
    InvalidAddrIdx(u8),
    InvalidAddrRange(String),
    AddrRangeLimitExceed,
}

impl std::fmt::Display for ETMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ETMError::InvalidAddrIdx(idx) => {
                write!(f, "Invalid Address Index {}", idx)
            }
            ETMError::InvalidAddrRange(addr) => {
                write!(f, "Invalid Address Range {}", addr)
            }
            ETMError::AddrRangeLimitExceed => {
                write!(f, "Address Range Limit Exceed")
            }
        }
    }
}

impl std::fmt::Debug for ETMError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ETMError::InvalidAddrIdx(idx) => {
                write!(f, "Invalid Address Index {}", idx)
            }
            ETMError::InvalidAddrRange(addr) => {
                write!(f, "Invalid Address Range {}", addr)
            }
            ETMError::AddrRangeLimitExceed => {
                write!(f, "Address Range Limit Exceed")
            }
        }
    }
}

impl std::error::Error for ETMError {
    fn description(&self) -> &str {
        match self {
            ETMError::InvalidAddrIdx(_) => "Invalid Address Index",
            ETMError::InvalidAddrRange(_) => "Invalid Address Range",
            ETMError::AddrRangeLimitExceed => "Address Range Limit Exceed",
        }
    }
}
