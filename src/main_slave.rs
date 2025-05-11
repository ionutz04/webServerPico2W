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
    // config.slave_mode=true;
    let miso = peripherals.PIN_16;
    let mosi = peripherals.PIN_19;
    let clk = peripherals.PIN_18;
    let mut cs = Output::new(peripherals.PIN_17, Level::High);
    let mut spi = Spi::new(peripherals.SPI0, clk, mosi, miso, peripherals.DMA_CH0, peripherals.DMA_CH1, config);
    loop {
        // TX: Command byte + 4 dummy bytes for reading
        let tx_buf = [0xAAu8, 0x00, 0x00, 0x00, 0x00];  // Replace 0xAA with your sensor's read command
        
        // RX: Buffer to store 5 bytes (command response + 4 data bytes)
        let mut rx_buf = [0u8; 5];
        
        // Perform SPI transfer
        match spi.transfer(&mut rx_buf, &tx_buf).await {
            Ok(_) => {
                // Extract 4 data bytes (ignore first command response byte)
                let [_, d1, d2, d3, d4] = rx_buf;
                defmt::info!("Sensor values: [{}, {}, {}, {}]", d1, d2, d3, d4);
            }
            Err(e) => defmt::error!("SPI error: {:?}", e),
        }

        Timer::after_millis(100).await;  // Adjust delay as needed
    }
}
