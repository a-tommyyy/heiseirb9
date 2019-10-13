#![allow(non_snake_case)]
extern crate libc;
use std::ffi::CString;
use std::thread;

type VALUE = libc::c_ulong;

extern {
    fn rb_define_module(name: *const libc::c_char) -> VALUE;
    fn rb_define_module_function(module: VALUE,
                                 name: *const libc::c_char,
                                 value: extern fn(),
                                 argc: libc::c_int) -> libc::c_void;
}

extern fn rb_process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(move || {
            let mut x = 0;
            for _ in 0..5_000_000 {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    };
}

#[no_mangle]
pub extern fn Init_rustex() {
    let module_name = CString::new("RustEx").unwrap();
    let process = CString::new("process").unwrap();

    unsafe {
        let rb_cRustEx = rb_define_module(module_name.as_ptr());
        rb_define_module_function(rb_cRustEx, process.as_ptr(), rb_process, 0);
    }
}
