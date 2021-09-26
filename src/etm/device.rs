use std::{error::Error, fs};

/*
addr_context	   cntr_val	  event_vinst	    numcidc	     seq_state
addr_ctxtype	   cntrldvr	  mgmt		    numvmidc	     subsystem
addr_idx	   cpu		  mode		    pe		     syncfreq
addr_instdatatype  ctxid_idx	  nr_addr_cmp	    power	     trcidr
addr_range	   ctxid_masks	  nr_cntr	    res_ctrl	     uevent
addr_single	   ctxid_pid	  nr_ext_inp	    res_idx	     vmid_idx
addr_start	   cyc_threshold  nr_pe_cmp	    reset	     vmid_masks
addr_stop	   enable_source  nr_resource	    s_exlevel_vinst  vmid_val
bb_ctrl		   event	  nr_ss_cmp	    seq_event
cntr_ctrl	   event_instren  nrseqstate	    seq_idx
cntr_idx	   event_ts	  ns_exlevel_vinst  seq_reset_event
*/

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub sysfs: String,
}

impl Device {
    fn new(name: String, sysfs: String) -> Self {
        Device { name, sysfs }
    }
}

/// find available devices by list files in /sys/bus/coresight/devices/etm<N>
pub fn find_availabe_devices() -> Result<Vec<Device>, Box<dyn Error>>
{
    let paths = fs::read_dir("/sys/bus/coresight/devices")?;

    let devices: Vec<Device> = paths
        .into_iter()
        .filter(|p| {
            p.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .contains("etm")
        })
        .map(|p| {
            Device::new(
                p.as_ref()
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_string(),
                format!("{}", p.unwrap().path().display()),
            )
        })
        .collect();
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_availabe_devices() {
        let devices = find_availabe_devices();

        match devices {
            Ok(devices) => {
                devices.iter().for_each(|d| println!("{:?}", d))
            }
            Err(err) => eprintln!("[ERROR] {}", err),
        }
    }
}
