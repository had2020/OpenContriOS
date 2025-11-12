#![no_main]
#![no_std]

use core::time::Duration;
use log::info;
use uefi::prelude::*;
//use uefi::system USE later
use alloc::vec;
use alloc::vec::Vec;
use core::{mem, ptr};
use uefi::boot;
use uefi::memory::{MemoryDescriptor, MemoryType};
use uefi::table;

// BootInfo
#[repr(C)]
pub struct BootInfo {
    // Memory map
    pub mem_map: *const EfiMemoryDesc,
    pub mem_map_len: usize, // bytes
    pub mem_map_desc_size: usize,

    // Framebuffer
    pub fb_base: u64, // phys addr
    pub fb_size: u64, // bytes
    pub fb_width: u32,
    pub fb_height: u32,
    pub fb_pitch: u32,                // pixels per scanline
    pub fb_format: FramebufferFormat, // 0=Rgb, 1=Bgr, 2=Bitmask

    // ACPI / SMBIOS
    pub acpi_rsdp: u64, // phys addr 0 if none
    pub smbios: u64,    // phys addr 0 if none

    // Boot device / path
    pub device_path: u64,
    pub device_path_len: u32,

    // Modules
    pub modules: *const Module,
    pub modules_len: usize,

    // Cmdline
    pub cmdline: *const u8,
    pub cmdline_len: usize,
}

#[repr(C)]
pub struct EfiMemoryDesc {
    pub typ: u32,
    pub pad: u32,
    pub phys_start: u64,
    pub virt_start: u64,
    pub page_count: u64,
    pub attr: u64,
}

#[repr(C)]
pub struct Module {
    pub start: u64,  // phys
    pub length: u64, // bytes
    pub name_ptr: u64,
    pub name_len: u32,
}

#[repr(u32)]
pub enum FramebufferFormat {
    Rgb,
    Bgr,
    Bitmask,
    Unknown,
}

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Initiating OpenContriOS Bootloader.");
    info!("Proceeding to idenfiy minimal BootInfo.");

    // memory map

    //boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}
