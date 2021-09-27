# etm-rs

Embedded Trace for Linux Program with Coresight ETMv4

This project is under development and not yet released.

# Example

```rust
let etm0 = &(etm::get_etms().unwrap())[0];
// get device info
println!("{:?}", etm0);
// enable and disable
etm0.enable().unwrap();
etm0.disable().unwrap();
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
```

# TODOs

ETM
- [x] Get ETM sysFS interface
- [x] Get and Set address space
- [x] Get and Set ETM mode
- [ ] Get and Set Context ID & PID
- [x] Enable ETM
- [x] Reset ETM

ETR
- [ ] Enable ETR
- [ ] Set buffer size
- [ ] Get trace stream

ptm2human
- [ ] Integration