# ISM330IS ISPU Normalization Data Streaming on STM32F401RE Nucleo (Embassy Executor)

This example demonstrates how to configure and read **ISPU (Intelligent Sensor Processing Unit)** normalization data from the **ISM330IS** inertial measurement unit (IMU) sensor using an **STM32F401RE** microcontroller board with the **Embassy** asynchronous executor framework. The ISPU block is a programmable core embedded in the sensor that processes raw accelerometer and gyroscope data to provide normalized outputs and other advanced processing results.

The program loads a predefined ISPU configuration (UCF file), initializes the sensor, and continuously reads normalized data samples upon interrupt, outputting the results via UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** ISM330IS IMU with embedded ISPU normalization block
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PC0 configured as input with external interrupt for ISPU data ready

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from ISPU data ready |

The ISM330IS sensor is connected to the STM32F401RE via the I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The sensor's ISPU data ready interrupt line is connected to PC0, configured to trigger an external interrupt on the rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes STM32F401RE peripherals including clocks, GPIO pins, I2C, and UART using the Embassy HAL.
- I2C1 is configured for 100 kHz Standard Mode with open-drain pins PB8 and PB9.
- USART2 is configured on PA2 at 115200 baud for serial output.
- PC0 is configured as an input pin with external interrupt capability.
- The external interrupt line is enabled in the NVIC and linked to the EXTI0 interrupt handler.

### ISPU Configuration

- The ISM330IS sensor is initialized over I2C with the low I2C address.
- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration.
- The ISPU normalization program is loaded from a UCF-generated Rust array (`NORM`).
- The ISPU data rate is read and printed over UART.

### Data Acquisition Loop

- The main async task waits asynchronously for rising edge interrupts on the ISPU data ready pin.
- When an ISPU data ready interrupt occurs, the program reads the ISPU interrupt status.
- If new ISPU data is available, it reads 10 bytes of raw ISPU data.
- The raw data is parsed into X, Y, Z components and a 32-bit floating-point normalization value.
- The normalized data is printed over UART.
- UART writes are performed in a blocking manner; DMA-based asynchronous UART transmission is not implemented.

### Interrupt Handler

- The `EXTI0` interrupt handler clears the interrupt pending bit on PC0 to allow further interrupts.

---

## Usage

1. Connect the ISM330IS sensor to the STM32F401RE Nucleo board via I2C on pins PB8 (SCL) and PB9 (SDA).
2. Connect the sensor's ISPU data ready interrupt line to PC0 on the STM32F401RE.
3. Build and flash the Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud on the USART2 TX pin (PA2).
5. Observe normalized ISPU data printed continuously over UART.

---

## Notes

- The ISPU is a programmable 32-bit RISC core embedded in the sensor for advanced processing.
- The ISPU program must be loaded at each power-up; no onboard non-volatile memory is available.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`panic_probe`).
- The example uses the `libm` crate for floating-point math functions.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [ISM330IS Datasheet](https://www.st.com/resource/en/datasheet/ism330is.pdf)
- [Embassy Embedded Rust Framework](https://embassy.dev/)
- [Application Note: AN5850](https://www.st.com/resource/en/application_note/an5850-ism330is-alwayson-3axis-accelerometer-and-3axis-gyroscope-with-ispu--intelligent-sensor-processing-unit-stmicroelectronics.pdf)

---

*This README provides a detailed explanation of the embedded Rust program for ISPU normalization data streaming on STM32F401RE using the ISM330IS sensor and Embassy framework.*
