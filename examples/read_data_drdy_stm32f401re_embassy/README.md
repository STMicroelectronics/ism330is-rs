# ISM330IS Accelerometer, Gyroscope, and Temperature Data Acquisition on STM32F401RE Nucleo (Embassy Executor)

This example demonstrates how to configure and read accelerometer, gyroscope, and temperature data from the **ISM330IS** inertial measurement unit (IMU) sensor using an **STM32F401RE** microcontroller board with the **Embassy** asynchronous executor framework. The sensor is configured to generate data-ready signals for accelerometer and gyroscope, and the program reads and outputs the sensor data over UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** ISM330IS 3-axis accelerometer and 3-axis gyroscope with temperature sensor
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PC0 configured as input with external interrupt for data-ready signals

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from sensor data-ready signals |

The ISM330IS sensor is connected to the STM32F401RE via the I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The sensor's data-ready interrupt line is connected to PC0, configured to trigger an external interrupt on the rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes STM32F401RE peripherals including clocks, GPIO pins, I2C, and UART using the Embassy HAL.
- I2C1 is configured for 100 kHz Standard Mode with open-drain pins PB8 and PB9.
- USART2 is configured on PA2 at 115200 baud for serial output.
- PC0 is configured as an input pin with external interrupt capability.
- The external interrupt line is enabled in the NVIC and linked to the EXTI0 interrupt handler.

### Sensor Configuration

- The ISM330IS sensor is initialized over I2C with the low I2C address.
- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration.
- Data-ready signals for accelerometer and gyroscope are routed to INT1 pin (PC0).
- Block Data Update (BDU) is recommended but commented out in this example.
- Accelerometer and gyroscope output data rates are set to 12.5 Hz in high-performance mode.
- Accelerometer full scale is set to ±2g.
- Gyroscope full scale is set to ±2000 dps.
- Filtering chain configuration is available but disabled by default.

### Data Acquisition Loop

- The main async task waits asynchronously for rising edge interrupts on the data-ready pin.
- When new accelerometer data is available, it is read, converted to mg, and printed over UART.
- When new gyroscope data is available, it is read, converted to mdps, and printed over UART.
- When new temperature data is available, it is read, converted to degrees Celsius, and printed over UART.
- UART writes are performed in a blocking manner; DMA-based asynchronous UART transmission is not implemented.

### Interrupt Handler

- The `EXTI0` interrupt handler clears the interrupt pending bit on PC0 to allow further interrupts.

---

## Usage

1. Connect the ISM330IS sensor to the STM32F401RE Nucleo board via I2C on pins PB8 (SCL) and PB9 (SDA).
2. Connect the sensor's data-ready interrupt line to PC0 on the STM32F401RE.
3. Build and flash the Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud on the USART2 TX pin (PA2).
5. Observe accelerometer, gyroscope, and temperature data printed continuously over UART.

---

## Notes

- The example uses polling of data-ready flags combined with interrupt-driven low-power wait.
- Block Data Update (BDU) is recommended to ensure data consistency during reads.
- Filtering chain configuration is available but disabled by default.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to use `panic_probe` for debugging.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [ISM330IS Datasheet](https://www.st.com/resource/en/datasheet/ism330is.pdf)
- [Embassy Embedded Rust Framework](https://embassy.dev/)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README provides a detailed explanation of the embedded Rust program for accelerometer, gyroscope, and temperature data acquisition on STM32F401RE using the ISM330IS sensor and Embassy framework.*
