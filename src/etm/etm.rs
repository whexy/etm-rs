use super::{addr::*, ctx::*, find::*, mode::*};
use crate::device::Device;
use std::error::Error;
use std::mem;

#[derive(Debug)]
pub struct ETM {
    device: Device,
    cpu: u8,
    mode: EtmMode,
    bb_ctrl: bool,
}

/// Get ETM info from Device
impl ETM {
    pub fn from_device(device: Device) -> Result<Self, Box<dyn Error>> {
        let cpu: u8 = device.get("cpu")?.parse()?;
        let mode: EtmMode = get_device_mode(&device)?;
        let bb_ctrl_raw: u8 = device.get_from_hex("bb_ctrl")?;
        let bb_ctrl = bb_ctrl_raw == 1;
        Ok(ETM {
            device,
            cpu,
            mode,
            bb_ctrl,
        })
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

    pub fn reset(&mut self) -> Result<(), Box<dyn Error>> {
        {
            let d = &self.device;
            d.set("reset", "1")?
        }
        let _ = mem::replace(self, Self::from_device(self.device.clone())?);
        Ok(())
    }

    pub fn set_mode(&mut self, mode: EtmMode) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        set_device_mode(d, &mode)?;
        self.mode = mode;
        Ok(())
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

    pub fn get_pid_group(&self) -> Result<Vec<u32>, Box<dyn Error>> {
        get_pid_group(&self.device)
    }

    pub fn set_pid_group(
        &self,
        group: &Vec<u32>,
    ) -> Result<(), Box<dyn Error>> {
        set_pid_group(&self.device, group)
    }

    pub fn enable_bb_ctrl(&mut self) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        d.set("bb_ctrl", "1")?;
        self.bb_ctrl = true;
        Ok(())
    }

    pub fn disable_bb_ctrl(&mut self) -> Result<(), Box<dyn Error>> {
        let d = &self.device;
        d.set("bb_ctrl", "0")?;
        self.bb_ctrl = false;
        Ok(())
    }
}

pub fn get_etms() -> Result<Vec<ETM>, Box<dyn Error>> {
    find_availabe_etms()
}

pub enum ETMError {
    InvalidAddrIdx(u8),
    InvalidAddrRange(String),
    InvalidCtxIdx(u8),
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
            ETMError::InvalidCtxIdx(idx) => {
                write!(f, "Invalid Context ID {}", idx)
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
            ETMError::InvalidCtxIdx(idx) => {
                write!(f, "Invalid Context ID {}", idx)
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
            ETMError::InvalidCtxIdx(_) => "Invalid Context ID",
            ETMError::AddrRangeLimitExceed => "Address Range Limit Exceed",
        }
    }
}
