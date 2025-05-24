//HARDCODARE TASTATURA
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output, Input, Pull};
use embassy_rp::uart::{Config, Uart};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::bind_interrupts;

bind_interrupts!(struct Irqs {
    UART0_IRQ => embassy_rp::uart::InterruptHandler<embassy_rp::peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut uart = Uart::new(
        p.UART0,
        p.PIN_0,
        p.PIN_1,
        Irqs,
        p.DMA_CH0,
        p.DMA_CH1,
        Config::default(),
    );

    let mut rows = [
        Output::new(p.PIN_11, Level::High),
        Output::new(p.PIN_12, Level::High),
        Output::new(p.PIN_13, Level::High),
        Output::new(p.PIN_14, Level::High),
    ];
    let cols = [
        Input::new(p.PIN_15, Pull::Up),
        Input::new(p.PIN_16, Pull::Up),
        Input::new(p.PIN_17, Pull::Up),
    ];

    let mut last_key: Option<u8> = None;
    let mut arrow_mode = false; // false = number entry, true = navigation

    loop {
        let mut key_pressed = None;

        for row_idx in 0..rows.len() {
            for r in 0..rows.len() {
                rows[r].set_high();
            }
            rows[row_idx].set_low();

            for col_idx in 0..cols.len() {
                if cols[col_idx].is_low() {
                    let key = if row_idx == 0 && col_idx == 0 {
                        b'1'
                    } else if row_idx == 0 && col_idx == 1 {
                        b'2'
                    } else if row_idx == 0 && col_idx == 2 {
                        b'3'
                    } else if row_idx == 1 && col_idx == 0 {
                        b'4'
                    } else if row_idx == 1 && col_idx == 1 {
                        b'5'
                    } else if row_idx == 1 && col_idx == 2 {
                        b'6'
                    } else if row_idx == 2 && col_idx == 0 {
                        b'7'
                    } else if row_idx == 2 && col_idx == 1 {
                        b'8'
                    } else if row_idx == 2 && col_idx == 2 {
                        b'9'
                    } else if row_idx == 3 && col_idx == 0 {
                        b'*'
                    } else if row_idx == 3 && col_idx == 1 {
                        b'0'
                    } else if row_idx == 3 && col_idx == 2 {
                        b'#'
                    } else {
                        b'?'
                    };
                    key_pressed = Some(key);
                }
            }
        }

        if key_pressed != last_key {
            if let Some(key) = key_pressed {
                // Toggle arrow mode on '*' key press
                if key == b'*' {
                    arrow_mode = !arrow_mode;
                } else if key == b'#' {
                    // Send delete command
                    let _ = uart.write(b"DEL\n").await;
                } else if arrow_mode {
                    let msg: &[u8] = match key {
                        b'2' => b"UP\n".as_ref(),
                        b'4' => b"LEFT\n".as_ref(),
                        b'6' => b"RIGHT\n".as_ref(),
                        b'8' => b"DOWN\n".as_ref(),
                        _ => b"".as_ref(),
                    };
                    if !msg.is_empty() {
                        let _ = uart.write(msg).await;
                    }
                } else {
                    // In number mode, just send the key
                    let _ = uart.write(&[key, b'\n']).await;
                }
            }
            last_key = key_pressed;
        }
        

        Timer::after(Duration::from_millis(50)).await;
    }
}


// +---------------------------------------------------------------------------+
// |                             PM/MA lab skel                                |
// +---------------------------------------------------------------------------+

// By default, this app prints a "Hello world" message with `defmt`.

// #![no_std]
// #![no_main]

// use embassy_executor::Spawner;
// use embassy_net::StackResources;
// use embassy_time::{Duration, Timer};
// use static_cell::StaticCell;
// use {defmt_rtt as _, panic_probe as _};

// // Use the logging macros provided by defmt.
// use defmt::*;

// // Import interrupts definition module
// mod irqs;

// const SOCK: usize = 4;
// static RESOURCES: StaticCell<StackResources<SOCK>> = StaticCell::<StackResources<SOCK>>::new();

// #[embassy_executor::main]
// async fn main(spawner: Spawner) {
//     // Get a handle to the RP's peripherals.
//     let peripherals = embassy_rp::init(Default::default());

//     // Init WiFi driver
//     let (net_device, mut _control) = embassy_lab_utils::init_wifi!(&spawner, peripherals).await;

//     // Default config for dynamic IP address
//     let config = embassy_net::Config::dhcpv4(Default::default());

//     // Init network stack
//     let _stack = embassy_lab_utils::init_network_stack(&spawner, net_device, &RESOURCES, config);

//     info!("Hello world!");

//     let delay = Duration::from_secs(1);
//     loop {
//         Timer::after(delay).await;
//     }
// }

// #![no_std]
// #![no_main]

// use core::fmt::Write;
// use embassy_executor::Spawner;
// use embassy_rp::gpio::{Level, Output, Input, Pull};
// use embassy_rp::uart::{Config, Uart, Async};
// use embassy_time::{Duration, Timer};
// use heapless::String;
// use {defmt_rtt as _, panic_probe as _};
// use embassy_rp::bind_interrupts;

// bind_interrupts!(struct Irqs {
//     UART0_IRQ => embassy_rp::uart::InterruptHandler<embassy_rp::peripherals::UART0>;
// });

// #[derive(Clone, Copy)]
// struct Sudoku {
//     grid: [[u8; 9]; 9],
//     solution: [[u8; 9]; 9],
//     initial: [[u8; 9]; 9],
//     selected_row: usize,
//     selected_col: usize,
// }

// impl Sudoku {
//     const INITIAL_PUZZLE: [[u8; 9]; 9] = [
//         [5, 3, 0, 0, 7, 0, 0, 0, 0],
//         [6, 0, 0, 1, 9, 5, 0, 0, 0],
//         [0, 9, 8, 0, 0, 0, 0, 6, 0],
//         [8, 0, 0, 0, 6, 0, 0, 0, 3],
//         [4, 0, 0, 8, 0, 3, 0, 0, 1],
//         [7, 0, 0, 0, 2, 0, 0, 0, 6],
//         [0, 6, 0, 0, 0, 0, 2, 8, 0],
//         [0, 0, 0, 4, 1, 9, 0, 0, 5],
//         [0, 0, 0, 0, 8, 0, 0, 7, 9],
//     ];

//     const SOLUTION: [[u8; 9]; 9] = [
//         [5, 3, 4, 6, 7, 8, 9, 1, 2],
//         [6, 7, 2, 1, 9, 5, 3, 4, 8],
//         [1, 9, 8, 3, 4, 2, 5, 6, 7],
//         [8, 5, 9, 7, 6, 1, 4, 2, 3],
//         [4, 2, 6, 8, 5, 3, 7, 9, 1],
//         [7, 1, 3, 9, 2, 4, 8, 5, 6],
//         [9, 6, 1, 5, 3, 7, 2, 8, 4],
//         [2, 8, 7, 4, 1, 9, 6, 3, 5],
//         [3, 4, 5, 2, 8, 6, 1, 7, 9],
//     ];

//     fn new() -> Self {
//         Sudoku {
//             grid: Self::INITIAL_PUZZLE,
//             solution: Self::SOLUTION,
//             initial: Self::INITIAL_PUZZLE,
//             selected_row: 0,
//             selected_col: 0,
//         }
//     }

//     fn move_cursor(&mut self, direction: &str) {
//         match direction {
//             "UP" => self.selected_row = self.selected_row.saturating_sub(1),
//             "DOWN" => self.selected_row = (self.selected_row + 1).min(8),
//             "LEFT" => self.selected_col = self.selected_col.saturating_sub(1),
//             "RIGHT" => self.selected_col = (self.selected_col + 1).min(8),
//             _ => (),
//         }
//     }

//     fn set_cell(&mut self, value: u8) -> Result<(), &'static str> {
//         let (r, c) = (self.selected_row, self.selected_col);
        
//         if self.initial[r][c] != 0 {
//             return Err("Cell is pre-filled");
//         }
        
//         if value == 0 || value > 9 {
//             return Err("Invalid value");
//         }
        
//         if value != self.solution[r][c] {
//             return Err("Incorrect number");
//         }
        
//         self.grid[r][c] = value;
//         Ok(())
//     }

//     async fn send_initial_state(&self, uart: &mut Uart<'_, embassy_rp::peripherals::UART0, Async>) {
//         let mut buffer: String<256> = String::new();
        
//         // Send initial grid
//         for (i, row) in self.grid.iter().enumerate() {
//             for (j, &cell) in row.iter().enumerate() {
//                 buffer.clear();
//                 let _ = write!(buffer, "CELL {} {} {}\n", i, j, cell);
//                 let _ = uart.write(buffer.as_bytes()).await;
//             }
//         }
        
//         // Send initial selection
//         let mut msg: String<32> = String::new();
//         let _ = write!(msg, "SEL {} {}\n", self.selected_row, self.selected_col);
//         let _ = uart.write(msg.as_bytes()).await;
//     }
// }

// #[embassy_executor::main]
// async fn main(_spawner: Spawner) {
//     let p = embassy_rp::init(Default::default());

//     let mut uart = Uart::new(
//         p.UART0,
//         p.PIN_0,
//         p.PIN_1,
//         Irqs,
//         p.DMA_CH0,
//         p.DMA_CH1,
//         Config::default(),
//     );

//     let mut rows = [
//         Output::new(p.PIN_11, Level::High),
//         Output::new(p.PIN_12, Level::High),
//         Output::new(p.PIN_13, Level::High),
//         Output::new(p.PIN_14, Level::High),
//     ];
//     let cols = [
//         Input::new(p.PIN_15, Pull::Up),
//         Input::new(p.PIN_16, Pull::Up),
//         Input::new(p.PIN_17, Pull::Up),
//     ];

//     let mut last_key: Option<u8> = None;
//     let mut arrow_mode = false;
//     let mut sudoku = Sudoku::new();
//     let mut uart_buf = [0u8; 32];

//     // Send initial state
//     sudoku.send_initial_state(&mut uart).await;

//     let mut uart_buf = [0u8; 32];

//     loop {
//         // Handle incoming UART commands
//         match uart.read(&mut uart_buf).await {
//             Ok(count) => {
//                 // Now 'count' is properly typed as usize
//                 if count > 0 {
//                     let cmd = core::str::from_utf8(&uart_buf[..count]).unwrap_or("");
//                     if cmd.trim() == "RST" {
//                         sudoku = Sudoku::new();
//                         sudoku.send_initial_state(&mut uart).await;
//                     }
//                 }
//             }
//             Err(_) => {} // Handle errors if needed
//         }
        
        
    
//         // Key scanning logic
//         let mut key_pressed = None;
//         for row_idx in 0..rows.len() {
//             for r in 0..rows.len() {
//                 rows[r].set_high();
//             }
//             rows[row_idx].set_low();

//             for col_idx in 0..cols.len() {
//                 if cols[col_idx].is_low() {
//                     let key = match (row_idx, col_idx) {
//                         (0, 0) => b'1',
//                         (0, 1) => b'2',
//                         (0, 2) => b'3',
//                         (1, 0) => b'4',
//                         (1, 1) => b'5',
//                         (1, 2) => b'6',
//                         (2, 0) => b'7',
//                         (2, 1) => b'8',
//                         (2, 2) => b'9',
//                         (3, 0) => b'*',
//                         (3, 1) => b'0',
//                         (3, 2) => b'#',
//                         _ => b'?',
//                     };
//                     key_pressed = Some(key);
//                 }
//             }
//         }

//         // Key handling
//         if key_pressed != last_key {
//             if let Some(key) = key_pressed {
//                 if key == b'*' {
//                     arrow_mode = !arrow_mode;
//                 } else if key == b'#' {
//                     let _ = uart.write(b"DEL\n").await;
//                 } else if arrow_mode {
//                     let dir = match key {
//                         b'2' => "UP",
//                         b'4' => "LEFT",
//                         b'6' => "RIGHT",
//                         b'8' => "DOWN",
//                         _ => "",
//                     };
//                     if !dir.is_empty() {
//                         sudoku.move_cursor(dir);
//                         let mut msg: String<32> = String::new();
//                         let _ = write!(msg, "SEL {} {}\n", sudoku.selected_row, sudoku.selected_col);
//                         let _ = uart.write(msg.as_bytes()).await;
//                     }
//                 } else if (b'1'..=b'9').contains(&key) {
//                     let num = key - b'0';
//                     match sudoku.set_cell(num) {
//                         Ok(_) => {
//                             let mut msg: String<32> = String::new();
//                             let _ = write!(msg, "CELL {} {} {}\n", 
//                                 sudoku.selected_row, 
//                                 sudoku.selected_col, 
//                                 num
//                             );
//                             let _ = uart.write(msg.as_bytes()).await;
//                         }
//                         Err(e) => {
//                             let mut msg: String<64> = String::new();
//                             let _ = write!(msg, "ERR {}\n", e);
//                             let _ = uart.write(msg.as_bytes()).await;
//                         }
//                     }
//                 }
//             }
//             last_key = key_pressed;
//         }

//         Timer::after(Duration::from_millis(50)).await;
//     }
// }
