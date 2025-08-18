#![no_std]
#![no_main]
#![deny(unsafe_code)]

use core::fmt::Write;
use cortex_m_rt::entry;
use ism330is_rs::prelude::*;
use ism330is_rs::*;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::{DutyCycle, I2c, Mode},
    pac,
    prelude::*,
    serial::{config::Config, Serial},
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);
    let tim1 = dp.TIM1.delay_us(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        &clocks,
    );

    let tx_pin = gpioa.pa2.into_alternate();

    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    )
    .unwrap();

    delay.delay_ms(5);
    let mut sensor = Ism330is::new_i2c(i2c, I2CAddress::I2cAddL, tim1);

    // Check device ID
    let whoam_i = sensor.device_id_get().unwrap();
    if whoam_i != ism330is_rs::ID {
        loop {}
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
                tx,
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
                tx,
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
            writeln!(tx, "Temperature [degC]: {:.2}", temperature_deg_c).unwrap();
        }

        delay.delay_ms(1000_u32);
    }
}
