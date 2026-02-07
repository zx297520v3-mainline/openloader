use crate::drivers::{Driver, bit, readl, writel};

const TOPCRM_BASE: usize = 0x13b000;
const TOPCRM_MPLL_CFG0: usize = TOPCRM_BASE + 0x008;
const TOPCRM_UPLL_CFG0: usize = TOPCRM_BASE + 0x010;
const TOPCRM_GPLL_CFG0: usize = TOPCRM_BASE + 0x110;
const FLAG_LOCKED: usize = bit(30);

const TOPCRM_M0_SEL: usize = TOPCRM_BASE + 0x038;
const TOPCRM_HS_CLK: usize = TOPCRM_BASE + 0x03c;

const MATRIX_BASE: usize = 0x1306000;
const MATRIX_SEL: usize = MATRIX_BASE;
const MATRIX_PS_SEL: usize = MATRIX_BASE + 0x20;
const MATRIX_PHY_SEL: usize = MATRIX_BASE + 0x30;
const MATRIX_AP_SEL: usize = MATRIX_BASE + 0x40;

pub struct Clock;
impl Driver for Clock {
    unsafe fn init() {
        unsafe {
            writel(TOPCRM_MPLL_CFG0, 0x8040c11);
            writel(TOPCRM_UPLL_CFG0, 0x8347811);
            writel(TOPCRM_GPLL_CFG0, 0x8347d29);

            while Self::is_locked(TOPCRM_MPLL_CFG0) {}
            while Self::is_locked(TOPCRM_UPLL_CFG0) {}
            while Self::is_locked(TOPCRM_GPLL_CFG0) {}

            writel(TOPCRM_M0_SEL, 5);
            writel(TOPCRM_HS_CLK, 0x10);
            writel(MATRIX_SEL, 0x10001);
            writel(MATRIX_PS_SEL, 1);
            writel(MATRIX_PHY_SEL, 1);
            writel(MATRIX_AP_SEL, 1);
        }
    }
}

impl Clock {
    #[inline(always)]
    unsafe fn is_locked(reg: usize) -> bool {
        (unsafe { readl(reg) } & FLAG_LOCKED) == 0
    }
}
