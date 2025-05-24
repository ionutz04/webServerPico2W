#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::peripherals::UART1;
use embassy_rp::uart::{Async, Config, UartTx};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut config = Config::default();
    config.baudrate = 2_000_000;  // 2 Mbps

    let mut uart_tx = UartTx::new(
        p.UART1,  // Changed to UART1
        p.PIN_4,  // TX pin (GPIO4)
        p.DMA_CH0,
        config
    );

    info!("Transmitter Ready!");
    
    loop {
        let data = [1u8, 2, 3, 4];
        info!("TX {:?}", data);
        uart_tx.write(&data).await.unwrap();
        Timer::after_secs(1).await;
    }
}
