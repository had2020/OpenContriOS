#![no_main]
#![no_std]

use core::ptr::NonNull;
use core::time::Duration;
use log::info;
use uefi::Identify;
use uefi::boot::OpenProtocolParams;
use uefi::boot::exit_boot_services;
use uefi::boot::memory_map;
use uefi::prelude::*;
use uefi::proto::console::gop::FrameBuffer;
use uefi::table::system_table_raw;
//use uefi::system USE later
use core::{mem, ptr};
use uefi::Handle;
use uefi::boot;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::proto::console::gop::ModeInfo;

#[repr(C)]
pub struct BootInfo {
    // frame_buffer
    pub frame_buffer_ptr: *mut u8, // virtual addr after map
    pub frame_buffer_base: u64,    // phys addr
    pub frame_buffer_size: u64,    // total bytes
    pub frame_buffer_resolution_x: u64,
    pub frame_buffer_resolution_y: u64,
    pub frame_buffer_pixel_format: u8, // UEFI magics PixelFormat
    pub frame_buffer_stride: usize,

    // Conventional Memory
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
    info!("uefi::helpers::init success! UEFI (Logging, and Allocator) Now Online!");

    /*
    unsafe {
        boot::open_protocol(
            OpenProtocolParams {
                handle: Handle::new(),
                agent: Handle::new(ptr),
                controller: OP,
            },
            attributes,
        )
    }
    *

    

    let gop: &mut GraphicsOutput = &mut GraphicsOutput;

    //let sys_table = system_table_raw().unwrap();

    //boot::open_protocol(params, attributes);

    //let gop_mode_info: ModeInfo = GraphicsOutput::current_mode_info();
    //let frame_buffer_info = GraphicsOutput::frame_buffer();

    /* TODO
    // memory map
    let mem_map = memory_map();

    unsafe {
        exit_boot_services(custom_memory_type);
    }
    */

    Status::SUCCESS
}
