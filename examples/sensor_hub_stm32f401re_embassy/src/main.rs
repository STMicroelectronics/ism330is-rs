#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, Config};
use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{self, Config as I2cConfig, I2c};
use embassy_stm32::peripherals::{self, USART2};
use embassy_stm32::time::{khz, mhz};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv, PllSource,
    Sysclk,
};
use embassy_stm32::usart::{
    BufferedInterruptHandler, Config as UsartConfig, DataBits, Parity, UartTx,
};
use embassy_time::Delay;
use heapless::String;
use ism330is_rs as ism330is;
use ism330is_rs::prelude::*;
use lis2mdl_rs as lis2mdl;
use lps22df_rs as lps22df;
use st_mems_bus;
use st_mems_bus::i2c::I2cBus;
use {defmt_rtt as _, panic_probe as _};

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
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(8),
        mode: HseMode::Bypass,
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.pll_src = PllSource::HSI;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV16,
        mul: PllMul::MUL336,
        divp: Some(PllPDiv::DIV4),
        divq: Some(PllQDiv::DIV7),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.hsi = true;

    let mut usart_config: UsartConfig = UsartConfig::default();
    usart_config.baudrate = 115200;
    usart_config.data_bits = DataBits::DataBits8;
    usart_config.parity = Parity::ParityNone;
    let p = embassy_stm32::init(config);

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
    // On the Nucleo FR401 the interrupt pin is connected to pin PC0.
    let interrupt = Input::new(p.PC0, Pull::None);
    let mut interrupt = ExtiInput::new(interrupt, p.EXTI0);

    delay.delay_ms(5_u32);

    let mut msg: String<256> = String::new();

    let ism330is_addr = ism330is::I2CAddress::I2cAddL;
    let lis2mdl_addr = lis2mdl::I2CAddress::I2cAdd as u8;
    let lps22df_addr = lps22df::I2CAddress::I2cAddH as u8;

    // initialize the master
    let master =
        ism330is::Ism330isMaster::from_bus(I2cBus::new(i2c, ism330is_addr as u8), delay.clone());

    // initialize lis2mdl as Ism330is slave
    let ism330is_for_lis2mdl = master.as_passthrough(lis2mdl_addr);
    let mut lis2mdl = lis2mdl::Lis2mdl::new_bus(ism330is_for_lis2mdl);

    // initialize lps22df as Ism330is slave
    let ism330is_for_lps22df = master.as_passthrough(lps22df_addr);
    let mut lps22df = lps22df::Lps22df::new_bus(ism330is_for_lps22df, delay.clone());

    // Check device ID
    match master.borrow_mut().device_id_get() {
        Ok(id) => {
            if id != ism330is::ID {
                writeln!(&mut msg, "Device (ISM330IS) ID mismatch: {:#02x}", id).unwrap();
                let _ = tx.blocking_write(msg.as_bytes());
                msg.clear();
                loop {}
            }
        }
        Err(e) => {
            writeln!(&mut msg, "Error in reading id: {:?}", e).unwrap();
            let _ = tx.blocking_write(msg.as_bytes());
            msg.clear();
        }
    }

    // Restore default configuration
    master.borrow_mut().software_reset().unwrap();

    // Enable Block Data Update
    master.borrow_mut().block_data_update_set(1).unwrap();
    // Set full scale
    master
        .borrow_mut()
        .xl_full_scale_set(XlFullScale::_2g)
        .unwrap();

    /*
     * Configure LIS2MDL target
     */

    match lis2mdl.device_id_get() {
        Ok(id) => {
            if id != lis2mdl::ID {
                writeln!(&mut msg, "Device (LIS2MDL) ID mismatch: {:#02x}", id).unwrap();
                let _ = tx.blocking_write(msg.as_bytes());
                msg.clear();
                loop {}
            }
        }
        Err(e) => {
            writeln!(&mut msg, "Error in reading id: {:?}", e).unwrap();
            let _ = tx.blocking_write(msg.as_bytes());
            msg.clear();
        }
    }

    // Restore default configuration
    lis2mdl.reset_set(1).unwrap();

    loop {
        let rst = lis2mdl.reset_get().unwrap();
        if rst == 0 {
            break;
        }
    }

    lis2mdl.block_data_update_set(1).unwrap();
    lis2mdl.offset_temp_comp_set(1).unwrap();
    lis2mdl
        .operating_mode_set(lis2mdl::prelude::Md::ContinuousMode)
        .unwrap();
    lis2mdl.data_rate_set(lis2mdl::prelude::Odr::_20hz).unwrap();
    /*
     * Configure LPS22DF target
     */

    // Check if LPS22DF connected to Sensor Hub.
    match lps22df.id_get() {
        Ok(id) => {
            if id != lps22df::ID {
                writeln!(&mut msg, "Device (LPS22DF) ID mismatch: {:#02x}", id).unwrap();
                let _ = tx.blocking_write(msg.as_bytes());
                msg.clear();
                loop {}
            }
        }
        Err(e) => {
            writeln!(&mut msg, "Error in reading id: {:?}", e).unwrap();
            let _ = tx.blocking_write(msg.as_bytes());
            msg.clear();
        }
    }

    // Restore default configuration
    lps22df.init_set(lps22df::prelude::Init::Reset).unwrap();
    loop {
        let status = lps22df.status_get().unwrap();
        if status.sw_reset == 0 {
            break;
        }
    }

    // Set bdu and if if_inc recommeded for driver usage
    lps22df.init_set(lps22df::prelude::Init::DrvRdy).unwrap();

    // Select bus interface
    let mut bus_mode = lps22df::prelude::BusMode::default();
    bus_mode.filter = lps22df::prelude::Filter::FilterAuto;
    bus_mode.interface = lps22df::prelude::Interface::SelByHw;
    lps22df.bus_mode_set(&bus_mode).unwrap();

    // Set Output Data Rate
    let mut md = lps22df::prelude::Md::default();
    md.odr = lps22df::prelude::Odr::_4hz;
    md.avg = lps22df::prelude::Avg::_16;
    md.lpf = lps22df::prelude::LowPassFilter::OdrDiv4;
    lps22df.mode_set(&md).unwrap();

    /*
     *  Slave settings ended: take direct ownership of the master
     */
    // drop to ensure no other borrow_mut from slaves, otherwise borrow_mut()
    // call should be called for every function call of the master
    drop(lps22df);
    drop(lis2mdl);

    let mut ism330is = master.borrow_mut();

    let mut pin_int = PinInt1Route::default();
    pin_int.drdy_xl = 1;
    ism330is.pin_int1_route_set(pin_int).unwrap();

    // Set Output Data Rate
    ism330is.xl_data_rate_set(XlDataRate::_26hzHp).unwrap();

    // Set full scale
    ism330is.xl_full_scale_set(XlFullScale::_2g).unwrap();
    ism330is.xl_data_rate_set(XlDataRate::Off).unwrap();

    /*
     * Prepare sensor hub to read data from external slave0 (lis2mdl) and
     * slave1 (lps22df) continuously in order to store data in FIFO.
     */

    let mut sh_cfg_read = ShCfgRead::default();
    sh_cfg_read.slv_add = lis2mdl::I2CAddress::I2cAdd as u8; // 7bit I2C address
    sh_cfg_read.slv_subadd = lis2mdl::prelude::Reg::OutxLReg as u8;
    sh_cfg_read.slv_len = 6;
    ism330is.sh_slv_cfg_read(0, &sh_cfg_read).unwrap();

    sh_cfg_read.slv_add = lps22df::I2CAddress::I2cAddH as u8; // 7bit I2C address
    sh_cfg_read.slv_subadd = lps22df::prelude::Reg::PressOutXl as u8;
    sh_cfg_read.slv_len = 6;
    ism330is.sh_slv_cfg_read(1, &sh_cfg_read).unwrap();

    // Configure Sensor Hub data rate
    ism330is.sh_data_rate_set(ShDataRate::_52hz).unwrap();

    // Configure Sensor Hub to read two slave.
    ism330is
        .sh_slave_connected_set(ShSlaveConnected::_01)
        .unwrap();

    // Set SHUB write_once bit
    ism330is
        .sh_write_mode_set(ShWriteMode::OnlyFirstCycle)
        .unwrap();

    // Enable I2C Master
    ism330is.sh_master_set(1).unwrap();

    /* Set Output Data Rate.
     * Selected data rate have to be equal or greater with respect
     * with MLC data rate.
     */
    ism330is.xl_data_rate_set(XlDataRate::_26hzHp).unwrap();

    let mut acceleration_mg = [0f32; 3];

    loop {
        let acceleration_raw = ism330is.acceleration_raw_get().unwrap();

        for i in 0..3 {
            acceleration_mg[i] = ism330is::from_fs2g_to_mg(acceleration_raw[i]);
        }

        writeln!(
            &mut msg,
            "Acceleration [mg]:{:.2}\t{:.2}\t{:.2}",
            acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
        )
        .unwrap();
        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();

        let mut data_raw_sh = [0u8; 12];
        ism330is.sh_read_data_raw_get(&mut data_raw_sh).unwrap();

        // magnetometer conversion
        let magx = ((data_raw_sh[1] as i16) << 8) + data_raw_sh[0] as i16;
        let magy = ((data_raw_sh[3] as i16) << 8) + data_raw_sh[2] as i16;
        let magz = ((data_raw_sh[5] as i16) << 8) + data_raw_sh[4] as i16;

        let magx = lis2mdl::from_lsb_to_mgauss(magx);
        let magy = lis2mdl::from_lsb_to_mgauss(magy);
        let magz = lis2mdl::from_lsb_to_mgauss(magz);
        writeln!(
            &mut msg,
            "LIS2MDL [mGa]:\t{:.2}\t{:.2}\t{:.2}",
            magx, magy, magz
        )
        .unwrap();
        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();

        // pressure conversion
        let baro = data_raw_sh[8] as i32;
        let baro = (baro << 8) + data_raw_sh[7] as i32;
        let baro = (baro << 8) + data_raw_sh[6] as i32;
        let baro = baro << 8;
        let baro = lps22df::from_lsb_to_hpa(baro as i32);

        // temperature conversion
        let temp = ((data_raw_sh[10] as u16) << 8) + data_raw_sh[9] as u16;
        let temp = lps22df::from_lsb_to_celsius(temp as i16);

        writeln!(&mut msg, "LPS22DF [hPa]:{:.2} [degC]{:.2}", baro, temp).unwrap();
        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();

        interrupt.wait_for_rising_edge().await;
    }
}
/*
struct Ism330isWrapper<B, T> {
    instance: ism330is::Ism330is<B, T>,
    slave_address: SevenBitAddress
}


impl<B, T> bus::BusOperation for Ism330isWrapper<B, T> where B: bus::BusOperation, T: DelayNs{
    type Error = Error<B::Error>;

    fn read_bytes(&mut self, _rbuf: &mut [u8]) -> Result<(), Self::Error> {
        Err(Error::UnexpectedValue)
    }

    fn write_bytes(&mut self, wbuf: &[u8]) -> Result<(), Self::Error> {
        let mut sh_cfg_write = ShCfgWrite::default();

        for i in 1_u8..(wbuf.len() as u8) {
            // Configure Sensor Hub to read data
            sh_cfg_write.slv0_add = self.slave_address;
            sh_cfg_write.slv0_subadd = wbuf[0] + i - 1;
            sh_cfg_write.slv0_data = wbuf[i as usize];
            self.sensor_instance.sh_cfg_write(sh_cfg_write)?;

            // Disable accelerometer
            self.sensor_instance.xl_data_rate_set(XlDataRate::Off)?;
            // Enable I2C Master
            self.sensor_instance.sh_master_set(1)?;
            // Enable accelerometer to trigger Sensor Hub operation.
            self.sensor_instance.xl_data_rate_set(XlDataRate::_26hzHp)?;
            // Wait Sensor Hub operation flag set.
            let _dummy = self.sensor_instance.acceleration_raw_get();

            let mut drdy = 0;
            while drdy == 0 {
                self.sensor_instance.tim.delay_ms(20);
                drdy = self.sensor_instance.xl_flag_data_ready_get()?;
            }

            let mut end_op = 0;
            while end_op == 0 {
                self.sensor_instance.tim.delay_ms(20);
                end_op = self.sensor_instance.sh_status_get()?.sens_hub_endop();
            }

            // Disable I2C master and XL (triger).
            self.sensor_instance.sh_master_set(0)?;
            self.sensor_instance.xl_data_rate_set(XlDataRate::Off)?;
        }

        Ok(())
    }

    fn write_byte_read_bytes(
        &mut self,
        wbuf: &[u8; 1],
        rbuf: &mut [u8],
    ) -> Result<(), Self::Error> {
        // Disable accelerometer
        self.sensor_instance.xl_data_rate_set(XlDataRate::Off)?;
        // Configure Sensor Hub to read
        let sh_cfg_read = ShCfgRead {
            slv_add: self.slave_address,
            slv_subadd: wbuf[0],
            slv_len: rbuf.len() as u8,
        };
        self.sensor_instance.sh_slv_cfg_read(0, &sh_cfg_read)?; // dummy read
        self.sensor_instance.sh_slave_connected_set(ShSlaveConnected::_01)?;
        // Enable I2C Master
        self.sensor_instance.sh_master_set(1)?;
        // Enable accelerometer to trigger Sensor Hub operation.
        self.sensor_instance.xl_data_rate_set(XlDataRate::_26hzHp)?;
        // Wait Sensor Hub operation flag set
        let _dummy = self.sensor_instance.acceleration_raw_get()?;

        let mut drdy = 0;
        while drdy == 0 {
            self.sensor_instance.tim.delay_ms(20);
            drdy = self.sensor_instance.xl_flag_data_ready_get()?;
        }

        let mut end_op = 0;
        while end_op == 0 {
            //self.sensor_instance.tim.delay_ms(20);
            end_op = self.sensor_instance.sh_status_get()?.sens_hub_endop();
        }

        // Disable I2C master and XL(trigger)
        self.sensor_instance.sh_master_set(0)?;
        self.sensor_instance.xl_data_rate_set(XlDataRate::Off)?;

        // Read SensorHub registers
        self.sensor_instance.sh_read_data_raw_get(rbuf)?;

        Ok(())
    }
}
*/
