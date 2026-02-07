#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

use ufmt::uwriteln;

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {}
}

global_asm!(
    ".syntax unified
    .code 16

    .global start
    .section .text.start
    start:
        b main"
);

mod drivers;
use drivers::uart::Serial;

use crate::drivers::Driver;
use crate::drivers::clk::Clock;
use crate::drivers::efuse::Efuse;
use crate::drivers::iram::IRAM;

unsafe fn early_init() {
    uwriteln!(&mut Serial, "Early init triggered");

    uwriteln!(&mut Serial, "Clock init");
    unsafe { Clock::init() };

    uwriteln!(&mut Serial, "UART re-init");
    unsafe { Serial::init() };

    uwriteln!(&mut Serial, "Early init finished");
}

unsafe fn init() {
    uwriteln!(&mut Serial, "Init triggered");

    uwriteln!(&mut Serial, "IRAM setup");
    unsafe { IRAM::init() };

    uwriteln!(&mut Serial, "Efuse init");
    unsafe { Efuse::init() };
    uwriteln!(&mut Serial, "Efuse provided info:");
    uwriteln!(
        &mut Serial,
        "\tFused device: {}",
        if unsafe { Efuse::is_fused() } {
            "yes"
        } else {
            "no"
        }
    );
    uwriteln!(&mut Serial, "\tDRAM size: {}", unsafe {
        Efuse::dram_size()
    });

    uwriteln!(&mut Serial, "Init finished");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    uwriteln!(&mut Serial, "Hello from Rust :)");

    unsafe {
        early_init();
        init();
    }

    uwriteln!(&mut Serial, "All done, spinning forever");
    loop {}
}
