#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{self, Config as I2cConfig, I2c};
use embassy_stm32::peripherals::{self, USART2};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{
    BufferedInterruptHandler, Config as UsartConfig, DataBits, Parity, UartTx,
};
use embassy_time::Delay;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

use ism330is_rs::prelude::*;
use ism330is_rs::*;

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

    delay.delay_ms(5_u32);

    let mut msg: String<256> = String::new();

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

    // Disable I3C interface (if needed)
    // sensor.i3c_disable_set(ISM330IS_I3C_DISABLE).unwrap();

    // Enable Block Data Update (if needed)
    // sensor.block_data_update_set(true).unwrap();

    // Set Output Data Rate for accelerometer and gyroscope
    sensor.xl_data_rate_set(XlDataRate::_12_5hzHp).unwrap();
    sensor.gy_data_rate_set(GyDataRate::_12_5hzHp).unwrap();

    // Set full scale for accelerometer and gyroscope
    sensor.xl_full_scale_set(XlFullScale::_2g).unwrap();
    sensor.gy_full_scale_set(GyFullScale::_2000dps).unwrap();

    // Configure filtering chain (No aux interface)
    // sensor.xl_hp_path_on_out_set(LpOdrDiv::LpOdrDiv100).unwrap();
    // sensor.xl_filter_lp2_set(true).unwrap();

    // Read samples in polling mode (no int)
    loop {
        let drdy = sensor.xl_flag_data_ready_get().unwrap();
        if drdy != 0 {
            // Read acceleration data
            let data_raw_acceleration = sensor.acceleration_raw_get().unwrap();
            let acceleration_mg = [
                ism330is_rs::from_fs2g_to_mg(data_raw_acceleration[0]),
                ism330is_rs::from_fs2g_to_mg(data_raw_acceleration[1]),
                ism330is_rs::from_fs2g_to_mg(data_raw_acceleration[2]),
            ];
            writeln!(
                &mut msg,
                "Acceleration [mg]: {:.2}\t{:.2}\t{:.2}",
                acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();
        }

        // Read output only if new gyroscope value is available
        let drdy = sensor.gy_flag_data_ready_get().unwrap();
        if drdy != 0 {
            // Read angular rate data
            let data_raw_angular_rate = sensor.angular_rate_raw_get().unwrap();
            let angular_rate_mdps = [
                ism330is_rs::from_fs2000dps_to_mdps(data_raw_angular_rate[0]),
                ism330is_rs::from_fs2000dps_to_mdps(data_raw_angular_rate[1]),
                ism330is_rs::from_fs2000dps_to_mdps(data_raw_angular_rate[2]),
            ];
            writeln!(
                &mut msg,
                "Angular rate [mdps]: {:.2}\t{:.2}\t{:.2}",
                angular_rate_mdps[0], angular_rate_mdps[1], angular_rate_mdps[2]
            )
            .unwrap();
        }

        // Read output only if new temperature value is available
        let drdy = sensor.temp_flag_data_ready_get().unwrap();
        if drdy != 0 {
            // Read temperature data
            let data_raw_temperature = sensor.temperature_raw_get().unwrap();
            let temperature_deg_c = ism330is_rs::from_lsb_to_celsius(data_raw_temperature);
            writeln!(&mut msg, "Temperature [degC]: {:.2}", temperature_deg_c).unwrap();
        }

        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();
        delay.delay_ms(1000_u32);
    }
}
