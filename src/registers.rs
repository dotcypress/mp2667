use crate::*;
use modular_bitfield::prelude::*;

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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum ChargeStatus {
    NotCharging,
    PreCharge,
    Charge,
    ChargeDone,
}

#[bitfield]
#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum ThermalThreshold {
    T60C,
    T80C,
    T100C,
    T120C,
}

#[bitfield]
#[derive(Debug)]
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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum SafetyTimerPeriod {
    P20h,
    P5h,
    P8h,
    P12h,
}

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum WatchdogTimerLimit {
    Disabled,
    L40s,
    L80s,
    L160s,
}

#[bitfield]
#[derive(Debug)]
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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum RechargeThreshold {
    U150mV,
    U300mV,
}

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum PrechargeThreshold {
    U2800mV,
    U3000mV,
}

#[bitfield]
#[derive(Debug)]
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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
pub enum TerminalCurrent {
    I24mA,
    I52mA,
    I80mA,
    I108mA,
}

#[bitfield]
#[derive(Debug)]
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
#[derive(Debug)]
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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
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
#[derive(Debug)]
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

#[derive(BitfieldSpecifier, Debug, Eq, PartialEq)]
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
#[derive(Debug)]
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
