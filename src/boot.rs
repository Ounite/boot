extern "C" {
    pub static BOOT_DISK_NUMBER: u8;
    pub fn abort() -> !;
    pub fn start_envz() -> !;
}
