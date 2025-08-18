use crate::IspuMemAddr;
use embedded_hal::delay::DelayNs;
use bus::BusOperation;
use crate::RegisterAccess;
use crate::Ism330is;


#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum MemBank {
    MainMemBank = 0x0,
    SensorHubMemBank = 0x2,
    IspuMemBank = 0x3,
}

impl MemBank {

    pub fn operate_over_sensor_hub<B, T, F, R>(sensor: &mut Ism330is<B, T>, f: F) -> Result<R, crate::Error<B::Error>>
        where B: BusOperation, T: DelayNs, F: FnOnce(&mut SensorHubMemBankLock<B, T>) -> Result<R, crate::Error<B::Error>> {

        sensor.mem_bank_set(MemBank::SensorHubMemBank)?;
        let mut lock = SensorHubMemBankLock { sensor };
        let res = f(&mut lock);
        sensor.mem_bank_set(MemBank::MainMemBank)?;

        res
    }

    pub fn operate_over_ispu<B, T, F, R>(sensor: &mut Ism330is<B, T>, f: F) -> Result<R, crate::Error<B::Error>>
        where B: BusOperation, T: DelayNs, F: FnOnce(&mut IspuMemBankLock<B, T>) -> Result<R, crate::Error<B::Error>> {

        sensor.mem_bank_set(MemBank::IspuMemBank)?;
        let mut lock = IspuMemBankLock { sensor };
        let res = f(&mut lock);
        sensor.mem_bank_set(MemBank::MainMemBank)?;

        res
    }
}

pub struct SensorHubMemBankLock<'a, B, T> 
    where B: BusOperation, T: DelayNs
{
    sensor: &'a mut Ism330is<B, T>
}

pub struct IspuMemBankLock<'a, B, T> 
    where B: BusOperation, T: DelayNs
{
    sensor: &'a mut Ism330is<B, T>
}

impl<B, T> SensorHubMemBankLock<'_, B, T> where B: BusOperation, T: DelayNs {
    pub fn write_to_register(&mut self, reg: u8, buf: &[u8]) -> Result<(), crate::Error<B::Error>> {
        self.sensor.write_to_register(reg, buf)
    }

    pub fn read_from_register(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), crate::Error<B::Error>> {
        self.sensor.read_from_register(reg, buf)
    }
}

impl<B, T> IspuMemBankLock<'_, B, T> where B: BusOperation, T: DelayNs {
    pub fn write_to_register(&mut self, reg: u8, buf: &[u8]) -> Result<(), crate::Error<B::Error>> {
        self.sensor.write_to_register(reg, buf)
    }

    pub fn read_from_register(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), crate::Error<B::Error>> {
        self.sensor.read_from_register(reg, buf)
    }

    pub(crate) fn ispu_sel_memory_addr(&mut self, mem_addr: u16) -> Result<(), crate::Error<B::Error>> {
        IspuMemAddr(mem_addr).write(self)
    }
}


