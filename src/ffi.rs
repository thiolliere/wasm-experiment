#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::os::raw::{c_int, c_char};

pub type em_callback_func = Option<unsafe extern "C" fn()>;
pub type EM_BOOL = c_int;
pub const EM_TRUE: EM_BOOL = 1;
pub const EM_FALSE: EM_BOOL = 0;
extern "C" {
    pub fn emscripten_set_main_loop(
        func: em_callback_func, fps: c_int, simulate_infinite_loop: EM_BOOL);

    pub fn emscripten_asm_const(code: *const c_char, ... );
}
