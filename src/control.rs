extern "C" {
    pub static BOOT_DISK_NUMBER: u8;
}

pub fn stop() -> ! {
    unsafe {
        core::arch::asm!(
            "cli",
            "hlt",
            options(noreturn)
        )
    }
}


pub fn is_cf_set() -> bool {
    let is_set: u8;
    unsafe {
        core::arch::asm!(
            "jc 3f",
            "mov {0}, 0",
            "jmp 2f",
            "3:",
            "mov {0}, 1",
            "2:",
            out(reg_byte) is_set
        )
    };
    is_set == 1
}
