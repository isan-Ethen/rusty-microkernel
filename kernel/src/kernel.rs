#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::{arch::asm, panic::PanicInfo, ptr};

extern "C" {
    static mut __bss: u32;
    static __bss_end: u32;
    static __stack_top: u32;
}

#[no_mangle]
fn kernel_main() {
    /* versiong 1
     * unsafe {
     *     let bss = ptr::addr_of_mut!(__bss);
     *     let bss_end = ptr::addr_of!(__bss_end);
     *     ptr::write_bytes(bss, 0, bss_end as usize - bss as usize);
     * }
     *
     *
     * loop {}
     */
    let s = "\n\nHello World!\n";
    for c in s.chars() {
        putchar(c);
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[link_section = ".text.boot"]
#[naked]
#[no_mangle]
extern "C" fn boot() {
    unsafe {
        asm!(
            "la sp, {stack_top}",
            "j kernel_main",
            stack_top = sym  __stack_top,
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod sbi;

#[no_mangle]
fn putchar(ch: char) {
    unsafe {
        sbi::SbiRet::sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1 /* Console putchar */);
    }
}
