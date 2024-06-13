#![no_std]
#![no_main]
#![feature(naked_functions)]

use common::{print, Argument};

use core::{arch::asm, panic::PanicInfo};

extern "C" {
    static mut __bss: u32;
    static __bss_end: u32;
    static __stack_top: u32;
}

#[no_mangle]
fn kernel_main() {
    print("\n\nHello %s\n", &[Argument::new_string("World!")]);
    print(
        "1 + 2 = %d, %x\n",
        &[
            Argument::new_decimal(1 + 2),
            Argument::new_hexadecimal(0x1234abcd),
        ],
    );

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
