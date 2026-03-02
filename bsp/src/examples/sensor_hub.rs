use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[cfg(feature = "async")]
use lis2mdl_rs::asynchronous as lis2mdl;
#[cfg(feature = "async")]
use lps22df_rs::asynchronous as lps22df;

#[cfg(not(feature = "async"))]
use lis2mdl_rs::blocking as lis2mdl;
#[cfg(not(feature = "async"))]
use lps22df_rs::blocking as lps22df;
const LIS2MDL_ADDR: u8 = lis2mdl::I2CAddress::I2cAdd as u8;
const LPS22DF_ADDR: u8 = lps22df::I2CAddress::I2cAddH as u8;

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

    // Check device ID
    match sensor.device_id_get().await {
        Ok(id) => {
            if id != ism330is::ID {
                writeln!(tx, "Device (ISM330IS) ID mismatch: {:#02x}", id).unwrap();
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
    }

    // Restore default configuration
    sensor.software_reset().await.unwrap();

    // Enable Block Data Update
    sensor.block_data_update_set(1).await.unwrap();
    // Set full scale
    sensor.xl_full_scale_set(XlFullScale::_2g).await.unwrap();

    /*
     * Configure LIS2MDL target
     */

    let pass = Ism330isPassthrough::new_from_sensor(&mut sensor, LIS2MDL_ADDR);
    let mut lis2mdl = lis2mdl::Lis2mdl::new_bus(pass, delay.clone());
    info!("initializing LIS2MDL");

    match lis2mdl.device_id_get().await {
        Ok(id) => {
            if id != lis2mdl::ID {
                writeln!(tx, "Device (LIS2MDL) ID mismatch: {:#02x}", id).unwrap();
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
    }

    // Restore default configuration
    lis2mdl.sw_reset().await.unwrap();

    lis2mdl.block_data_update_set(1).await.unwrap();
    lis2mdl.offset_temp_comp_set(1).await.unwrap();
    lis2mdl
        .operating_mode_set(lis2mdl::prelude::Md::ContinuousMode)
        .await
        .unwrap();
    lis2mdl.data_rate_set(lis2mdl::prelude::Odr::_20hz).await.unwrap();

    /*
     * Configure LPS22DF target
     */
    let pass = Ism330isPassthrough::new_from_sensor(&mut sensor, LPS22DF_ADDR);
    let mut lps22df = lps22df::Lps22df::new_bus(pass, delay.clone());
    info!("initializing LPS22DF");

    // Check if LPS22DF connected to Sensor Hub.
    match lps22df.id_get().await {
        Ok(id) => {
            if id != lps22df::ID {
                writeln!(tx, "Device (LPS22DF) ID mismatch: {:#02x}", id).unwrap();
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
    }

    // Restore default configuration
    lps22df.init_set(lps22df::prelude::Init::Reset).await.unwrap();
    loop {
        let status = lps22df.status_get().await.unwrap();
        if status.sw_reset == 0 {
            break;
        }
    }

    // Set bdu and if if_inc recommeded for driver usage
    lps22df.init_set(lps22df::prelude::Init::DrvRdy).await.unwrap();

    // Select bus interface
    let mut bus_mode = lps22df::prelude::BusMode::default();
    bus_mode.filter = lps22df::prelude::Filter::FilterAuto;
    bus_mode.interface = lps22df::prelude::Interface::SelByHw;
    lps22df.bus_mode_set(&bus_mode).await.unwrap();

    // Set Output Data Rate
    let mut md = lps22df::prelude::Md::default();
    md.odr = lps22df::prelude::Odr::_4hz;
    md.avg = lps22df::prelude::Avg::_16;
    md.lpf = lps22df::prelude::LowPassFilter::OdrDiv4;
    lps22df.mode_set(&md).await.unwrap();

    /*
     *  Slave settings ended: take direct ownership of the master
     */
    let mut ism330is = sensor;

    let mut pin_int = PinInt1Route::default();
    pin_int.drdy_xl = 1;
    ism330is.pin_int1_route_set(pin_int).await.unwrap();

    // Set Output Data Rate
    ism330is
        .xl_data_rate_set(XlDataRate::_26hzHp)
        .await.unwrap();

    // Set full scale
    ism330is.xl_full_scale_set(XlFullScale::_2g).await.unwrap();
    ism330is.xl_data_rate_set(XlDataRate::Off).await.unwrap();

    /*
     * Prepare sensor hub to read data from external slave0 (lis2mdl) and
     * slave1 (lps22df) continuously in order to store data in FIFO.
     */

    let mut sh_cfg_read = ShCfgRead::default();
    sh_cfg_read.slv_add = lis2mdl::I2CAddress::I2cAdd as u8; // 7bit I2C address
    sh_cfg_read.slv_subadd = lis2mdl::prelude::Reg::OutxLReg as u8;
    sh_cfg_read.slv_len = 6;
    ism330is.sh_slv_cfg_read(0, &sh_cfg_read).await.unwrap();

    sh_cfg_read.slv_add = lps22df::I2CAddress::I2cAddH as u8; // 7bit I2C address
    sh_cfg_read.slv_subadd = lps22df::prelude::Reg::PressOutXl as u8;
    sh_cfg_read.slv_len = 6;
    ism330is.sh_slv_cfg_read(1, &sh_cfg_read).await.unwrap();

    // Configure Sensor Hub data rate
    ism330is.sh_data_rate_set(ShDataRate::_52hz).await.unwrap();

    // Configure Sensor Hub to read two slave.
    ism330is
        .sh_slave_connected_set(ShSlaveConnected::_01)
        .await.unwrap();

    // Set SHUB write_once bit
    ism330is
        .sh_write_mode_set(ShWriteMode::OnlyFirstCycle)
        .await.unwrap();

    // Enable I2C Master
    ism330is.sh_master_set(1).await.unwrap();

    /* Set Output Data Rate.
     * Selected data rate have to be equal or greater with respect
     * with MLC data rate.
     */
    ism330is
        .xl_data_rate_set(XlDataRate::_26hzHp)
        .await.unwrap();

    let mut acceleration_mg = [0f32; 3];

    loop {
        int_pin.wait_for_event().await;

        let acceleration_raw = ism330is.acceleration_raw_get().await.unwrap();

        for i in 0..3 {
            acceleration_mg[i] = from_fs2g_to_mg(acceleration_raw[i]);
        }

        writeln!(
            tx,
            "Acceleration [mg]:{:.2}\t{:.2}\t{:.2}",
            acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
        )
        .unwrap();

        let mut data_raw_sh = [0u8; 12];
        ism330is
            .sh_read_data_raw_get(&mut data_raw_sh)
            .await.unwrap();

        // magnetometer conversion
        let magx = ((data_raw_sh[1] as i16) << 8) + data_raw_sh[0] as i16;
        let magy = ((data_raw_sh[3] as i16) << 8) + data_raw_sh[2] as i16;
        let magz = ((data_raw_sh[5] as i16) << 8) + data_raw_sh[4] as i16;

        let magx = lis2mdl::from_lsb_to_mgauss(magx);
        let magy = lis2mdl::from_lsb_to_mgauss(magy);
        let magz = lis2mdl::from_lsb_to_mgauss(magz);
        writeln!(tx, "LIS2MDL [mGa]:\t{:.2}\t{:.2}\t{:.2}", magx, magy, magz).unwrap();

        // pressure conversion
        let baro = data_raw_sh[8] as i32;
        let baro = (baro << 8) + data_raw_sh[7] as i32;
        let baro = (baro << 8) + data_raw_sh[6] as i32;
        let baro = baro << 8;
        let baro = lps22df::from_lsb_to_hpa(baro as i32);

        // temperature conversion
        let temp = ((data_raw_sh[10] as u16) << 8) + data_raw_sh[9] as u16;
        let temp = lps22df::from_lsb_to_celsius(temp as i16);

        writeln!(tx, "LPS22DF [hPa]:{:.2} [degC]{:.2}", baro, temp).unwrap();
    }
}
