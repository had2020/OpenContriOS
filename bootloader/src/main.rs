#![no_main]
#![no_std]

use core::time::Duration;
use log::info;
use uefi::boot::exit_boot_services;
use uefi::boot::memory_map;
use uefi::prelude::*;
//use uefi::system USE later
use core::{mem, ptr};
use uefi::boot;
use uefi::table;

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: FramebufferInfo,
    pub memory_map_ptr: u64, // later
    pub memory_map_len: usize,
    pub rsdp_addr: u64,
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
}

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    info!("Initiating OpenContriOS Bootloader.");
    info!("uefi::helpers::init success! (Logging, and Allocator) Now Online!");
    // now we have uefi::allocator::Allocator!

    /* TODO
    // memory map
    let mem_map = memory_map();

    unsafe {
        exit_boot_services(custom_memory_type);
    }
    */

    Status::SUCCESS
}
