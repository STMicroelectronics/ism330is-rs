use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, mut delay: D, mut int_pin : I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
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

    let mut val = sensor.pin_int1_route_get().await.unwrap();
    val.drdy_xl = 1;
    val.drdy_gy = 1;
    sensor.pin_int1_route_set(val).await.unwrap();

    /* Enable Block Data Update */
    //ism330is_rs_block_data_update_set(1).await.unwrap();
    /* Set Output Data Rate */
    sensor.xl_data_rate_set(XlDataRate::_12_5hzHp).await.unwrap();
    sensor.gy_data_rate_set(GyDataRate::_12_5hzHp).await.unwrap();

    /* Set full scale */
    sensor.xl_full_scale_set(XlFullScale::_2g).await.unwrap();
    sensor.gy_full_scale_set(GyFullScale::_2000dps).await.unwrap();

    /* Configure filtering chain(No aux interface)
     * Accelerometer - LPF1 + LPF2 path
     */
    //ism330is_rs_xl_hp_path_on_out_set(&dev_ctx, LSM6DSO16IS_LP_ODR_DIV_100);
    //ism330is_rs_xl_filter_lp2_set(&dev_ctx, PROPERTY_ENABLE);

    /* Read samples */
    let mut acc_mg = [0f32; 3];
    let mut angular_rate_mdps = [0f32; 3];

    loop {
        int_pin.wait_for_event().await;

        let flag = sensor.xl_flag_data_ready_get().await.unwrap();

        if flag != 0 {
            let data_raw_acceleration = sensor.acceleration_raw_get().await.unwrap();

            for i in 0..3 {
                acc_mg[i] = from_fs2g_to_mg(data_raw_acceleration[i]);
            }

            writeln!(
                tx,
                "Acceleration [mg]: {:4.2}\t{:4.2}\t{:4.2}",
                acc_mg[0], acc_mg[1], acc_mg[2]
            )
            .unwrap();
        }

        let flag = sensor.gy_flag_data_ready_get().await.unwrap();

        if flag != 0 {
            let data_raw_angular_rate = sensor.angular_rate_raw_get().await.unwrap();

            for i in 0..3 {
                angular_rate_mdps[i] =
                    from_fs2000dps_to_mdps(data_raw_angular_rate[i]);
            }

            writeln!(
                tx,
                "Angular rate [mdps]: {:4.2}\t{:4.2}\t{:4.2}",
                angular_rate_mdps[0], angular_rate_mdps[1], angular_rate_mdps[2]
            )
            .unwrap();
        }

        let flag = sensor.temp_flag_data_ready_get().await.unwrap();

        if flag != 0 {
            let data_raw_temperature = sensor.temperature_raw_get().await.unwrap();
            let temperature_deg_c = from_lsb_to_celsius(data_raw_temperature);

            writeln!(tx, "Temperature [degC]: {:6.2}", temperature_deg_c).unwrap();
        }
    }
}
