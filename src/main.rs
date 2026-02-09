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
        ldr r1, =0
        ldr r1, [r1]
        msr msp, r1
        ldr r0, =main
        bx r0"
);

mod drivers;
use drivers::uart::Serial;

use crate::drivers::clk::pll::PLL;
use crate::drivers::clk::soc::SoCClocks;
use crate::drivers::dram::Dram;
use crate::drivers::efuse::Efuse;
use crate::drivers::iram::IRAM;
use crate::drivers::usb::Usb;
use crate::drivers::zte_protocol::ZteProtocol;
use crate::drivers::{Driver, DriverMut, StatelessDriver};

unsafe fn early_init() {
    uwriteln!(&mut Serial, "Early init triggered");

    uwriteln!(&mut Serial, "PLL init");
    unsafe { PLL::init() };

    uwriteln!(&mut Serial, "Clock init");
    unsafe { SoCClocks::init() };

    uwriteln!(&mut Serial, "UART re-init");
    unsafe { Serial::init() };

    uwriteln!(&mut Serial, "Early init finished");
}

unsafe fn init() {
    uwriteln!(&mut Serial, "Init triggered");

    uwriteln!(&mut Serial, "IRAM setup");
    unsafe { IRAM::init() };

    uwriteln!(&mut Serial, "Efuse init");
    let efuse = unsafe { Efuse::init() };
    uwriteln!(&mut Serial, "Efuse provided info:");
    uwriteln!(
        &mut Serial,
        "\tFused device: {}",
        if efuse.secure { "yes" } else { "no" }
    );
    uwriteln!(&mut Serial, "\tDRAM size: {}", efuse.dram_size);

    uwriteln!(&mut Serial, "DRAM init");
    unsafe { Dram::new(efuse.dram_size).init() };

    uwriteln!(&mut Serial, "Init finished");
}

unsafe fn late_init() {
    uwriteln!(&mut Serial, "Late init triggered");

    unsafe {
        let mut usb = Usb::new();
        usb.init();

        let mut protocol = ZteProtocol::new(usb);
        protocol.dispatch();
    }

    uwriteln!(&mut Serial, "Late init finished");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> ! {
    uwriteln!(&mut Serial, "Hello from Rust :)");

    unsafe {
        early_init();
        init();
        late_init();
    }

    uwriteln!(&mut Serial, "All done, spinning forever");

    loop {}
}
