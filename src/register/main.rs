use crate::Error;
use crate::Ism330is;
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::{named_register, register};
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Reg {
    FuncCfgAccess = 0x1,
    PinCtrl = 0x2,
    DrdyPulsedReg = 0x0B,
    Int1Ctrl = 0x0D,
    Int2Ctrl = 0x0E,
    WhoAmI = 0x0F,
    Ctrl1Xl = 0x10,
    Ctrl2G = 0x11,
    Ctrl3C = 0x12,
    Ctrl4C = 0x13,
    Ctrl5C = 0x14,
    Ctrl6C = 0x15,
    Ctrl7G = 0x16,
    Ctrl9C = 0x18,
    Ctrl10C = 0x19,
    IspuIntStatus0Mainpage = 0x1A,
    IspuIntStatus1Mainpage = 0x1B,
    IspuIntStatus2Mainpage = 0x1C,
    IspuIntStatus3Mainpage = 0x1D,
    StatusReg = 0x1E,
    OutTempL = 0x20,
    OutTempH = 0x21,
    OutxLG = 0x22,
    OutxHG = 0x23,
    OutyLG = 0x24,
    OutyHG = 0x25,
    OutzLG = 0x26,
    OutzHG = 0x27,
    OutxLA = 0x28,
    OutxHA = 0x29,
    OutyLA = 0x2A,
    OutyHA = 0x2B,
    OutzLA = 0x2C,
    OutzHA = 0x2D,
    StatusMasterMainpage = 0x39,
    Timestamp0 = 0x40,
    Timestamp1 = 0x41,
    Timestamp2 = 0x42,
    Timestamp3 = 0x43,
    Md1Cfg = 0x5E,
    Md2Cfg = 0x5F,
    InternalFreqFine = 0x63,
    IspuDummyCfg1L = 0x73,
    IspuDummyCfg1H = 0x74,
    IspuDummyCfg2L = 0x75,
    IspuDummyCfg2H = 0x76,
    IspuDummyCfg3L = 0x77,
    IspuDummyCfg3H = 0x78,
    IspuDummyCfg4L = 0x79,
    IspuDummyCfg4H = 0x7A,
}

/// FUNC_CFG_ACCESS (0x01)
///
/// Enable ISPU / sensor hub functions register (R/W)
#[register(address = Reg::FuncCfgAccess, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FuncCfgAccess {
    #[bits(1, access = RO)]
    pub not_used0: u8,
    /// Software reset of ISPU core. Set to 1 to activate reset sequence; must be written back to 0 manually.
    /// Default value: 0
    #[bits(1)]
    pub sw_reset_ispu: u8,
    #[bits(4, access = RO)]
    pub not_used1: u8,
    /// Enables access to the sensor hub (I²C master) registers.
    /// Default value: 0
    #[bits(1)]
    pub shub_reg_access: u8,
    /// Enables access to the ISPU interaction registers.
    /// Default value: 0
    #[bits(1)]
    pub ispu_reg_access: u8,
}

/// PIN_CTRL (0x02)
///
/// SDO pin pull-up register (R/W)
#[register(address = Reg::PinCtrl, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PinCtrl {
    #[bits(6, access = RO)]
    pub not_used0: u8,
    /// Enables pull-up on SDO pin.
    /// 0: SDO pin pull-up disconnected (default)
    /// 1: SDO pin with pull-up enabled
    #[bits(1)]
    pub sdo_pu_en: u8,
    #[bits(1, access = RO)]
    pub not_used1: u8,
}

/// DRDY_PULSED_REG (0x0B)
///
/// Pulsed data-ready mode register (R/W)
#[register(address = Reg::DrdyPulsedReg, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct DrdyPulsedReg {
    #[bits(7, access = RO)]
    pub not_used0: u8,
    /// Enables pulsed data-ready mode.
    /// 0: Data-ready latched mode (default)
    /// 1: Data-ready pulsed mode (75 μs pulses)
    #[bits(1)]
    pub drdy_pulsed: u8,
}

/// INT1_CTRL (0x0D)
///
/// INT1 pin control register (R/W)
/// Output on INT1 pin is OR combination of signals selected here and in MD1_CFG (0x5E).
#[register(address = Reg::Int1Ctrl, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Int1Ctrl {
    /// Enables accelerometer data-ready interrupt on INT1 pin.
    /// Default: 0 (disabled)
    #[bits(1)]
    pub int1_drdy_xl: u8,
    /// Enables gyroscope data-ready interrupt on INT1 pin.
    /// Default: 0 (disabled)
    #[bits(1)]
    pub int1_drdy_g: u8,
    /// Boot status available on INT1 pin.
    /// Default: 0 (disabled)
    #[bits(1)]
    pub int1_boot: u8,
    #[bits(5, access = RO)]
    pub not_used0: u8,
}

/// INT2_CTRL (0x0E)
///
/// INT2 pin control register (R/W)
/// Output on INT2 pin is OR combination of signals selected here and in MD2_CFG (0x5F).
#[register(address = Reg::Int2Ctrl, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Int2Ctrl {
    /// Enables accelerometer data-ready interrupt on INT2 pin.
    /// Default: 0 (disabled)
    #[bits(1)]
    pub int2_drdy_xl: u8,
    /// Enables gyroscope data-ready interrupt on INT2 pin.
    /// Default: 0 (disabled)
    #[bits(1)]
    pub int2_drdy_g: u8,
    /// Enables temperature sensor data-ready interrupt on INT2 pin.
    /// Default: 0 (disabled)
    #[bits(1)]
    pub int2_drdy_temp: u8,
    #[bits(4, access = RO)]
    pub not_used0: u8,
    /// Enables ISPU sleep state signal on INT2 pin.
    /// 0: disabled (default)
    /// 1: enabled; INT2 low = ISPU running, INT2 high = ISPU sleep state
    #[bits(1)]
    pub int2_sleep_ispu: u8,
}

/// WHO_AM_I (0x0F)
///
/// WHO_AM_I register (R)
/// Read-only register fixed at 0x22.
#[register(address = Reg::WhoAmI, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WhoAmI {
    /// Device identifier, fixed value 0x22.
    #[bits(8)]
    pub id: u8,
}

/// CTRL1_XL (0x10)
///
/// Control register 1 for accelerometer (R/W)
#[register(address = Reg::Ctrl1Xl, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl1Xl {
    #[bits(2, access = RO)]
    pub not_used0: u8,
    /// Accelerometer full-scale selection (2 bits)
    /// 00: ±2 g (default)
    /// 01: ±16 g
    /// 10: ±4 g
    /// 11: ±8 g
    #[bits(2)]
    pub fs_xl: u8,
    /// Accelerometer output data rate selection (4 bits)
    /// See Table 31 in datasheet for ODR values.
    #[bits(4)]
    pub odr_xl: u8,
}

/// CTRL2_G (0x11)
///
/// Control register 2 for gyroscope (R/W)
#[register(address = Reg::Ctrl2G, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl2G {
    #[bits(1, access = RO)]
    pub not_used0: u8,
    /// Gyroscope chain full-scale selection for ±125 dps.
    /// 0: FS selected through FS[1:0]_G bits (default)
    /// 1: FS set to ±125 dps
    #[bits(1)]
    pub fs_125: u8,
    /// Gyroscope full-scale selection (2 bits)
    /// 00: ±250 dps (default)
    /// 01: ±500 dps
    /// 10: ±1000 dps
    /// 11: ±2000 dps
    #[bits(2)]
    pub fs_g: u8,
    /// Gyroscope output data rate selection (4 bits)
    /// See Table 35 in datasheet for ODR values.
    #[bits(4)]
    pub odr_g: u8,
}

/// CTRL3_C (0x12)
///
/// Control register 3 (R/W)
#[register(address = Reg::Ctrl3C, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl3C {
    /// Software reset. Writing 1 resets device; bit auto-cleared.
    #[bits(1)]
    pub sw_reset: u8,
    #[bits(1, access = RO)]
    pub not_used0: u8,
    /// Register address auto-increment during multiple byte access.
    /// 0: disabled
    /// 1: enabled (default)
    #[bits(1)]
    pub if_inc: u8,
    /// SPI serial interface mode selection.
    /// 0: 4-wire interface (default)
    /// 1: 3-wire interface
    #[bits(1)]
    pub sim: u8,
    /// Push-pull/open-drain selection on INT1 and INT2 pins.
    /// Must be 0 when H_LACTIVE is 1.
    /// 0: push-pull mode (default)
    /// 1: open-drain mode
    #[bits(1)]
    pub pp_od: u8,
    /// Interrupt activation level.
    /// 0: active-high (default)
    /// 1: active-low
    #[bits(1)]
    pub h_lactive: u8,
    /// Block data update.
    /// 0: continuous update (default)
    /// 1: output registers not updated until MSB and LSB read
    #[bits(1)]
    pub bdu: u8,
    /// Reboot memory content.
    /// 0: normal mode (default)
    /// 1: reboot memory content
    #[bits(1)]
    pub boot: u8,
}
/// CTRL4_C (0x13)
///
/// Control register 4 (R/W)
#[register(address = Reg::Ctrl4C, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl4C {
    #[bits(2, access = RO)]
    pub not_used0: u8,
    /// Disables I²C interface.
    /// 0: SPI and I²C enabled (default)
    /// 1: I²C disabled
    #[bits(1)]
    pub i2c_disable: u8,
    #[bits(2, access = RO)]
    pub not_used1: u8,
    /// Enables all interrupt signals on INT1 pin.
    /// 0: interrupt signals divided between INT1 and INT2 (default)
    /// 1: all interrupt signals on INT1
    #[bits(1)]
    pub int2_on_int1: u8,
    /// Enables gyroscope sleep mode.
    /// 0: disabled (default)
    /// 1: enabled
    #[bits(1)]
    pub sleep_g: u8,
    #[bits(1, access = RO)]
    pub not_used2: u8,
}

/// CTRL5_C (0x14)
///
/// Control register 5 (R/W)
#[register(address = Reg::Ctrl5C, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl5C {
    /// Enables linear acceleration sensor self-test (2 bits).
    /// 00: Self-test disabled (default)
    /// Other values: see Table 43 in datasheet.
    #[bits(2)]
    pub st_xl: u8,
    /// Enables angular rate sensor self-test (2 bits).
    /// 00: Self-test disabled (default)
    /// Other values: see Table 42 in datasheet.
    #[bits(2)]
    pub st_g: u8,
    #[bits(4, access = RO)]
    pub not_used0: u8,
}

/// CTRL6_C (0x15)
///
/// Control register 6 (R/W)
#[register(address = Reg::Ctrl6C, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl6C {
    #[bits(4, access = RO)]
    pub not_used0: u8,
    /// Disables high-performance operating mode for accelerometer.
    /// 0: high-performance mode enabled (default)
    /// 1: high-performance mode disabled
    #[bits(1)]
    pub xl_hm_mode: u8,
    #[bits(3, access = RO)]
    pub not_used1: u8,
}

/// CTRL7_G (0x16)
///
/// Control register 7 (R/W)
#[register(address = Reg::Ctrl7G, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl7G {
    #[bits(7, access = RO)]
    pub not_used0: u8,
    /// Disables high-performance operating mode for gyroscope.
    /// 0: high-performance mode enabled (default)
    /// 1: high-performance mode disabled
    #[bits(1)]
    pub g_hm_mode: u8,
}

/// CTRL9_C (0x18)
///
/// Control register 9 (R/W)
#[register(address = Reg::Ctrl9C, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl9C {
    /// Block data update (BDU) for ISPU output registers (2 bits).
    /// See Table 50 in datasheet for configuration details.
    #[bits(2)]
    pub ispu_bdu: u8,
    #[bits(2, access = RO)]
    pub not_used0: u8,
    /// ISPU IRQ rate selection (4 bits).
    /// 0000: power-down (default)
    /// 0001: 12.5 Hz
    /// 0010: 26 Hz
    /// 0011: 52 Hz
    /// 0100: 104 Hz
    /// 0101: 208 Hz
    /// 0110: 416 Hz
    /// 0111: 833 Hz
    /// 1000: 1667 Hz
    /// 1001: 3333 Hz
    /// 1010: 6667 Hz
    /// 1011-1111: reserved
    #[bits(4)]
    pub ispu_rate: u8,
}

/// CTRL10_C (0x19)
///
/// Control register 10 (R/W)
///
/// - TIMESTAMP_EN: Enables timestamp counter. Default: 0 (disabled).
///   The counter is readable in TIMESTAMP0 (0x40), TIMESTAMP1 (0x41), TIMESTAMP2 (0x42), and TIMESTAMP3 (0x43).
/// - ISPU_CLK_SEL: Selects ISPU core clock frequency.
///   0: core clock frequency set to 5 MHz (default)
///   1: core clock frequency set to 10 MHz
#[register(address = Reg::Ctrl10C, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl10C {
    #[bits(2, access = RO)]
    pub not_used0: u8,
    #[bits(1)]
    pub ispu_clk_sel: u8,
    #[bits(2, access = RO)]
    pub not_used1: u8,
    #[bits(1)]
    pub timestamp_en: u8,
    #[bits(2, access = RO)]
    pub not_used2: u8,
}

#[register(address = Reg::IspuIntStatus0Mainpage, access_type = Ism330is, generics = 2)]
pub struct IspuIntStatusMainPage(pub u32);

/// STATUS_REG (0x1E)
///
/// Status register (R)
///
/// - XLDA: Accelerometer new data available. 0: no new data; 1: new data available.
/// - GDA: Gyroscope new data available. 0: no new data; 1: new data available.
/// - TDA: Temperature new data available. 0: no new data; 1: new data available.
/// - TIMESTAMP_ENDCOUNT: Alerts timestamp overflow within 6.4 ms.
#[register(address = Reg::StatusReg, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusReg {
    #[bits(1)]
    pub xlda: u8,
    #[bits(1)]
    pub gda: u8,
    #[bits(1)]
    pub tda: u8,
    #[bits(4, access = RO)]
    pub not_used0: u8,
    #[bits(1)]
    pub timestamp_endcount: u8,
}

/// OUT_TEMP_L - OUT_TEMP_H (0x20 - 0x21)
///
/// Temperature data output register (R).
///
/// The value is expressed as a 16-bit word in two’s complement.
#[register(address = Reg::OutTempL, access_type = Ism330is, generics = 2)]
pub struct OutTemp(pub i16);

/// OUTX_L_G - OUTZ_H_G (0x22 - 0x27)
///
/// Angular rate sensor pitch axes (X, Y, Z) angular rate output register (R).
///
/// The value is expressed as a 16-bit word in two’s complement.
/// Data is according to the full-scale and ODR settings
/// (CTRL2_G (11h)) of the gyroscope.
#[named_register(address = Reg::OutxLG, access_type = Ism330is, generics = 2)]
pub struct OutXYZG {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// OUTX_L_A - OUTZ_H_A (0x28 - 0x2D)
///
/// Linear acceleration sensor X-axis output register (R).
///
/// The value is expressed as a 16-bit word in two’s complement.
/// Data are according to the full-scale and ODR settings (CTRL1_XL (10h)) of the accelerometer
#[named_register(address = Reg::OutxLA, access_type = Ism330is, generics = 2)]
pub struct OutXYZA {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// STATUS_MASTER_MAINPAGE (0x39)
///
/// Sensor hub status register (R)
///
/// - SENS_HUB_ENDOP: Sensor hub communication status.
///   0: communication not concluded; 1: communication concluded.
/// - SLAVE0_NACK to SLAVE3_NACK: Not acknowledge flags for slaves 0 to 3.
/// - WR_ONCE_DONE: Write operation on slave 0 completed when WRITE_ONCE bit in MASTER_CONFIG (0x14) is set.
#[register(address = Reg::StatusMasterMainpage, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusMasterMainpage {
    #[bits(1)]
    pub sens_hub_endop: u8,
    #[bits(2, access = RO)]
    pub not_used0: u8,
    #[bits(1)]
    pub slave0_nack: u8,
    #[bits(1)]
    pub slave1_nack: u8,
    #[bits(1)]
    pub slave2_nack: u8,
    #[bits(1)]
    pub slave3_nack: u8,
    #[bits(1)]
    pub wr_once_done: u8,
}

#[register(address = Reg::Timestamp0, access_type = Ism330is, generics = 2)]
pub struct Timestamp(pub u32);

/// MD1_CFG (0x5E)
///
/// Functions routing to INT1 pin register (R/W)
///
/// - INT1_SHUB: Routing sensor hub communication concluded event to INT1 pin.
/// - INT1_ISPU: Routing ISPU event to INT1 pin.
#[register(address = Reg::Md1Cfg, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Md1Cfg {
    #[bits(1)]
    pub int1_shub: u8,
    #[bits(1)]
    pub int1_ispu: u8,
    #[bits(6, access = RO)]
    pub not_used0: u8,
}

/// MD2_CFG (0x5F)
///
/// Functions routing to INT2 pin register (R/W)
///
/// - INT2_TIMESTAMP: Enables routing alert for timestamp overflow within 6.4 ms to INT2 pin.
/// - INT2_ISPU: Routing ISPU event to INT2 pin.
#[register(address = Reg::Md2Cfg, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Md2Cfg {
    #[bits(1)]
    pub int2_timestamp: u8,
    #[bits(1)]
    pub int2_ispu: u8,
    #[bits(6, access = RO)]
    pub not_used0: u8,
}

/// INTERNAL_FREQ_FINE (0x63)
///
/// Internal frequency register (R)
///
/// - FREQ_FINE[7:0]: Difference in percentage of effective ODR and timestamp rate with respect to typical.
///   Step: 0.15%. 8-bit two's complement format.
#[register(address = Reg::InternalFreqFine, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct InternalFreqFine {
    #[bits(8)]
    pub freq_fine: u8,
}

/// ISPU_DUMMY_CFG_1_L (0x73)
///
/// General-purpose input configuration register 1 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg1L, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg1L {
    #[bits(8)]
    pub ispu_dummy_cfg_1: u8,
}

/// ISPU_DUMMY_CFG_1_H (0x74)
///
/// General-purpose input configuration register 1 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg1H, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg1H {
    #[bits(8)]
    pub ispu_dummy_cfg_1: u8,
}

/// ISPU_DUMMY_CFG_2_L (0x75)
///
/// General-purpose input configuration register 2 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg2L, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg2L {
    #[bits(8)]
    pub ispu_dummy_cfg_2: u8,
}

/// ISPU_DUMMY_CFG_2_H (0x76)
///
/// General-purpose input configuration register 2 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg2H, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg2H {
    #[bits(8)]
    pub ispu_dummy_cfg_2: u8,
}

/// ISPU_DUMMY_CFG_3_L (0x77)
///
/// General-purpose input configuration register 3 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg3L, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg3L {
    #[bits(8)]
    pub ispu_dummy_cfg_3: u8,
}

/// ISPU_DUMMY_CFG_3_H (0x78)
///
/// General-purpose input configuration register 3 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg3H, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg3H {
    #[bits(8)]
    pub ispu_dummy_cfg_3: u8,
}

/// ISPU_DUMMY_CFG_4_L (0x79)
///
/// General-purpose input configuration register 4 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg4L, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg4L {
    #[bits(8)]
    pub ispu_dummy_cfg_4: u8,
}

/// ISPU_DUMMY_CFG_4_H (0x7A)
///
/// General-purpose input configuration register 4 for ISPU (R/W)
#[register(address = Reg::IspuDummyCfg4H, access_type = Ism330is, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDummyCfg4H {
    #[bits(8)]
    pub ispu_dummy_cfg_4: u8,
}

/// It routes interrupt signals on INT 1 pin.
///
/// The output of the INT1 pin is the OR combination of the signals selected here and in register MD1_CFG (5Eh).
/// The signals include accelerometer data-ready, gyroscope data-ready, boot status, sensor hub communication
/// concluded event, and ISPU event routing.
#[derive(Default)]
pub struct PinInt1Route {
    /// Enables accelerometer data-ready interrupt on the INT1 pin.
    pub drdy_xl: u8,
    /// Enables gyroscope data-ready interrupt on the INT1 pin.
    pub drdy_gy: u8,
    /// Boot status available on the INT1 pin.
    pub boot: u8,
    /// Routing sensor hub communication concluded event to INT1.
    pub sh_endop: u8,
    /// Routing ISPU event to INT1.
    pub ispu: u8,
}

/// It routes interrupt signals on INT 2 pin.
///
/// The output of the INT2 pin is the OR combination of the signals selected here and in register MD2_CFG (5Fh).
/// Signals include ISPU sleep state, temperature sensor data-ready, gyroscope data-ready, accelerometer data-ready,
/// timestamp overflow alert, and ISPU event routing.
#[derive(Default)]
pub struct PinInt2Route {
    /// Enables accelerometer data-ready interrupt on the INT2 pin.
    pub drdy_xl: u8,
    /// Enables gyroscope data-ready interrupt on the INT2 pin.
    pub drdy_gy: u8,
    /// Enables temperature sensor data-ready interrupt on the INT2 pin.
    pub drdy_temp: u8,
    /// Enables routing the alert for timestamp overflow within 6.4 ms to the INT2 pin.
    pub timestamp: u8,
    /// Enables ISPU sleep state signal on the INT2 pin.
    /// When enabled:
    /// - INT2 low: ISPU is running;
    /// - INT2 high: ISPU is in sleep state.
    pub ispu_sleep: u8,
    /// Routing ISPU event to INT2.
    pub ispu: u8,
}

/// Data ready signal mode
///
/// Selects between latched or pulsed data-ready mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DataReadyMode {
    /// Data-ready latched mode (returns to 0 only after an interface reading) (default)
    #[default]
    Latched = 0x0,
    /// Data-ready pulsed mode (the data-ready pulses are 75 μs long)
    Pulsed = 0x1,
}

/// High-performance mode enable/disable
///
/// Enables or disables high-performance operating mode for accelerometer or gyroscope.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum HighPerformanceMode {
    /// High-performance operating mode enabled (default)
    #[default]
    Enabled = 0x0,
    /// High-performance operating mode disabled
    Disabled = 0x1,
}

/// Accelerometer full-scale selection
///
/// Selects the full-scale range for the accelerometer.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum XlFullScale {
    /// ±2 g full scale (default)
    #[default]
    _2g = 0x0,
    /// ±16 g full scale
    _16g = 0x1,
    /// ±4 g full scale
    _4g = 0x2,
    /// ±8 g full scale
    _8g = 0x3,
}

/// Accelerometer output data rate (ODR)
///
/// Includes both high-performance and low-power mode ODRs.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum XlDataRate {
    /// Power-down mode
    #[default]
    Off = 0x0,
    /// 12.5 Hz ODR in high-performance mode
    _12_5hzHp = 0x1,
    /// 26 Hz ODR in high-performance mode
    _26hzHp = 0x2,
    /// 52 Hz ODR in high-performance mode
    _52hzHp = 0x3,
    /// 104 Hz ODR in high-performance mode
    _104hzHp = 0x4,
    /// 208 Hz ODR in high-performance mode
    _208hzHp = 0x5,
    /// 416 Hz ODR in high-performance mode
    _416hzHp = 0x6,
    /// 833 Hz ODR in high-performance mode
    _833hzHp = 0x7,
    /// 1667 Hz ODR in high-performance mode
    _1667hzHp = 0x8,
    /// 3333 Hz ODR in high-performance mode
    _3333hzHp = 0x9,
    /// 6667 Hz ODR in high-performance mode
    _6667hzHp = 0xa,
    /// 12.5 Hz ODR in low-power mode
    _12_5hzLp = 0x11,
    /// 26 Hz ODR in low-power mode
    _26hzLp = 0x12,
    /// 52 Hz ODR in low-power mode
    _52hzLp = 0x13,
    /// 104 Hz ODR in low-power mode
    _104hzLp = 0x14,
    /// 208 Hz ODR in low-power mode
    _208hzLp = 0x15,
    /// 416 Hz ODR in low-power mode
    _416hzLp = 0x16,
    /// 833 Hz ODR in low-power mode
    _833hzLp = 0x17,
    /// 1667 Hz ODR in low-power mode
    _1667hzLp = 0x18,
    /// 3333 Hz ODR in low-power mode
    _3333hzLp = 0x19,
    /// 6667 Hz ODR in low-power mode
    _6667hzLp = 0x1a,
    /// 1.6 Hz ODR in low-power mode
    _1_6hzLp = 0x1b,
}

/// Gyroscope full-scale selection
///
/// Selects the full-scale range for the gyroscope.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum GyFullScale {
    /// ±250 dps full scale
    _250dps = 0x0,
    /// ±500 dps full scale
    _500dps = 0x1,
    /// ±1000 dps full scale
    _1000dps = 0x2,
    /// ±2000 dps full scale
    _2000dps = 0x3,
    /// ±125 dps full scale (default)
    #[default]
    _125dps = 0x10,
}

/// Gyroscope output data rate (ODR)
///
/// Includes both high-performance and low-power mode ODRs.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum GyDataRate {
    /// Power-down mode
    #[default]
    Off = 0x0,
    /// 12.5 Hz ODR in high-performance mode
    _12_5hzHp = 0x1,
    /// 26 Hz ODR in high-performance mode
    _26hzHp = 0x2,
    /// 52 Hz ODR in high-performance mode
    _52hzHp = 0x3,
    /// 104 Hz ODR in high-performance mode
    _104hzHp = 0x4,
    /// 208 Hz ODR in high-performance mode
    _208hzHp = 0x5,
    /// 416 Hz ODR in high-performance mode
    _416hzHp = 0x6,
    /// 833 Hz ODR in high-performance mode
    _833hzHp = 0x7,
    /// 1667 Hz ODR in high-performance mode
    _1667hzHp = 0x8,
    /// 3333 Hz ODR in high-performance mode
    _3333hzHp = 0x9,
    /// 6667 Hz ODR in high-performance mode
    _6667hzHp = 0xa,
    /// 12.5 Hz ODR in low-power mode
    _12_5hzLp = 0x11,
    /// 26 Hz ODR in low-power mode
    _26hzLp = 0x12,
    /// 52 Hz ODR in low-power mode
    _52hzLp = 0x13,
    /// 104 Hz ODR in low-power mode
    _104hzLp = 0x14,
    /// 208 Hz ODR in low-power mode
    _208hzLp = 0x15,
    /// 416 Hz ODR in low-power mode
    _416hzLp = 0x16,
    /// 833 Hz ODR in low-power mode
    _833hzLp = 0x17,
    /// 1667 Hz ODR in low-power mode
    _1667hzLp = 0x18,
    /// 3333 Hz ODR in low-power mode
    _3333hzLp = 0x19,
    /// 6667 Hz ODR in low-power mode
    _6667hzLp = 0x1a,
}
/// Sleep mode for gyroscope
///
/// Enables or disables the gyroscope independently of the accelerometer.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Sleep {
    /// Gyroscope enabled (accelerometer can be independently controlled)
    #[default]
    GyroEnable = 0x0,
    /// Gyroscope disabled (accelerometer can be independently controlled)
    GyroDisable = 0x1,
}

/// Linear acceleration sensor self-test mode
///
/// Enables self-test with positive or negative sign or disables it.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum XlSelfTest {
    /// Linear acceleration sensor self-test disabled
    #[default]
    Disable = 0x0,
    /// Linear acceleration sensor self-test positive sign
    Positive = 0x1,
    /// Linear acceleration sensor self-test negative sign
    Negative = 0x2,
}

/// Angular rate sensor self-test mode
///
/// Enables self-test with positive or negative sign or disables it.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum GySelfTest {
    /// Angular rate sensor self-test disabled
    #[default]
    Disable = 0x0,
    /// Angular rate sensor self-test positive sign
    Positive = 0x1,
    /// Angular rate sensor self-test negative sign
    Negative = 0x3,
}

/// SPI interface mode selection
///
/// Selects between 4-wire and 3-wire SPI interface modes.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SpiMode {
    /// SPI 4-wire interface mode
    #[default]
    Spi4Wire = 0x0,
    /// SPI 3-wire interface mode
    Spi3Wire = 0x1,
}

/// I²C interface enable/disable mode
///
/// Enables or disables the I²C interface.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum UiI2cMode {
    /// I²C interface enabled (default)
    #[default]
    Enable = 0x0,
    /// I²C interface disabled
    Disable = 0x1,
}

/// Interrupt pin mode selection
///
/// Configures interrupt pins as push-pull or open-drain.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IntPinMode {
    /// Interrupt pins configured as push-pull (default)
    #[default]
    PushPull = 0x0,
    /// Interrupt pins configured as open-drain
    OpenDrain = 0x1,
}

/// Interrupt pin polarity
///
/// Configures interrupt pins active high or active low.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum PinPolarity {
    /// Interrupt pins active high (default)
    #[default]
    ActiveHigh = 0x0,
    /// Interrupt pins active low
    ActiveLow = 0x1,
}

/// ISPU core clock frequency selection
///
/// Selects the clock frequency of the ISPU core.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IspuClockSel {
    /// ISPU core clock frequency set to 5 MHz (default)
    #[default]
    _5mHz = 0x0,
    /// ISPU core clock frequency set to 10 MHz
    _10mHz = 0x1,
}

/// ISPU output data rate
///
/// Selects the output data rate of the ISPU.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IspuDataRate {
    /// ISPU power-down (off)
    #[default]
    Off = 0x0,
    /// ISPU output data rate 12.5 Hz
    _12_5hz = 0x1,
    /// ISPU output data rate 26 Hz
    _26hz = 0x2,
    /// ISPU output data rate 52 Hz
    _52hz = 0x3,
    /// ISPU output data rate 104 Hz
    _104hz = 0x4,
    /// ISPU output data rate 208 Hz
    _208hz = 0x5,
    /// ISPU output data rate 416 Hz
    _416hz = 0x6,
    /// ISPU output data rate 833 Hz
    _833hz = 0x7,
    /// ISPU output data rate 1667 Hz
    _1667hz = 0x8,
    /// ISPU output data rate 3333 Hz
    _3333hz = 0x9,
    /// ISPU output data rate 6667 Hz
    _6667hz = 0xa,
}
impl IspuDataRate {
    pub fn to_str(&self) -> &str {
        match self {
            IspuDataRate::Off => "off",
            IspuDataRate::_12_5hz => "12.5hz",
            IspuDataRate::_26hz => "26hz",
            IspuDataRate::_52hz => "52hz",
            IspuDataRate::_104hz => "104hz",
            IspuDataRate::_208hz => "208hz",
            IspuDataRate::_416hz => "416hz",
            IspuDataRate::_833hz => "833hz",
            IspuDataRate::_1667hz => "1667hz",
            IspuDataRate::_3333hz => "3333hz",
            IspuDataRate::_6667hz => "6667hz",
        }
    }
}

/// ISPU block data update configuration
///
/// Configures block data update for ISPU output registers.
/// Configuration permit to specify 2 or 4 bytes for the two following sectors:
/// ISPU_DOUT_00_L - ISPU_DOUT_15_H
/// ISPU_DOUT_16_L - ISPU_DOUT_31_H
/// 2 bytes correspond to 16 outputs while 4 bytes to 8 outputs.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IspuBdu {
    /// Block data update disabled (default)
    #[default]
    Off = 0x0,
    /// BDU on 2 bytes (16 outputs) for ISPU_DOUT_00_L to ISPU_DOUT_15_H
    /// and BDU on 4 bytes (8 outpus) for ISPU_DOUT_16_L to ISPU_DOUT_31_H
    On2b4b = 0x1,
    /// BDU on 2 bytes (16 outputs) for ISPU_DOUT_00_L to ISPU_DOUT_15_H
    /// and BDU on 2 bytes (16 outpus) for ISPU_DOUT_16_L to ISPU_DOUT_31_H
    On2b2b = 0x2,
    /// BDU on 4 bytes (8 outputs) for ISPU_DOUT_00_L to ISPU_DOUT_15_H
    /// and BDU on 4 bytes (8 outpus) for ISPU_DOUT_16_L to ISPU_DOUT_31_H
    On4b4b = 0x3,
}
