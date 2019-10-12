#![crate_type="dylib"]

#[no_mangle]
pub extern fn greet() -> String {
    return "Hello world, This is Rust extension".to_string();
}
