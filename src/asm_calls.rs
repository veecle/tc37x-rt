// This module implements some assembler routines

use core::arch::asm;

// global interrupts enable
pub fn enable_interrupts() {
    unsafe {
        asm!("enable");
    }
}
// global interrupts disable
pub fn disable_interrupts() {
    unsafe {
        asm!("disable");
    }
}

/** \brief FE1C, CPUx Core Identification Register */
#[allow(dead_code)]
const CPU_CORE_ID: u32 = 0xFE1C;

/**
 * Read CPU core id.
 */
pub fn read_cpu_core_id() -> u32 {
    #[cfg(target = "tc162-htc-none")]
    {
        #[allow(unused_assignments)]
        let mut value: u32;
        unsafe {
            asm!("mfcr {0}, 0xFE1C", out(reg32) value);
        }
        value
    }
    #[cfg(not(target = "tc162-htc-none"))]
    unimplemented!("This function is only available on tricore");
}
