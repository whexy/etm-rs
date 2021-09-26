use std::io::Read;

use super::device::Device;

#[derive(Debug)]
pub struct EtmReg {
    cpu: u8,
}

pub fn get_device_info(d: &Device) -> EtmReg {
    let path = format!("{}/cpu", d.sysfs);
    let mut file = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let cpu: u8 = contents.as_str().trim().parse().unwrap();
    EtmReg { cpu }
}
