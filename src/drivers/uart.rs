use ufmt::uWrite;

use crate::drivers::{Driver, bit, readl, writel};

const UART_CLOCK: usize = 26000000;
const UART_BAUD: usize = 115200;

const UART1_BASE: usize = 0x01408000;
const UART_DR: usize = UART1_BASE + 0x04;
const UART_FR: usize = UART1_BASE + 0x14;
const UART_IBRD: usize = UART1_BASE + 0x24;
const UART_FBRD: usize = UART1_BASE + 0x28;
const UART_LCR: usize = UART1_BASE + 0x30;
const UART_CR: usize = UART1_BASE + 0x34;
const UART_IMSC: usize = UART1_BASE + 0x40;
const UART_ICR: usize = UART1_BASE + 0x4c;

const FLAG_ENABLE: usize = bit(0);
const FLAG_TX_ENABLE: usize = bit(8);

const FLAG_BREAK: usize = bit(0);
const FLAG_PARITY: usize = bit(1);
const FLAG_TWO_STOP_BITS: usize = bit(3);
const FLAG_FIFO: usize = bit(4);
const FLAG_TX_8BITS: usize = 3 << 5;

const FLAG_BUSY: usize = bit(8);

pub struct Serial;
impl Serial {
    fn raw_putc(c: u8) {
        unsafe {
            while Self::busy() {}
            writel(UART_DR, c as usize);
        };
    }

    pub fn putc(c: u8) {
        if c == b'\n' {
            Self::raw_putc(b'\r');
        }
        Self::raw_putc(c);
    }

    #[inline(always)]
    unsafe fn busy() -> bool {
        (unsafe { readl(UART_FR) } & FLAG_BUSY) != 0
    }

    unsafe fn setbrg() {
        const IBRD: usize = UART_CLOCK / (UART_BAUD << 4);
        const FBRD: usize =
            (((UART_CLOCK - ((IBRD * UART_BAUD) << 4)) << 6) + (UART_BAUD << 3)) / (UART_BAUD << 4);

        unsafe {
            writel(UART_IBRD, IBRD);
            writel(UART_FBRD, FBRD);
        }
    }
}

impl uWrite for Serial {
    type Error = core::convert::Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for c in s.as_bytes() {
            Self::putc(*c);
        }
        Ok(())
    }
}

impl Driver for Serial {
    unsafe fn init() {
        unsafe {
            while Self::busy() {}

            // Stop
            writel(UART_ICR, 0xffff);
            writel(UART_CR, !(FLAG_ENABLE | FLAG_TX_ENABLE));

            // Set baud
            Self::setbrg();

            writel(
                UART_LCR,
                readl(UART_LCR) & !(FLAG_BREAK | FLAG_PARITY | FLAG_TWO_STOP_BITS),
            );
            writel(UART_LCR, FLAG_FIFO | FLAG_TX_8BITS);

            // Disable interrupts
            writel(UART_IMSC, 0);

            writel(UART_CR, FLAG_ENABLE | FLAG_TX_ENABLE);
        }
    }
}
