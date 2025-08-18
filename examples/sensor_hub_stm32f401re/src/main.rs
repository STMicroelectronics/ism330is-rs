#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write;
use st_mems_bus;
use st_mems_bus::i2c::I2cBus;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use ism330is::prelude::*;
use ism330is_rs as ism330is;
use lis2mdl_rs as lis2mdl;
use lps22df_rs as lps22df;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio,
    gpio::{Edge, Input},
    i2c::{DutyCycle, I2c, Mode},
    interrupt, pac,
    prelude::*,
    serial::{config::Config, Serial},
};

type IntPin = gpio::PC0<Input>;

static INT_PIN: Mutex<RefCell<Option<IntPin>>> = Mutex::new(RefCell::new(None));
static MEMS_EVENT: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);
    let tim1 = dp.TIM1.delay_us(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();
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

    let mut int_pin = gpioc.pc0.into_input();
    // Configure Pin for Interrupts
    // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
    let mut syscfg = dp.SYSCFG.constrain();
    // 2) Make an interrupt source
    int_pin.make_interrupt_source(&mut syscfg);
    // 3) Make an interrupt source
    int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // 4) Enable gpio interrupt
    int_pin.enable_interrupt(&mut dp.EXTI); // Enable the external interrupt in the NVIC

    // Enable the external interrupt in the NVIC by passing the interrupt number
    unsafe {
        cortex_m::peripheral::NVIC::unmask(int_pin.interrupt());
    }

    // Now that pin is configured, move pin into global context
    cortex_m::interrupt::free(|cs| {
        INT_PIN.borrow(cs).replace(Some(int_pin));
    });

    delay.delay_ms(5);

    let ref_tim = RefCell::new(tim1);

    let ism330is_rs_addr = ism330is::I2CAddress::I2cAddL;
    let lis2mdl_addr = lis2mdl::I2CAddress::I2cAdd as u8;
    let lps22df_addr = lps22df::I2CAddress::I2cAddH as u8;

    // initialize the master
    let master = ism330is_rs::Ism330isMaster::from_bus(
        I2cBus::new(i2c, ism330is_rs_addr as u8),
        st_mems_bus::Shared::new(&ref_tim),
    );

    // initialize lis2mdl as Ism330is slave
    let ism330is_rs_for_lis2mdl = master.as_passthrough(lis2mdl_addr);
    let mut lis2mdl = lis2mdl::Lis2mdl::new_bus(ism330is_rs_for_lis2mdl);

    // initialize lps22df as Ism330is slave
    let ism330is_rs_for_lps22df = master.as_passthrough(lps22df_addr);
    let mut lps22df = lps22df::Lps22df::new_bus(
        ism330is_rs_for_lps22df,
        st_mems_bus::Shared::new(&ref_tim),
    );

    // Check device ID
    match master.borrow_mut().device_id_get() {
        Ok(id) => {
            if id != ism330is_rs::ID {
                writeln!(tx, "Device (ISM330IS) ID mismatch: {:#02x}", id).unwrap();
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
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
                writeln!(tx, "Device (LIS2MDL) ID mismatch: {:#02x}", id).unwrap();
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
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
                writeln!(tx, "Device (LPS22DF) ID mismatch: {:#02x}", id).unwrap();
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
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

    let mut ism330is_rs = master.borrow_mut();

    let mut pin_int = PinInt1Route::default();
    pin_int.drdy_xl = 1;
    ism330is_rs.pin_int1_route_set(pin_int).unwrap();

    // Set Output Data Rate
    ism330is_rs
        .xl_data_rate_set(XlDataRate::_26hzHp)
        .unwrap();

    // Set full scale
    ism330is_rs.xl_full_scale_set(XlFullScale::_2g).unwrap();
    ism330is_rs.xl_data_rate_set(XlDataRate::Off).unwrap();

    /*
     * Prepare sensor hub to read data from external slave0 (lis2mdl) and
     * slave1 (lps22df) continuously in order to store data in FIFO.
     */

    let mut sh_cfg_read = ShCfgRead::default();
    sh_cfg_read.slv_add = lis2mdl::I2CAddress::I2cAdd as u8; // 7bit I2C address
    sh_cfg_read.slv_subadd = lis2mdl::prelude::Reg::OutxLReg as u8;
    sh_cfg_read.slv_len = 6;
    ism330is_rs.sh_slv_cfg_read(0, &sh_cfg_read).unwrap();

    sh_cfg_read.slv_add = lps22df::I2CAddress::I2cAddH as u8; // 7bit I2C address
    sh_cfg_read.slv_subadd = lps22df::prelude::Reg::PressOutXl as u8;
    sh_cfg_read.slv_len = 6;
    ism330is_rs.sh_slv_cfg_read(1, &sh_cfg_read).unwrap();

    // Configure Sensor Hub data rate
    ism330is_rs.sh_data_rate_set(ShDataRate::_52hz).unwrap();

    // Configure Sensor Hub to read two slave.
    ism330is_rs
        .sh_slave_connected_set(ShSlaveConnected::_01)
        .unwrap();

    // Set SHUB write_once bit
    ism330is_rs
        .sh_write_mode_set(ShWriteMode::OnlyFirstCycle)
        .unwrap();

    // Enable I2C Master
    ism330is_rs.sh_master_set(1).unwrap();

    /* Set Output Data Rate.
     * Selected data rate have to be equal or greater with respect
     * with MLC data rate.
     */
    ism330is_rs
        .xl_data_rate_set(XlDataRate::_26hzHp)
        .unwrap();

    let mut acceleration_mg = [0f32; 3];

    loop {
        let acceleration_raw = ism330is_rs.acceleration_raw_get().unwrap();

        for i in 0..3 {
            acceleration_mg[i] = ism330is_rs::from_fs2g_to_mg(acceleration_raw[i]);
        }

        writeln!(
            tx,
            "Acceleration [mg]:{:.2}\t{:.2}\t{:.2}",
            acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
        )
        .unwrap();

        let mut data_raw_sh = [0u8; 12];
        ism330is_rs
            .sh_read_data_raw_get(&mut data_raw_sh)
            .unwrap();

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

        // Wait for interrupt
        let mems_event = cortex_m::interrupt::free(|cs| {
            let flag = *MEMS_EVENT.borrow(cs).borrow();
            if flag {
                MEMS_EVENT.borrow(cs).replace(false);
            }
            flag
        });
        if !mems_event {
            continue;
        }
    }
}

#[interrupt]
fn EXTI0() {
    cortex_m::interrupt::free(|cs| {
        let mut int_pin = INT_PIN.borrow(cs).borrow_mut();
        if int_pin.as_mut().unwrap().check_interrupt() {
            int_pin.as_mut().unwrap().clear_interrupt_pending_bit();
        }
        MEMS_EVENT.borrow(cs).replace(true);
    })
}

/*

// To provide custom Passthrough implementation use this code:

struct Ism330isWrapper<B, T> {
    instance: ism330is_rs::Ism330is<B, T>,
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
