use std::{
    error::Error,
    fs::{self},
};

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub sysfs: String,
}

impl Device {
    fn new(name: String, sysfs: String) -> Self {
        Device { name, sysfs }
    }
}

/// find available devices by list files in /sys/bus/coresight/devices/etm<N>
pub fn find_availabe_devices() -> Result<Vec<Device>, Box<dyn Error>>
{
    let paths = fs::read_dir("/sys/bus/coresight/devices")?;

    let devices: Vec<Device> = paths
        .into_iter()
        .map(|p| p.unwrap())
        .filter(|p| p.file_name().to_str().unwrap().contains("etm"))
        .map(|p| {
            Device::new(
                p.file_name().to_str().unwrap().to_string(),
                format!("{}", p.path().display()),
            )
        })
        .collect();
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_availabe_devices() {
        let devices = find_availabe_devices();

        match devices {
            Ok(devices) => {
                devices.iter().for_each(|d| println!("{:?}", d))
            }
            Err(err) => eprintln!("[ERROR] {}", err),
        }
    }
}
