#![no_std]

extern crate alloc;

mod joystick;

use joystick::Joystick;

use xpanse_api::{
    bus::allocator::BusAllocator,
    driver::{Driver, DriverError, DriverMeta},
    gpio_bank::{BankPins, GpioBank},
    metadata::{ModuleDetectResistor, ModuleID, ModuleSlot},
    registry::Registry,
};

pub struct JoystickDriver;

impl DriverMeta for JoystickDriver {
    const ID: ModuleID = ModuleID {
        md0: ModuleDetectResistor::R1K1,
        md1: ModuleDetectResistor::R1K2,
    };
}

impl<G: BankPins> Driver<G> for JoystickDriver {
    async fn create(
        gpio_bank: GpioBank<G>,
        slot: ModuleSlot,
        registry: &mut Registry,
        bus_allocator: &mut BusAllocator,
    ) -> Result<(), DriverError> {
        let joystick = Joystick::new(
            gpio_bank.gpio7,
            gpio_bank.gpio8,
            gpio_bank.gpio9.into(),
        );

        registry.register(
            slot,
            JoystickDriver::ID,
            joystick,
        );

        let _ = bus_allocator;

        Ok(())
    }
}