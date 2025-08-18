use super::IspuState;
use crate::Error;
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::register;
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum IspuReg {
    IspuConfig = 0x2,
    IspuStatus = 0x4,
    IspuMemSel = 0x8,
    IspuMemAddr0 = 0x9,
    IspuMemAddr1 = 0xA,
    IspuMemData = 0x0B,
    IspuIf2sFlagL = 0x0C,
    IspuIf2sFlagH = 0x0D,
    IspuS2ifFlagL = 0x0E,
    IspuS2ifFlagH = 0x0F,
    IspuDout00L = 0x10,
    IspuDout00H = 0x11,
    IspuDout01L = 0x12,
    IspuDout01H = 0x13,
    IspuDout02L = 0x14,
    IspuDout02H = 0x15,
    IspuDout03L = 0x16,
    IspuDout03H = 0x17,
    IspuDout04L = 0x18,
    IspuDout04H = 0x19,
    IspuDout05L = 0x1A,
    IspuDout05H = 0x1B,
    IspuDout06L = 0x1C,
    IspuDout06H = 0x1D,
    IspuDout07L = 0x1E,
    IspuDout07H = 0x1F,
    IspuDout08L = 0x20,
    IspuDout08H = 0x21,
    IspuDout09L = 0x22,
    IspuDout09H = 0x23,
    IspuDout10L = 0x24,
    IspuDout10H = 0x25,
    IspuDout11L = 0x26,
    IspuDout11H = 0x27,
    IspuDout12L = 0x28,
    IspuDout12H = 0x29,
    IspuDout13L = 0x2A,
    IspuDout13H = 0x2B,
    IspuDout14L = 0x2C,
    IspuDout14H = 0x2D,
    IspuDout15L = 0x2E,
    IspuDout15H = 0x2F,
    IspuDout16L = 0x30,
    IspuDout16H = 0x31,
    IspuDout17L = 0x32,
    IspuDout17H = 0x33,
    IspuDout18L = 0x34,
    IspuDout18H = 0x35,
    IspuDout19L = 0x36,
    IspuDout19H = 0x37,
    IspuDout20L = 0x38,
    IspuDout20H = 0x39,
    IspuDout21L = 0x3A,
    IspuDout21H = 0x3B,
    IspuDout22L = 0x3C,
    IspuDout22H = 0x3D,
    IspuDout23L = 0x3E,
    IspuDout23H = 0x3F,
    IspuDout24L = 0x40,
    IspuDout24H = 0x41,
    IspuDout25L = 0x42,
    IspuDout25H = 0x43,
    IspuDout26L = 0x44,
    IspuDout26H = 0x45,
    IspuDout27L = 0x46,
    IspuDout27H = 0x47,
    IspuDout28L = 0x48,
    IspuDout28H = 0x49,
    IspuDout29L = 0x4A,
    IspuDout29H = 0x4B,
    IspuDout30L = 0x4C,
    IspuDout30H = 0x4D,
    IspuDout31L = 0x4E,
    IspuDout31H = 0x4F,
    IspuInt1Ctrl0 = 0x50,
    IspuInt1Ctrl1 = 0x51,
    IspuInt1Ctrl2 = 0x52,
    IspuInt1Ctrl3 = 0x53,
    IspuInt2Ctrl0 = 0x54,
    IspuInt2Ctrl1 = 0x55,
    IspuInt2Ctrl2 = 0x56,
    IspuInt2Ctrl3 = 0x57,
    IspuIntStatus0 = 0x58,
    IspuIntStatus1 = 0x59,
    IspuIntStatus2 = 0x5A,
    IspuIntStatus3 = 0x5B,
    IspuAlgo0 = 0x70,
    IspuAlgo1 = 0x71,
    IspuAlgo2 = 0x72,
    IspuAlgo3 = 0x73,
}

/// ISPU_CONFIG (0x02)
///
/// ISPU configuration register (R/W)
///
/// - ISPU_RST_N: ISPU active-low reset.
/// - CLK_DIS: When active, stops the clock of ISPU.
/// - LATCHED: Configures interrupt generation.
///   0: interrupt pulsed; 1: interrupt latched.
#[register(address = IspuReg::IspuConfig, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuConfig {
    #[bits(1)]
    pub ispu_rst_n: u8,
    #[bits(1)]
    pub clk_dis: u8,
    #[bits(2, access = RO)]
    pub not_used0: u8,
    #[bits(1)]
    pub latched: u8,
    #[bits(3, access = RO)]
    pub not_used1: u8,
}

/// ISPU_STATUS (0x04)
///
/// ISPU status register (R)
///
/// - BOOT_END: End of ISPU boot procedure.
/// - Other bits are reserved and read-only.
#[register(address = IspuReg::IspuStatus, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuStatus {
    #[bits(2, access = RO)]
    pub not_used0: u8,
    #[bits(1)]
    pub boot_end: u8,
    #[bits(5, access = RO)]
    pub not_used1: u8,
}

/// ISPU_MEM_SEL (0x08)
///
/// ISPU memory selection register (R/W)
///
/// - MEM_SEL: Selects the memory to be accessed.
///   0: data RAM memory (default)
///   1: program RAM memory
/// - READ_MEM_EN: Enables reading from program or data memories.
///   0: disabled (default)
///   1: enabled
#[register(address = IspuReg::IspuMemSel, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuMemSel {
    #[bits(1)]
    pub mem_sel: u8,
    #[bits(5, access = RO)]
    pub not_used0: u8,
    #[bits(1)]
    pub read_mem_en: u8,
    #[bits(1, access = RO)]
    pub not_used1: u8,
}

/// ISPU_MEM_ADDR (0x09, 0x0A)
///
/// ISPU memory address register (R/W)
///
/// 16-bit address to be read/written.
#[register(address = IspuReg::IspuMemAddr0, access_type = IspuState, generics = 2)]
pub struct IspuMemAddr(pub u16);

/// ISPU_MEM_DATA (0x0B)
///
/// ISPU memory data register (R/W)
///
/// Byte to write to memory in write transaction or data read from memory in read transaction.
#[register(address = IspuReg::IspuMemData, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuMemData {
    #[bits(8)]
    pub mem_data: u8,
}

/// ISPU_IF2S_FLAG (0x0C, 0x0D)
///
/// Interface to ISPU register (R/W, set only)
///
/// 16-bit general purpose bits which can be set from the interface and cleared by ISPU.
#[register(address = IspuReg::IspuIf2sFlagL, access_type = IspuState, generics = 2)]
pub struct IspuIf2sFlag(pub u16);

/// ISPU_S2IF_FLAG (0x0E, 0x0F)
///
/// ISPU to interface register (R/W, clear only)
///
/// 16-bit general purpose bits which can be set from ISPU and cleared by the interface.
#[register(address = IspuReg::IspuS2ifFlagL, access_type = IspuState, generics = 2)]
pub struct IspuS2ifFlag(pub u16);

/// ISPU_S2IF_FLAG_L (0x0E)
#[register(address = IspuReg::IspuS2ifFlagL, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuS2ifFlagL {
    #[bits(8)]
    pub s2if: u8,
}

/// ISPU_S2IF_FLAG_H (0x0F)
#[register(address = IspuReg::IspuS2ifFlagH, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuS2ifFlagH {
    #[bits(8)]
    pub s2if: u8,
}

/// ISPU_DOUT_00_L (0x10)
///
/// ISPU output register 0 low byte (R)
#[register(address = IspuReg::IspuDout00L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout00L {
    #[bits(8)]
    pub dout0: u8,
}

/// ISPU_DOUT_00_H (0x11)
///
/// ISPU output register 0 high byte (R)
#[register(address = IspuReg::IspuDout00H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout00H {
    #[bits(8)]
    pub dout0: u8,
}

/// ISPU_DOUT_01_L (0x12)
///
/// ISPU output register 01 low byte (R)
#[register(address = IspuReg::IspuDout01L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout01L {
    #[bits(8)]
    pub dout1: u8,
}

/// ISPU_DOUT_01_H (0x13)
///
/// ISPU output register 01 high byte (R)
#[register(address = IspuReg::IspuDout01H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout01H {
    #[bits(8)]
    pub dout1: u8,
}

/// ISPU_DOUT_02_L (0x14)
///
/// ISPU output register 02 low byte (R)
#[register(address = IspuReg::IspuDout02L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout02L {
    #[bits(8)]
    pub dout2: u8,
}

/// ISPU_DOUT_02_H (0x15)
///
/// ISPU output register 02 high byte (R)
#[register(address = IspuReg::IspuDout02H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout02H {
    #[bits(8)]
    pub dout2: u8,
}

/// ISPU_DOUT_03_L (0x16)
///
/// ISPU output register 03 low byte (R)
#[register(address = IspuReg::IspuDout03L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout03L {
    #[bits(8)]
    pub dout3: u8,
}

/// ISPU_DOUT_03_H (0x17)
///
/// ISPU output register 03 high byte (R)
#[register(address = IspuReg::IspuDout03H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout03H {
    #[bits(8)]
    pub dout3: u8,
}

/// ISPU_DOUT_04_L (0x18)
///
/// ISPU output register 04 low byte (R)
#[register(address = IspuReg::IspuDout04L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout04L {
    #[bits(8)]
    pub dout4: u8,
}

/// ISPU_DOUT_04_H (0x19)
///
/// ISPU output register 04 high byte (R)
#[register(address = IspuReg::IspuDout04H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout04H {
    #[bits(8)]
    pub dout4: u8,
}

/// ISPU_DOUT_05_L (0x1A)
///
/// ISPU output register 05 low byte (R)
#[register(address = IspuReg::IspuDout05L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout05L {
    #[bits(8)]
    pub dout5: u8,
}

/// ISPU_DOUT_05_H (0x1B)
///
/// ISPU output register 05 high byte (R)
#[register(address = IspuReg::IspuDout05H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout05H {
    #[bits(8)]
    pub dout5: u8,
}

/// ISPU_DOUT_06_L (0x1C)
///
/// ISPU output register 06 low byte (R)
#[register(address = IspuReg::IspuDout06L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout06L {
    #[bits(8)]
    pub dout6: u8,
}

/// ISPU_DOUT_06_H (0x1D)
///
/// ISPU output register 06 high byte (R)
#[register(address = IspuReg::IspuDout06H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout06H {
    #[bits(8)]
    pub dout6: u8,
}

/// ISPU_DOUT_07_L (0x1E)
///
/// ISPU output register 07 low byte (R)
#[register(address = IspuReg::IspuDout07L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout07L {
    #[bits(8)]
    pub dout7: u8,
}

/// ISPU_DOUT_07_H (0x1F)
///
/// ISPU output register 07 high byte (R)
#[register(address = IspuReg::IspuDout07H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout07H {
    #[bits(8)]
    pub dout7: u8,
}

/// ISPU_DOUT_08_L (0x20)
///
/// ISPU output register 08 low byte (R)
#[register(address = IspuReg::IspuDout08L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout08L {
    #[bits(8)]
    pub dout8: u8,
}

/// ISPU_DOUT_08_H (0x21)
///
/// ISPU output register 08 high byte (R)
#[register(address = IspuReg::IspuDout08H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout08H {
    #[bits(8)]
    pub dout8: u8,
}

/// ISPU_DOUT_09_L (0x22)
///
/// ISPU output register 09 low byte (R)
#[register(address = IspuReg::IspuDout09L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout09L {
    #[bits(8)]
    pub dout9: u8,
}

/// ISPU_DOUT_09_H (0x23)
///
/// ISPU output register 09 high byte (R)
#[register(address = IspuReg::IspuDout09H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout09H {
    #[bits(8)]
    pub dout9: u8,
}

/// ISPU_DOUT_10_L (0x24)
///
/// ISPU output register 10 low byte (R)
#[register(address = IspuReg::IspuDout10L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout10L {
    #[bits(8)]
    pub dout10: u8,
}

/// ISPU_DOUT_10_H (0x25)
///
/// ISPU output register 10 high byte (R)
#[register(address = IspuReg::IspuDout10H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout10H {
    #[bits(8)]
    pub dout10: u8,
}

/// ISPU_DOUT_11_L (0x26)
///
/// ISPU output register 11 low byte (R)
#[register(address = IspuReg::IspuDout11L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout11L {
    #[bits(8)]
    pub dout11: u8,
}

/// ISPU_DOUT_11_H (0x27)
///
/// ISPU output register 11 high byte (R)
#[register(address = IspuReg::IspuDout11H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout11H {
    #[bits(8)]
    pub dout11: u8,
}

/// ISPU_DOUT_12_L (0x28)
///
/// ISPU output register 12 low byte (R)
#[register(address = IspuReg::IspuDout12L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout12L {
    #[bits(8)]
    pub dout12: u8,
}

/// ISPU_DOUT_12_H (0x29)
///
/// ISPU output register 12 high byte (R)
#[register(address = IspuReg::IspuDout12H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout12H {
    #[bits(8)]
    pub dout12: u8,
}

/// ISPU_DOUT_13_L (0x2A)
///
/// ISPU output register 13 low byte (R)
#[register(address = IspuReg::IspuDout13L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout13L {
    #[bits(8)]
    pub dout13: u8,
}

/// ISPU_DOUT_13_H (0x2B)
///
/// ISPU output register 13 high byte (R)
#[register(address = IspuReg::IspuDout13H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout13H {
    #[bits(8)]
    pub dout13: u8,
}

/// ISPU_DOUT_14_L (0x2C)
///
/// ISPU output register 14 low byte (R)
#[register(address = IspuReg::IspuDout14L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout14L {
    #[bits(8)]
    pub dout14: u8,
}

/// ISPU_DOUT_14_H (0x2D)
///
/// ISPU output register 14 high byte (R)
#[register(address = IspuReg::IspuDout14H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout14H {
    #[bits(8)]
    pub dout14: u8,
}

/// ISPU_DOUT_15_L (0x2E)
///
/// ISPU output register 15 low byte (R)
#[register(address = IspuReg::IspuDout15L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout15L {
    #[bits(8)]
    pub dout15: u8,
}

/// ISPU_DOUT_15_H (0x2F)
///
/// ISPU output register 15 high byte (R)
#[register(address = IspuReg::IspuDout15H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout15H {
    #[bits(8)]
    pub dout15: u8,
}

/// ISPU_DOUT_16_L (0x30)
///
/// ISPU output register 16 low byte (R)
#[register(address = IspuReg::IspuDout16L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout16L {
    #[bits(8)]
    pub dout16: u8,
}

/// ISPU_DOUT_16_H (0x31)
///
/// ISPU output register 16 high byte (R)
#[register(address = IspuReg::IspuDout16H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout16H {
    #[bits(8)]
    pub dout16: u8,
}

/// ISPU_DOUT_17_L (0x32)
///
/// ISPU output register 17 low byte (R)
#[register(address = IspuReg::IspuDout17L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout17L {
    #[bits(8)]
    pub dout17: u8,
}

/// ISPU_DOUT_17_H (0x33)
///
/// ISPU output register 17 high byte (R)
#[register(address = IspuReg::IspuDout17H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout17H {
    #[bits(8)]
    pub dout17: u8,
}

/// ISPU_DOUT_18_L (0x34)
///
/// ISPU output register 18 low byte (R)
#[register(address = IspuReg::IspuDout18L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout18L {
    #[bits(8)]
    pub dout18: u8,
}

/// ISPU_DOUT_18_H (0x35)
///
/// ISPU output register 18 high byte (R)
#[register(address = IspuReg::IspuDout18H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout18H {
    #[bits(8)]
    pub dout18: u8,
}

/// ISPU_DOUT_19_L (0x36)
///
/// ISPU output register 19 low byte (R)
#[register(address = IspuReg::IspuDout19L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout19L {
    #[bits(8)]
    pub dout19: u8,
}

/// ISPU_DOUT_19_H (0x37)
///
/// ISPU output register 19 high byte (R)
#[register(address = IspuReg::IspuDout19H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout19H {
    #[bits(8)]
    pub dout19: u8,
}

/// ISPU_DOUT_20_L (0x38)
///
/// ISPU output register 20 low byte (R)
#[register(address = IspuReg::IspuDout20L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout20L {
    #[bits(8)]
    pub dout20: u8,
}

/// ISPU_DOUT_20_H (0x39)
///
/// ISPU output register 20 high byte (R)
#[register(address = IspuReg::IspuDout20H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout20H {
    #[bits(8)]
    pub dout20: u8,
}

/// ISPU_DOUT_21_L (0x3A)
///
/// ISPU output register 21 low byte (R)
#[register(address = IspuReg::IspuDout21L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout21L {
    #[bits(8)]
    pub dout21: u8,
}

/// ISPU_DOUT_21_H (0x3B)
///
/// ISPU output register 21 high byte (R)
#[register(address = IspuReg::IspuDout21H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout21H {
    #[bits(8)]
    pub dout21: u8,
}

/// ISPU_DOUT_22_L (0x3C)
///
/// ISPU output register 22 low byte (R)
#[register(address = IspuReg::IspuDout22L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout22L {
    #[bits(8)]
    pub dout22: u8,
}

/// ISPU_DOUT_22_H (0x3D)
///
/// ISPU output register 22 high byte (R)
#[register(address = IspuReg::IspuDout22H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout22H {
    #[bits(8)]
    pub dout22: u8,
}

/// ISPU_DOUT_23_L (0x3E)
///
/// ISPU output register 23 low byte (R)
#[register(address = IspuReg::IspuDout23L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout23L {
    #[bits(8)]
    pub dout23: u8,
}

/// ISPU_DOUT_23_H (0x3F)
///
/// ISPU output register 23 high byte (R)
#[register(address = IspuReg::IspuDout23H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout23H {
    #[bits(8)]
    pub dout23: u8,
}

/// ISPU_DOUT_24_L (0x40)
///
/// ISPU output register 24 low byte (R)
#[register(address = IspuReg::IspuDout24L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout24L {
    #[bits(8)]
    pub dout24: u8,
}

/// ISPU_DOUT_24_H (0x41)
///
/// ISPU output register 24 high byte (R)
#[register(address = IspuReg::IspuDout24H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout24H {
    #[bits(8)]
    pub dout24: u8,
}

/// ISPU_DOUT_25_L (0x42)
///
/// ISPU output register 25 low byte (R)
#[register(address = IspuReg::IspuDout25L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout25L {
    #[bits(8)]
    pub dout25: u8,
}

/// ISPU_DOUT_25_H (0x43)
///
/// ISPU output register 25 high byte (R)
#[register(address = IspuReg::IspuDout25H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout25H {
    #[bits(8)]
    pub dout25: u8,
}

/// ISPU_DOUT_26_L (0x44)
///
/// ISPU output register 26 low byte (R)
#[register(address = IspuReg::IspuDout26L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout26L {
    #[bits(8)]
    pub dout26: u8,
}

/// ISPU_DOUT_26_H (0x45)
///
/// ISPU output register 26 high byte (R)
#[register(address = IspuReg::IspuDout26H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout26H {
    #[bits(8)]
    pub dout26: u8,
}

/// ISPU_DOUT_27_L (0x46)
///
/// ISPU output register 27 low byte (R)
#[register(address = IspuReg::IspuDout27L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout27L {
    #[bits(8)]
    pub dout27: u8,
}

/// ISPU_DOUT_27_H (0x47)
///
/// ISPU output register 27 high byte (R)
#[register(address = IspuReg::IspuDout27H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout27H {
    #[bits(8)]
    pub dout27: u8,
}

/// ISPU_DOUT_28_L (0x48)
///
/// ISPU output register 28 low byte (R)
#[register(address = IspuReg::IspuDout28L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout28L {
    #[bits(8)]
    pub dout28: u8,
}

/// ISPU_DOUT_28_H (0x49)
///
/// ISPU output register 28 high byte (R)
#[register(address = IspuReg::IspuDout28H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout28H {
    #[bits(8)]
    pub dout28: u8,
}

/// ISPU_DOUT_29_L (0x4A)
///
/// ISPU output register 29 low byte (R)
#[register(address = IspuReg::IspuDout29L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout29L {
    #[bits(8)]
    pub dout29: u8,
}

/// ISPU_DOUT_29_H (0x4B)
///
/// ISPU output register 29 high byte (R)
#[register(address = IspuReg::IspuDout29H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout29H {
    #[bits(8)]
    pub dout29: u8,
}

/// ISPU_DOUT_30_L (0x4C)
///
/// ISPU output register 30 low byte (R)
#[register(address = IspuReg::IspuDout30L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout30L {
    #[bits(8)]
    pub dout30: u8,
}

/// ISPU_DOUT_30_H (0x4D)
///
/// ISPU output register 30 high byte (R)
#[register(address = IspuReg::IspuDout30H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout30H {
    #[bits(8)]
    pub dout30: u8,
}

/// ISPU_DOUT_31_L (0x4E)
///
/// ISPU output register 31 low byte (R)
#[register(address = IspuReg::IspuDout31L, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout31L {
    #[bits(8)]
    pub dout31: u8,
}

/// ISPU_DOUT_31_H (0x4F)
///
/// ISPU output register 31 high byte (R)
#[register(address = IspuReg::IspuDout31H, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuDout31H {
    #[bits(8)]
    pub dout31: u8,
}

/// ISPU_INT1_CTRL (0x50 - 0x53)
///
/// ISPU INT1 configuration registers (R/W)
/// These registers route 30-bit interrupt flags from ISPU_INT_STATUS registers to the INT1 pin.
/// Note: INT1_ISPU must be set to 1 to enable routing.
#[register(address = IspuReg::IspuInt1Ctrl0, access_type = IspuState, generics = 2)]
pub struct IspuInt1Ctrl(pub u32);

/// ISPU_INT1_CTRL0 (0x50)
#[register(address = IspuReg::IspuInt1Ctrl0, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt1Ctrl0 {
    #[bits(8)]
    pub ispu_int1_ctrl: u8,
}

/// ISPU_INT1_CTRL1 (0x51)
#[register(address = IspuReg::IspuInt1Ctrl1, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt1Ctrl1 {
    #[bits(8)]
    pub ispu_int1_ctrl: u8,
}

/// ISPU_INT1_CTRL2 (0x52)
#[register(address = IspuReg::IspuInt1Ctrl2, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt1Ctrl2 {
    #[bits(8)]
    pub ispu_int1_ctrl: u8,
}

/// ISPU_INT1_CTRL3 (0x53)
#[register(address = IspuReg::IspuInt1Ctrl3, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt1Ctrl3 {
    #[bits(6)]
    pub ispu_int1_ctrl: u8,
    #[bits(2, access = RO)]
    pub not_used0: u8,
}

/// ISPU_INT2_CTRL (0x54 - 0x57)
///
/// ISPU INT2 configuration registers (R/W)
/// These registers route 30-bit interrupt flags from ISPU_INT_STATUS registers to the INT2 pin.
/// Note: INT2_ISPU must be set to 1 to enable routing.
#[register(address = IspuReg::IspuInt2Ctrl0, access_type = IspuState, generics = 2)]
pub struct IspuInt2Ctrl(pub u32);

/// ISPU_INT2_CTRL0 (0x54)
#[register(address = IspuReg::IspuInt2Ctrl0, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt2Ctrl0 {
    #[bits(8)]
    pub ispu_int2_ctrl: u8,
}

/// ISPU_INT2_CTRL1 (0x55)
#[register(address = IspuReg::IspuInt2Ctrl1, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt2Ctrl1 {
    #[bits(8)]
    pub ispu_int2_ctrl: u8,
}

/// ISPU_INT2_CTRL2 (0x56)
#[register(address = IspuReg::IspuInt2Ctrl2, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt2Ctrl2 {
    #[bits(8)]
    pub ispu_int2_ctrl: u8,
}

/// ISPU_INT2_CTRL3 (0x57)
#[register(address = IspuReg::IspuInt2Ctrl3, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuInt2Ctrl3 {
    #[bits(6)]
    pub ispu_int2_ctrl: u8,
    #[bits(2, access = RO)]
    pub not_used0: u8,
}

/// ISPU_INT_STATUS (0x58 - 0x5B)
///
/// ISPU interrupt status registers (R)
/// Each register contains 8 bits of the 30-bit interrupt flags from ISPU.
#[register(address = IspuReg::IspuIntStatus0, access_type = IspuState, generics = 2)]
pub struct IspuIntStatus(pub u32);

/// ISPU_INT_STATUS0 (0x58)
#[register(address = IspuReg::IspuIntStatus0, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuIntStatus0 {
    #[bits(8)]
    pub ispu_int_status: u8,
}

/// ISPU_INT_STATUS1 (0x59)
#[register(address = IspuReg::IspuIntStatus1, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuIntStatus1 {
    #[bits(8)]
    pub ispu_int_status: u8,
}

/// ISPU_INT_STATUS2 (0x5A)
#[register(address = IspuReg::IspuIntStatus2, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuIntStatus2 {
    #[bits(8)]
    pub ispu_int_status: u8,
}

/// ISPU_INT_STATUS3 (0x5B)
#[register(address = IspuReg::IspuIntStatus3, access_type = IspuState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IspuIntStatus3 {
    #[bits(6)]
    pub ispu_int_status: u8,
    #[bits(2, access = RO)]
    pub not_used0: u8,
}

/// ISPU_ALGO (0x70 - 0x73)
///
/// ISPU algorithm enable registers (R/W)
/// Enable configurations to run up to 30 independent algorithms.
/// Each bit corresponds to an algorithm; setting bit i=1 generates IRQ for ISPU_ALGO_(i-1).
/// Bit remains set until algorithm routine completes.
#[register(address = IspuReg::IspuAlgo0, access_type = IspuState, generics = 2)]
pub struct IspuAlgo(pub u32);

/// ISPU boot latched mode
///
/// Controls ISPU boot latched mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum IspuBootLatched {
    /// ISPU boot latched mode enabled
    On = 0x0,
    /// ISPU boot latched mode disabled
    Off = 0x1,
}

/// ISPU interrupt latched mode
///
/// Configures ISPU interrupt generation mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IspuInterrupt {
    /// ISPU interrupt pulsed mode (default)
    #[default]
    Pulsed = 0x0,
    /// ISPU interrupt latched mode
    Latched = 0x1,
}

/// ISPU boot status
///
/// Indicates the end of ISPU boot procedure.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IspuBootStatus {
    /// ISPU boot in progress (default)
    #[default]
    InProgress = 0x0,
    /// ISPU boot ended
    Ended = 0x1,
}

/// ISPU memory type selection
///
/// Selects ISPU memory type for access.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum IspuMemoryType {
    /// Data RAM memory selected
    DataRamMemory = 0x0,
    /// Program RAM memory selected
    ProgramRamMemory = 0x1,
}
