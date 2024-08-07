use core::arch::asm;

pub struct SbiRet {
    _error: i32,
    _value: i32,
}

impl SbiRet {
    pub unsafe fn sbi_call(
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
        fid: i32,
        eid: i32,
    ) -> Self {
        let mut error;
        let mut value;
        asm!(
            "ecall",
            inout("a0") arg0 => error, inout("a1") arg1 => value,
            in("a2") arg2, in("a3") arg3, in("a4") arg4, in("a5") arg5,
            in("a6") fid, in("a7") eid
        );
        Self {
            _error: error,
            _value: value,
        }
    }
}
