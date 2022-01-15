use crate::*;

pub trait ReadOnlyRegister: From<u8> {
    const ADDR: u8;

    fn read<E, I2C: i2c::WriteRead<Error = E>>(i2c: &mut I2C) -> Result<Self, E> {
        let buf = &mut [0u8; 1];
        i2c.write_read(I2C_ADDR, &[Self::ADDR], buf)
            .map(|_| buf[0].into())
    }
}

impl<RWR: ReadWriteRegister> ReadOnlyRegister for RWR {
    const ADDR: u8 = RWR::ADDR;
}

pub trait ReadWriteRegister: From<u8> + Into<u8> {
    const ADDR: u8;

    fn write<E, I2C: i2c::Write<Error = E>>(self, i2c: &mut I2C) -> Result<(), E> {
        i2c.write(I2C_ADDR, &[Self::ADDR, self.into()])
    }
}

#[macro_export]
macro_rules! register {
    ($Reg:ident, $addr:literal, RO) => {
        impl ReadOnlyRegister for $Reg {
            const ADDR: u8 = $addr;
        }

        impl From<u8> for $Reg {
            fn from(raw: u8) -> Self {
                Self::from_bytes([raw])
            }
        }
    };
    ($Reg:ident, $addr:literal, RW) => {
        impl ReadWriteRegister for $Reg {
            const ADDR: u8 = $addr;
        }

        impl From<u8> for $Reg {
            fn from(raw: u8) -> Self {
                Self::from_bytes([raw])
            }
        }

        impl From<$Reg> for u8 {
            fn from(reg: $Reg) -> Self {
                reg.into_bytes()[0]
            }
        }
    };
}

#[macro_export]
macro_rules! register_map {
    ($($Reg:ident: $addr:literal, $rw:tt,)+) => {
        $(
            register!($Reg, $addr, $rw);
        )+
    };
}
