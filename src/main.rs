#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

mod multiboot2;

static KERNEL_STACK: [u32; 0x1000] = [0; 0x1000];

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
