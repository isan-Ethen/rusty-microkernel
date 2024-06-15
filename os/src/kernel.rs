#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![allow(unused)]

mod memory;

use common::{print, println, Argument, PAddr};
use memory::alloc_pages;

use core::{arch::asm, fmt::Write, panic::PanicInfo, ptr};

extern "C" {
    static mut __bss: u32;
    static __bss_end: u32;
    static __stack_top: u32;
}

macro_rules! read_csr {
    ($csr:expr) => {
        unsafe {
            use core::arch::asm;
            let mut csrr: u32;
            asm!(concat!("csrr {r}, ", $csr), r = out(reg) csrr);
            csrr
        }
    };
}

macro_rules! write_csr {
    ($csr:expr, $value:expr) => {
        unsafe {
            use core::arch::asm;
            asm!(concat!("csrw ", $csr, ", {r}"), r = in(reg) $value);
        }
    };
}

#[no_mangle]
fn kernel_main() {
    unsafe {
        let bss = ptr::addr_of_mut!(__bss);
        let bss_end = ptr::addr_of!(__bss_end);
        ptr::write_bytes(bss, 0, bss_end as usize - bss as usize);
    }

    let paddr0: PAddr = alloc_pages(2);
    let paddr1: PAddr = alloc_pages(1);

    println(
        "alloc_pages: paddr0=%x",
        &[Argument::new_hexadecimal(paddr0 as i32)],
    );
    println(
        "alloc_pages: paddr1=%x",
        &[Argument::new_hexadecimal(paddr1 as i32)],
    );

    panic!("booted");
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
            "PANIC!: Panic occurred in file %s at line %d",
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

    if let Some(message) = info.message().as_str() {
        println(": %s", &[Argument::new_string(message)]);
    } else {
        let mut putchar_writer = PutcharWriter;
        print(": ", &[]);
        let _ = core::fmt::write(&mut putchar_writer, info.message());
        println("", &[]);
    }
}

struct PutcharWriter;

impl core::fmt::Write for PutcharWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
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

#[naked]
#[no_mangle]
extern "C" fn kernel_entry() {
    unsafe {
        asm!(
            "csrrw sp, sscratch, sp",
            "addi sp, sp, -4 * 31",
            "sw ra,  4 * 0(sp)",
            "sw gp,  4 * 1(sp)",
            "sw tp,  4 * 2(sp)",
            "sw t0,  4 * 3(sp)",
            "sw t1,  4 * 4(sp)",
            "sw t2,  4 * 5(sp)",
            "sw t3,  4 * 6(sp)",
            "sw t4,  4 * 7(sp)",
            "sw t5,  4 * 8(sp)",
            "sw t6,  4 * 9(sp)",
            "sw a0,  4 * 10(sp)",
            "sw a1,  4 * 11(sp)",
            "sw a2,  4 * 12(sp)",
            "sw a3,  4 * 13(sp)",
            "sw a4,  4 * 14(sp)",
            "sw a5,  4 * 15(sp)",
            "sw a6,  4 * 16(sp)",
            "sw a7,  4 * 17(sp)",
            "sw s0,  4 * 18(sp)",
            "sw s1,  4 * 19(sp)",
            "sw s2,  4 * 20(sp)",
            "sw s3,  4 * 21(sp)",
            "sw s4,  4 * 22(sp)",
            "sw s5,  4 * 23(sp)",
            "sw s6,  4 * 24(sp)",
            "sw s7,  4 * 25(sp)",
            "sw s8,  4 * 26(sp)",
            "sw s9,  4 * 27(sp)",
            "sw s10, 4 * 28(sp)",
            "sw s11, 4 * 29(sp)",
            "csrr a0, sscratch",
            "sw a0, 4 * 30(sp)",
            "addi a0, sp, 4*31",
            "csrw sscratch, a0",
            "mv a0, sp",
            "call handle_trap",
            "lw ra,  4 * 0(sp)",
            "lw gp,  4 * 1(sp)",
            "lw tp,  4 * 2(sp)",
            "lw t0,  4 * 3(sp)",
            "lw t1,  4 * 4(sp)",
            "lw t2,  4 * 5(sp)",
            "lw t3,  4 * 6(sp)",
            "lw t4,  4 * 7(sp)",
            "lw t5,  4 * 8(sp)",
            "lw t6,  4 * 9(sp)",
            "lw a0,  4 * 10(sp)",
            "lw a1,  4 * 11(sp)",
            "lw a2,  4 * 12(sp)",
            "lw a3,  4 * 13(sp)",
            "lw a4,  4 * 14(sp)",
            "lw a5,  4 * 15(sp)",
            "lw a6,  4 * 16(sp)",
            "lw a7,  4 * 17(sp)",
            "lw s0,  4 * 18(sp)",
            "lw s1,  4 * 19(sp)",
            "lw s2,  4 * 20(sp)",
            "lw s3,  4 * 21(sp)",
            "lw s4,  4 * 22(sp)",
            "lw s5,  4 * 23(sp)",
            "lw s6,  4 * 24(sp)",
            "lw s7,  4 * 25(sp)",
            "lw s8,  4 * 26(sp)",
            "lw s9,  4 * 27(sp)",
            "lw s10, 4 * 28(sp)",
            "lw s11, 4 * 29(sp)",
            "lw sp,  4 * 30(sp)",
            "sret",
            options(noreturn),
        );
    }
}

#[repr(C, packed)]
#[derive(Debug)]
struct TrapFrame {
    ra: u32,
    gp: u32,
    tp: u32,
    t0: u32,
    t1: u32,
    t2: u32,
    t3: u32,
    t4: u32,
    t5: u32,
    t6: u32,
    a0: u32,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
    a7: u32,
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
    s5: u32,
    s6: u32,
    s7: u32,
    s8: u32,
    s9: u32,
    s10: u32,
    s11: u32,
    sp: u32,
}

#[no_mangle]
fn handle_trap(f: *mut TrapFrame) {
    let scause: u32 = read_csr!("scause");
    let stval: u32 = read_csr!("stval");
    let user_pc: u32 = read_csr!("sepc");

    panic!(
        "unexpected trap scause={}, stval={}, sepc={}",
        scause, stval, user_pc
    );
}
