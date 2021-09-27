use etm_rs::etm;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;
fn main() {
    pretty_env_logger::init();
    if let Ok(devices) = etm::get_devices() {
        for ref device in devices {
            // get device info
            device.get_info();

            // get device mode
            device.set_mode(&etm::EtmMode::default());

            // set address space
            device.set_addr_range(&vec![(0x4000_0000u64, 0x4000_2000u64)]);
            device.get_addr_range();

            // enable and disable
            // match device.enable() {
            //     Ok(_) => info!("Success enable device."),
            //     Err(err) => {
            //         error!("Cannot enable device {:?}, {}.", device, err)
            //     }
            // }
            // match device.disable() {
            //     Ok(_) => info!("Success disable device."),
            //     Err(err) => {
            //         error!("Cannot disable device {:?}, {}.", device, err)
            //     }
            // }
            println!()
        }
    } else {
        error!("Cannot find device.")
    }
}
