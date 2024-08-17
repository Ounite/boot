extern "C" {
    pub static BOOT_DISK_NUMBER: u8;
}

pub fn stop() -> ! {
    unsafe {
        core::arch::asm!(
            "cli",
            "hlt"
        );
    };
    
    stop()
}
