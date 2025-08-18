pub mod ispu;
pub mod main;
pub mod sensor_hub;

use crate::Error;
use crate::Ism330is;
use embedded_hal::delay::DelayNs;
use ispu::IspuMemAddr;
use st_mem_bank_macro::mem_bank;
use st_mems_bus::{BusOperation, MemBankFunctions};

/// Memory bank selection for register access
///
/// Main memory bank, sensor hub memory bank, or ISPU memory bank.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[mem_bank(Ism330is, generics = 2)]
pub enum MemBank {
    /// Main memory bank
    #[main]
    MainMemBank = 0x0,
    /// Sensor hub memory bank
    #[state(SensorHubState, fn_name = "operate_over_sensor_hub")]
    SensorHubMemBank = 0x2,
    /// ISPU memory bank
    #[state(IspuState, fn_name = "operate_over_ispu")]
    IspuMemBank = 0x3,
}

impl<B, T> IspuState<'_, B, T>
where
    B: BusOperation,
    T: DelayNs,
{
    pub fn ispu_sel_memory_addr(&mut self, mem_addr: u16) -> Result<(), Error<B::Error>> {
        IspuMemAddr(mem_addr).write(self)
    }
}
