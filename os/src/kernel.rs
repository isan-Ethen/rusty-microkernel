#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(panic_info_message)]

use common::{print, println, Argument};

use core::{arch::asm, fmt, panic::PanicInfo, ptr};

extern "C" {
    static mut __bss: u32;
    static __bss_end: u32;
    static __stack_top: u32;
}

#[no_mangle]
fn kernel_main() {
    unsafe {
        let bss = ptr::addr_of_mut!(__bss);
        let bss_end = ptr::addr_of!(__bss_end);
        ptr::write_bytes(bss, 0, bss_end as usize - bss as usize);
    }
    println("\n\nHello %s", &[Argument::new_string("World!")]);
    print(
        "1 + 2 = %d, %x\n",
        &[
            Argument::new_decimal(1 + 2),
            Argument::new_hexadecimal(0x1234abcd),
        ],
    );

    panic!("booted!");

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
fn panic(info: &PanicInfo) -> ! {
    print_panic_info(info);
    loop {}
}

fn print_panic_info(info: &PanicInfo) {
    if let Some(location) = info.location() {
        print(
            "PANIC!: Panic occurred in file %s at line %s",
            &[
                Argument::new_string(location.file()),
                Argument::new_decimal(location.line() as i32),
            ],
        );
    } else {
        print(
            "PANIC!: Panic occurred but can't get infomation of the location",
            &[],
        );
    }

    if let Some(message) = info.message() {
        let mut putchar_writer = PutcharWriter;
        print(": ", &[]);
        let _ = fmt::write(&mut putchar_writer, *message);
        println("", &[]);
    }
}

struct PutcharWriter;

impl fmt::Write for PutcharWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            putchar(c);
        }
        Ok(())
    }
}

mod sbi;

#[no_mangle]
fn putchar(ch: char) {
    unsafe {
        sbi::SbiRet::sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1 /* Console putchar */);
    }
}
