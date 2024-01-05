#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern "C" fn _start() -> ! {
    let screen = 0xB8000 as *mut u16;
    unsafe { screen.write_volatile(0x0741) };

    loop {}
}

#[repr(C)]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
}

#[link_section = ".multiboot"]
#[used]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0xE85250D6,
    architecture: 0,
    header_length: 16,
    checksum: -(0xE85250D6u32 as i32 + 16) as u32,
};
