use crate::control;
use crate::control::is_cf_set;


#[repr(packed)]
pub struct MBRRecord {
    pub bootstrap: [u8; 440],
    pub disk_id: u32,
    pub record_flag: u16,
    pub partitions: [PartitionRecord; 4],
    pub signature: u16,
}


#[repr(packed)]
pub struct CHSAddress([u8; 3]);


impl CHSAddress {
    pub fn head(&self) -> u8 {
        self.0[0]
    }

    pub fn sector(&self) -> u8 {
        self.0[1] & 0b11111100 >> 2
    }

    pub fn cylinder(&self) -> u16 {
        ((self.0[1] & 0b00000011) as u16) << 8 | self.0[2] as u16
    }
}


#[repr(packed)]
pub struct PartitionRecord {
    pub attributes: PartitionAttributes,
    pub start_chs_addr: CHSAddress,
    pub sys_id: u8,
    pub end_chs_addr: CHSAddress,
    pub start_lba_addr: u32,
    pub blocks_count: u32,
}


pub struct PartitionAttributes(u8);


impl PartitionAttributes {
    pub fn is_set<const BIT: u8>(&self) -> bool {
        self.0 & (1 << BIT) != 0
    }

    pub fn set<const BIT: u8>(&mut self, state: bool) {
        if state {
            self.0 |= 1 << BIT;
        } else {
            self.0 &= 0xFF ^ (1 << BIT);
        };
    }
}



#[repr(packed)]
struct LBAAddressPacket {
    size: u8,
    _pad: u8,
    blocks_count: u16,
    buffer: u16,
    mem_page: u16,
    lba_addr: u64,
}


pub enum DiskReadError {
    ReadLess(u16),
    BufferTooSmall,
    BufferTooFarAway,
    Other(u8)
}


pub fn read(addr: u64, blocks: u16, buffer: &mut [u8]) -> Result<(), DiskReadError> {
    if buffer.len() < blocks as usize * 512 {
        return Err(DiskReadError::BufferTooSmall);
    };

    let mut packet = LBAAddressPacket {
        size: 16,
        _pad: 0x00,
        blocks_count: blocks,
        buffer: (buffer.as_ptr() as usize).try_into().map_err(|_| DiskReadError::BufferTooFarAway)?,
        mem_page: 0,
        lba_addr: addr,
    };

    unsafe {
        core::arch::asm!(
            "push si",
            "mov si, cx",
            "int 0x13",
            "pop si",
            in("dl") unsafe { control::BOOT_DISK_NUMBER },
            in("cx") &mut packet as *mut _ as u16,
            in("ah") 0x42_u8
        );
    };

    let ah_value;
    unsafe {
        core::arch::asm!(
            "mov {}, ah",
            out(reg_byte) ah_value
        );
    };

    if is_cf_set() {
        return Err(DiskReadError::Other(ah_value));
    };

    (packet.blocks_count == blocks).then_some(()).ok_or(DiskReadError::ReadLess(packet.blocks_count))
}
