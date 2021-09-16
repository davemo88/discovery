#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // TODO implement this
        // hint: this will look very similar to the previous program
        for &c in s.as_bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1
                .tdr
                .write(|w| w.tdr().bits(u16::from(c)) );
        }
        Ok(())
    }
}
 
use heapless::Vec;

#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, mut itm) = aux11::init();

    let mut buffer: Vec<u8, 32> = Vec::new();

    let mut serial = SerialPort { usart1 };

    loop {
        buffer.clear();
        // Wait until there's data available
        loop {
            while serial.usart1.isr.read().rxne().bit_is_clear() {}

            // Retrieve the data
            let byte = serial.usart1.rdr.read().rdr().bits() as u8;

            if byte == 13 {
                buffer.reverse();

                for &b in buffer.iter() {
                    while serial.usart1.isr.read().txe().bit_is_clear() {}

                    serial.usart1
                        .tdr
                        .write(|w| w.tdr().bits(u16::from(b)) );
                }
                break
            } else {
                match buffer.push(byte) {
                    Ok(_) => (),
                    Err(e) => {
                        uprintln!(serial, "error: {}", e).unwrap();
                        break
                    }
                }
            }
        }

//        aux11::bkpt();
    }
}

