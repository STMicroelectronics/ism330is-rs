use defmt::info;
use maybe_async::maybe_async;
use crate::*;

use crate::config::ispu_config::NORM;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, mut delay: D, mut int_pin : I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
{
    use ism330is::*;

    info!("Configuring the sensor");
    let mut sensor = Ism330is::from_bus(bus, delay.clone());

    // boot time
    delay.delay_ms(5).await;

    // Check device ID
    let id = sensor.device_id_get().await.unwrap();
    info!("Device ID: {:x}", id);
    if id != ID {
        info!("Unexpected device ID: {:x}", id);
        writeln!(tx, "Unexpected device ID: {:x}", id).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.software_reset().await.unwrap();

    // Load ISPU configuration
    for ucf_entry in NORM {
        match ucf_entry.op {
            MemsUcfOp::Delay => {
                delay.delay_ms(ucf_entry.data.into()).await;
            }
            MemsUcfOp::Write => {
                sensor
                    .bus
                    .write_to_register(ucf_entry.address, &[ucf_entry.data])
                    .await.unwrap();
            }
            _ => {}
        }
    }

    let ispu_odr = sensor.ispu_data_rate_get().await.unwrap();
    writeln!(tx, "ISPU started: at rate {}", ispu_odr.to_str()).unwrap();

    let mut dout: [u8; 10] = [0; 10];
    let mut x: i16;
    let mut y: i16;
    let mut z: i16;
    let mut temp: u32;
    let mut norm: f32;

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let ispu_int = sensor.ispu_int_status_get().await.unwrap();

        if (ispu_int & 0x1) == 0 {
            continue;
        }

        sensor.ispu_read_data_raw_get(&mut dout, 10).await.unwrap();

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
