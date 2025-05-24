#![no_std]
#![no_main]

use defmt::{info, error};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::{
    bind_interrupts,
    peripherals::{UART1, DMA_CH1, USB},
    uart::{Async, Config as UartConfig, InterruptHandler, UartRx},
    usb::Driver,
};
use embassy_time::{Timer, Duration};
use embassy_usb::{Builder, Config as UsbConfig, UsbDevice};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART1_IRQ => InterruptHandler<UART1>;
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // UART Receiver Setup (from other MCU)
    let mut uart_config = UartConfig::default();
    uart_config.baudrate = 2_000_000;
    let mut uart_rx = UartRx::new(
        p.UART1,
        p.PIN_5,    // RX pin (GPIO5)
        Irqs,
        p.DMA_CH1,
        uart_config,
    );

    // USB CDC ACM Setup (for Web Serial API)
    let mut usb_config = UsbConfig::new(0x2E8A, 0x000A);
    usb_config.manufacturer = Some("Raspberry Pi");
    usb_config.product = Some("Frequency/Amplitude Monitor");
    usb_config.serial_number = Some("123456");
    
    let driver = Driver::new(p.USB, Irqs);
    let mut builder = Builder::new(
        driver,
        usb_config,
        &mut [],
        &mut [],
        &mut [],
        &mut [],
    );
    
    let mut state: State<'_> = State::new();
    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);
    let usb = builder.build();
    spawner.spawn(usb_task(usb)).unwrap();

    info!("System ready - waiting for data...");
    
    let mut buf = [0u8; 8];
    loop {
        match uart_rx.read(&mut buf).await {
            Ok(()) => {
                // Use the writer interface directly
                if let Err(e) = class.write_packet(&buf).await {
                    error!("USB write error: {:?}", e);
                } else {
                    let freq = f32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
                    let amp = f32::from_le_bytes([buf[4], buf[5], buf[6], buf[7]]);
                    info!("Forwarded: {=f32} Hz, {=f32} V", freq, amp);
                }
            }
            Err(e) => {
                error!("UART error: {:?}", e);
            }
        }
        Timer::after(Duration::from_millis(5)).await;
    }
}

#[embassy_executor::task]
async fn usb_task(mut usb: UsbDevice<'static, Driver<'static, USB>>) {
    usb.run().await;
}
