#![no_std]
#![no_main]

mod signal;

use cortex_m_rt;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::gpio::Pin;
use lib::{Button, Led, LedMode, Never};
use panic_probe as _;

use lib::error::Result;
use signal::SIGNAL;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // `!` is not yet stable and `Never` (`enum {}`) doesn't work here
    main_inner(spawner).await.unwrap_or_else(|err| panic!("{err}"));
}

async fn main_inner(spawner: Spawner) -> Result<Never> {
    let periphs: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    let led_pin = periphs.PIN_2.degrade();
    let mut led = Led::new(led_pin, spawner, &SIGNAL)?;

    let button_pin = periphs.PIN_13.degrade();
    let mut button = Button::new(button_pin);

    loop {
        let press = button.wait_for_press().await;

        use lib::PressKind as Pk;
        match press {
            Pk::Long => led.set_mode(LedMode::FastFlash).await,
            Pk::Short => led.advance_state().await,
        };
    }
}
