#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C)]
pub struct EfiTable; // stub – define real types if you need them

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(_image: usize, _st: *mut EfiTable) -> usize {
    0 // EFI_SUCCESS
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // named `_start` by default
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
