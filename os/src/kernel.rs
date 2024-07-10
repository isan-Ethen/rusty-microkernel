#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(panic_info_message)]
#![allow(unused)]

use common::{print, println, Argument};

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

static mut proc_a: Process = Process {
    pid: 0,
    state: PROC_UNUSED,
    sp: 0,
    page_table: 0,
    stack: [0; 8192],
};

static mut proc_b: Process = Process {
    pid: 0,
    state: PROC_UNUSED,
    sp: 0,
    page_table: 0,
    stack: [0; 8192],
};

use process::switch_context;

#[no_mangle]
unsafe fn proc_a_entry() {
    println("starting process A", &[]);
    loop {
        putchar('A');
        _yield();

        for i in 0..3000000 {
            asm!("nop");
        }
    }
}

#[no_mangle]
unsafe fn proc_b_entry() {
    println("starting process B", &[]);
    loop {
        putchar('B');
        _yield();

        for i in 0..3000000 {
            asm!("nop");
        }
    }
}

#[no_mangle]
unsafe fn idle() {
    println("idle!", &[]);
    loop {
        asm!("nop");
    }
}

#[no_mangle]
fn kernel_main() {
    unsafe {
        let bss = ptr::addr_of_mut!(__bss);
        let bss_end = ptr::addr_of!(__bss_end);
        ptr::write_bytes(bss, 0, bss_end as usize - bss as usize);
    }

    write_csr!("stvec", kernel_entry);

    unsafe {
        let mut idle_proc = create_process(idle as u32);
        idle_proc.set_pid(-1);
        CURRENT_PROC.store(idle_proc as *const _ as *mut _, Ordering::Release);
        IDLE_PROC = Some(*idle_proc);

        proc_a = *create_process(proc_a_entry as u32);
        proc_b = *create_process(proc_b_entry as u32);
    }
    _yield();
    panic!("switched to idle process")
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
        // println(": PanicInfo does not support argument", &[]);
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

    println(
        "unexpected trap scause=%u, stval=%u, sepc=%u",
        &[
            Argument::new_uint(scause),
            Argument::new_uint(stval),
            Argument::new_uint(user_pc),
        ],
    );

    panic!(
        "unexpected trap scause={}, stval={}, sepc={}",
        scause, stval, user_pc
    );
}

use common::{VAddr, PAGE_SIZE};
mod memory;
use memory::*;
mod process;
use process::*;

static mut PROCS: [Option<Process>; PROCS_MAX as usize] = [None; PROCS_MAX as usize];
extern "C" {
    static mut __kernel_base: u8;
}

unsafe fn create_process(pc: u32) -> &'static mut Process {
    let mut process: Option<&'static mut Process> = None;
    let mut i = 0;
    while i < PROCS_MAX {
        if let Some(p) = &mut PROCS[i as usize] {
            if p.state == PROC_UNUSED {
                process = Some(p);
                break;
            }
        } else {
            let proc = Some(Process::new());
            PROCS[i as usize] = proc;
            if let Some(p) = &mut PROCS[i as usize] {
                process = Some(p)
            } else {
                panic!("process have unundastandable bug!");
            }
            break;
        }
        i += 1;
    }

    if let Some(proc) = process {
        let stack = ptr::addr_of_mut!(proc.stack) as *mut u32;
        let sp = stack.add(proc.stack.len());
        *sp.offset(-1) = 0; //s11
        *sp.offset(-2) = 0; //s10
        *sp.offset(-3) = 0; //s9
        *sp.offset(-4) = 0; //s8
        *sp.offset(-5) = 0; //s7
        *sp.offset(-6) = 0; //s6
        *sp.offset(-7) = 0; //s5
        *sp.offset(-8) = 0; //s4
        *sp.offset(-9) = 0; //s3
        *sp.offset(-10) = 0; //s2
        *sp.offset(-11) = 0; //s1
        *sp.offset(-12) = 0; //s0
        *sp.offset(-13) = pc; //ra

        let mut page_table = alloc_pages(1);

        let free_ram_end = __free_ram_end as *const u32 as u32;
        let mut paddr = __kernel_base as *const u32 as u32;
        while paddr < free_ram_end {
            map_page(&mut page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
            paddr += PAGE_SIZE;
        }

        proc.set_pid(i as i32 + 1);
        proc.set_state(PROC_RUNNABLE);
        proc.set_sp(sp.offset(-13) as VAddr);
        proc.set_page_table(page_table);

        return proc;
    } else {
        panic!("no free process slots");
    };
}

use core::sync::atomic::{AtomicPtr, Ordering};

static CURRENT_PROC: AtomicPtr<Process> = AtomicPtr::new(core::ptr::null_mut());
static mut IDLE_PROC: Option<Process> = None;

fn _yield() {
    unsafe {
        let mut current_proc = CURRENT_PROC.load(Ordering::Acquire);
        if current_proc.is_null() || IDLE_PROC.is_none() {
            panic!("No current process or idle process");
        }

        let current_proc = &mut *current_proc;
        let idle = IDLE_PROC.as_mut().unwrap();

        let mut next = idle;

        for i in 0..PROCS_MAX {
            let num = (current_proc.pid + i as i32);
            if num >= 0 {
                let index = (num as usize % PROCS_MAX as usize);
                if let Some(proc) = &mut PROCS[index] {
                    if proc.state == PROC_RUNNABLE && proc.pid > 0 {
                        next = proc;
                        break;
                    }
                }
            }
        }

        if next == current_proc {
            return;
        }

        asm!(
            "sfence.vma",
            "csrw satp, {satp}",
            "sfence.vma",
            "csrw sscratch, {sscratch}",
            satp = in(reg) SATP_SV32 | next.page_table / PAGE_SIZE,
            sscratch = in(reg) next.stack.as_ptr_range().end as u32,
        );

        CURRENT_PROC.store(next as *const _ as *mut _, Ordering::Release);

        switch_context(&mut current_proc.sp as *mut u32, &mut next.sp as *mut u32);
    }
}
