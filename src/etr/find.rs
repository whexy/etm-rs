use std::{
    error::Error,
    fs::{self},
};

use super::ETR;
use crate::device::Device;

/// find available ETR by list files in /sys/bus/coresight/devices/tmc_etr<N>
pub fn find_availabe_etrs() -> Result<Vec<ETR>, Box<dyn Error>> {
    let paths = fs::read_dir("/sys/bus/coresight/devices")?;

    let etr: Vec<ETR> = paths
        .into_iter()
        .map(|p| p.unwrap())
        .filter(|p| p.file_name().to_str().unwrap().contains("tmc_etr"))
        .map(|p| {
            Device::new(
                p.file_name().to_str().unwrap().to_string(),
                format!("{}", p.path().display()),
            )
        })
        .filter_map(|d| match ETR::from_device(d.clone()) {
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
    Ok(etr)
}
