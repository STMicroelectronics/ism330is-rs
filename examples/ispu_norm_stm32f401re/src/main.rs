#![no_std]
#![no_main]

mod ispu_norm;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use panic_halt as _;
use panic_halt as _;
use stm32f4xx_hal::{
    gpio::{self, Edge, Input},
    i2c::{DutyCycle, I2c, Mode},
    pac::{self, interrupt},
    prelude::*,
    serial::{config::Config, Serial},
};

use ism330is_rs::*;
use ispu_norm::NORM_PROGRAM as NORM;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

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
    int_pin.enable_interrupt(&mut dp.EXTI);

    // Enable the external interrupt in the NVIC by passing the interrupt number
    unsafe {
        cortex_m::peripheral::NVIC::unmask(int_pin.interrupt());
    }

    // Now that pin is configured, move pin into global context
    cortex_m::interrupt::free(|cs| {
        INT_PIN.borrow(cs).replace(Some(int_pin));
    });

    delay.delay_ms(5);
    let mut sensor = Ism330is::new_i2c(i2c, I2CAddress::I2cAddL, tim1);

    // Check device ID
    match sensor.device_id_get() {
        Ok(id) => {
            if id != ism330is_rs::ID {
                loop {}
            }
        }
        Err(e) => writeln!(tx, "Error in reading id: {:?}", e).unwrap(),
    }

    // Restore default configuration
    sensor.software_reset().unwrap();

    // Load ISPU configuration
    for ucf_entry in NORM {
        match ucf_entry.op {
            MemsUcfOp::Delay => {
                delay.delay_ms(ucf_entry.data.into());
            }
            MemsUcfOp::Write => {
                sensor
                    .write_to_register(ucf_entry.address, &[ucf_entry.data])
                    .unwrap();
            }
            _ => {}
        }
    }

    let ispu_odr = sensor.ispu_data_rate_get().unwrap();
    writeln!(tx, "ISPU started: at rate {}", ispu_odr.to_str()).unwrap();

    let mut dout: [u8; 10] = [0; 10];
    let mut x: i16;
    let mut y: i16;
    let mut z: i16;
    let mut temp: u32;
    let mut norm: f32;

    loop {
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

        let ispu_int = sensor.ispu_int_status_get().unwrap();

        if (ispu_int & 0x1) == 0 {
            continue;
        }

        sensor.ispu_read_data_raw_get(&mut dout, 10).unwrap();

        x = dout[1] as i16;
        x = (x << 8) + (dout[0] as i16);
        y = dout[3] as i16;
        y = (y << 8) + (dout[2] as i16);
        z = dout[5] as i16;
        z = (z << 8) + (dout[4] as i16);

        temp = dout[9] as u32;
        temp = (temp << 8) + dout[8] as u32;
        temp = (temp << 8) + dout[7] as u32;
        temp = (temp << 8) + dout[6] as u32;
        norm = f32::from_bits(temp);
        writeln!(tx, "x: {}\ty: {}\tz: {}\tnorm: {:.2}", x, y, z, norm).unwrap();
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
