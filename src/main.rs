#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![no_std]
#![no_main]

mod video;
mod panic;
mod control;
mod disk;

use video::text;
use crate::disk::DiskReadError;


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
            part.blocks_count as u16,
            unsafe { &mut*core::ptr::slice_from_raw_parts_mut(ENVZ_LOC_ADDR as *mut _, 492543) },
        ).unwrap_or_else(|err| match err {
            DiskReadError::BufferTooSmall => abort(core::ascii::Char::SmallB),
            DiskReadError::ReadLess(_) => abort(core::ascii::Char::SmallR),
            DiskReadError::Other(code) => {
                text::print_u16(code as u16);
                control::stop();
            },
            DiskReadError::BufferTooFarAway => unsafe { core::hint::unreachable_unchecked() },
        });

        let envz_entry = unsafe { core::mem::transmute::<_, extern "C" fn(u8) -> !>(ENVZ_LOC_ADDR as *const ()) };

        unsafe {
            core::arch::asm!(
                "mov esp, 0x7c00",
                "mov ebp, esp"
            )
        }

        envz_entry(unsafe { control::BOOT_DISK_NUMBER })
    } else {
        control::stop()
    }
}

pub fn abort(reason: core::ascii::Char) -> ! {
    text::print_char(reason);
    control::stop()
}
