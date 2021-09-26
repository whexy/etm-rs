use std::{error::Error, fs::File, io::Read};

use super::device::Device;

#[derive(Debug)]
pub struct EtmReg {
    cpu: u8,
}

/// read ETM reg from sysfs
fn read_etm_reg(
    d: &Device,
    reg: &str,
) -> Result<String, Box<dyn Error>> {
    let path = format!("{}/{}", d.sysfs, reg);
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

pub fn get_device_info(d: &Device) -> Result<EtmReg, Box<dyn Error>> {
    let cpu: u8 = read_etm_reg(d, "cpu")?.parse()?;
    Ok(EtmReg { cpu })
}
