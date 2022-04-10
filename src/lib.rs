//! A platform agnostic Rust driver for the MP2667, based on the
//! [`embedded-hal`](https://github.com/japaric/embedded-hal) traits.
//!
//! ## The Device
//!
//! The MP2667 is a highly integrated, single-cell, Li-ion/Li-polymer battery charger
//! with system power path management for space-limited portable applications.
//!
//! - [Details and datasheet](https://www.monolithicpower.com/en/mp2667.html)

#![no_std]

extern crate embedded_hal as hal;

pub mod registers;

use hal::blocking::i2c;
use registers::*;

/// Driver for the MP2667
#[derive(Debug)]
pub struct MP2667<I2C> {
    i2c: I2C,
}

impl<I2C, E> MP2667<I2C>
where
    I2C: i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    /// Initialize the MP2667 driver.
    pub fn new(i2c: I2C) -> Self {
        MP2667 { i2c }
    }

    /// Realeses I2C bus.
    pub fn release(self) -> I2C {
        self.i2c
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
