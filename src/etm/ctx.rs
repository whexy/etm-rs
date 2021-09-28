use std::error::Error;

use crate::{device::Device, etm::ETMError};

/// get ETM numcidc
fn get_numcidc(d: &Device) -> Result<u8, Box<dyn Error>> {
    d.get_from_hex("numcidc")
}

/// set ctxid_idx
fn set_ctxid_idx(d: &Device, idx: u8) -> Result<(), Box<dyn Error>> {
    d.set("ctxid_idx", format!("{:#02X}", idx).as_str())
}

/// set addr_ctxtype
fn set_addr_ctxtype(d: &Device) -> Result<(), Box<dyn Error>> {
    d.set("addr_ctxtype", "ctxid")
}

/// get pid for idx
fn get_ctxid_pid(d: &Device, idx: u8) -> Result<u32, Box<dyn Error>> {
    // check the idx is valid
    if idx > get_numcidc(d)? {
        return Err(Box::new(ETMError::InvalidCtxIdx(idx)));
    }
    set_ctxid_idx(d, idx)?;
    d.get_from_hex("ctxid_pid")
}

/// set pid for idx
fn set_ctxid_pid(d: &Device, idx: u8, pid: u32) -> Result<(), Box<dyn Error>> {
    // check the idx is valid
    if idx >= get_numcidc(d)? {
        return Err(Box::new(ETMError::InvalidCtxIdx(idx)));
    }
    set_ctxid_idx(d, idx)?;
    d.set("ctxid_pid", format!("{:#02X}", pid).as_str())
}

/// set pid group
pub fn set_pid_group(
    d: &Device,
    pid_group: &[u32],
) -> Result<(), Box<dyn Error>> {
    // check the idx is valid
    if pid_group.len() > get_numcidc(d)? as usize {
        return Err(Box::new(ETMError::InvalidCtxIdx(pid_group.len() as u8)));
    }
    set_addr_ctxtype(d)?;
    for (idx, pid) in pid_group.iter().enumerate() {
        set_ctxid_pid(d, idx as u8, *pid)?;
    }
    Ok(())
}

/// get pid group
pub fn get_pid_group(d: &Device) -> Result<Vec<u32>, Box<dyn Error>> {
    let mut pid_group = Vec::new();
    for idx in 0..get_numcidc(d)? {
        pid_group.push(get_ctxid_pid(d, idx)?);
    }
    Ok(pid_group)
}
