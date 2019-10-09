#![allow(non_snake_case)]
extern crate libc;
use std::ffi::CString;

type VALUE = libc::c_ulong;

extern {
    static rb_cObject: VALUE;

    fn rb_define_class(name: *const libc::c_char, rb_super: VALUE) -> VALUE;
}

#[no_mangle]
pub extern fn Init_fizzbuzz() {
    let klass_name = CString::new("FizzBuzz").unwrap();
    unsafe {
        let _rb_cFizzBuzz = rb_define_class(klass_name.as_ptr(), rb_cObject);
    }
}
