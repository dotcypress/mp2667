use hal::blocking::i2c;
use modular_bitfield::prelude::*;

const I2C_ADDR: u8 = 0x09;

pub(crate) trait ReadOnlyRegister: From<u8> {
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

pub(crate) trait ReadWriteRegister: From<u8> + Into<u8> {
    const ADDR: u8;

    fn write<E, I2C: i2c::Write<Error = E>>(self, i2c: &mut I2C) -> Result<(), E> {
        i2c.write(I2C_ADDR, &[Self::ADDR, self.into()])
    }
}

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

macro_rules! register_map {
    ($($Reg:ident: $addr:literal, $rw:tt,)+) => {
        $(
            register!($Reg, $addr, $rw);
        )+
    };
}

register_map!(
    InputSourceControl: 0x00, RW,
    PowerOnConfiguration: 0x01, RW,
    ChargeCurrentControl: 0x02, RW,
    DischargeAndTerminationCurrent: 0x03, RW,
    ChargeVoltageControl: 0x04, RW,
    ChargeTerminationAndTimerControl: 0x05, RW,
    MiscellaneousOperationControl: 0x06, RW,
    SystemStatus: 0x07, RO,
    FaultFlags: 0x08, RO,
);

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ChargeStatus {
    NotCharging,
    PreCharge,
    Charge,
    ChargeDone,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SystemStatus {
    #[skip(setters)]
    pub thermal_regulation: bool,
    #[skip(setters)]
    pub power_good: bool,
    #[skip(setters)]
    pub power_path_enabled: bool,
    #[skip(setters)]
    pub charge_status: ChargeStatus,
    #[skip(setters)]
    pub revision: B2,
    #[skip]
    __: bool,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FaultFlags {
    #[skip]
    __: B2,
    #[skip(setters)]
    pub safety_timer_expired: bool,
    #[skip(setters)]
    pub battery_fault: bool,
    #[skip(setters)]
    pub thermal_shutdown: bool,
    #[skip(setters)]
    pub input_fault: bool,
    #[skip(setters)]
    pub watchdog_timer_expired: bool,
    #[skip]
    __: bool,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum ThermalThreshold {
    T60C,
    T80C,
    T100C,
    T120C,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MiscellaneousOperationControl {
    pub thermal_regulation_threshold: ThermalThreshold,
    #[skip]
    __: bool,
    pub ntc_enabled: bool,
    #[skip]
    __: bool,
    pub battery_fet_disabled: bool,
    pub extended_safety_timer: bool,
    #[skip]
    __: bool,
}

impl Default for MiscellaneousOperationControl {
    fn default() -> Self {
        Self {
            bytes: [0b0000_1011],
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum SafetyTimerPeriod {
    P20h,
    P5h,
    P8h,
    P12h,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum WatchdogTimerLimit {
    Disabled,
    L40s,
    L80s,
    L160s,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ChargeTerminationAndTimerControl {
    pub termination_control_enabled: bool,
    pub timer_period: SafetyTimerPeriod,
    pub timer_enabled: bool,
    pub timer_limit: WatchdogTimerLimit,
    pub termination_enabled: bool,
    #[skip]
    __: bool,
}

impl Default for ChargeTerminationAndTimerControl {
    fn default() -> Self {
        Self {
            bytes: [0b0100_1010],
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum RechargeThreshold {
    U150mV,
    U300mV,
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrechargeThreshold {
    U2800mV,
    U3000mV,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ChargeVoltageControl {
    pub recharge_threshold: RechargeThreshold,
    pub precharge_threshold: PrechargeThreshold,
    pub regulation_voltage: B6,
}

impl Default for ChargeVoltageControl {
    fn default() -> Self {
        Self {
            bytes: [0b1010_0011],
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum TerminalCurrent {
    I24mA,
    I52mA,
    I80mA,
    I108mA,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DischargeAndTerminationCurrent {
    pub terminal_current: TerminalCurrent,
    #[skip]
    __: bool,
    pub discharge_current_limit: B4,
    #[skip]
    __: bool,
}

impl Default for DischargeAndTerminationCurrent {
    fn default() -> Self {
        Self {
            bytes: [0b0100_1001],
        }
    }
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ChargeCurrentControl {
    pub charge_current: B5,
    #[skip]
    __: B3,
}

impl Default for ChargeCurrentControl {
    fn default() -> Self {
        Self {
            bytes: [0b0000_0111],
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum UVLOThreshold {
    U2400mV,
    U2500mV,
    U2600mV,
    U2700mV,
    U2800mV,
    U2900mV,
    U3000mV,
    U3100mV,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PowerOnConfiguration {
    pub uvlo_threshold: UVLOThreshold,
    pub charge_disabled: bool,
    #[skip]
    __: B2,
    pub watchdog_timer_reset: bool,
    pub settings_reset: bool,
}

impl Default for PowerOnConfiguration {
    fn default() -> Self {
        Self {
            bytes: [0b0000_0100],
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Clone, Copy, Eq, PartialEq)]
pub enum InputCurrentLimit {
    I77mA,
    I118mA,
    I345mA,
    I470mA,
    I540mA,
    I635mA,
    I734mA,
    I993mA,
}

#[bitfield]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct InputSourceControl {
    pub input_current_limit: InputCurrentLimit,
    pub input_minimum_voltage: B4,
    pub ldo_fet_disabled: bool,
}

impl Default for InputSourceControl {
    fn default() -> Self {
        Self {
            bytes: [0b0100_1011],
        }
    }
}
