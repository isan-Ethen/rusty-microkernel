use core::ptr;

use common::{PAddr, PAGE_SIZE};

extern "C" {
    static mut __free_ram: u8;
    static mut __free_ram_end: u8;
}

static mut NEXT_PADDR: *mut u8 = unsafe { ptr::addr_of_mut!(__free_ram) };

pub fn alloc_pages(n: u32) -> PAddr {
    unsafe {
        let paddr: PAddr = NEXT_PADDR as PAddr;
        NEXT_PADDR = NEXT_PADDR.add((n * PAGE_SIZE) as usize);

        if NEXT_PADDR > unsafe { ptr::addr_of_mut!(__free_ram_end) } {
            panic!("out of memory");
        }

        ptr::write_bytes(paddr as *mut u8, 0u8, (n * PAGE_SIZE) as usize);
        paddr
    }
}
