#![no_std]
#![no_main]

use core::fmt::Write;
use core::{arch::global_asm, panic::PanicInfo};

use vga::{ColorPalette, Screen, ScreenController};

mod multiboot2;
mod vga;

static KERNEL_STACK: [u32; 0x1000] = [0; 0x1000];

#[panic_handler]
fn panic_handler(pi: &PanicInfo) -> ! {
    let mut screen = ScreenController::new(
        Screen::get(),
        ColorPalette::new(vga::Color::White, vga::Color::Red),
    );
    screen.reset();

    let header = ">> KERNEL PANIC! <<";
    let padding = (vga::SCREEN_WIDTH - header.len()) / 2;

    write!(screen, "\n").unwrap();
    for _ in 0..padding {
        write!(screen, " ").unwrap();
    }

    write!(screen, "{}\n\n{}", header, pi).unwrap();

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
    let mut screen = ScreenController::new(Screen::get(), Default::default());
    screen.reset();

    writeln!(screen, "Hello, kernel!").unwrap();

    panic!("Test kernel panic");
}
