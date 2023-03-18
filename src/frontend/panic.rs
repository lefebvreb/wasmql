use core::panic::PanicInfo;

use alloc::string::ToString;

use super::ffi;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    let val = ffi::from_string("wasmql codec panic");
    ffi::throw(val);
}