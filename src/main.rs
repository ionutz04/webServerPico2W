// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

//! By default, this app prints a "Hello world" message with `defmt`.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_net::StackResources;
use embassy_rp::{config, gpio::{Level, Output}, peripherals, spi::{self, Spi}};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

// Use the logging macros provided by defmt.
use defmt::*;
// Import interrupts definition module
mod irqs;

const SOCK: usize = 4;
static RESOURCES: StaticCell<StackResources<SOCK>> = StaticCell::<StackResources<SOCK>>::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    let mut config = spi::Config::default();
    config.frequency = 1_000_000;
    config.phase = spi::Phase::CaptureOnFirstTransition;
    config.polarity = spi::Polarity::IdleLow;
    let miso = peripherals.PIN_16;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;
    let cs = Output::new(peripherals.PIN_17, Level::High);
    let mut spi = Spi::new(peripherals.SPI0, clk, mosi, miso, peripherals.DMA_CH0, peripherals.DMA_CH1, config);
    
    info!("Hello world!");

    let delay = Duration::from_secs(1);
    loop {
        Timer::after(delay).await;
    }
}
