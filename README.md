# etm-rs

<center>
<img src="https://img.shields.io/badge/language-rust-orange?style=flat&logo=rust"/>
<img src="https://img.shields.io/github/license/whexy/etm-rs?color=brightgreen"/>

Embedded Trace for Linux Program with Coresight ETMv4.
</center>


## Example

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
etm0.set_pid_group(&[114514]).unwrap();
// set address range
etm0.set_addr_range(&[(0x400000, 0x400200), (0x400400, 0x400500)])
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
```

## etm-cli

A command line tool to trace program with Coresight ETMv4 based on `etm-rs`.

### One-line example

```shell
RUST_LOG=warn taskset 0x1 ./etm-rs -o <output_file> -- /bin/ls -la /
```

This command means:

- Set the environment variable `RUST_LOG`. Support levels: debug, info, warn, error.
- Tie the program in CPU 0 with `taskset 0x1`.
- Use `etm-rs` to trace program `/bin/ls -la /`.
- Write tracing stream to <output_file>.

### Usage

```
USAGE:
    etm-rs [OPTIONS] <trace>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <output>    Set the output file of ETM trace stream

ARGS:
    <trace>...    The prgram and its arguments to trace
```

## Features

ETM
- [x] Get ETM sysFS interface
- [x] Get and Set address space
- [x] Get and Set ETM mode
- [x] Get and Set Context ID & PID
- [x] Enable and Disable bb branch trace
- [x] Enable ETM
- [x] Reset ETM

ETR
- [x] Enable ETR
- [x] Set buffer size

RUNTIME
- [x] Get trace stream
- [ ] ptm2human Integration