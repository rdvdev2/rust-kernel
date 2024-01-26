#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(
    ".global _start",
    "_start:",
    "mov ebp, [{} + 0x4000]",
    "call {}",
    sym KERNEL_STACK,
    sym kernel_main,
);

extern "C" fn kernel_main() -> ! {
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
    end_type: u16,
    end_flags: u16,
    end_size: u32,
}

#[link_section = ".multiboot"]
#[used]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0xE85250D6,
    architecture: 0,
    header_length: 24,
    checksum: -(0xE85250D6u32 as i32 + 24) as u32,
    end_type: 0,
    end_flags: 0,
    end_size: 0,
};

static KERNEL_STACK: [u32; 0x1000] = [0; 0x1000];
