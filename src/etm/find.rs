use std::{
    error::Error,
    fs::{self},
};

use super::ETM;
use crate::device::Device;

/// find available ETMs by list files in /sys/bus/coresight/devices/etm<N>
pub fn find_availabe_etms() -> Result<Vec<ETM>, Box<dyn Error>> {
    let paths = fs::read_dir("/sys/bus/coresight/devices")?;

    let etm: Vec<ETM> = paths
        .into_iter()
        .map(|p| p.unwrap())
        .filter(|p| p.file_name().to_str().unwrap().contains("etm"))
        .map(|p| {
            Device::new(
                p.file_name().to_str().unwrap().to_string(),
                format!("{}", p.path().display()),
            )
        })
        .filter_map(|d| match ETM::from_device(d.clone()) {
            Ok(e) => Some(e),
            Err(err) => {
                error!(
                    "Device {:?} cannot be converted into ETM device. {}",
                    &d, err
                );
                None
            }
        })
        .collect();

    Ok(etm)
}
