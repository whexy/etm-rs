# etm-rs

Embedded Trace for Linux Program with Coresight ETMv4

This project is under development and not yet released.

# Example

```rust
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
etm0.set_addr_range(&vec![
    (0x400000, 0x400200),
    (0x400400, 0x400500),
])
.unwrap();
etm0.enable().unwrap();
    
// disable ETR and ETM
etm0.disable().unwrap();
etr0.disable().unwrap();
// reset device
etm0.reset().unwrap();
```

# TODOs

ETM
- [x] Get ETM sysFS interface
- [x] Get and Set address space
- [x] Get and Set ETM mode
- [x] Get and Set Context ID & PID
- [ ] Enable and Disable bb branch trace
- [x] Enable ETM
- [x] Reset ETM

ETR
- [x] Enable ETR
- [x] Set buffer size
- [ ] Get trace stream

ptm2human
- [ ] Integration