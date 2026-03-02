use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L>(bus: B, mut tx: L, mut delay: D, _irq: ()) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write
{
    use ism330is::prelude::*;
    use ism330is::*;

    info!("Configuring the sensor");
    let mut sensor = Ism330is::from_bus(bus, delay.clone());

    // boot time
    delay.delay_ms(5).await;

    // Check device ID
    let id = sensor.device_id_get().await.unwrap();
    info!("Device ID: {:x}", id);
    if id != ID {
        info!("Unexpected device ID: {:x}", id);
        writeln!(tx, "Unexpected device ID: {:x}", id).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.software_reset().await.unwrap();

    // Disable I3C interface (if needed)
    // sensor.i3c_disable_set(ISM330IS_I3C_DISABLE).await.unwrap();

    // Enable Block Data Update (if needed)
    // sensor.block_data_update_set(true).await.unwrap();

    // Set Output Data Rate for accelerometer and gyroscope
    sensor.xl_data_rate_set(XlDataRate::_12_5hzHp).await.unwrap();
    sensor.gy_data_rate_set(GyDataRate::_12_5hzHp).await.unwrap();

    // Set full scale for accelerometer and gyroscope
    sensor.xl_full_scale_set(XlFullScale::_2g).await.unwrap();
    sensor.gy_full_scale_set(GyFullScale::_2000dps).await.unwrap();

    // Configure filtering chain (No aux interface)
    // sensor.xl_hp_path_on_out_set(LpOdrDiv::LpOdrDiv100).await.unwrap();
    // sensor.xl_filter_lp2_set(true).await.unwrap();

    // Read samples in polling mode (no int)
    loop {
        let drdy = sensor.xl_flag_data_ready_get().await.unwrap();
        if drdy != 0 {
            // Read acceleration data
            let data_raw_acceleration = sensor.acceleration_raw_get().await.unwrap();
            let acceleration_mg = [
                from_fs2g_to_mg(data_raw_acceleration[0]),
                from_fs2g_to_mg(data_raw_acceleration[1]),
                from_fs2g_to_mg(data_raw_acceleration[2]),
            ];
            writeln!(
                tx,
                "Acceleration [mg]: {:.2}\t{:.2}\t{:.2}",
                acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();
        }

        // Read output only if new gyroscope value is available
        let drdy = sensor.gy_flag_data_ready_get().await.unwrap();
        if drdy != 0 {
            // Read angular rate data
            let data_raw_angular_rate = sensor.angular_rate_raw_get().await.unwrap();
            let angular_rate_mdps = [
                from_fs2000dps_to_mdps(data_raw_angular_rate[0]),
                from_fs2000dps_to_mdps(data_raw_angular_rate[1]),
                from_fs2000dps_to_mdps(data_raw_angular_rate[2]),
            ];
            writeln!(
                tx,
                "Angular rate [mdps]: {:.2}\t{:.2}\t{:.2}",
                angular_rate_mdps[0], angular_rate_mdps[1], angular_rate_mdps[2]
            )
            .unwrap();
        }

        // Read output only if new temperature value is available
        let drdy = sensor.temp_flag_data_ready_get().await.unwrap();
        if drdy != 0 {
            // Read temperature data
            let data_raw_temperature = sensor.temperature_raw_get().await.unwrap();
            let temperature_deg_c = from_lsb_to_celsius(data_raw_temperature);
            writeln!(tx, "Temperature [degC]: {:.2}", temperature_deg_c).unwrap();
        }

        delay.delay_ms(1000_u32).await;
    }}
