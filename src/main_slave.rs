#![no_std]
#![no_main]

use defmt::{info, error};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::{UART1, DMA_CH1},
    uart::{Async, Config as UartConfig, InterruptHandler, UartRx},
};
use embassy_time::{Timer, Duration};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART1_IRQ => InterruptHandler<UART1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut config = UartConfig::default();
    config.baudrate = 2_000_000;

    let mut uart_rx = UartRx::new(
        p.UART1,
        p.PIN_5,    // RX pin (GPIO5)
        Irqs,
        p.DMA_CH1,
        config,
    );

    info!("UART Receiver ready!");

    let mut buf = [0u8; 8];
    loop {
        // Await exactly 8 bytes (blocking until all are received)
        match uart_rx.read(&mut buf).await {
            Ok(()) => {
                let freq = f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                let amp  = f32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
                info!("Received: Frequency = {} Hz, Amplitude = {} V", freq, amp);
            }
            Err(e) => {
                error!("UART read error: {:?}", e);
            }
        }
        // Optional: short pause to avoid log flooding if sender is very fast
        Timer::after(Duration::from_millis(5)).await;
    }
}
