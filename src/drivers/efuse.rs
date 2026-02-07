use crate::drivers::{Driver, bit, dram::DramSize, readl, writel};

const EFUSE_BASE: usize = 0x121b000;
const EFUSE_CONTROL: usize = EFUSE_BASE + 0x4;
const EFUSE_STATUS: usize = EFUSE_BASE + 0x14;

const FLAG_BUSY: usize = bit(0);
const FLAG_SETUP_NOT_DONE: usize = bit(1);

const EFUSE_RAM_BASE: usize = EFUSE_BASE + 0x40;
const EFUSE_SECURE_FLAG: usize = EFUSE_RAM_BASE;

pub struct Efuse;
impl Driver for Efuse {
    unsafe fn init() {
        unsafe {
            while readl(EFUSE_CONTROL) & FLAG_BUSY == 1 {}
            writel(EFUSE_CONTROL, 1);
            while readl(EFUSE_STATUS) & FLAG_SETUP_NOT_DONE == 0 {}
        }
    }
}

const WINBOND_256M: usize = 0xF86308;
const UNILC_256M: usize = 0xF86309;
const AP_MEMORY_256M: usize = 0xF8630a;
const WINBOND_256M_2: usize = 0x1E8724;
const UNILC_256M_2: usize = 0x1E8725;
const AP_MEMORY_256M_2: usize = 0x1E8726;

const UNILC_512M: usize = 0xF86304;
const AP_MEMORY_512M: usize = 0xF86305;
const ESMT_512M: usize = 0xF8630b;
const UNILC_512M_2: usize = 0x1E8720;
const AP_MEMORY_512M_2: usize = 0x1E8721;
const ESMT_512M_2: usize = 0x1E8727;
const UNILC_512M_3: usize = 0x1F9801;

const NYC_2G: usize = 0xF86311;
const NYC_2G_2: usize = 0xF86314;
const NYC_2G_3: usize = 0xF86316;
const NYC_2G_4: usize = 0xF86317;

const NYB_4G: usize = 0xF86313;
const NYB_4G_2: usize = 0xF86315;

impl Efuse {
    pub unsafe fn dram_size() -> DramSize {
        match unsafe { readl(EFUSE_SECURE_FLAG) } >> 8 {
            WINBOND_256M | WINBOND_256M_2 | UNILC_256M | UNILC_256M_2 | AP_MEMORY_256M
            | AP_MEMORY_256M_2 => DramSize::Dram32M,

            UNILC_512M | AP_MEMORY_512M | ESMT_512M | UNILC_512M_2 | UNILC_512M_3
            | AP_MEMORY_512M_2 | ESMT_512M_2 => DramSize::Dram64M,

            NYC_2G | NYC_2G_2 | NYC_2G_3 | NYC_2G_4 => DramSize::Dram256M,

            NYB_4G | NYB_4G_2 => DramSize::Dram512M,

            _ => DramSize::Dram128M,
        }
    }

    #[inline(always)]
    pub unsafe fn is_fused() -> bool {
        (unsafe { readl(EFUSE_SECURE_FLAG) } & 0xff) != 0
    }
}
