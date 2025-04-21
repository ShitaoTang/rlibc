extern "C" {
    pub fn __syscall_cp_asm(
        cancel_ptr: *const i32,
        nr: i64,
        u: i64,
        v: i64,
        w: i64,
        x: i64,
        _y: i64,
        _z: i64,
    ) -> i64;
}