use ufmt::{uDisplay, uwrite};

pub enum Error {
    DRAM,
    USB(USBError),
}

impl From<USBError> for Error {
    fn from(value: USBError) -> Self {
        Self::USB(value)
    }
}

impl uDisplay for Error {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            Self::USB(usb) => uwrite!(f, "USB: {}", usb),
            Self::DRAM => uwrite!(f, "DRAM R/W test failed"),
        }
    }
}

pub enum USBError {
    Timeout,
}

impl uDisplay for USBError {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self {
            Self::Timeout => uwrite!(f, "Timed out"),
        }
    }
}
