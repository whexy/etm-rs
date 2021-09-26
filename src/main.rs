use etm_rs::etm;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
fn main() {
    pretty_env_logger::init();
    if let Ok(devices) = etm::get_devices() {
        for ref device in devices {
            device.get_info();
            match device.enable() {
                Ok(_) => info!("Success enable device."),
                Err(err) => {
                    error!("Cannot enable device {:?}, {}.", device, err)
                }
            }
            match device.disable() {
                Ok(_) => info!("Success disable device."),
                Err(err) => {
                    error!("Cannot disable device {:?}, {}.", device, err)
                }
            }
        }
    } else {
        error!("Cannot find device.")
    }
}
