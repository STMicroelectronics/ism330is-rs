#![no_std]
#![no_main]

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use ism330is_rs::prelude::*;
use ism330is_rs::*;
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

    let mut val = sensor.pin_int1_route_get().unwrap();
    val.drdy_xl = 1;
    val.drdy_gy = 1;
    sensor.pin_int1_route_set(val).unwrap();

    /* Enable Block Data Update */
    //ism330is_rs_block_data_update_set(1).unwrap();
    /* Set Output Data Rate */
    sensor.xl_data_rate_set(XlDataRate::_12_5hzHp).unwrap();
    sensor.gy_data_rate_set(GyDataRate::_12_5hzHp).unwrap();

    /* Set full scale */
    sensor.xl_full_scale_set(XlFullScale::_2g).unwrap();
    sensor.gy_full_scale_set(GyFullScale::_2000dps).unwrap();

    /* Configure filtering chain(No aux interface)
     * Accelerometer - LPF1 + LPF2 path
     */
    //ism330is_rs_xl_hp_path_on_out_set(&dev_ctx, LSM6DSO16IS_LP_ODR_DIV_100);
    //ism330is_rs_xl_filter_lp2_set(&dev_ctx, PROPERTY_ENABLE);

    /* Read samples */
    let mut acc_mg = [0f32; 3];
    let mut angular_rate_mdps = [0f32; 3];

    loop {
        let flag = sensor.xl_flag_data_ready_get().unwrap();

        if flag != 0 {
            let data_raw_acceleration = sensor.acceleration_raw_get().unwrap();

            for i in 0..3 {
                acc_mg[i] = ism330is_rs::from_fs2g_to_mg(data_raw_acceleration[i]);
            }

            writeln!(
                tx,
                "Acceleration [mg]: {:4.2}\t{:4.2}\t{:4.2}",
                acc_mg[0], acc_mg[1], acc_mg[2]
            )
            .unwrap();
        }

        let flag = sensor.gy_flag_data_ready_get().unwrap();

        if flag != 0 {
            let data_raw_angular_rate = sensor.angular_rate_raw_get().unwrap();

            for i in 0..3 {
                angular_rate_mdps[i] =
                    ism330is_rs::from_fs2000dps_to_mdps(data_raw_angular_rate[i]);
            }

            writeln!(
                tx,
                "Angular rate [mdps]: {:4.2}\t{:4.2}\t{:4.2}",
                angular_rate_mdps[0], angular_rate_mdps[1], angular_rate_mdps[2]
            )
            .unwrap();
        }

        let flag = sensor.temp_flag_data_ready_get().unwrap();

        if flag != 0 {
            let data_raw_temperature = sensor.temperature_raw_get().unwrap();
            let temperature_deg_c = ism330is_rs::from_lsb_to_celsius(data_raw_temperature);

            writeln!(tx, "Temperature [degC]: {:6.2}", temperature_deg_c).unwrap();
        }

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
