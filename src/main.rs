#![no_std]
#![no_main]

mod video;
mod panic;
mod boot;
mod disk;
use video::text;


const ENVZ_PART_ID: u8 = 0x2b;
const MBR_LOC_ADDR: usize = 0x0900;
const ENVZ_LOC_ADDR: usize = 0x7c00;


#[no_mangle]
extern "C" fn main() -> ! {
    let mbr_record = unsafe { &*(MBR_LOC_ADDR as *const disk::MBRRecord) };

    let mut target_part = mbr_record.partitions.iter().find(|p| p.sys_id == ENVZ_PART_ID);
    
    if let Some(part) = target_part {
        disk::read(
            part.start_lba_addr as u64,
            part.blocks_count.try_into().unwrap_or_else(|_| abort("ps")),
            unsafe { &mut*core::ptr::slice_from_raw_parts_mut(ENVZ_LOC_ADDR as *mut _, 492543) },
        ).unwrap_or_else(|_| abort("pr"));
        
        let envz_entry = unsafe { core::mem::transmute::<_, extern "C" fn(u8) -> !>(ENVZ_LOC_ADDR as *const ()) };

        unsafe {
            core::arch::asm!(
                "mov esp, 0x7c00",
                "mov ebp, esp"
            )
        }
        
        envz_entry(unsafe { boot::BOOT_DISK_NUMBER })
    } else {
        abort("p")
    }
}

pub fn abort(reason: &str) -> ! {
    text::print_string(reason);
    boot::stop()
}
