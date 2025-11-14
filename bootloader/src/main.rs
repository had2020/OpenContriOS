#![no_std]
#![no_main]

#![feature(lang_items)]

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

//use core::panic::PanicInfo;
//use core::sync::atomic;
//use core::sync::atomic::Ordering;

#[unsafe(no_mangle)]
fn _start() -> ! {
    loop {}
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
       // atomic::compiler_fence(Ordering::SeqCst);
    }
}

