use etm_rs::{etm, etr};
use std::ffi::CString;
use std::io::{stdin, Read};
use std::process::Command;
extern crate pretty_env_logger;
use log::warn;
use nix::sys::wait::wait;
use nix::unistd::execv;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid};

const TRACE_OUTPUT_PATH: &str = "/home/root/wenxuan/etm-rs/etmtrace";

/// An example using etm-rs to track `ls`
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
    // enable BB_CTRL
    etm0.enable_bb_ctrl().unwrap();

    // Safety: No safety here. It is just an example.
    let pid = unsafe { fork() }.expect("Fork Failed");
    match pid {
        Child => {
            /* The executor environment */
            warn!("Execution process set as pid={}", getpid());
            let path = CString::new("/bin/ls").unwrap();
            let argv =
                &[CString::new("-l").unwrap(), CString::new("/").unwrap()];
            println!("Ready to trace, press <ENTER> to begin...");
            let _ = stdin().read(&mut [0]).unwrap();
            execv(&path, argv).expect("Execution Failed");
        }
        Parent { child } => {
            /* ETM */
            let pid = child.as_raw() as u32;
            // set pid group
            etm0.set_pid_group(&[pid]).unwrap();
            // set address range
            etm0.set_addr_range(&[(0x0, 0xffffffffffff)]).unwrap();
            // Print Settings and enable ETM
            warn!("{:?}", etm0);
            etm0.enable().unwrap();
            // wait until end
            let _ = wait().expect("Wait Failed");
            let _ = Command::new("/bin/dd")
                .arg("if=/dev/tmc_etr0")
                .arg(format!("of={}", TRACE_OUTPUT_PATH))
                .status();
            // disable ETR and ETM
            etm0.disable().unwrap();
            etr0.disable().unwrap();
            // reset device
            etm0.reset().unwrap();
        }
    }
}
