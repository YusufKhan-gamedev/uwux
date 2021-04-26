use core::{intrinsics, mem};

use x86_64::{structures::paging::*, PhysAddr, VirtAddr};

use super::sdt::Sdt;

pub const SIGNATURE: &str = "APIC";
pub const TRAMPOLINE: u64 = 0x8000;

static TRAMPOLINE_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/trampoline"));

#[derive(Clone, Copy, Debug)]
pub struct Madt {
    pub sdt: &'static Sdt,
    pub local_apic_address: u32,
    pub flags: u32,
}

impl Madt {
    pub fn new(
        sdt: Option<&'static Sdt>,
        frame_allocator: &mut impl FrameAllocator<Size4KiB>,
        offset_table: &mut OffsetPageTable,
    ) {
        if let Some(sdt) = sdt {
            // Not a valid MADT table without the local apic address and the flags.
            if sdt.data_len() < 8 {
                return;
            }

            let this = unsafe { sdt.as_ptr::<Self>() };

            log::info!("Enabling multicore");

            unsafe {
                let trampoline_frame = PhysFrame::containing_address(PhysAddr::new(TRAMPOLINE));
                let trampoline_page: Page<Size4KiB> =
                    Page::containing_address(VirtAddr::new(TRAMPOLINE));

                offset_table
                    .map_to(
                        trampoline_page,
                        trampoline_frame,
                        PageTableFlags::PRESENT | PageTableFlags::WRITABLE,
                        frame_allocator,
                    )
                    .unwrap()
                    .flush();

                // Atomic store the AP trampoline code to a fixed address in low conventional memory.
                for i in 0..TRAMPOLINE_BIN.len() {
                    intrinsics::atomic_store((TRAMPOLINE as *mut u8).add(i), TRAMPOLINE_BIN[i]);
                }
            }
        }
    }

    pub fn iter(&self) -> MadtIterator {
        unsafe {
            MadtIterator {
                ptr: (self as *const Self as *const u8).add(mem::size_of::<Self>()),
                i: self.sdt.length as usize - mem::size_of::<Self>(),
            }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct EntryHeader {
    pub entry_type: u8,
    pub length: u8,
}

#[repr(C, packed)]
pub struct MadtLocalApic {
    pub header: EntryHeader,
    pub processor_id: u8,
    pub apic_id: u8,
    pub flags: u32,
}

#[repr(C, packed)]
pub struct MadtIoApic {
    pub header: EntryHeader,
    pub io_apic_id: u8,
    reserved: u8,
    pub io_apic_address: u32,
    pub global_system_interrupt_base: u32,
}

#[repr(C, packed)]
pub struct MadtIntSrcOverride {
    pub header: EntryHeader,
    pub bus: u8,
    pub irq: u8,
    pub global_system_interrupt: u32,
    pub flags: u16,
}

pub enum MadtEntry {
    LocalApic(&'static MadtLocalApic),
    IoApic(&'static MadtIoApic),
    IntSrcOverride(&'static MadtIntSrcOverride),

    Unknown(u8),
}

pub struct MadtIterator {
    ptr: *const u8,
    i: usize,
}

impl Iterator for MadtIterator {
    type Item = MadtEntry;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i > 0 {
            unsafe {
                let entry_pointer = self.ptr;
                let header = *(self.ptr as *const EntryHeader);

                self.ptr = self.ptr.offset(header.length.into());
                self.i -= header.length as usize;

                let item = match header.entry_type {
                    0 => MadtEntry::LocalApic(&*(entry_pointer as *const MadtLocalApic)),
                    1 => MadtEntry::IoApic(&*(entry_pointer as *const MadtIoApic)),
                    2 => MadtEntry::IntSrcOverride(&*(entry_pointer as *const MadtIntSrcOverride)),

                    0x10..=0x7f => continue,
                    0x80..=0xff => continue,

                    _ => MadtEntry::Unknown(header.entry_type),
                };

                return Some(item);
            }
        }

        None
    }
}
