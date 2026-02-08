use crate::drivers::{bit, genmask, readl, writel};

pub mod pll;
pub mod soc;

pub trait Gate {
    const ADDR: usize;
    const SHIFT: usize;

    unsafe fn gate() {
        unsafe {
            writel(Self::ADDR, readl(Self::ADDR) & !bit(Self::SHIFT));
        }
    }
    unsafe fn ungate() {
        unsafe {
            writel(Self::ADDR, readl(Self::ADDR) | bit(Self::SHIFT));
        }
    }
}

macro_rules! gate {
    ($name: ident, $addr:expr, $shift:expr) => {
        struct $name;

        impl Gate for $name {
            const ADDR: usize = $addr;
            const SHIFT: usize = $shift;
        }
    };
}

pub trait Mux {
    type Parent: Copy + Into<usize>;

    const ADDR: usize;
    const SHIFT: usize;
    const WIDTH: usize;

    unsafe fn set_parent(parent: Self::Parent) {
        unsafe {
            let mut value = readl(Self::ADDR);
            value &= !genmask(Self::SHIFT + Self::WIDTH - 1, Self::SHIFT);
            value |= parent.into() << Self::SHIFT;
            writel(Self::ADDR, value)
        }
    }
}

pub trait Parents {
    const COUNT: usize;
}

macro_rules! parents {
    ($name:ident : $($variant:ident),+ $(,)?) => {
        #[derive(Copy, Clone)]
        #[repr(u8)]
        enum $name {
            $($variant),+
        }

        impl From<$name> for usize {
            #[inline(always)]
            fn from(p: $name) -> usize {
                p as usize
            }
        }

        impl super::Parents for $name {
            const COUNT: usize = {
                let mut n = 0;
                $(let _ = $name::$variant; n += 1;)+
                n
            };
        }
    };
}

macro_rules! mux {
    ($name:ident, $addr:expr, $shift:expr, $width:expr, $parent:ty) => {
        struct $name;

        impl Mux for $name {
            type Parent = $parent;

            const ADDR: usize = $addr;
            const SHIFT: usize = $shift;
            const WIDTH: usize = $width;
        }

        const _: () = {
            if <$parent as super::Parents>::COUNT > (1 << $width) {
                panic!("mux width too small for parent enum");
            }
        };
    };
}

pub(super) use gate;
pub(super) use mux;
pub(super) use parents;
