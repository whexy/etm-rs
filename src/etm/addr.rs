use std::error::Error;

use super::{etmerror::ETMError, Device};

type AddrPair = (u64, u64);

/// get ETM nr_addr_cmp
fn get_nr_addr_cmp(d: &Device) -> Result<u8, Box<dyn Error>> {
    let nr_addr_cmp = d.get_from_hex("nr_addr_cmp")?;
    Ok(nr_addr_cmp)
}

/// get ETM addr_idx
fn get_addr_idx(d: &Device) -> Result<u8, Box<dyn Error>> {
    let addr_idx = d.get_from_hex("addr_idx")?;
    Ok(addr_idx)
}

/// set ETM addr_idx
fn set_addr_idx(d: &Device, idx: u8) -> Result<(), Box<dyn Error>> {
    // check if the idx < nr_addr_cmp * 2
    let nr_addr_cmp = get_nr_addr_cmp(d)?;
    if idx >= nr_addr_cmp * 2 {
        return Err(Box::new(ETMError::InvalidAddrIdx(idx)));
    }
    d.set_from_hex("addr_idx", idx)?;
    Ok(())
}

/// get ETM addr_range
fn get_addr_range(d: &Device, idx: u8) -> Result<AddrPair, Box<dyn Error>> {
    // check idx < nr_addr_cmp * 2
    let nr_addr_cmp = get_nr_addr_cmp(d)?;
    if idx >= nr_addr_cmp * 2 {
        return Err(Box::new(ETMError::InvalidAddrIdx(idx)));
    }
    // set addr idx
    set_addr_idx(d, idx)?;

    // get addr range
    let addr_range_r = d.get("addr_range")?;
    let addr_range = addr_range_r.as_str();
    let mut addr_split = addr_range.split(' ');
    let start_addr_str = addr_split
        .next()
        .ok_or(ETMError::InvalidAddrRange(addr_range.to_string()))?;

    let start_addr =
        u64::from_str_radix(start_addr_str.trim_start_matches("0x"), 16)?;

    let end_addr_str = addr_split
        .next()
        .ok_or(ETMError::InvalidAddrRange(addr_range.to_string()))?;

    let end_addr =
        u64::from_str_radix(end_addr_str.trim_start_matches("0x"), 16)?;

    Ok((start_addr, end_addr))
}

fn set_addr_range(
    d: &Device,
    idx: u8,
    range: AddrPair,
) -> Result<(), Box<dyn Error>> {
    // check idx < nr_addr_cmp * 2
    let nr_addr_cmp = get_nr_addr_cmp(d)?;
    if idx >= nr_addr_cmp * 2 {
        return Err(Box::new(ETMError::InvalidAddrIdx(idx)));
    }

    // set addr idx
    set_addr_idx(d, idx)?;

    d.set(
        "addr_range",
        format!("{:#02X} {:#02X}", range.0, range.1).as_str(),
    )?;

    Ok(())
}

/// get all ETM address range
pub fn get_all_addr_range(d: &Device) -> Result<Vec<AddrPair>, Box<dyn Error>> {
    let nr_addr_cmp = get_nr_addr_cmp(d)?;

    let mut addr_range_space = Vec::<AddrPair>::new();

    for addr_idx in (0..nr_addr_cmp).map(|x| 2 * x) {
        let addr_range = get_addr_range(d, addr_idx)?;
        addr_range_space.push(addr_range);
    }
    Ok(addr_range_space)
}

/// set ETM address range
pub fn set_all_addr_range(
    d: &Device,
    range_space: &Vec<AddrPair>,
) -> Result<(), Box<dyn Error>> {
    let nr_addr_cmp = get_nr_addr_cmp(d)?;
    // check length of range_space is less than nr_addr_cmp
    if range_space.len() >= nr_addr_cmp as usize {
        return Err(Box::new(ETMError::AddrRangeLimitExceed));
    }
    // set index => set range
    for (idx, addr_pair) in range_space.iter().enumerate() {
        set_addr_range(d, idx as u8, *addr_pair)?;
    }
    Ok(())
}
