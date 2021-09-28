use etm_rs::{etm, etr};
extern crate pretty_env_logger;
fn main() {
    pretty_env_logger::init();

    // Set ETR, buffer size = 256MB
    let etr0 = &mut (etr::get_etrs().unwrap())[0];
    etr0.set_buffer_size(256).unwrap();
    // Enable ETR
    etr0.enable().unwrap();

    // Set ETM
    // Reset device, device mode = default
    let etm0 = &mut (etm::get_etms().unwrap())[0];
    etm0.reset().unwrap();
    etm0.set_mode(etm::EtmMode::default()).unwrap();
    // set pid group
    etm0.set_pid_group(&vec![114514]).unwrap();
    // set address range
    etm0.set_addr_range(&vec![(0x400000, 0x400200), (0x400400, 0x400500)])
        .unwrap();
    // enable BB_CTRL
    etm0.enable_bb_ctrl().unwrap();

    // Print Settings and enable ETM
    println!("{:?}", etm0);
    etm0.enable().unwrap();

    // disable ETR and ETM
    etm0.disable().unwrap();
    etr0.disable().unwrap();
    // reset device
    etm0.reset().unwrap();
}
