// acpi

use crate::{print,println};

#[repr(C, packed)] // necessary
#[derive(Debug, Copy, Clone)] // not necessary
pub struct RSDP {
	signature: [u8; 8],
	checksum: u8,
	oemid: [u8; 6],
	revision: u8,
	rsdt_address: u32,
}

pub fn rsdt_init(rsdp: RSDP) {
	println!("{}", rsdp.revision);
}