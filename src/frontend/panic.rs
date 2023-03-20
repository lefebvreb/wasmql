use core::panic::PanicInfo;

use alloc::string::ToString;

use super::{ffi, JsValue};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    match info.payload().downcast_ref::<&str>() {
        Some(s) => JsValue::from_string(s),
        _ => JsValue::undefined(),
    }.throw()
}