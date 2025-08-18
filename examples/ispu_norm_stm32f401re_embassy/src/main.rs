#![no_std]
#![no_main]

mod ispu_norm;

use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{self, Config as I2cConfig, I2c};
use embassy_stm32::peripherals::{self, USART2};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{
    BufferedInterruptHandler, Config as UsartConfig, DataBits, Parity, UartTx,
};
use embassy_time::Delay;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

use ism330is_rs::*;
use ispu_norm::NORM_PROGRAM as NORM;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!`")
}

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut usart_config: UsartConfig = UsartConfig::default();
    usart_config.baudrate = 115200;
    usart_config.data_bits = DataBits::DataBits8;
    usart_config.parity = Parity::ParityNone;

    let mut tx: UartTx<_> = UartTx::new(p.USART2, p.PA2, NoDma, usart_config).unwrap();

    let i2c: I2c<_> = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        khz(100),
        I2cConfig::default(),
    );

    let mut delay = Delay;

    // Configure the interrupt pin (if needed) and obtain handler.
    // On the Nucleo FR401 the interrupt pin is connected to pin PB0.
    let interrupt = Input::new(p.PC0, Pull::None);
    let mut interrupt = ExtiInput::new(interrupt, p.EXTI0);

    delay.delay_ms(5_u32);

    let mut msg: String<64> = String::new();

    let mut sensor = Ism330is::new_i2c(i2c, I2CAddress::I2cAddL, delay.clone());

    // Check device ID
    match sensor.device_id_get() {
        Ok(id) => {
            if id != ism330is_rs::ID {
                loop {}
            }
        }
        Err(e) => {
            writeln!(&mut msg, "Error in reading id: {:?}", e).unwrap();
            msg.clear();
        }
    }

    // Restore default configuration
    sensor.software_reset().unwrap();

    // Load ISPU configuration
    for ucf_entry in NORM {
        match ucf_entry.op {
            MemsUcfOp::Delay => {
                delay.delay_ms(ucf_entry.data as u32);
            }
            MemsUcfOp::Write => {
                sensor
                    .write_to_register(ucf_entry.address, &[ucf_entry.data])
                    .unwrap();
            }
            _ => {}
        }
    }

    let ispu_odr = sensor.ispu_data_rate_get().unwrap();
    writeln!(&mut msg, "ISPU started: at rate {}", ispu_odr.to_str()).unwrap();
    let _ = tx.blocking_write(msg.as_bytes());
    msg.clear();

    let mut dout: [u8; 10] = [0; 10];
    let mut x: i16;
    let mut y: i16;
    let mut z: i16;
    let mut temp: u32;
    let mut norm: f32;

    loop {
        interrupt.wait_for_rising_edge().await;

        let ispu_int = sensor.ispu_int_status_get().unwrap();

        if (ispu_int & 0x1) == 0 {
            continue;
        }

        sensor.ispu_read_data_raw_get(&mut dout, 10).unwrap();

        x = dout[1] as i16;
        x = (x << 8) + (dout[0] as i16);
        y = dout[3] as i16;
        y = (y << 8) + (dout[2] as i16);
        z = dout[5] as i16;
        z = (z << 8) + (dout[4] as i16);

        temp = dout[9] as u32;
        temp = (temp << 8) + dout[8] as u32;
        temp = (temp << 8) + dout[7] as u32;
        temp = (temp << 8) + dout[6] as u32;
        norm = f32::from_bits(temp);
        writeln!(&mut msg, "x: {}\ty: {}\tz: {}\tnorm: {:.2}", x, y, z, norm).unwrap();
        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();
    }
}
