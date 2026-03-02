use defmt::info;
use maybe_async::maybe_async;
use crate::*;
use libm::fabsf;
use ism330is::*;
use ism330is::prelude::*;
use core::fmt::{self, Debug, Display};

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
            StTestType::StPos => core::write!(f, "\nPOS"),
            StTestType::StNeg => core::write!(f, "NEG"),
        }
    }
}

const ST_XL_RANGE_MG_MIN: u16 = 50;
const ST_XL_RANGE_MG_MAX: u16 = 1700;
const ST_GY_RANGE_MDPS_MIN: f32 = 150000.0;
const ST_GY_RANGE_MDPS_MAX: f32 = 700000.0;

#[maybe_async]
async fn avg_5_xl_samples<B, T>(sensor: &mut Ism330is<B, T, MainBank>) -> Result<[f32; 3], Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let mut out = [0f32; 3];

    for _ in 0..5 {
        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.xl_flag_data_ready_get().await?;
        }

        let temp_raw = sensor.acceleration_raw_get().await?;

        for i in 0..3 {
            out[i] += from_fs4g_to_mg(temp_raw[i]);
        }
    }

    for i in 0..3 {
        out[i] /= 5.0;
    }

    Ok(out)
}

#[maybe_async]
async fn avg_5_gy_samples<B, T>(sensor: &mut Ism330is<B, T, MainBank>) -> Result<[f32; 3], Error<B::Error>>
where
    B: BusOperation,
    T: DelayNs,
{
    let mut out = [0f32; 3];

    for _ in 0..5 {
        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.gy_flag_data_ready_get().await?;
        }

        let temp_raw = sensor.angular_rate_raw_get().await?;

        for i in 0..3 {
            out[i] += from_fs2000dps_to_mdps(temp_raw[i]);
        }
    }

    for i in 0..3 {
        out[i] /= 5.0;
    }

    Ok(out)
}

#[maybe_async]
pub async fn run<B, D, L>(bus: B, mut tx: L, mut delay: D, _int_pin: ()) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
{

    info!("Configuring the sensor");
    let mut sensor = Ism330is::from_bus(bus, delay.clone());

    delay.delay_ms(5).await;

    // Check device ID
    let whoami = sensor.device_id_get().await.unwrap();
    info!("Device ID: {:x}", whoami);
    if whoami != ID {
        writeln!(tx, "Device ID mismatch: {:#02x}", whoami).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.software_reset().await.unwrap();

    /*
     * Accelerometer SELF-TEST
     */
    for test in [StTestType::StPos, StTestType::StNeg] {
        /*
         * Initialize and turn on XL sensor
         * Set BDU = 1, FS = +/- 4g, ODR = 52hz
         */
        sensor.block_data_update_set(1).await.unwrap();
        sensor.xl_data_rate_set(XlDataRate::_52hzHp).await.unwrap();
        sensor.xl_full_scale_set(XlFullScale::_4g).await.unwrap();

        /*
         * Power up, wait 100ms for stable output
         * Discard data
         */
        delay.delay_ms(100).await;

        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.xl_flag_data_ready_get().await.unwrap();
        }

        let _temp_raw = sensor.acceleration_raw_get().await.unwrap();

        /*
         * For 5 times, after checking XLDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_nost_mg = avg_5_xl_samples(&mut sensor).await.unwrap();

        /*
         * Enable xl self-test
         * wait 100ms for stable output
         * Discard data
         */
        if test == StTestType::StPos {
            sensor.xl_self_test_set(XlSelfTest::Positive).await.unwrap();
        } else {
            sensor.xl_self_test_set(XlSelfTest::Negative).await.unwrap();
        }
        delay.delay_ms(100).await;

        flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.xl_flag_data_ready_get().await.unwrap();
        }

        let _temp_raw = sensor.acceleration_raw_get().await.unwrap();

        /*
         * For 5 times, after checking XLDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_st_mg = avg_5_xl_samples(&mut sensor).await.unwrap();

        /*
         * Disable self-test, disable XL sensor
         */
        sensor.xl_self_test_set(XlSelfTest::Disable).await.unwrap();
        sensor.xl_data_rate_set(XlDataRate::Off).await.unwrap();

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
        sensor.block_data_update_set(1).await.unwrap();
        sensor.gy_data_rate_set(GyDataRate::_208hzHp).await.unwrap();
        sensor.gy_full_scale_set(GyFullScale::_2000dps).await.unwrap();

        /*
         * Power up, wait 100ms for stable output
         * Discard data
         */
        delay.delay_ms(100).await;

        let mut flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.gy_flag_data_ready_get().await.unwrap();
        }

        let _temp_raw = sensor.angular_rate_raw_get().await.unwrap();

        /*
         * For 5 times, after checking GDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_nost_mg = avg_5_gy_samples(&mut sensor).await.unwrap();

        /*
         * Enable gy self-test
         * wait 100ms for stable output
         * Discard data
         */
        if test == StTestType::StPos {
            sensor.gy_self_test_set(GySelfTest::Positive).await.unwrap();
        } else {
            sensor.gy_self_test_set(GySelfTest::Negative).await.unwrap();
        }
        delay.delay_ms(100).await;

        flag_data = 0;
        while flag_data == 0 {
            flag_data = sensor.gy_flag_data_ready_get().await.unwrap();
        }

        let _temp_raw = sensor.angular_rate_raw_get().await.unwrap();

        /*
         * For 5 times, after checking GDA bit, read the output registers
         * Average the stored data on each axis
         */
        let out_st_mg = avg_5_gy_samples(&mut sensor).await.unwrap();

        /*
         * Disable self-test, disable sensor
         */
        sensor.gy_self_test_set(GySelfTest::Disable).await.unwrap();
        sensor.gy_data_rate_set(GyDataRate::Off).await.unwrap();

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

    loop {}}
