#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod terminal;
mod idt;

use core::{panic::PanicInfo};
use limine::*;

use crate::idt::load_idt;


static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);
static MEMORY_MAP: LimineMmapRequest = LimineMmapRequest::new(0);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Disable name mangling as this will cause problems
#[no_mangle]
extern "C" fn x86_64_main() -> ! {
    let bootloader_info = BOOTLOADER_INFO
        .get_response()
        .get()
        .expect("error: unable to get bootloader info");

    
        println!(
            "bootloader: name={}, version={}",
            bootloader_info.name.to_string().unwrap(),
            bootloader_info.version.to_string().unwrap()
        );

        let mmap = MEMORY_MAP
            .get_response()
            .get()
            .expect("Recieved no mmap response")
            .mmap()
            .expect("Recieved no mmap");

        println!("Memory map: \n{:#?}", mmap);

        load_idt();

        let mut usable_mem_size: u64 = 0;

        for area in mmap {
            if area.typ == LimineMemoryMapEntryType::Usable {
                usable_mem_size += area.len;
            }
        }

        println!("{} MiB", usable_mem_size / 1049000);


        loop {
            
        }
    
}