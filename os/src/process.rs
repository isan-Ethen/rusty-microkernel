use common::VAddr;

use core::arch::asm;
use core::cmp::{Ord, Ordering};

pub const PROCS_MAX: u32 = 8;
pub const PROC_UNUSED: u32 = 0;
pub const PROC_RUNNABLE: u32 = 1;

#[derive(Copy, Clone)]
pub struct Process {
    pub pid: i32,
    pub state: u32,
    pub sp: VAddr,
    pub page_table: u32,
    pub stack: [u8; 8192],
}

impl Process {
    pub fn new() -> Process {
        Process {
            pid: 0,
            state: PROC_UNUSED,
            sp: 0,
            page_table: 0,
            stack: [0; 8192],
        }
    }

    pub fn set_pid(&mut self, pid: i32) {
        self.pid = pid;
    }

    pub fn set_state(&mut self, state: u32) {
        self.state = state;
    }

    pub fn set_sp(&mut self, sp: VAddr) {
        self.sp = sp;
    }

    pub fn set_page_table(&mut self, page_table: u32) {
        self.page_table = page_table;
    }
}

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
            && self.state == other.state
            && self.sp == other.sp
            && self.stack == other.stack
    }
}

impl Eq for Process {}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pid.cmp(&other.pid)
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn switch_context(prev_sp: *mut u32, next_sp: *const u32) {
    unsafe {
        asm!(
            "addi sp, sp, -13 * 4",
            "sw ra,  0  * 4(sp)",
            "sw s0,  1  * 4(sp)",
            "sw s1,  2  * 4(sp)",
            "sw s2,  3  * 4(sp)",
            "sw s3,  4  * 4(sp)",
            "sw s4,  5  * 4(sp)",
            "sw s5,  6  * 4(sp)",
            "sw s6,  7  * 4(sp)",
            "sw s7,  8  * 4(sp)",
            "sw s8,  9  * 4(sp)",
            "sw s9,  10 * 4(sp)",
            "sw s10, 11 * 4(sp)",
            "sw s11, 12 * 4(sp)",
            "sw sp, (a0)",
            "lw sp, (a1)",
            "lw ra,  0  * 4(sp)",
            "lw s0,  1  * 4(sp)",
            "lw s1,  2  * 4(sp)",
            "lw s2,  3  * 4(sp)",
            "lw s3,  4  * 4(sp)",
            "lw s4,  5  * 4(sp)",
            "lw s5,  6  * 4(sp)",
            "lw s6,  7  * 4(sp)",
            "lw s7,  8  * 4(sp)",
            "lw s8,  9  * 4(sp)",
            "lw s9,  10 * 4(sp)",
            "lw s10, 11 * 4(sp)",
            "lw s11, 12 * 4(sp)",
            "addi sp, sp, 13 * 4",
            "ret",
            options(noreturn),
        );
    }
}
