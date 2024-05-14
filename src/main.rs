#![no_std]
#![no_main]

mod PL011;
use PL011::pl011;
use core::panic::PanicInfo;

/// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    let uart = pl011::new();


    loop {}
}
