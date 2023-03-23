// lock/unlock the EDNINIT bit in the cpu WDTCON and safety WDTCON
// to access ENDINIT protected SFRs such as BIV, BTY, CLC.

use core::arch::asm;
use tc37x_pac;

/// Clears endinit bit for  protection against unintentional modifications.
/// See section 11.4 of AURIXTM TC3xx Target specification
pub fn clear_safety_endinit() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let scu = p.SCU;

    let mut passwd = scu.wdtscon0.read().bits();

    passwd &= 0xffffff00;

    scu.wdtscon0.write(|w| unsafe { w.bits(passwd | 0xf1) });
    unsafe {
        asm! {"dsync"}
    };
    scu.wdtscon0.write(|w| unsafe { w.bits(passwd | 0xf2) });
    // read back new value >
    scu.wdtscon0.read().bits();
}

/// Sets endinit bit for  protection against unintentional modifications.
pub fn set_safety_endinit() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let scu = p.SCU;

    let mut passwd = scu.wdtscon0.read().bits();

    passwd &= 0xffffff00;

    scu.wdtscon0.write(|w| unsafe { w.bits(passwd | 0xf1) });
    unsafe {
        asm! {"dsync"}
    };
    passwd |= 3;
    scu.wdtscon0.write(|w| unsafe { w.bits(passwd | 0xf2) });
    // read back new value >
    scu.wdtscon0.read().bits();
}

/// Clears endinit bit for  protection against unintentional modifications for CPU0 core.
/// See section 11.4 of AURIXTM TC3xx Target specification
pub fn clear_cpu_endinit() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let scu = p.SCU;

    let mut passwd = scu.wdtcpu0con0.read().bits();

    passwd &= 0xffffff00;

    scu.wdtcpu0con0.write(|w| unsafe { w.bits(passwd | 0xf1) });
    unsafe {
        asm! {"dsync"}
    };
    scu.wdtcpu0con0.write(|w| unsafe { w.bits(passwd | 0xf2) });
    // read back new value >
    scu.wdtcpu0con0.read().bits();
}

/// Sets endinit bit for  protection against unintentional modifications for current core.
pub fn set_cpu_endinit() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let scu = p.SCU;

    let mut passwd = scu.wdtcpu0con0.read().bits();

    passwd &= 0xffffff00;

    scu.wdtcpu0con0.write(|w| unsafe { w.bits(passwd | 0xf1) });
    unsafe {
        asm! {"dsync"}
    };
    passwd |= 3;
    scu.wdtcpu0con0.write(|w| unsafe { w.bits(passwd | 0xf2) });
    // read back new value >
    scu.wdtcpu0con0.read().bits();
}

/// Disable safety watchdog. The Safety Watchdog Timer provides an overall system level watchdog which is independent from the CPU Watchdog Timers
/// See section 11.4 of AURIXTM TC3xx Target specification
#[no_mangle]
pub fn disable_safety_watchdog() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let scu = p.SCU;

    clear_safety_endinit();
    scu.wdtscon1.modify(|_, w| w.dr().set_bit());
    set_safety_endinit();
}

/// Disable safety watchdog for CPU0 core.
/// See section 11.4 of AURIXTM TC3xx Target specification
#[no_mangle]
pub fn disable_cpu_watchdog() {
    let p = unsafe { tc37x_pac::Peripherals::steal() };
    let scu = p.SCU;

    clear_cpu_endinit();
    scu.wdtcpu0con1.modify(|_, w| w.dr().set_bit());
    set_cpu_endinit();
}
