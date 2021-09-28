use std::error::Error;

use tock_registers::register_bitfields;
use tock_registers::LocalRegisterCopy;

use crate::device::Device;

register_bitfields! [
    u32,
    ETM_MODE [
        EXCLUDE OFFSET(0) NUMBITS(1) [
            exclude = 0,
            include = 1
        ],
        BB OFFSET(4) NUMBITS(1) [],
        CYCACC OFFSET(5) NUMBITS(1) [],
        CTXID OFFSET(6) NUMBITS(1) [],
        VMID OFFSET(7) NUMBITS(1) [],
        TIMESTAMP OFFSET(11) NUMBITS(1) [],
        RETURNSTACK OFFSET(12) NUMBITS(1) [],
        QELEM OFFSET(12) NUMBITS(2) [
            disable = 0,
            instruction_count = 1,
            enable = 3,
        ],
        ATB_TRIGGER OFFSET(19) NUMBITS(1) [],
        LPOVERRIDE OFFSET(20) NUMBITS(1) [],
        ISTALL_EN OFFSET(21) NUMBITS(1) [],
        INSTPRIO OFFSET(23) NUMBITS(1) [],
        NOOVERFLOW OFFSET(24) NUMBITS(1) [],
        TRACE_RESET OFFSET(25) NUMBITS(1) [],
        TRACE_ERR OFFSET(26) NUMBITS(1) [],
        VIEWINST_STARTSTOP OFFSET(27) NUMBITS(1) [],
        EXCL_KERN OFFSET(30) NUMBITS(1) [],
        EXCL_USER OFFSET(31) NUMBITS(1) [],
    ]
];

#[derive(Debug)]
pub struct EtmMode {
    exclude: u32,
    bb: u32,
    cycacc: u32,
    ctxid: u32,
    vmid: u32,
    timestamp: u32,
    returnstack: u32,
    qelem: u32,
    atb_trigger: u32,
    lpoverride: u32,
    istall_en: u32,
    instprio: u32,
    nooverflow: u32,
    trace_reset: u32,
    trace_err: u32,
    viewinst_startstop: u32,
    excl_kern: u32,
    excl_user: u32,
}

impl From<LocalRegisterCopy<u32, ETM_MODE::Register>> for EtmMode {
    fn from(mode: LocalRegisterCopy<u32, ETM_MODE::Register>) -> Self {
        let exclude = mode.read(ETM_MODE::EXCLUDE);
        let bb = mode.read(ETM_MODE::BB);
        let cycacc = mode.read(ETM_MODE::CYCACC);
        let ctxid = mode.read(ETM_MODE::CTXID);
        let vmid = mode.read(ETM_MODE::VMID);
        let timestamp = mode.read(ETM_MODE::TIMESTAMP);
        let returnstack = mode.read(ETM_MODE::RETURNSTACK);
        let qelem = mode.read(ETM_MODE::QELEM);
        let atb_trigger = mode.read(ETM_MODE::ATB_TRIGGER);
        let lpoverride = mode.read(ETM_MODE::LPOVERRIDE);
        let istall_en = mode.read(ETM_MODE::ISTALL_EN);
        let instprio = mode.read(ETM_MODE::INSTPRIO);
        let nooverflow = mode.read(ETM_MODE::NOOVERFLOW);
        let trace_reset = mode.read(ETM_MODE::TRACE_RESET);
        let trace_err = mode.read(ETM_MODE::TRACE_ERR);
        let viewinst_startstop = mode.read(ETM_MODE::VIEWINST_STARTSTOP);
        let excl_kern = mode.read(ETM_MODE::EXCL_KERN);
        let excl_user = mode.read(ETM_MODE::EXCL_USER);

        EtmMode {
            exclude,
            bb,
            cycacc,
            ctxid,
            vmid,
            timestamp,
            returnstack,
            qelem,
            atb_trigger,
            lpoverride,
            istall_en,
            instprio,
            nooverflow,
            trace_reset,
            trace_err,
            viewinst_startstop,
            excl_kern,
            excl_user,
        }
    }
}

impl From<&EtmMode> for LocalRegisterCopy<u32, ETM_MODE::Register> {
    fn from(mode: &EtmMode) -> Self {
        let mut reg = LocalRegisterCopy::<u32, ETM_MODE::Register>::new(0);
        reg.modify(ETM_MODE::EXCLUDE.val(mode.exclude));
        reg.modify(ETM_MODE::BB.val(mode.bb));
        reg.modify(ETM_MODE::CYCACC.val(mode.cycacc));
        reg.modify(ETM_MODE::CTXID.val(mode.ctxid));
        reg.modify(ETM_MODE::VMID.val(mode.vmid));
        reg.modify(ETM_MODE::TIMESTAMP.val(mode.timestamp));
        reg.modify(ETM_MODE::RETURNSTACK.val(mode.returnstack));
        reg.modify(ETM_MODE::QELEM.val(mode.qelem));
        reg.modify(ETM_MODE::ATB_TRIGGER.val(mode.atb_trigger));
        reg.modify(ETM_MODE::LPOVERRIDE.val(mode.lpoverride));
        reg.modify(ETM_MODE::ISTALL_EN.val(mode.istall_en));
        reg.modify(ETM_MODE::INSTPRIO.val(mode.instprio));
        reg.modify(ETM_MODE::NOOVERFLOW.val(mode.nooverflow));
        reg.modify(ETM_MODE::TRACE_RESET.val(mode.trace_reset));
        reg.modify(ETM_MODE::TRACE_ERR.val(mode.trace_err));
        reg.modify(ETM_MODE::VIEWINST_STARTSTOP.val(mode.viewinst_startstop));
        reg.modify(ETM_MODE::EXCL_KERN.val(mode.excl_kern));
        reg.modify(ETM_MODE::EXCL_USER.val(mode.excl_user));
        reg
    }
}

impl From<u32> for EtmMode {
    fn from(mode_num: u32) -> Self {
        let mode = LocalRegisterCopy::<u32, ETM_MODE::Register>::new(mode_num);
        EtmMode::from(mode)
    }
}

impl From<&EtmMode> for u32 {
    fn from(mode: &EtmMode) -> Self {
        let reg: LocalRegisterCopy<u32, ETM_MODE::Register> = mode.into();
        reg.get()
    }
}

impl EtmMode {
    pub fn default() -> Self {
        EtmMode::from(0x4c000850u32)
    }
}

/// get ETM mode
pub fn get_device_mode(d: &Device) -> Result<EtmMode, Box<dyn Error>> {
    let mode_num: u32 = d.get_from_hex("mode")?;
    Ok(EtmMode::from(mode_num))
}

/// set ETM mode
pub fn set_device_mode(
    d: &Device,
    mode: &EtmMode,
) -> Result<(), Box<dyn Error>> {
    let mode_num: u32 = mode.into();
    d.set("mode", format!("{:#010X}", mode_num).as_str())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etm_mode() {
        let mode = EtmMode::from(0x4c000850u32);
        println!("{:?}", mode);
    }
}
