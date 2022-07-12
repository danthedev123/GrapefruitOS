#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod terminal;
mod idt;
mod acpi;

use core::{panic::PanicInfo};
use limine::*;

use crate::{idt::load_idt};


static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);
static BOOTLOADER_INFO: LimineBootInfoRequest = LimineBootInfoRequest::new(0);
static MEMORY_MAP: LimineMmapRequest = LimineMmapRequest::new(0);
static RSDP: LimineRsdpRequest = LimineRsdpRequest::new(0);
static ENTRY_POINT_REQUEST: LimineEntryPointRequest =  LimineEntryPointRequest::new(0);

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

        let rsdp_addr = &RSDP
            .get_response()
            .get()
            .expect("Recieved no RSDP")
            .address;

        
        println!("Recieved RSDP: {:#?}", rsdp_addr);

        println!("Printing RSDP signature");


        unsafe {
            for i in 0..8 {
                print!("{}", *(rsdp_addr.as_ptr().expect("No rsd ptr found").offset(i)) as char);
            }
        }

        //24

        unsafe {
            print!("{}", *(rsdp_addr.as_ptr().expect("No rsd ptr found").offset(24)) as i32);
        }


        unsafe { acpi::rsdt_init(*(rsdp_addr.as_ptr().expect("No rsd ptr found. ") as *const acpi::RSDP)) };

        let entry_point = ENTRY_POINT_REQUEST
        .get_response()
        .get()
        .unwrap()
        .revision;
        


        println!("{}", entry_point);


        loop {
            
        }
    
}