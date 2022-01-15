//! A platform agnostic Rust driver for the MP2667, based on the
//! [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! ## The Device
//!
//!The MP2667 is a highly integrated, single-cell, Li-ion/Li-polymer battery charger
//! with system power path management for space-limited portable applications.
//!
//! - [Details and datasheet](https://www.monolithicpower.com/en/mp2667.html)

#![no_std]

extern crate embedded_hal as hal;

mod macros;

use hal::blocking::i2c::{Read, Write, WriteRead};
use macros::*;
use modular_bitfield::prelude::*;

pub const I2C_ADDR: u8 = 0x09;

/// Driver for the MP2667
#[derive(Debug)]
pub struct MP2667<I2C> {
    i2c: I2C,
}

impl<I2C, E> MP2667<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    /// Initialize the MP2667 driver.
    pub fn new(i2c: I2C) -> Self {
        MP2667 { i2c }
    }

    /// Gets the charger input source config.
    pub fn get_input_source(&mut self) -> Result<InputSourceControl, E> {
        InputSourceControl::read(&mut self.i2c)
    }

    /// Sets the charger input source config.
    pub fn set_input_source(&mut self, ctrl: InputSourceControl) -> Result<(), E> {
        ctrl.write(&mut self.i2c)
    }

    /// Gets the charger power on config.
    pub fn get_power_on_config(&mut self) -> Result<PowerOnConfiguration, E> {
        PowerOnConfiguration::read(&mut self.i2c)
    }

    /// Sets the charger power on config.
    pub fn set_power_on_config(&mut self, cfg: PowerOnConfiguration) -> Result<(), E> {
        cfg.write(&mut self.i2c)
    }

    /// Gets the charger charge current control.
    pub fn get_charge_current_control(&mut self) -> Result<ChargeCurrentControl, E> {
        ChargeCurrentControl::read(&mut self.i2c)
    }

    /// Sets the charger charge current control.
    pub fn set_charge_current_control(&mut self, cfg: ChargeCurrentControl) -> Result<(), E> {
        cfg.write(&mut self.i2c)
    }

    /// Gets the charger discharge and termination current.
    pub fn get_discharge_and_termination_current(
        &mut self,
    ) -> Result<DischargeAndTerminationCurrent, E> {
        DischargeAndTerminationCurrent::read(&mut self.i2c)
    }

    /// Sets the charger discharge and termination current.
    pub fn set_discharge_and_termination_current(
        &mut self,
        cfg: DischargeAndTerminationCurrent,
    ) -> Result<(), E> {
        cfg.write(&mut self.i2c)
    }

    /// Gets charge voltage control.
    pub fn get_charge_voltage_control(&mut self) -> Result<ChargeVoltageControl, E> {
        ChargeVoltageControl::read(&mut self.i2c)
    }

    /// Sets charge voltage control.
    pub fn set_charge_voltage_control(&mut self, cfg: ChargeVoltageControl) -> Result<(), E> {
        cfg.write(&mut self.i2c)
    }

    /// Gets charge termination and timer control.
    pub fn get_charge_termination_and_timer_control(
        &mut self,
    ) -> Result<ChargeTerminationAndTimerControl, E> {
        ChargeTerminationAndTimerControl::read(&mut self.i2c)
    }

    /// Sets charge termination and timer control.
    pub fn set_charge_termination_and_timer_control(
        &mut self,
        cfg: ChargeTerminationAndTimerControl,
    ) -> Result<(), E> {
        cfg.write(&mut self.i2c)
    }

    /// Gets the charger miscellaneous operation control.
    pub fn get_miscellaneous_operation_control(
        &mut self,
    ) -> Result<MiscellaneousOperationControl, E> {
        MiscellaneousOperationControl::read(&mut self.i2c)
    }

    /// Sets the charger miscellaneous operation control.
    pub fn set_miscellaneous_operation_control(
        &mut self,
        cfg: MiscellaneousOperationControl,
    ) -> Result<(), E> {
        cfg.write(&mut self.i2c)
    }

    /// Gets the charger status.
    pub fn get_status(&mut self) -> Result<SystemStatus, E> {
        SystemStatus::read(&mut self.i2c)
    }

    /// Gets the charger fault flags.
    pub fn get_faults(&mut self) -> Result<FaultFlags, E> {
        FaultFlags::read(&mut self.i2c)
    }
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

#[derive(BitfieldSpecifier)]
pub enum ChargeStatus {
    NotCharging,
    PreCharge,
    Charge,
    ChargeDone,
}

#[bitfield]
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

#[derive(BitfieldSpecifier)]
pub enum ThermalThreshold {
    T60C,
    T80C,
    T100C,
    T120C,
}

#[bitfield]
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

#[derive(BitfieldSpecifier)]
pub enum SafetyTimerPeriod {
    P20h,
    P5h,
    P8h,
    P12h,
}

#[derive(BitfieldSpecifier)]
pub enum WatchdogTimerLimit {
    Disabled,
    L40s,
    L80s,
    L160s,
}

#[bitfield]
pub struct ChargeTerminationAndTimerControl {
    pub termination_control_enabled: bool,
    pub timer_period: SafetyTimerPeriod,
    pub timer_enabled: bool,
    pub timer_limit: WatchdogTimerLimit,
    pub termination_enabled: bool,
    #[skip]
    __: bool,
}

#[derive(BitfieldSpecifier)]
pub enum RechargeThreshold {
    U150mV,
    U300mV,
}

#[derive(BitfieldSpecifier)]
pub enum PrechargeThreshold {
    U2800mV,
    U3000mV,
}

#[bitfield]
pub struct ChargeVoltageControl {
    pub recharge_threshold: RechargeThreshold,
    pub precharge_threshold: PrechargeThreshold,
    pub regulation_voltage: B6,
}

#[derive(BitfieldSpecifier)]
pub enum TerminalCurrent {
    I24mA,
    I52mA,
    I80mA,
    I108mA,
}

#[bitfield]
pub struct DischargeAndTerminationCurrent {
    pub terminal_current: TerminalCurrent,
    #[skip]
    __: bool,
    pub discharge_current_limit: B4,
    #[skip]
    __: bool,
}

#[bitfield]
pub struct ChargeCurrentControl {
    pub charge_current: B5,
    #[skip]
    __: B3,
}

#[derive(BitfieldSpecifier)]
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
pub struct PowerOnConfiguration {
    pub uvlo_threshold: UVLOThreshold,
    pub charge_disabled: bool,
    #[skip]
    __: B2,
    pub watchdog_timer_reset: bool,
    pub settings_reset: bool,
}

#[derive(BitfieldSpecifier)]
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
pub struct InputSourceControl {
    pub input_current_limit: InputCurrentLimit,
    pub input_minimum_voltage: B4,
    pub ldo_fet_disabled: bool,
}
