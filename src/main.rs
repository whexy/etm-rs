use etm_rs::etm::{device::find_availabe_devices, reg::get_device_info};

fn main() {
    let devices = find_availabe_devices();

    match devices {
        Ok(devices) => {
            devices.iter().for_each(|d| {
                println!("{:?}", d);
                println!("{:?}", get_device_info(d));
            })
        }
        Err(err) => eprintln!("[ERROR] {}", err),
    }
}
