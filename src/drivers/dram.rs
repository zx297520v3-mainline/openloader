use ufmt::{uDisplay, uwrite};

pub enum DramSize {
    Dram32M,
    Dram64M,
    Dram128M,
    Dram256M,
    Dram512M,
}

impl uDisplay for DramSize {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            Self::Dram32M => uwrite!(f, "32 MB"),
            Self::Dram64M => uwrite!(f, "64 MB"),
            Self::Dram128M => uwrite!(f, "128 MB"),
            Self::Dram256M => uwrite!(f, "256 MB"),
            Self::Dram512M => uwrite!(f, "512 MB"),
        }
    }
}
