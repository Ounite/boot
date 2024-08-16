use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { crate::boot::abort() }
}
