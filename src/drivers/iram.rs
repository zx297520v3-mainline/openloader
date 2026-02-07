use crate::drivers::{Driver, readl, writel};

const IRAM1_BASE: usize = 0x102000;

const IRAM2_BASE: usize = 0x82000400;
const IRAM2_END: usize = 0x82003404;
const IRAM2_SWITCH_ADDR: usize = 0x82002bc0;

pub struct IRAM;
impl Driver for IRAM {
    unsafe fn init() {
        unsafe {
            writel(IRAM1_BASE, 0);

            let value = readl(IRAM2_SWITCH_ADDR);
            for i in (IRAM2_BASE..IRAM2_END).step_by(4) {
                writel(i, 0);
            }
            writel(IRAM2_SWITCH_ADDR, value);
        }
    }
}
