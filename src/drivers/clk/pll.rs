use crate::drivers::{Driver, bit, readl, writel};

pub(super) const TOPCRM_BASE: usize = 0x13b000;
const TOPCRM_MPLL_CFG0: usize = TOPCRM_BASE + 0x008;
const TOPCRM_UPLL_CFG0: usize = TOPCRM_BASE + 0x010;
const TOPCRM_GPLL_CFG0: usize = TOPCRM_BASE + 0x110;
const FLAG_LOCKED: usize = bit(30);

pub struct PLL;
impl Driver for PLL {
    unsafe fn init() {
        unsafe {
            writel(TOPCRM_MPLL_CFG0, 0x8040c11);
            writel(TOPCRM_UPLL_CFG0, 0x8347811);
            writel(TOPCRM_GPLL_CFG0, 0x8347d29);

            while Self::is_locked(TOPCRM_MPLL_CFG0) {}
            while Self::is_locked(TOPCRM_UPLL_CFG0) {}
            while Self::is_locked(TOPCRM_GPLL_CFG0) {}
        }
    }
}

impl PLL {
    #[inline(always)]
    unsafe fn is_locked(reg: usize) -> bool {
        (unsafe { readl(reg) } & FLAG_LOCKED) == 0
    }
}
