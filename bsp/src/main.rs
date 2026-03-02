#![no_std]
#![no_main]

// ── Imports ───────────────────────────────────────────────────────────────────
use defmt::*;
use {defmt_rtt as _, panic_probe as _};
#[allow(unused)]
#[cfg(feature = "async")]
pub use embedded_io_async as embed_io;
#[cfg(feature = "blocking")]
pub use embedded_io as embed_io;


#[cfg(feature = "async")]
mod mode {
    pub use st_mems_bus::asynchronous::{BusOperation, i2c::I2cBus};
    pub use embedded_hal_async::delay::DelayNs;
    pub use embedded_hal_async::i2c;
    pub use ism330is_rs::asynchronous as ism330is;
    pub use embassy_stm32::interrupt;
}

#[cfg(feature = "blocking")]
mod mode {
    pub use st_mems_bus::blocking::{BusOperation, i2c::I2cBus};
    pub use embedded_hal::delay::DelayNs;
    pub use embedded_hal::i2c;
    pub use ism330is_rs::blocking as ism330is;
    pub use cortex_m_rt::interrupt;
}

pub use mode::*;
pub use ism330is::I2CAddress;

mod board_macro;

// ── Board configurations ──────────────────────────────────────────────────────

#[cfg(feature = "nucleo-f401re-embassy")]
define_embassy_with_st_link! {
    i2c = {
        address: I2CAddress::I2cAddL as u8,
        periph: I2C1,
        scl: PB8,
        sda: PB9,
        ev_irq: I2C1_EV,
        er_irq: I2C1_ER,
        dma_tx: DMA1_CH7,
        dma_rx: DMA1_CH0,
    },
    uart = {
        periph: USART2,
        tx: PA2,
        dma_tx: DMA1_CH6,
        baud: 115200,
    },
    int_pin = {
        pin: PC0,
        exti_line: EXTI0,
        exti_mux: EXTI0
    }
}

#[cfg(feature = "nucleo-f401re")]
define_stm32_rs_with_st_link!(
    i2c = {
        address: I2CAddress::I2cAddL as u8,
        periph: I2C1,
        scl: (port_b, pb8),
        sda: (port_b, pb9),
    },
    uart = {
        periph: USART2,
        tx: (port_a, pa2),
    },
    interrupt = {
        pin: (port_c, pc0),
        exti_irq: EXTI0
    }
);

// ── Example definitions ─────────────────────────────────────────────────────────

// Ispu norm
#[cfg(feature = "ispu_norm")]
mod examples {
    mod ispu_norm;
    pub use ispu_norm::run;
}

#[cfg(feature = "ispu_norm")]
mod config {
    pub mod ispu_config;
}

// Read in polling mode
#[cfg(feature = "read_polling")]
mod examples {
    mod read_polling;
    pub use read_polling::run;
}

// Read with interrupt
#[cfg(feature = "read_irq")]
mod examples {
    mod read_irq;
    pub use read_irq::run;
}

// Self test
#[cfg(feature = "self_test")]
mod examples {
    mod self_test;
    pub use self_test::run;
}

// Sensor hub - LPS22DF and LIS2MDL
#[cfg(feature = "sensor_hub")]
mod examples {
    mod sensor_hub;
    pub use sensor_hub::run;
}

// ─── Compilation checks ──────────────────────────────────────────
#[cfg(not(any(
    feature = "read_irq",
    feature = "read_polling",
    feature = "self_test",
    feature = "ispu_norm",
    feature = "sensor_hub",
)))]
compile_error!("No example selected! Please enable at least one example by passing --features **example_name**.");


// ── ASYNC entry point ─────────────────────────────────────────────────────────
#[cfg(feature = "async")]
use embassy_executor::Spawner;

#[cfg(feature = "async")]
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting async main...");

    let (i2c, uart, delay, irq) = board_init(spawner);
    let bus = I2cBus::new(i2c, I2CADDRESS as u8);
    examples::run(bus, uart, delay, irq).await;
}

// ── BLOCKING entry point ──────────────────────────────────────────────────────
#[cfg(feature = "blocking")]
use cortex_m_rt::entry;

#[cfg(feature = "blocking")]
#[entry]
fn main() -> ! {
    info!("Starting blocking main...");

    let (i2c, uart, delay, int_pin) = board_init();
    let bus = I2cBus::new(i2c, I2CADDRESS as u8);
    examples::run(bus, uart, delay, int_pin)
}

// ─── Wrappers ──────────────────────────────────────────────────────

// SerialWriter provide embedded_io::Write trait on top of stm32-rs framework uart channel
#[cfg(feature = "blocking")]
struct SerialWriter<T>(T);

#[cfg(feature = "blocking")]
impl<T> embedded_io::ErrorType for SerialWriter<T> {
    type Error = embedded_io::ErrorKind;
}

#[cfg(feature = "blocking")]
impl<T> embedded_io::Write for SerialWriter<T>
where
    T: core::fmt::Write,
{
    fn write(&mut self, buf: &[u8]) -> Result<usize, embedded_io::ErrorKind> {
        for &b in buf {
            self.0.write_char(b as char).map_err(|_| embedded_io::ErrorKind::Other)?;
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), embedded_io::ErrorKind> {
        Ok(())
    }
}


// ─── Interrupt abstraction ──────────────────────────────────────────────────────
#[cfg(feature = "async")]
pub trait InterruptPin {
    fn wait_for_event(&mut self) -> impl core::future::Future<Output = ()> + Send;
}

#[cfg(feature = "blocking")]
pub trait InterruptPin {
    fn wait_for_event(&mut self);
}
