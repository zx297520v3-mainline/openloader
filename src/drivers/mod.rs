pub mod clk;
pub mod dram;
pub mod efuse;
pub mod iram;
pub mod uart;

pub trait Driver {
    unsafe fn init();
}

pub(super) unsafe fn readl_raw<T>(reg: *const T) -> T {
    unsafe { reg.read_volatile() }
}

pub(super) unsafe fn writel_raw<T>(reg: *mut T, value: T) {
    unsafe { reg.write_volatile(value) };
}

pub(super) unsafe fn readl(reg: usize) -> usize {
    unsafe { readl_raw(reg as *const usize) }
}

pub(super) unsafe fn writel(reg: usize, value: usize) {
    unsafe { writel_raw(reg as *mut usize, value) };
}

pub(super) const fn bit(n: usize) -> usize {
    1 << n
}

pub(super) const fn genmask(h: usize, l: usize) -> usize {
    (!0 << l) & (!0 >> (32 - 1 - h))
}
