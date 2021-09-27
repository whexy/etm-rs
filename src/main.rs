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

            // reset device
            device.reset().unwrap();

            // get device info
            device.get_info();

            // set device mode
            device.set_mode(&etm::EtmMode::default());

            // set address range
            device.set_addr_range(&vec![(0x4000_0000u64, 0x4000_2000u64)]);

            // get all address range
            device.get_addr_range();

            // enable and disable
            // device.enable().unwrap();
            // device.disable().unwrap();

            // reset device
            device.reset().unwrap();

            println!()
        }
    } else {
        error!("Cannot find device.")
    }
}
