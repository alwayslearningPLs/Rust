#![no_std]
#![no_main]

use core::panic::PanicInfo;

// double check version: rustc --version --verbose
// Rust assumes that we have an underlying OS with a C runtime, so we need to add an empty host
// to the rustc env, using `rustup target add thumbv7em-none-eabihf`
// and to build with that target in mind: `cargo build --target thumbv7em-none-eabihf`

// In a typical Rust binary that links the stdlib, execution starts in a C runtime library called
// crt0 ("C RunTime 0")
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop { }
}

// The function should never return, so it is marked as a _diverging function_ by returning the
// "never" type !.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
