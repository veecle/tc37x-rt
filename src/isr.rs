use core::arch::asm;

extern "C" {
    static __INTERRUPT_TABLE: u8;
}

/// This function will load the interrupt table using inline assembly mtcr
/// (move to core register) instruction. Interrupt table is defined in file lib.rs .
pub fn load_interrupt_table() {
    crate::call_without_endinit(|| unsafe {
        let interrupt_table = &__INTERRUPT_TABLE as *const u8 as u32;
        asm!("mtcr	$biv, {0}", in(reg32) interrupt_table);
        asm!("isync");
    });
}
