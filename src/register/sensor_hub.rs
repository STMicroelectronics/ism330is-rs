use super::SensorHubState;
use crate::Error;
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::register;
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum SensHubReg {
    SensorHub1 = 0x2,
    SensorHub2 = 0x3,
    SensorHub3 = 0x4,
    SensorHub4 = 0x5,
    SensorHub5 = 0x6,
    SensorHub6 = 0x7,
    SensorHub7 = 0x8,
    SensorHub8 = 0x9,
    SensorHub9 = 0x0A,
    SensorHub10 = 0x0B,
    SensorHub11 = 0x0C,
    SensorHub12 = 0x0D,
    SensorHub13 = 0x0E,
    SensorHub14 = 0x0F,
    SensorHub15 = 0x10,
    SensorHub16 = 0x11,
    SensorHub17 = 0x12,
    SensorHub18 = 0x13,
    MasterConfig = 0x14,
    Slv0Add = 0x15,
    Slv0Subadd = 0x16,
    Slv0Config = 0x17,
    Slv1Add = 0x18,
    Slv1Subadd = 0x19,
    Slv1Config = 0x1A,
    Slv2Add = 0x1B,
    Slv2Subadd = 0x1C,
    Slv2Config = 0x1D,
    Slv3Add = 0x1E,
    Slv3Subadd = 0x1F,
    Slv3Config = 0x20,
    DatawriteSlv0 = 0x21,
    StatusMaster = 0x22,
}

/// SENSOR_HUB_1 (0x02)
///
/// Sensor hub output register (R)
/// First byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations.
#[register(address = SensHubReg::SensorHub1, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub1 {
    #[bits(8)]
    pub sensorhub1: u8,
}

/// SENSOR_HUB_2 (0x03)
///
/// Sensor hub output register (R)
/// Second byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations.
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub2 {
    #[bits(8)]
    pub sensorhub2: u8,
}

/// SENSOR_HUB_3 (0x04)
///
/// Sensor hub output register (R)
/// Third byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub3 {
    #[bits(8)]
    pub sensorhub3: u8,
}

/// SENSOR_HUB_4 (0x05)
///
/// Sensor hub output register (R)
/// Fourth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub4 {
    #[bits(8)]
    pub sensorhub4: u8,
}

/// SENSOR_HUB_5 (0x06)
///
/// Sensor hub output register (R)
/// Fifth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub5 {
    #[bits(8)]
    pub sensorhub5: u8,
}

/// SENSOR_HUB_6 (0x07)
///
/// Sensor hub output register (R)
/// Sixth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub6 {
    #[bits(8)]
    pub sensorhub6: u8,
}

/// SENSOR_HUB_7 (0x08)
///
/// Sensor hub output register (R)
/// Seventh byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub7 {
    #[bits(8)]
    pub sensorhub7: u8,
}

/// SENSOR_HUB_8 (0x09)
///
/// Sensor hub output register (R)
/// Eighth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub8 {
    #[bits(8)]
    pub sensorhub8: u8,
}

/// SENSOR_HUB_9 (0x0A)
///
/// Sensor hub output register (R)
/// Ninth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub9 {
    #[bits(8)]
    pub sensorhub9: u8,
}

/// SENSOR_HUB_10 (0x0B)
///
/// Sensor hub output register (R)
/// Tenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub10 {
    #[bits(8)]
    pub sensorhub10: u8,
}

/// SENSOR_HUB_11 (0x0C)
///
/// Sensor hub output register (R)
/// Eleventh byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub11 {
    #[bits(8)]
    pub sensorhub11: u8,
}

/// SENSOR_HUB_12 (0x0D)
///
/// Sensor hub output register (R)
/// Twelfth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub12 {
    #[bits(8)]
    pub sensorhub12: u8,
}

/// SENSOR_HUB_13 (0x0E)
///
/// Sensor hub output register (R)
/// Thirteenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub13 {
    #[bits(8)]
    pub sensorhub13: u8,
}

/// SENSOR_HUB_14 (0x0F)
///
/// Sensor hub output register (R)
/// Fourteenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub14 {
    #[bits(8)]
    pub sensorhub14: u8,
}

/// SENSOR_HUB_15 (0x10)
///
/// Sensor hub output register (R)
/// Fifteenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub15 {
    #[bits(8)]
    pub sensorhub15: u8,
}

/// SENSOR_HUB_16 (0x11)
///
/// Sensor hub output register (R)
/// Sixteenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub16 {
    #[bits(8)]
    pub sensorhub16: u8,
}

/// SENSOR_HUB_17 (0x12)
///
/// Sensor hub output register (R)
/// Seventeenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub17 {
    #[bits(8)]
    pub sensorhub17: u8,
}

/// SENSOR_HUB_18 (0x13)
///
/// Sensor hub output register (R)
/// Eighteenth byte associated to external sensors.
/// Content consistent with SLVx_CONFIG number of read operation configurations (for external sensors 0 to 3).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub18 {
    #[bits(8)]
    pub sensorhub18: u8,
}

/// MASTER_CONFIG (0x14)
///
/// Master configuration register (R/W)
///
/// - AUX_SENS_ON[1:0]: Number of external sensors to be read by the sensor hub.
///   00: one sensor; 01: two sensors; 10: three sensors; 11: four sensors.
/// - MASTER_ON: Enables sensor hub I²C master.
///   0: disabled; 1: enabled.
/// - SHUB_PU_EN: Enables master I²C pull-up.
///   0: internal pull-up disabled; 1: enabled.
/// - PASS_THROUGH_MODE: I²C interface pass-through.
///   0: disabled; 1: enabled (main I²C line short-circuited with auxiliary line).
/// - START_CONFIG: Selects sensor hub trigger signal.
///   0: accelerometer/gyro data-ready; 1: external INT2 pin.
/// - WRITE_ONCE: Slave 0 write operation performed only at first sensor hub cycle.
///   0: write operation each cycle; 1: only first cycle.
/// - RST_MASTER_REGS: Resets master logic and output registers.
///   Must be set to 1 then 0.
#[register(address = SensHubReg::MasterConfig, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MasterConfig {
    #[bits(2)]
    pub aux_sens_on: u8,
    #[bits(1)]
    pub master_on: u8,
    #[bits(1)]
    pub shub_pu_en: u8,
    #[bits(1)]
    pub pass_through_mode: u8,
    #[bits(1)]
    pub start_config: u8,
    #[bits(1)]
    pub write_once: u8,
    #[bits(1)]
    pub rst_master_regs: u8,
}

/// SLV0_ADD (0x15)
///
/// I²C slave address of the first external sensor (sensor 1) register (R/W)
///
/// - RW_0: Read/write operation on sensor 1.
///   0: write operation; 1: read operation.
/// - SLAVE0_ADD[6:0]: I²C slave address of sensor 1.
#[register(address = SensHubReg::Slv0Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv0Add {
    #[bits(1)]
    pub rw_0: u8,
    #[bits(7)]
    pub slave0_add: u8,
}

/// SLV0_SUBADD (0x16)
///
/// Address of register on the first external sensor (sensor 1) (R/W)
#[register(address = SensHubReg::Slv0Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv0Subadd {
    #[bits(8)]
    pub slave0_reg: u8,
}

/// SLV0_CONFIG (0x17)
///
/// First external sensor (sensor 1) configuration and sensor hub settings register (R/W)
///
/// - SLAVE0_NUMOP[2:0]: Number of read operations on sensor 1.
/// - SHUB_ODR[1:0]: Rate at which the master communicates.
///   00: 104 Hz (or max ODR between accelerometer and gyro if less than 104 Hz)
///   01: 52 Hz
///   10: 26 Hz
///   11: 12.5 Hz
#[register(address = SensHubReg::Slv0Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv0Config {
    #[bits(3)]
    pub slave0_numop: u8,
    #[bits(3, access = RO)]
    pub not_used0: u8,
    #[bits(2)]
    pub shub_odr: u8,
}

/// SLV1_ADD (0x18)
///
/// I²C slave address of the second external sensor (sensor 2) register (R/W)
///
/// - R_1: Enables read operation on sensor 2.
///   0: disabled; 1: enabled.
/// - SLAVE1_ADD[6:0]: I²C slave address of sensor 2.
#[register(address = SensHubReg::Slv1Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv1Add {
    #[bits(1)]
    pub r_1: u8,
    #[bits(7)]
    pub slave1_add: u8,
}

/// SLV1_SUBADD (0x19)
///
/// Address of register on the second external sensor (sensor 2) (R/W)
#[register(address = SensHubReg::Slv1Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv1Subadd {
    #[bits(8)]
    pub slave1_reg: u8,
}

/// SLV1_CONFIG (0x1A)
///
/// Second external sensor (sensor 2) configuration register (R/W)
///
/// - SLAVE1_NUMOP[2:0]: Number of read operations on sensor 2.
#[register(address = SensHubReg::Slv1Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv1Config {
    #[bits(3)]
    pub slave1_numop: u8,
    #[bits(5, access = RO)]
    pub not_used0: u8,
}

/// SLV2_ADD (0x1B)
///
/// I²C slave address of the third external sensor (sensor 3) register (R/W)
///
/// - R_2: Enables read operation on sensor 3.
///   0: disabled; 1: enabled.
/// - SLAVE2_ADD[6:0]: I²C slave address of sensor 3.
#[register(address = SensHubReg::Slv2Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv2Add {
    #[bits(1)]
    pub r_2: u8,
    #[bits(7)]
    pub slave2_add: u8,
}

/// SLV2_SUBADD (0x1C)
///
/// Address of register on the third external sensor (sensor 3) (R/W)
#[register(address = SensHubReg::Slv2Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv2Subadd {
    #[bits(8)]
    pub slave2_reg: u8,
}

/// SLV2_CONFIG (0x1D)
///
/// Third external sensor (sensor 3) configuration register (R/W)
///
/// - SLAVE2_NUMOP[2:0]: Number of read operations on sensor 3.
#[register(address = SensHubReg::Slv2Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv2Config {
    #[bits(3)]
    pub slave2_numop: u8,
    #[bits(5, access = RO)]
    pub not_used0: u8,
}

/// SLV3_ADD (0x1E)
///
/// I²C slave address of the fourth external sensor (sensor 4) register (R/W)
///
/// - R_3: Enables read operation on sensor 4.
///   0: disabled; 1: enabled.
/// - SLAVE3_ADD[6:0]: I²C slave address of sensor 4.
#[register(address = SensHubReg::Slv3Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv3Add {
    #[bits(1)]
    pub r_3: u8,
    #[bits(7)]
    pub slave3_add: u8,
}

/// SLV3_SUBADD (0x1F)
///
/// Address of register on the fourth external sensor (sensor 4) (R/W)
#[register(address = SensHubReg::Slv3Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv3Subadd {
    #[bits(8)]
    pub slave3_reg: u8,
}

/// SLV3_CONFIG (0x20)
///
/// Fourth external sensor (sensor 4) configuration register (R/W)
///
/// - SLAVE3_NUMOP[2:0]: Number of read operations on sensor 4.
#[register(address = SensHubReg::Slv3Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Slv3Config {
    #[bits(3)]
    pub slave3_numop: u8,
    #[bits(5, access = RO)]
    pub not_used0: u8,
}

/// DATAWRITE_SLV0 (0x21)
///
/// Data to be written into the slave device register (R/W)
#[register(address = SensHubReg::DatawriteSlv0, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct DatawriteSlv0 {
    #[bits(8)]
    pub slave0_dataw: u8,
}

/// STATUS_MASTER (0x22)
///
/// Sensor hub source register (R)
///
/// - SENS_HUB_ENDOP: Sensor hub communication status.
///   0: communication not concluded; 1: communication concluded.
/// - SLAVE0_NACK to SLAVE3_NACK: Not acknowledge flags for slaves 0 to 3.
/// - WR_ONCE_DONE: Write operation on slave 0 completed when WRITE_ONCE bit in MASTER_CONFIG (0x14) is set.
#[register(address = SensHubReg::StatusMaster, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusMaster {
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

/// Sensor hub connected slaves configuration
///
/// Selects the number of external sensors connected to the sensor hub.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShSlaveConnected {
    /// Sensor hub connected to slave 0 only
    #[default]
    _0 = 0x0,
    /// Sensor hub connected to slaves 0 and 1
    _01 = 0x1,
    /// Sensor hub connected to slaves 0, 1, and 2
    _012 = 0x2,
    /// Sensor hub connected to slaves 0, 1, 2, and 3
    _0123 = 0x3,
}

/// Sensor hub synchronization mode
///
/// Selects the trigger signal for the sensor hub.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShSyncroMode {
    /// Sensor hub trigger signal is accelerometer/gyroscope data-ready (default)
    #[default]
    TrigXlGyDrdy = 0x0,
    /// Sensor hub trigger signal is external INT2 pin
    TrigInt2 = 0x1,
}

/// Sensor hub write mode
///
/// Selects write operation mode for sensor hub cycles.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShWriteMode {
    /// Write operation for each sensor hub cycle (default)
    #[default]
    EachShCycle = 0x0,
    /// Write operation only for the first sensor hub cycle
    OnlyFirstCycle = 0x1,
}

/// Sensor hub data rate
///
/// Selects the communication rate of the sensor hub.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShDataRate {
    /// Sensor hub communication rate 104 Hz
    _104hz = 0x0,
    /// Sensor hub communication rate 52 Hz
    _52hz = 0x1,
    /// Sensor hub communication rate 26 Hz
    _26hz = 0x2,
    /// Sensor hub communication rate 12.5 Hz (default)
    #[default]
    _12_5hz = 0x3,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct ShCfgWrite {
    pub slv0_add: u8,
    pub slv0_subadd: u8,
    pub slv0_data: u8,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub struct ShCfgRead {
    pub slv_add: u8,
    pub slv_subadd: u8,
    pub slv_len: u8,
}
