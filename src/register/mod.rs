pub mod ispu;
pub mod main;
pub mod sensor_hub;

use super::{
    BusOperation, DelayNs, Error, Ism330is, MemBankFunctions, RegisterOperation, bisync,
    only_async, only_sync, register::ispu::IspuMemAddr,
};

use st_mem_bank_macro::mem_bank;

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
    #[state(SensorHubBank, fn_name = "operate_over_sensor_hub")]
    SensorHubMemBank = 0x2,
    /// ISPU memory bank
    #[state(IspuBank, fn_name = "operate_over_ispu")]
    IspuMemBank = 0x3,
}

#[bisync]
impl<B, T> Ism330is<B, T, IspuBank>
where
    B: BusOperation,
    T: DelayNs,
{
    pub async fn ispu_sel_memory_addr(&mut self, mem_addr: u16) -> Result<(), Error<B::Error>> {
        IspuMemAddr(mem_addr).write(self).await
    }
}
