#![allow(non_snake_case)]
extern crate libc;
use std::ffi::CString;
use std::convert::TryInto;

type VALUE = libc::c_ulong;

extern {
    fn rb_define_module(name: *const libc::c_char) -> VALUE;
    fn rb_define_module_function(module: VALUE,
                                 name: *const libc::c_char,
                                 value: extern fn() -> VALUE,
                                 argc: libc::c_int) -> libc::c_void;
    fn rb_str_new(prt: *const libc::c_char, len: libc::c_long) -> VALUE;
}

extern fn rb_greet() -> VALUE {
    let result = CString::new("Hello world, This is Rust extension").unwrap();
    unsafe {
        rb_str_new(
            result.as_ptr(),
            libc::strlen(result.as_ptr()).try_into().unwrap()
        )
    }
}

#[no_mangle]
pub extern fn Init_fizzbuzz() {
    let module_name = CString::new("RustEx").unwrap();
    let greet = CString::new("greet").unwrap();

    unsafe {
        let rb_cRustEx = rb_define_module(module_name.as_ptr());
        rb_define_module_function(rb_cRustEx, greet.as_ptr(), rb_greet, 0);
    }
}
