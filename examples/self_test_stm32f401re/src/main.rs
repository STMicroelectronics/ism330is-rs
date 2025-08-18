#![no_std]
#![no_main]
#![deny(unsafe_code)]

use core::fmt::{self, Debug, Display, Write};
use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use ism330is_rs::prelude::*;
use ism330is_rs::*;
use panic_halt as _;
use st_mems_bus::BusOperation;
use stm32f4xx_hal::{
    i2c::{DutyCycle, I2c, Mode},
    pac,
    prelude::*,
    serial::{config::Config, Serial},
};

use libm::fabsf;

#[repr(u8)]
#[derive(PartialEq, Debug)]
enum StTestType {
    StPos = 0,
    StNeg = 1,
}

#[repr(u8)]
#[derive(PartialEq)]
enum StResult {
    StPass = 1,
    StFail = 0,
}

impl Display for StTestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StTestType::StPos => write!(f, "\nPOS"),
            StTestType::StNeg => write!(f, "NEG"),
        }
    }
}

const ST_XL_RANGE_MG_MIN: u16 = 50;
const ST_XL_RANGE_MG_MAX: u16 = 1700;
const ST_GY_RANGE_MDPS_MIN: f32 = 150000.0;
const ST_GY_RANGE_MDPS_MAX: f32 = 700000.0;

fn avg_5_xl_samples<B, T>(sensor: &mut Ism330is<B, T>) -> Result<[f32; 3], Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let mut out = [0f32; 3];

    for _ in 0..5 {
        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.xl_flag_data_ready_get()?;
        }

        let temp_raw = sensor.acceleration_raw_get()?;

        for i in 0..3 {
            out[i] += ism330is_rs::from_fs4g_to_mg(temp_raw[i]);
        }
    }

    for i in 0..3 {
        out[i] /= 5.0;
    }

    Ok(out)
}

fn avg_5_gy_samples<B, T>(sensor: &mut Ism330is<B, T>) -> Result<[f32; 3], Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let mut out = [0f32; 3];

    for _ in 0..5 {
        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.gy_flag_data_ready_get()?;
        }

        let temp_raw = sensor.angular_rate_raw_get().unwrap();

        for i in 0..3 {
            out[i] += ism330is_rs::from_fs2000dps_to_mdps(temp_raw[i]);
        }
    }

    for i in 0..3 {
        out[i] /= 5.0;
    }

    Ok(out)
}

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
    match sensor.device_id_get() {
        Ok(id) => {
            if id != ism330is_rs::ID {
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
    }

    // Restore default configuration
    sensor.software_reset().unwrap();

    /*
     * Accelerometer SELF-TEST
     */
    for test in [StTestType::StPos, StTestType::StNeg] {
        /*
         * Initialize and turn on XL sensor
         * Set BDU = 1, FS = +/- 4g, ODR = 52hz
         */
        sensor.block_data_update_set(1).unwrap();
        sensor.xl_data_rate_set(XlDataRate::_52hzHp).unwrap();
        sensor.xl_full_scale_set(XlFullScale::_4g).unwrap();

        /*
         * Power up, wait 100ms for stable output
         * Discard data
         */
        delay.delay_ms(100);

        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.xl_flag_data_ready_get().unwrap();
        }

        let _temp_raw = sensor.acceleration_raw_get().unwrap();

        /*
         * For 5 times, after checking XLDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_nost_mg = avg_5_xl_samples(&mut sensor).unwrap();

        /*
         * Enable xl self-test
         * wait 100ms for stable output
         * Discard data
         */
        if test == StTestType::StPos {
            sensor.xl_self_test_set(XlSelfTest::Positive).unwrap();
        } else {
            sensor.xl_self_test_set(XlSelfTest::Negative).unwrap();
        }
        delay.delay_ms(100);

        flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.xl_flag_data_ready_get().unwrap();
        }

        let _temp_raw = sensor.acceleration_raw_get().unwrap();

        /*
         * For 5 times, after checking XLDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_st_mg = avg_5_xl_samples(&mut sensor).unwrap();

        /*
         * Disable self-test, disable XL sensor
         */
        sensor.xl_self_test_set(XlSelfTest::Disable).unwrap();
        sensor.xl_data_rate_set(XlDataRate::Off).unwrap();

        /*
         * Test if data in range
         */
        let mut st_result = StResult::StPass;
        let mut abs_diff_mg = [0f32; 3];
        for i in 0..3 {
            abs_diff_mg[i] = fabsf(out_st_mg[i] - out_nost_mg[i]);

            if abs_diff_mg[i] < ST_XL_RANGE_MG_MIN.into()
                || abs_diff_mg[i] > ST_XL_RANGE_MG_MAX.into()
            {
                st_result = StResult::StFail;
            }
        }

        if st_result == StResult::StPass {
            writeln!(tx, "{} XL Self Test - PASS", test).unwrap();
        } else {
            writeln!(tx, "{} XL Self Test - FAIL!!!!", test).unwrap();
        }
    }

    /*
     * Gyro SELF-TEST
     */
    for test in [StTestType::StPos, StTestType::StNeg] {
        /*
         * Initialize and turn on GY sensor
         * Set BDU = 1, FS = +/- 2000dps, ODR = 208hz
         */
        sensor.block_data_update_set(1).unwrap();
        sensor.gy_data_rate_set(GyDataRate::_208hzHp).unwrap();
        sensor.gy_full_scale_set(GyFullScale::_2000dps).unwrap();

        /*
         * Power up, wait 100ms for stable output
         * Discard data
         */
        delay.delay_ms(100);

        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.gy_flag_data_ready_get().unwrap();
        }

        let _temp_raw = sensor.angular_rate_raw_get().unwrap();

        /*
         * For 5 times, after checking GDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_nost_mg = avg_5_gy_samples(&mut sensor).unwrap();

        /*
         * Enable gy self-test
         * wait 100ms for stable output
         * Discard data
         */
        if test == StTestType::StPos {
            sensor.gy_self_test_set(GySelfTest::Positive).unwrap();
        } else {
            sensor.gy_self_test_set(GySelfTest::Negative).unwrap();
        }
        delay.delay_ms(100);

        flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.gy_flag_data_ready_get().unwrap();
        }

        let _temp_raw = sensor.angular_rate_raw_get().unwrap();

        /*
         * For 5 times, after checking GDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_st_mg = avg_5_gy_samples(&mut sensor).unwrap();

        /*
         * Disable self-test, disable sensor
         */
        sensor.gy_self_test_set(GySelfTest::Disable).unwrap();
        sensor.gy_data_rate_set(GyDataRate::Off).unwrap();

        /*
         * Test if data in range
         */
        let mut abs_diff_mg = [0f32; 3];
        let mut st_result = StResult::StPass;
        for i in 0..3 {
            abs_diff_mg[i] = fabsf(out_st_mg[i] - out_nost_mg[i]);

            if abs_diff_mg[i] < ST_GY_RANGE_MDPS_MIN.into()
                || abs_diff_mg[i] > ST_GY_RANGE_MDPS_MAX.into()
            {
                st_result = StResult::StFail;
            }
        }

        if st_result == StResult::StPass {
            writeln!(tx, "{} GY Self Test - PASS", test).unwrap();
        } else {
            writeln!(tx, "{} GY Self Test - FAIL!!!!", test).unwrap();
        }
    }

    loop {}
}
