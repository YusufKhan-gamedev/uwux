use core::mem;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SDT {
    signature: [u8; 4],
    pub length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl SDT {}