#![no_std]

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
