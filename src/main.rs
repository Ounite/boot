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

    let mut target_part = None;
    for part in mbr_record.partitions.iter() {
        if part.sys_id == ENVZ_PART_ID {
            target_part = Some(part);
            break;
        };
    };
    
    if let Some(part) = target_part {
        disk::read(
            part.start_lba_addr as u64, 
            part.blocks_count.try_into().unwrap_or_else(|_| { text::print_string("ps"); unsafe { boot::abort() } }), 
            unsafe { &mut*core::ptr::slice_from_raw_parts_mut(ENVZ_LOC_ADDR as *mut _, part.blocks_count as usize * 512) },
        ).unwrap_or_else(|_| { text::print_string("pr"); unsafe { boot::abort() } });
        
        unsafe { boot::start_envz() }
    } else {
        text::print_string("p");
        unsafe { boot::abort() }
    }
}
