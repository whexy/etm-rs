use etm_rs::{etm, etr};
use std::ffi::CString;
use std::io::{stdin, Read};
use std::process::Command;
extern crate pretty_env_logger;
use log::{info, warn};
use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{execv, fork, getpid};
#[macro_use]
extern crate clap;
use clap::App;

const TRACE_OUTPUT_PATH: &str = "/home/root/trace/etmtrace";

/// An example using etm-rs to track `ls`
fn main() {
    pretty_env_logger::init();

    // ETR
    // - buffer size = 256MB
    let etr0 = &mut (etr::get_etrs().unwrap())[0];
    etr0.set_buffer_size(256).unwrap();
    etr0.enable().unwrap();

    // ETM
    // - mode = default
    // - bb_ctrl enabled
    let etm0 = &mut (etm::get_etms().unwrap())[0];
    etm0.reset().unwrap();
    etm0.set_mode(etm::EtmMode::default()).unwrap();
    etm0.enable_bb_ctrl().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let output_path = matches.value_of("output").unwrap_or(TRACE_OUTPUT_PATH);
    let trace_program: Vec<&str> =
        matches.values_of("trace").unwrap().collect();

    // Start tracing
    // Safety: No safety here. It is just an example.
    let pid = unsafe { fork() }.expect("Fork Failed");
    match pid {
        Child => {
            /* The executor environment */
            warn!("Execution process set as pid={}", getpid());
            let path = CString::new(trace_program[0]).unwrap();
            let argv: Vec<_> = trace_program
                .iter()
                .skip(1)
                .map(|s| CString::new(*s).unwrap())
                .collect();
            info!("Trace target set as {:?} {:?}", path, argv);
            let _ = stdin().read(&mut [0]).unwrap();
            execv(&path, &argv).expect("Execution Failed");
        }
        Parent { child } => {
            /* ETM */
            let pid = child.as_raw() as u32;
            etm0.set_pid_group(&[pid]).unwrap();
            // etm0.set_addr_range(&[(0x0, 0xffffffffffff)]).unwrap();
            warn!("{:?}", etm0);
            etm0.enable().unwrap();
            println!("Ready to trace, press <ENTER> to begin...");
            // wait until end
            let _ = wait().expect("Wait Failed");
            let _ = Command::new("/bin/dd")
                .arg("if=/dev/tmc_etr0")
                .arg(format!("of={}", output_path))
                .status();
            // disable ETR and ETM
            etm0.disable().unwrap();
            etr0.disable().unwrap();
            // reset device
            etm0.reset().unwrap();
        }
    }
}
