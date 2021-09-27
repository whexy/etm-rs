use etm_rs::etm;
extern crate pretty_env_logger;
fn main() {
    pretty_env_logger::init();
    let etm0 = &(etm::get_etms().unwrap())[0];
    // get device info
    println!("{:?}", etm0);
    // enable and disable
    // etm0.enable().unwrap();
    // etm0.disable().unwrap();
    // reset device
    etm0.reset().unwrap();
    // set device mode
    etm0.set_mode(&etm::EtmMode::default()).unwrap();
    // get all address range
    etm0.get_addr_range().unwrap();
    // set address range
    etm0.set_addr_range(&vec![
        (0x4000_0000u64, 0x4000_2000u64),
        (0x4000_4000, 0x4000_5000),
    ])
    .unwrap();
}
