#![no_std]
#![doc = include_str!("../README.md")]

pub mod prelude;
pub mod register;

#[cfg(feature = "passthrough")]
use core::cell::RefCell;
#[cfg(feature = "passthrough")]
use core::cell::RefMut;
use core::fmt::Debug;
use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::{I2c, SevenBitAddress};
use embedded_hal::spi::SpiDevice;
use prelude::*;

use st_mems_bus::{BusOperation, MemBankFunctions, i2c::*, spi::*};

/// Driver for the Ism330is sensor.
///
/// The struct takes a bus and a timer hardware object to write to the
/// registers.
/// The bus is generalized over the BusOperation trait, allowing the use
/// of I2C or SPI protocols; this also allows the user to implement sharing
/// techniques to share the underlying bus.
pub struct Ism330is<B, T> {
    /// The bus driver.
    pub bus: B,
    pub tim: T,
}

/// Driver errors.
#[derive(Debug)]
pub enum Error<B> {
    Bus(B),          // Error at the bus level
    UnexpectedValue, // Unexpected value read from a register
    FailedToReadMemBank,
    FailedToSetMemBank(MemBank),
}

impl<P, T> Ism330is<I2cBus<P>, T>
where
    P: I2c,
    T: DelayNs,
{
    /// Constructor method for using the I2C bus.
    pub fn new_i2c(i2c: P, address: I2CAddress, tim: T) -> Self {
        // Initialize the I2C bus with the COMPONENT address
        let bus = st_mems_bus::i2c::I2cBus::new(i2c, address as SevenBitAddress);
        Self { bus, tim }
    }
}

impl<B, T> Ism330is<B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    /// Constructor method using a generic Bus that implements BusOperation and a generic hardware
    /// timer
    pub fn from_bus(bus: B, tim: T) -> Self {
        Self { bus, tim }
    }
}

impl<P, T> Ism330is<SpiBus<P>, T>
where
    P: SpiDevice,
    T: DelayNs,
{
    /// Constructor method for using the SPI bus.
    pub fn new_spi(spi: P, tim: T) -> Self {
        // Initialize the SPI bus
        let bus = st_mems_bus::spi::SpiBus::new(spi);
        Self { bus, tim }
    }
}

impl<B, T> MemBankFunctions<MemBank> for Ism330is<B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    type Error = Error<B::Error>;

    /// Change memory bank.
    fn mem_bank_set(&mut self, val: MemBank) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::from_bits(0);

        // Set the shub_reg_access and ispu_reg_access fields based on the value of val
        func_cfg_access.set_shub_reg_access(if val == MemBank::SensorHubMemBank {
            1
        } else {
            0
        });
        func_cfg_access.set_ispu_reg_access(if val == MemBank::IspuMemBank { 1 } else { 0 });

        // Write the updated func_cfg_access to the register
        func_cfg_access
            .write(self)
            .map_err(|_| Error::FailedToSetMemBank(val))
    }

    /// Get the actual MemoryBank set
    fn mem_bank_get(&mut self) -> Result<MemBank, Error<B::Error>> {
        let func_cfg_access = FuncCfgAccess::read(self).map_err(|_| Error::FailedToReadMemBank)?;

        let val = if func_cfg_access.shub_reg_access() == 1 {
            MemBank::SensorHubMemBank
        } else if func_cfg_access.ispu_reg_access() == 1 {
            MemBank::IspuMemBank
        } else {
            MemBank::MainMemBank
        };

        Ok(val)
    }
}

impl<B: BusOperation, T: DelayNs> Ism330is<B, T> {
    pub fn write_to_register(&mut self, reg: u8, buf: &[u8]) -> Result<(), Error<B::Error>> {
        self.bus.write_to_register(reg, buf).map_err(Error::Bus)
    }

    pub fn read_from_register(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), Error<B::Error>> {
        self.bus.read_from_register(reg, buf).map_err(Error::Bus)
    }

    /// Difference in percentage of the effective ODR (and timestamp rate)
    /// with respect to the typical. (set)
    /// Step: 0.15%. 8-bit format, 2's complement.
    pub fn odr_cal_reg_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut internal_freq_fine = InternalFreqFine::read(self)?;
        internal_freq_fine.set_freq_fine(val);
        internal_freq_fine.write(self)?;

        Ok(())
    }
    /// Get the difference in percentage of the effective ODR (and timestamp rate)
    /// with respect to the typical.
    /// It changes the register `INTERNAL_FREQ_FINE`
    /// Step:  0.15%. 8-bit format, 2's complement.
    pub fn odr_cal_reg_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = InternalFreqFine::read(self).map(|reg| reg.freq_fine())?;

        Ok(val)
    }

    /// Enables pulsed data-ready mode: Latched/Pulsed(~75 us).
    pub fn data_ready_mode_set(&mut self, val: DataReadyMode) -> Result<(), Error<B::Error>> {
        let mut drdy_pulsed_reg = DrdyPulsedReg::read(self)?;
        drdy_pulsed_reg.set_drdy_pulsed((val as u8) & 0x1);
        drdy_pulsed_reg.write(self)?;

        Ok(())
    }
    /// Get the actual setting of data-ready mode: Latched/Pulsed(~75 us).
    pub fn data_ready_mode_get(&mut self) -> Result<DataReadyMode, Error<B::Error>> {
        let reg = DrdyPulsedReg::read(self)?;
        let val = DataReadyMode::try_from(reg.drdy_pulsed()).unwrap_or_default();

        Ok(val)
    }

    /// Get the Device ID.
    pub fn device_id_get(&mut self) -> Result<u8, Error<B::Error>> {
        WhoAmI::read(self).map(|reg| reg.into())
    }

    /// Software reset. Restore the default values in user registers.
    pub fn software_reset(&mut self) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;

        self.xl_data_rate_set(XlDataRate::Off)?;
        self.gy_data_rate_set(GyDataRate::Off)?;

        ctrl3_c.set_sw_reset(1);
        ctrl3_c.write(self)?;

        while {
            ctrl3_c = Ctrl3C::read(self)?;
            ctrl3_c.sw_reset() == 1
        } {}

        Ok(())
    }

    /// Reboot memory content. Reload the calibration parameters.
    ///
    /// If val equals to 1: reboot the memory content.
    pub fn boot_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;
        ctrl3_c.set_boot(val);
        ctrl3_c.write(self)?;

        Ok(())
    }

    /// Get the value of boot.
    pub fn boot_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = Ctrl3C::read(self).map(|reg| reg.boot())?;

        Ok(val)
    }
    /// Enable or disable accelerometer high-performance mode.
    pub fn xl_hm_mode_set(&mut self, val: HighPerformanceMode) -> Result<(), Error<B::Error>> {
        let mut ctrl6_c = Ctrl6C::read(self)?;
        ctrl6_c.set_xl_hm_mode((val as u8) & 0x1);
        ctrl6_c.write(self)?;

        Ok(())
    }

    /// Get the current accelerometer high-performance mode.
    pub fn xl_hm_mode_get(&mut self) -> Result<HighPerformanceMode, Error<B::Error>> {
        let ctrl6_c = Ctrl6C::read(self)?;

        let mode = HighPerformanceMode::try_from(ctrl6_c.xl_hm_mode()).unwrap_or_default();

        Ok(mode)
    }

    /// Set the accelerometer full-scale.
    pub fn xl_full_scale_set(&mut self, val: XlFullScale) -> Result<(), Error<B::Error>> {
        let mut ctrl1_xl = Ctrl1Xl::read(self)?;
        ctrl1_xl.set_fs_xl((val as u8) & 0x3);
        ctrl1_xl.write(self)?;

        Ok(())
    }

    /// Get the current accelerometer full-scale configuration.
    pub fn xl_full_scale_get(&mut self) -> Result<XlFullScale, Error<B::Error>> {
        let ctrl1_xl = Ctrl1Xl::read(self)?;

        let val = XlFullScale::try_from(ctrl1_xl.fs_xl()).unwrap_or_default();

        Ok(val)
    }

    /// Set the accelerometer output data rate (ODR).
    pub fn xl_data_rate_set(&mut self, val: XlDataRate) -> Result<(), Error<B::Error>> {
        let mut ctrl1_xl = Ctrl1Xl::read(self)?;

        if ((val as u8) & 0x10) == 0x10 {
            self.xl_hm_mode_set(HighPerformanceMode::Disabled)?;
        } else {
            self.xl_hm_mode_set(HighPerformanceMode::Enabled)?;
        }

        ctrl1_xl.set_odr_xl(val as u8 & 0xf);
        ctrl1_xl.write(self)?;

        Ok(())
    }

    /// Get the current accelerometer output data rate (ODR) configuration.
    pub fn xl_data_rate_get(&mut self) -> Result<XlDataRate, Error<B::Error>> {
        let ctrl1_xl = Ctrl1Xl::read(self)?;
        let ctrl6_c = Ctrl6C::read(self)?;

        let data_rate_selection = (ctrl6_c.xl_hm_mode() << 4) | ctrl1_xl.odr_xl();

        let val = XlDataRate::try_from(data_rate_selection).unwrap_or_default();

        Ok(val)
    }

    /// Enable or disable gyroscope high-performance mode.
    pub fn gy_hm_mode_set(&mut self, val: HighPerformanceMode) -> Result<(), Error<B::Error>> {
        let mut ctrl7_g = Ctrl7G::read(self)?;
        ctrl7_g.set_g_hm_mode((val as u8) & 0x1);
        ctrl7_g.write(self)?;

        Ok(())
    }
    /// Get the gyroscope high-performance mode.
    pub fn gy_hm_mode_get(&mut self) -> Result<HighPerformanceMode, Error<B::Error>> {
        let ctrl7_g = Ctrl7G::read(self)?;

        let val = HighPerformanceMode::try_from(ctrl7_g.g_hm_mode()).unwrap_or_default();
        Ok(val)
    }
    /// Set gyroscope full-scale.
    pub fn gy_full_scale_set(&mut self, val: GyFullScale) -> Result<(), Error<B::Error>> {
        let mut ctrl2_g = Ctrl2G::read(self)?;
        ctrl2_g.set_fs_g(val as u8 & 0x3);
        ctrl2_g.set_fs_125(val as u8 >> 4);
        ctrl2_g.write(self)?;

        Ok(())
    }

    /// Get actual gyroscope full-scale configuration.
    pub fn gy_full_scale_get(&mut self) -> Result<GyFullScale, Error<B::Error>> {
        let ctrl2_g = Ctrl2G::read(self)?;
        let value = (ctrl2_g.fs_125() << 4) | ctrl2_g.fs_g();

        let val = GyFullScale::try_from(value).unwrap_or_default();

        Ok(val)
    }

    /// Set gyroscope output data rate (ODR).
    pub fn gy_data_rate_set(&mut self, val: GyDataRate) -> Result<(), Error<B::Error>> {
        let mut ctrl2_g = Ctrl2G::read(self)?;

        if ((val as u8) & 0x10) == 0x10 {
            self.gy_hm_mode_set(HighPerformanceMode::Disabled)?;
        } else {
            self.gy_hm_mode_set(HighPerformanceMode::Enabled)?;
        }

        ctrl2_g.set_odr_g(val as u8 & 0xf);
        ctrl2_g.write(self)?;

        Ok(())
    }

    /// Get the current gyroscope output data rate (ODR) configuration.
    pub fn gy_data_rate_get(&mut self) -> Result<GyDataRate, Error<B::Error>> {
        let ctrl2_g = Ctrl2G::read(self)?;
        let ctrl7_g = Ctrl7G::read(self)?;

        let value = (ctrl7_g.g_hm_mode() << 4) | ctrl2_g.odr_g();

        let val = GyDataRate::try_from(value).unwrap_or_default();

        Ok(val)
    }

    /// Enable/Disable the automatical increment of register address during a multiple byte access
    /// with a serial interface (enabled by default).
    pub fn auto_increment_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;
        ctrl3_c.set_if_inc(val);
        ctrl3_c.write(self)?;

        Ok(())
    }
    /// Get the current value of auto incremented
    ///
    /// Register address automatically incremented during a multiple byte access
    /// with a serial interface (enabled by default).
    pub fn auto_increment_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = Ctrl3C::read(self).map(|reg| reg.if_inc())?;

        Ok(val)
    }
    /// Enable/Disable Block Data Update (BDU).
    ///
    /// Output registers are not updated until LSB and MSB have been read).
    pub fn block_data_update_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;
        ctrl3_c.set_bdu(val);
        ctrl3_c.write(self)?;

        Ok(())
    }
    /// Get the actual Block Data Update (BDU) configuration.
    ///
    /// Output registers are not updated until LSB and MSB have been read.
    pub fn block_data_update_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = Ctrl3C::read(self).map(|reg| reg.bdu())?;

        Ok(val)
    }

    /// Enable/Disable gyroscope sleep mode.
    pub fn sleep_set(&mut self, val: Sleep) -> Result<(), Error<B::Error>> {
        let mut ctrl4_c = Ctrl4C::read(self)?;
        ctrl4_c.set_sleep_g((val as u8) & 0x1);
        ctrl4_c.write(self)?;

        Ok(())
    }

    /// Get the actual gyroscope sleep mode.
    pub fn sleep_get(&mut self) -> Result<Sleep, Error<B::Error>> {
        let ctrl4_c = Ctrl4C::read(self)?;
        let val = Sleep::try_from(ctrl4_c.sleep_g()).unwrap_or_default();

        Ok(val)
    }

    /// Set accelerometer self-test mode.
    pub fn xl_self_test_set(&mut self, val: XlSelfTest) -> Result<(), Error<B::Error>> {
        let mut ctrl5_c = Ctrl5C::read(self)?;
        ctrl5_c.set_st_xl((val as u8) & 0x3);
        ctrl5_c.write(self)?;

        Ok(())
    }

    /// Get the actual accelerometer self-test mode.
    pub fn xl_self_test_get(&mut self) -> Result<XlSelfTest, Error<B::Error>> {
        let ctrl5_c = Ctrl5C::read(self)?;

        let val = XlSelfTest::try_from(ctrl5_c.st_xl()).unwrap_or_default();

        Ok(val)
    }

    /// Set the gyroscope self-test mode.
    pub fn gy_self_test_set(&mut self, val: GySelfTest) -> Result<(), Error<B::Error>> {
        let mut ctrl5_c = Ctrl5C::read(self)?;
        ctrl5_c.set_st_g((val as u8) & 0x3);
        ctrl5_c.write(self)?;

        Ok(())
    }

    /// Get the gyroscope self-test mode.
    pub fn gy_self_test_get(&mut self) -> Result<GySelfTest, Error<B::Error>> {
        let ctrl5_c = Ctrl5C::read(self)?;

        let val = GySelfTest::try_from(ctrl5_c.st_g()).unwrap_or_default();

        Ok(val)
    }

    /// Enable/Disable pull-up on SDO pin of UI (User Interface).
    pub fn ui_sdo_pull_up_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut pin_ctrl = PinCtrl::read(self)?;
        pin_ctrl.set_sdo_pu_en(val);
        pin_ctrl.write(self)?;

        Ok(())
    }
    /// Get the sdo_pu bit: pull-up on SDO pin of UI (User Interface).
    pub fn ui_sdo_pull_up_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = PinCtrl::read(self).map(|reg| reg.sdo_pu_en())?;

        Ok(val)
    }

    /// Set the SPI Serial Interface Mode.
    pub fn spi_mode_set(&mut self, val: SpiMode) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;
        ctrl3_c.set_sim(val as u8 & 0x1);
        ctrl3_c.write(self)?;

        Ok(())
    }

    /// Get the actual SPI Serial Interface Mode.
    pub fn spi_mode_get(&mut self) -> Result<SpiMode, Error<B::Error>> {
        let ctrl3_c = Ctrl3C::read(self)?;

        let val = SpiMode::try_from(ctrl3_c.sim()).unwrap_or_default();

        Ok(val)
    }

    /// Enable/Disable I2C on UI (User Interface).
    pub fn ui_i2c_mode_set(&mut self, val: UiI2cMode) -> Result<(), Error<B::Error>> {
        let mut ctrl4_c = Ctrl4C::read(self)?;
        ctrl4_c.set_i2c_disable((val as u8) & 0x1);
        ctrl4_c.write(self)?;

        Ok(())
    }

    /// Return the state (enable/disable) of I2C on UI (User Interface).
    pub fn ui_i2c_mode_get(&mut self) -> Result<UiI2cMode, Error<B::Error>> {
        let ctrl4_c = Ctrl4C::read(self)?;
        let val = UiI2cMode::try_from(ctrl4_c.i2c_disable()).unwrap_or_default();

        Ok(val)
    }

    /// Enable/Disable the timestamp counter.
    pub fn timestamp_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut ctrl10_c = Ctrl10C::read(self)?;
        ctrl10_c.set_timestamp_en(val);
        ctrl10_c.write(self)?;

        Ok(())
    }

    /// Get the state (enable/disable) of timestamp counter.
    pub fn timestamp_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = Ctrl10C::read(self).map(|reg| reg.timestamp_en())?;

        Ok(val)
    }

    /// Get the Timestamp data output.
    pub fn timestamp_raw_get(&mut self) -> Result<u32, Error<B::Error>> {
        Timestamp::read(self).map(|reg| reg.0)
    }

    /// Get the status of all the interrupt sources.
    pub fn all_sources_get(&mut self) -> Result<AllSources, Error<B::Error>> {
        let status_reg = StatusReg::read(self)?;
        let status_sh = StatusMasterMainpage::read(self)?;
        let ispu = IspuIntStatusMainPage::read(self).map(|reg| reg.0)?;

        let val = AllSources {
            drdy_xl: status_reg.xlda(),
            drdy_gy: status_reg.gda(),
            drdy_temp: status_reg.tda(),
            sh_endop: status_sh.sens_hub_endop(),
            sh_slave0_nack: status_sh.sens_hub_endop(),
            sh_slave1_nack: status_sh.sens_hub_endop(),
            sh_slave2_nack: status_sh.sens_hub_endop(),
            sh_slave3_nack: status_sh.sens_hub_endop(),
            sh_wr_once: status_sh.sens_hub_endop(),
            ispu,
        };

        Ok(val)
    }

    /// Retrive STATUS_REG register.
    ///
    /// Contains information about data-ready for: accelerometer, gyroscope, temperature.
    pub fn status_reg_get(&mut self) -> Result<StatusReg, Error<B::Error>> {
        StatusReg::read(self)
    }

    /// Returns 1 if new accelerometer data is available, otherwise 0.
    pub fn xl_flag_data_ready_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = StatusReg::read(self).map(|reg| reg.xlda())?;

        Ok(val)
    }
    /// Returns 1 if new gyroscope data is available, otherwise 0.
    pub fn gy_flag_data_ready_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = StatusReg::read(self).map(|reg| reg.gda())?;

        Ok(val)
    }

    /// Returns 1 if new temperature data is available, otherwise 0.
    pub fn temp_flag_data_ready_get(&mut self) -> Result<u8, Error<B::Error>> {
        let val: u8 = StatusReg::read(self).map(|reg| reg.tda())?;

        Ok(val)
    }
    /// Get the Temperature data.
    pub fn temperature_raw_get(&mut self) -> Result<i16, Error<B::Error>> {
        OutTemp::read(self).map(|reg| reg.0)
    }

    /// Retrive the Angular rate readings.
    pub fn angular_rate_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let val = OutXYZG::read(self)?;

        Ok([val.x, val.y, val.z])
    }

    /// Retrive the Linear acceleration readings.
    pub fn acceleration_raw_get(&mut self) -> Result<[i16; 3], Error<B::Error>> {
        let val = OutXYZA::read(self)?;

        Ok([val.x, val.y, val.z])
    }

    /// It routes interrupt signals on INT 1 pin.
    pub fn pin_int1_route_set(&mut self, val: PinInt1Route) -> Result<(), Error<B::Error>> {
        let mut int1_ctrl = Int1Ctrl::read(self)?;
        let mut md1_cfg = Md1Cfg::read(self)?;

        int1_ctrl.set_int1_drdy_xl(val.drdy_xl);
        int1_ctrl.set_int1_drdy_g(val.drdy_gy);
        int1_ctrl.set_int1_boot(val.boot);
        int1_ctrl.write(self)?;

        md1_cfg.set_int1_shub(val.sh_endop);
        md1_cfg.set_int1_ispu(val.ispu);
        md1_cfg.write(self)?;

        Ok(())
    }

    /// Get the interrupt signals routed on INT 1 pin.
    pub fn pin_int1_route_get(&mut self) -> Result<PinInt1Route, Error<B::Error>> {
        let int1_ctrl = Int1Ctrl::read(self)?;
        let md1_cfg = Md1Cfg::read(self)?;

        let val = PinInt1Route {
            drdy_xl: int1_ctrl.int1_drdy_xl(),
            drdy_gy: int1_ctrl.int1_drdy_g(),
            boot: int1_ctrl.int1_boot(),
            sh_endop: md1_cfg.int1_shub(),
            ispu: md1_cfg.int1_ispu(),
        };

        Ok(val)
    }

    /// It routes interrupt signals on INT 2 pin.
    pub fn pin_int2_route_set(&mut self, val: PinInt2Route) -> Result<(), Error<B::Error>> {
        let mut int2_ctrl = Int2Ctrl::read(self)?;
        let mut md2_cfg = Md2Cfg::read(self)?;

        int2_ctrl.set_int2_drdy_xl(val.drdy_xl);
        int2_ctrl.set_int2_drdy_g(val.drdy_gy);
        int2_ctrl.set_int2_drdy_temp(val.drdy_temp);
        int2_ctrl.set_int2_sleep_ispu(val.ispu_sleep);
        int2_ctrl.write(self)?;

        md2_cfg.set_int2_ispu(val.ispu);
        md2_cfg.set_int2_timestamp(val.timestamp);
        md2_cfg.write(self)?;

        Ok(())
    }

    /// Get the interrupt signals routed on INT 2 pin.
    pub fn pin_int2_route_get(&mut self) -> Result<PinInt2Route, Error<B::Error>> {
        let int2_ctrl = Int2Ctrl::read(self)?;
        let md2_cfg = Md2Cfg::read(self)?;

        let val = PinInt2Route {
            drdy_xl: int2_ctrl.int2_drdy_xl(),
            drdy_gy: int2_ctrl.int2_drdy_g(),
            drdy_temp: int2_ctrl.int2_drdy_temp(),
            ispu_sleep: int2_ctrl.int2_sleep_ispu(),
            ispu: md2_cfg.int2_ispu(),
            timestamp: md2_cfg.int2_timestamp(),
        };

        Ok(val)
    }

    /// Set Push-pull/open-drain on INT1 and INT2 pins.
    pub fn int_pin_mode_set(&mut self, val: IntPinMode) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;
        ctrl3_c.set_pp_od((val as u8) & 0x1);
        ctrl3_c.write(self)?;

        Ok(())
    }
    /// Get the configuration (Push-pull/open-drain) for INT1 and INT2 pins.
    pub fn int_pin_mode_get(&mut self) -> Result<IntPinMode, Error<B::Error>> {
        let ctrl3_c = Ctrl3C::read(self)?;
        let val = IntPinMode::try_from(ctrl3_c.pp_od()).unwrap_or_default();

        Ok(val)
    }

    /// Set the interrupt activation mode (high/low).
    pub fn pin_polarity_set(&mut self, val: PinPolarity) -> Result<(), Error<B::Error>> {
        let mut ctrl3_c = Ctrl3C::read(self)?;
        ctrl3_c.set_h_lactive(val as u8 & 0x1);
        ctrl3_c.write(self)?;

        Ok(())
    }
    /// Get the actual Interrupt activation level.
    pub fn pin_polarity_get(&mut self) -> Result<PinPolarity, Error<B::Error>> {
        let ctrl3_c = Ctrl3C::read(self)?;
        let val = PinPolarity::try_from(ctrl3_c.h_lactive()).unwrap_or_default();

        Ok(val)
    }

    /// Retrive the Sensor hub output data.
    pub fn sh_read_data_raw_get(&mut self, val: &mut [u8]) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| SensorHub1::read_more(lock, val))
    }
    /// Set the number of external sensors to be read by the sensor hub.
    pub fn sh_slave_connected_set(&mut self, val: ShSlaveConnected) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_aux_sens_on((val as u8) & 0x3);
            master_config.write(lock)
        })
    }

    /// Get the actual number of external sensors configured to be read by the sensor hub.
    pub fn sh_slave_connected_get(&mut self) -> Result<ShSlaveConnected, Error<B::Error>> {
        let master_config =
            MemBank::operate_over_sensor_hub(self, |lock| MasterConfig::read(lock))?;

        let aux_sens_on = master_config.aux_sens_on();
        let val = ShSlaveConnected::try_from(aux_sens_on).unwrap_or_default();

        Ok(val)
    }

    /// Enable/disable Sensor hub I2C master.
    pub fn sh_master_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_master_on(val);
            master_config.write(lock)
        })
    }

    /// Get the value (enable/disable) of Sensor hub I2C master.
    pub fn sh_master_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let master_config = MasterConfig::read(lock)?;
            Ok(master_config.master_on())
        })
    }

    /// Enable/Disable Sensor Hub master I2C pull-up.
    pub fn sh_master_interface_pull_up_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_shub_pu_en(val);
            master_config.write(lock)
        })
    }

    /// Get the current value (enable/disable) for Sensor Hub master I2C pull-up.
    pub fn sh_master_interface_pull_up_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            MasterConfig::read(lock).map(|reg| reg.shub_pu_en())
        })
    }

    /// Enable/Disable I2C interface pass-through for Sensor Hub.
    pub fn sh_pass_through_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_pass_through_mode(val);
            master_config.write(lock)
        })
    }

    /// Get the configuration (enable/disable) I2C interface pass-through for Sensor Hub.
    pub fn sh_pass_through_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            MasterConfig::read(lock).map(|reg| reg.pass_through_mode())
        })
    }

    /// Set the Sensor hub trigger signal (acc and gyro/int2).
    pub fn sh_syncro_mode_set(&mut self, val: ShSyncroMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_start_config(val as u8 & 0x01);
            master_config.write(lock)
        })
    }

    /// Get the current  Sensor hub trigger signal (acc and gyro/int2).
    pub fn sh_syncro_mode_get(&mut self) -> Result<ShSyncroMode, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let start_config = MasterConfig::read(lock)?.start_config();
            let val = ShSyncroMode::try_from(start_config).unwrap_or_default();
            Ok(val)
        })
    }

    /// Set the Slave 0 write mode (only first cycle/each sh cycle)
    pub fn sh_write_mode_set(&mut self, val: ShWriteMode) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_write_once((val as u8) & 0x01);
            master_config.write(lock)
        })
    }

    /// Get the actual Slave 0 write mode (only first cycle/each sh cycle)
    pub fn sh_write_mode_get(&mut self) -> Result<ShWriteMode, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let master_config = MasterConfig::read(lock)?;
            let val = ShWriteMode::try_from(master_config.write_once()).unwrap_or_default();
            Ok(val)
        })
    }

    /// Set Reset Master logic and output registers.
    ///
    /// Must be set to `1` and then set it to `0`.
    pub fn sh_reset_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut master_config = MasterConfig::read(lock)?;
            master_config.set_rst_master_regs(val);
            master_config.write(lock)
        })
    }

    /// Get the actual Reset configuration of Master logic.
    pub fn sh_reset_get(&mut self) -> Result<u8, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            MasterConfig::read(lock).map(|reg| reg.rst_master_regs())
        })
    }

    /// Configure target 0 for perform a write.
    ///
    /// # Arguments
    ///
    /// * `val`: A structure that contains:
    ///     - `tgt0_add`: 8-bit I2C device address
    ///     - `tgt0_subadd`: 8-bit register device address
    ///     - `tgt0_data`: 8-bit data to write
    pub fn sh_cfg_write(&mut self, val: ShCfgWrite) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut reg = Slv0Add::from_bits(0);
            reg.set_slave0_add(val.slv0_add);
            reg.set_rw_0(0);
            reg.write(lock)?;

            Slv0Subadd::from_bits(val.slv0_subadd).write(lock)?;
            DatawriteSlv0::from_bits(val.slv0_data).write(lock)
        })
    }

    /// Set the rate at which the master communicates.
    pub fn sh_data_rate_set(&mut self, val: ShDataRate) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let mut slv0_config = Slv0Config::read(lock)?;
            slv0_config.set_shub_odr((val as u8) & 0x07);
            slv0_config.write(lock)
        })
    }

    /// Get the actual rate configuration at which the master communicates.
    pub fn sh_data_rate_get(&mut self) -> Result<ShDataRate, Error<B::Error>> {
        MemBank::operate_over_sensor_hub(self, |lock| {
            let slv0_config = Slv0Config::read(lock)?;
            let shub_odr = slv0_config.shub_odr();

            let val = ShDataRate::try_from(shub_odr).unwrap_or_default();
            Ok(val)
        })
    }

    /// Perform a read from the slave indexed by idx
    ///
    /// # Arguments
    ///
    /// * `idx`: The index of the slave.
    /// * `val`: A structure containing:
    ///     - `slv_add`: 8-bit I2C device address
    ///     - `slv_subadd`: 8-bit register device address
    ///     - `slv_len`: Number of bits to read
    pub fn sh_slv_cfg_read(&mut self, idx: u8, val: &ShCfgRead) -> Result<(), Error<B::Error>> {
        let mut slv_add = Slv0Add::from_bits(0);

        self.mem_bank_set(MemBank::SensorHubMemBank)?;

        slv_add.set_slave0_add(val.slv_add);
        slv_add.set_rw_0(1);
        let slv_add_reg = match idx {
            0 => SensHubReg::Slv0Add,
            1 => SensHubReg::Slv1Add,
            2 => SensHubReg::Slv2Add,
            3 => SensHubReg::Slv3Add,
            _ => SensHubReg::Slv0Add,
        };
        self.write_to_register(slv_add_reg as u8, &[slv_add.into()])?;
        let slv_sub_add_reg = match idx {
            0 => SensHubReg::Slv0Subadd,
            1 => SensHubReg::Slv1Subadd,
            2 => SensHubReg::Slv2Subadd,
            3 => SensHubReg::Slv3Subadd,
            _ => SensHubReg::Slv0Subadd,
        };
        self.write_to_register(slv_sub_add_reg as u8, &[val.slv_subadd])?;

        let slv_config_reg = match idx {
            0 => SensHubReg::Slv0Config,
            1 => SensHubReg::Slv1Config,
            2 => SensHubReg::Slv2Config,
            3 => SensHubReg::Slv3Config,
            _ => SensHubReg::Slv0Config,
        };
        let mut config_buf = [0];
        self.read_from_register(slv_config_reg as u8, &mut config_buf)?;
        let mut slv_config = Slv0Config::from_bits(config_buf[0]);
        slv_config.set_slave0_numop(val.slv_len);
        self.write_to_register(slv_config_reg as u8, &[slv_config.into()])?;

        self.mem_bank_set(MemBank::MainMemBank)?;

        Ok(())
    }

    /// Retrive the SatutsMaster: contains nack for slaves, sens_hub_endop, wr_once_done.
    pub fn sh_status_get(&mut self) -> Result<StatusMaster, Error<B::Error>> {
        let value = StatusMasterMainpage::read(self)?;
        Ok(StatusMaster::from_bits(value.into()))
    }

    /// Enable/Disable the software reset of ISPU core.
    pub fn ispu_reset_set(&mut self, val: u8) -> Result<(), Error<B::Error>> {
        let mut func_cfg_access = FuncCfgAccess::read(self)?;
        func_cfg_access.set_sw_reset_ispu(val);
        func_cfg_access.write(self)
    }

    /// Get the actual Software reset configuration of ISPU core.
    pub fn ispu_reset_get(&mut self) -> Result<u8, Error<B::Error>> {
        FuncCfgAccess::read(self).map(|reg| reg.sw_reset_ispu())
    }

    /// Set the ISPU clock.
    pub fn ispu_clock_set(&mut self, val: IspuClockSel) -> Result<(), Error<B::Error>> {
        let mut ctrl10_c = Ctrl10C::read(self)?;
        ctrl10_c.set_ispu_clk_sel(val as u8);
        ctrl10_c.write(self)?;

        Ok(())
    }

    /// Get the actual ISPU clock.
    pub fn ispu_clock_get(&mut self) -> Result<IspuClockSel, Error<B::Error>> {
        let ctrl10_c = Ctrl10C::read(self)?;
        let val = IspuClockSel::try_from(ctrl10_c.ispu_clk_sel()).unwrap_or_default();

        Ok(val)
    }

    /// Set the ISPU output data rate (ODR).
    pub fn ispu_data_rate_set(&mut self, val: IspuDataRate) -> Result<(), Error<B::Error>> {
        let mut ctrl9_c = Ctrl9C::read(self)?;
        ctrl9_c.set_ispu_rate((val as u8) & 0x0F);
        ctrl9_c.write(self)?;

        Ok(())
    }

    /// Get the actual ISPU output data rate (ODR).
    pub fn ispu_data_rate_get(&mut self) -> Result<IspuDataRate, Error<B::Error>> {
        let ispu_rate = Ctrl9C::read(self).map(|reg| reg.ispu_rate())?;

        let val = IspuDataRate::try_from(ispu_rate).unwrap_or_default();
        Ok(val)
    }

    /// Configure the ISPU BDU mode.
    pub fn ispu_bdu_set(&mut self, val: IspuBdu) -> Result<(), Error<B::Error>> {
        let mut ctrl9_c = Ctrl9C::read(self)?;
        ctrl9_c.set_ispu_bdu((val as u8) & 0x3);
        ctrl9_c.write(self)?;

        Ok(())
    }

    /// Get the actual ISPU BDU mode.
    pub fn ispu_bdu_get(&mut self) -> Result<IspuBdu, Error<B::Error>> {
        let ispu_rate = Ctrl9C::read(self).map(|reg| reg.ispu_rate())?;

        Ok(IspuBdu::try_from(ispu_rate).unwrap_or_default())
    }

    /// Retrive IspuIntStatusMainPage: Generic Interrupt Flags from ISPU.
    pub fn ia_ispu_get(&mut self) -> Result<u32, Error<B::Error>> {
        IspuIntStatusMainPage::read(self).map(|reg| reg.0)
    }

    /// General purpose input write configuration register for ISPU.
    ///
    /// # Arguments
    ///
    /// * `offset`: Offset from ISPU_DUMMY_CFG_1 register.
    /// * `val`: General purpose input configuration register for ISPU.
    /// * `len`: Number of bytes to write.
    pub fn ispu_write_dummy_cfg(
        &mut self,
        offset: u8,
        val: &[u8],
        len: u8,
    ) -> Result<(), Error<B::Error>> {
        if Reg::IspuDummyCfg1L as u8 + offset + len > Reg::IspuDummyCfg4H as u8 {
            return Err(Error::UnexpectedValue);
        }

        self.write_to_register(Reg::IspuDummyCfg1L as u8 + offset, &val[..len.into()])?;

        Ok(())
    }

    /// General purpose read input configuration register for ISPU
    ///
    /// # Arguments
    ///
    /// * `offset`: Offset from ISPU_DUMMY_CFG_1 register.
    /// * `val`: General purpose input configuration register for ISPU.
    /// * `len`: Number of bytes to write.
    pub fn ispu_read_dummy_cfg(
        &mut self,
        offset: u8,
        val: &mut [u8],
        len: u8,
    ) -> Result<(), Error<B::Error>> {
        if Reg::IspuDummyCfg1L as u8 + offset + len > Reg::IspuDummyCfg4H as u8 {
            return Err(Error::UnexpectedValue);
        }

        self.read_from_register(
            Reg::IspuDummyCfg1L as u8 + offset,
            &mut val[0..len as usize],
        )?;

        Ok(())
    }

    /// Turn on/off the Boot ISPU core
    pub fn ispu_boot_set(&mut self, val: IspuBootLatched) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| {
            let mut ispu_config = IspuConfig::read(lock)?;
            ispu_config.set_ispu_rst_n(val as u8);
            ispu_config.set_clk_dis(val as u8);
            ispu_config.write(lock)
        })?;

        Ok(())
    }

    /// Get the actual Boot ISPU core configuration (on/off).
    pub fn ispu_boot_get(&mut self) -> Result<IspuBootLatched, Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| {
            let ispu_config = IspuConfig::read(lock)?;
            let mut val = IspuBootLatched::Off;

            if ispu_config.ispu_rst_n() == 1 || ispu_config.clk_dis() == 1 {
                val = IspuBootLatched::On;
            }

            Ok(val)
        })
    }

    /// Enable/Disable latched ISPU interrupt.
    pub fn ispu_int_latched_set(&mut self, val: IspuInterrupt) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| {
            let mut ispu_config = IspuConfig::read(lock)?;
            ispu_config.set_latched((val as u8) & 0x1);
            ispu_config.write(lock)
        })
    }

    /// Get the latched ISPU interrupt configuration (enable/disable).
    pub fn ispu_int_latched_get(&mut self) -> Result<IspuInterrupt, Error<B::Error>> {
        let ispu_config = MemBank::operate_over_ispu(self, |lock| IspuConfig::read(lock))?;

        let val = ispu_config.latched();
        Ok(IspuInterrupt::try_from(val).unwrap_or_default())
    }

    /// Returns ISPU boot status.
    pub fn ispu_get_boot_status(&mut self) -> Result<IspuBootStatus, Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| {
            let ispu_status = IspuStatus::read(lock)?;
            let ispu_boot_status =
                IspuBootStatus::try_from(ispu_status.boot_end()).unwrap_or_default();
            Ok(ispu_boot_status)
        })
    }

    /// ISPU write memory.
    ///
    /// ISPU clock is disabled inside the routine.
    ///
    /// # Arguments
    ///
    /// * `mem_sel`: IspuMemoryType
    /// * `mem_addr`: Memory address
    /// * `mem_data`: Memory data
    /// * `len`: Data length
    pub fn ispu_write_memory(
        &mut self,
        mem_sel: IspuMemoryType,
        mem_addr: u16,
        mem_data: &[u8],
        len: u16,
    ) -> Result<(), Error<B::Error>> {
        // Set memory bank to ISPU
        MemBank::operate_over_ispu(self, |lock| {
            let mut ispu_mem_sel = IspuMemSel::from_bits(0);

            // Disable ISPU clock
            let mut ispu_cfg = IspuConfig::read(lock)?;
            let clk_dis = ispu_cfg.clk_dis();
            ispu_cfg.set_clk_dis(1);
            ispu_cfg.write(lock)?;

            // Select memory to be written
            ispu_mem_sel.set_read_mem_en(0);
            ispu_mem_sel.set_mem_sel(mem_sel as u8);
            ispu_mem_sel.write(lock)?;

            if mem_sel == IspuMemoryType::ProgramRamMemory {
                let mut addr_s = [0u16; 4];
                let mut len_s = [0u16; 4];
                let mut j = 0;
                let mut k = 0;

                addr_s[0] = mem_addr;
                for i in 0..len {
                    if (mem_addr + i == 0x2000)
                        || (mem_addr + i == 0x4000)
                        || (mem_addr + i == 0x6000)
                    {
                        len_s[j] = k;
                        j += 1;
                        addr_s[j] = mem_addr + i;
                        k = 0;
                    }
                    k += 1;
                }
                len_s[j] = k;
                j += 1;

                k = 0;
                for i in 0..j {
                    lock.ispu_sel_memory_addr(addr_s[i])?;
                    lock.write_to_register(
                        IspuReg::IspuMemData as u8,
                        &mem_data[k as usize..(k + len_s[i]) as usize],
                    )?;
                    k += len_s[i];
                }
            } else {
                lock.ispu_sel_memory_addr(mem_addr)?;
                lock.write_to_register(IspuReg::IspuMemData as u8, &mem_data[0..len as usize])?;
            }

            // Restore ISPU clock
            ispu_cfg.set_clk_dis(clk_dis);
            ispu_cfg.write(lock)?;

            // Set memory bank back to main
            Ok(())
        })
    }

    /// ISPU read memory.
    ///
    /// ISPU clock is disabled inside the routine.
    ///
    /// # Arguments
    ///
    /// * `mem_sel`: IspuMemoryType
    /// * `mem_addr`: Memory address.
    /// * `mem_data`: Memory data.
    /// * `len`: Data length.
    pub fn ispu_read_memory(
        &mut self,
        mem_sel: IspuMemoryType,
        mem_addr: u16,
        mem_data: &mut [u8],
        len: u16,
    ) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| {
            let mut ispu_mem_sel = IspuMemSel::from_bits(0);

            // Disable ISPU clock
            let mut ispu_cfg = IspuConfig::read(lock)?;
            let clk_dis = ispu_cfg.clk_dis();
            ispu_cfg.set_clk_dis(1);
            ispu_cfg.write(lock)?;

            // Select memory to be read
            ispu_mem_sel.set_read_mem_en(1);
            ispu_mem_sel.set_mem_sel(mem_sel as u8);
            ispu_mem_sel.write(lock)?;

            // Select memory address
            lock.ispu_sel_memory_addr(mem_addr)?;

            // Read data
            let _dummy = IspuMemData::read(lock);
            IspuMemData::read_more(lock, &mut mem_data[0..len.into()])?;

            // Set ISPU clock back to previous value
            ispu_cfg.set_clk_dis(clk_dis);
            ispu_cfg.write(lock)
        })
    }

    /// ISPU write flags (IF2S)
    pub fn ispu_write_flags(&mut self, data: u16) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuIf2sFlag(data).write(lock))
    }

    /// ISPU read flags (S2IF)
    pub fn ispu_read_flags(&mut self) -> Result<u16, Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuS2ifFlag::read(lock).map(|reg| reg.0))
    }

    /// ISPU clear flags (S2IF)
    pub fn ispu_clear_flags(&mut self) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuS2ifFlagH::from_bits(1).write(lock))
    }

    /// Retrive ISPU DOUT registers data.
    ///
    /// The output is provided changing the input array (arr).
    pub fn ispu_read_data_raw_get(
        &mut self,
        arr: &mut [u8],
        len: usize,
    ) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuDout00L::read_more(lock, &mut arr[0..len]))
    }

    /// Get the ISPU int1_ctrl configuration.
    ///
    /// Each bit is a flag to route interrupt on INT1. INT1_ISPU must be also set to 1.
    pub fn ispu_int1_ctrl_get(&mut self) -> Result<u32, Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuInt1Ctrl::read(lock).map(|reg| reg.0))
    }

    /// Set the ISPU int1_ctrl configuration.
    ///
    /// Each bit is a flag to route interrupt on INT1. INT1_ISPU must be also set to 1.
    pub fn ispu_int1_ctrl_set(&mut self, val: u32) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuInt1Ctrl(val).write(lock))
    }

    /// Get the ISPU int2_ctrl configuration.
    ///
    /// Each bit is a flag to route interrupt on INT2. INT2_ISPU must be also set to 1.
    pub fn ispu_int2_ctrl_get(&mut self) -> Result<u32, Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuInt2Ctrl::read(lock).map(|reg| reg.0))
    }

    /// Set the ISPU int2_ctrl configuration.
    ///
    /// Each bit is a flag to route interrupt on INT2. INT2_ISPU must be also set to 1.
    pub fn ispu_int2_ctrl_set(&mut self, val: u32) -> Result<(), Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuInt2Ctrl(val).write(lock))
    }

    /// Retrive ISPU int_status.
    ///
    /// Get the actual 30 bit interrupt configuration.
    pub fn ispu_int_status_get(&mut self) -> Result<u32, Error<B::Error>> {
        // todo: this operation could use IspuIntStatus1Mainpage?
        MemBank::operate_over_ispu(self, |lock| IspuIntStatus::read(lock).map(|reg| reg.0))
    }

    /// Retrive ISPU algo.
    ///
    /// Enable configurations in order to run up to 30 independent algorithms.
    pub fn ispu_algo_get(&mut self) -> Result<u32, Error<B::Error>> {
        MemBank::operate_over_ispu(self, |lock| IspuAlgo::read(lock).map(|reg| reg.0))
    }

    /// Set ISPU algo: each bit enables the corresponding algorithm.
    ///
    /// Enable configurations in order to run up to 30 independent algorithms.
    pub fn ispu_algo_set(&mut self, val: u32) -> Result<(), Error<B::Error>> {
        let algo = IspuAlgo(val);

        MemBank::operate_over_ispu(self, |lock| algo.write(lock))
    }
}

pub fn from_fs2g_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.061
}

pub fn from_fs4g_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.122
}

pub fn from_fs8g_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.244
}

pub fn from_fs16g_to_mg(lsb: i16) -> f32 {
    (lsb as f32) * 0.488
}

pub fn from_fs125dps_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 4.375
}

pub fn from_fs250dps_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 8.75
}

pub fn from_fs500dps_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 17.50
}

pub fn from_fs1000dps_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 35.0
}

pub fn from_fs2000dps_to_mdps(lsb: i16) -> f32 {
    (lsb as f32) * 70.0
}

pub fn from_lsb_to_celsius(lsb: i16) -> f32 {
    (lsb as f32 / 256.0) + 25.0
}

#[cfg(feature = "passthrough")]
pub struct Ism330isMaster<B, T> {
    pub sensor: RefCell<Ism330is<B, T>>,
}

#[cfg(feature = "passthrough")]
impl<B: BusOperation, T: DelayNs> Ism330isMaster<B, T> {
    pub fn from_bus(bus: B, tim: T) -> Self {
        Self {
            sensor: RefCell::new(Ism330is::from_bus(bus, tim)),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<Ism330is<B, T>> {
        self.sensor.borrow_mut()
    }

    /// Generates a wrapper for the sensor to enable its use as a passthrough
    /// from another sensor.
    ///
    /// The Sensor Hub may require this setup to redirect writes from the
    /// bus to the sensor that executes them as a passthrough.
    pub fn as_passthrough<'a>(&'a self, address: SevenBitAddress) -> Ism330isPassthrough<'a, B, T> {
        Ism330isPassthrough {
            sensor: &self.sensor,
            slave_address: address,
        }
    }
}

#[cfg(feature = "passthrough")]
pub struct Ism330isPassthrough<'a, B, T> {
    sensor: &'a RefCell<Ism330is<B, T>>,
    slave_address: SevenBitAddress,
}

#[cfg(feature = "passthrough")]
// Ism330is acts like a bus when become the master of the sensor hub.
impl<B, T> BusOperation for Ism330isPassthrough<'_, B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    type Error = Error<B::Error>;

    fn read_bytes(&mut self, _rbuf: &mut [u8]) -> Result<(), Self::Error> {
        Err(Error::UnexpectedValue)
    }

    fn write_bytes(&mut self, wbuf: &[u8]) -> Result<(), Self::Error> {
        let mut master = self.sensor.borrow_mut();
        let mut sh_cfg_write = ShCfgWrite::default();

        for i in 1_u8..(wbuf.len() as u8) {
            // Configure Sensor Hub to read data
            sh_cfg_write.slv0_add = self.slave_address;
            sh_cfg_write.slv0_subadd = wbuf[0] + i - 1;
            sh_cfg_write.slv0_data = wbuf[i as usize];
            master.sh_cfg_write(sh_cfg_write)?;

            // Disable accelerometer
            master.xl_data_rate_set(XlDataRate::Off)?;
            // Enable I2C Master
            master.sh_master_set(1)?;
            // Enable accelerometer to trigger Sensor Hub operation.
            master.xl_data_rate_set(XlDataRate::_26hzHp)?;
            // Wait Sensor Hub operation flag set.
            let _dummy = master.acceleration_raw_get();

            let mut drdy = 0;
            while drdy == 0 {
                master.tim.delay_ms(20);
                drdy = master.xl_flag_data_ready_get()?;
            }

            let mut end_op = 0;
            while end_op == 0 {
                master.tim.delay_ms(20);
                end_op = master.sh_status_get()?.sens_hub_endop();
            }

            // Disable I2C master and XL (triger).
            master.sh_master_set(0)?;
            master.xl_data_rate_set(XlDataRate::Off)?;
        }

        Ok(())
    }

    fn write_byte_read_bytes(
        &mut self,
        wbuf: &[u8; 1],
        rbuf: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut master = self.sensor.borrow_mut();
        // Disable accelerometer
        master.xl_data_rate_set(XlDataRate::Off)?;
        // Configure Sensor Hub to read
        let sh_cfg_read = ShCfgRead {
            slv_add: self.slave_address,
            slv_subadd: wbuf[0],
            slv_len: rbuf.len() as u8,
        };
        master.sh_slv_cfg_read(0, &sh_cfg_read)?; // dummy read
        master.sh_slave_connected_set(ShSlaveConnected::_01)?;
        // Enable I2C Master
        master.sh_master_set(1)?;
        // Enable accelerometer to trigger Sensor Hub operation.
        master.xl_data_rate_set(XlDataRate::_26hzHp)?;
        // Wait Sensor Hub operation flag set
        let _dummy = master.acceleration_raw_get()?;

        let mut drdy = 0;
        while drdy == 0 {
            master.tim.delay_ms(20);
            drdy = master.xl_flag_data_ready_get()?;
        }

        let mut end_op = 0;
        while end_op == 0 {
            //master.tim.delay_ms(20);
            end_op = master.sh_status_get()?.sens_hub_endop();
        }

        // Disable I2C master and XL(trigger)
        master.sh_master_set(0)?;
        master.xl_data_rate_set(XlDataRate::Off)?;

        // Read SensorHub registers
        master.sh_read_data_raw_get(rbuf)?;

        Ok(())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum I2CAddress {
    I2cAddL = 0x6A,
    I2cAddH = 0x6B,
}

#[derive(Clone, Copy, PartialEq)]
pub struct AllSources {
    pub drdy_xl: u8,
    pub drdy_gy: u8,
    pub drdy_temp: u8,
    pub sh_endop: u8,
    pub sh_slave0_nack: u8,
    pub sh_slave1_nack: u8,
    pub sh_slave2_nack: u8,
    pub sh_slave3_nack: u8,
    pub sh_wr_once: u8,
    pub ispu: u32,
}

pub const ID: u8 = 0x22;
