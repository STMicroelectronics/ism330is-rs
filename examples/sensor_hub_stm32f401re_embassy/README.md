# ISM330IS Sensor Hub with LIS2MDL and LPS22DF on STM32F401RE Nucleo (Embassy Executor)

This example demonstrates how to use the **ISM330IS** inertial measurement unit (IMU) as a sensor hub to interface with external sensors—**LIS2MDL** magnetometer and **LPS22DF** barometer—on an **STM32F401RE** microcontroller board. The ISM330IS reads data from these external sensors via its embedded sensor hub feature and streams combined data through its FIFO buffer.

The program configures the ISM330IS and external sensors, sets up FIFO streaming with watermark interrupts, and outputs sensor data including accelerometer, magnetometer, barometer, and timestamps over UART.

The code is written in Rust using the Embassy embedded framework, the `embassy-stm32` hardware abstraction layer, and the `ism330is`, `lis2mdl`, and `lps22df` sensor driver crates.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensors:**
  - ISM330IS 6-axis IMU with embedded sensor hub
  - LIS2MDL 3-axis magnetometer (connected via ISM330IS sensor hub)
  - LPS22DF barometer (connected via ISM330IS sensor hub)
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PC0 configured as input with external interrupt for FIFO watermark

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from FIFO watermark |

The ISM330IS sensor is connected to the STM32F401RE via the I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The LIS2MDL and LPS22DF sensors are connected to the ISM330IS sensor hub internally. The FIFO watermark interrupt line is connected to PC0, configured to trigger an external interrupt on the rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes STM32F401RE peripherals including clocks, GPIO pins, I2C, and UART using the Embassy HAL.
- I2C1 is configured for 100 kHz Standard Mode with open-drain pins PB8 and PB9.
- USART2 is configured on PA2 at 115200 baud for serial output.
- PC0 is configured as an input pin with interrupt capability on rising edge.
- The external interrupt line is enabled in the NVIC and linked to the EXTI0 interrupt handler.
- The interrupt pin is stored in a global mutex-protected static to safely clear interrupt flags.

### Sensor Hub and External Sensors Configuration

- The ISM330IS sensor is initialized over I2C.
- The LIS2MDL magnetometer and LPS22DF barometer are initialized via the ISM330IS sensor hub interface.
- Device IDs of all sensors are verified.
- Sensors are reset to default configurations and configured for continuous measurement modes:
  - LIS2MDL set to continuous mode at 20 Hz.
  - LPS22DF configured with 4 Hz ODR, averaging, and low-pass filtering.
- The ISM330IS FIFO watermark is set to 64 samples.
- FIFO batching is configured to include accelerometer data at 26 Hz.
- FIFO mode is set to Stream Mode (continuous).
- FIFO watermark interrupt is routed to INT1 pin.
- Sensor hub data rate is set to 52 Hz.
- Sensor hub is configured to read two slaves (LIS2MDL and LPS22DF).
- Sensor hub write mode is set to write once.
- I2C master is enabled.
- Accelerometer output data rate is set to 26 Hz and then disabled (likely for sensor hub operation).

### Data Acquisition Loop

- The program waits for FIFO watermark interrupts.
- When FIFO watermark is reached, it reads all FIFO samples.
- Accelerometer data is read and converted to mg.
- Sensor hub data is read and parsed:
  - LIS2MDL magnetometer data converted to milliGauss.
  - LPS22DF barometer pressure converted to hPa and temperature to degrees Celsius.
- All sensor data are printed over UART.
- The program waits for interrupt (`wfi`) to reduce power consumption.

### Interrupt Handler

- The `EXTI0` interrupt handler clears the interrupt pending bit on PC0 to allow further interrupts.

### Optional Custom Sensor Hub Wrapper

- The code includes commented-out Rust wrapper structs and implementations to customize sensor hub operations, including one-shot read/write and continuous reading with proper synchronization and delays.

---

## Usage

1. Connect the ISM330IS sensor and external sensors (LIS2MDL, LPS22DF) to the STM32F401RE Nucleo board.
2. Connect the FIFO watermark interrupt line to PC0 on the STM32F401RE.
3. Build and flash the Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud on the USART2 TX pin (PA2).
5. Observe combined sensor data streamed via FIFO and printed over UART.

---

## Notes

- This example uses the ISM330IS sensor hub feature to read external sensors transparently.
- FIFO watermark interrupts enable efficient batch reading of sensor and sensor hub data.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`panic_halt`).
- UART writes are blocking; asynchronous DMA-based UART transmission is not implemented.

---

## Application Note Integration

This example is based on the **AN5850** application note by STMicroelectronics, which provides detailed information about the ISM330IS device, including:

- **ISPU (Intelligent Sensor Processing Unit):** An embedded programmable core for real-time sensor data processing, supporting advanced algorithms and machine learning.
- **Sensor Hub Mode:** Allows connection of up to four external sensors via the I²C master interface, with configurable trigger signals and data batching.
- **Power Modes and Data Rates:** Detailed configuration options for accelerometer and gyroscope power modes, output data rates, and filtering chains.
- **Memory Mapping and Program Loading:** Procedures for loading ISPU programs into device RAM and managing ISPU execution.
- **Interrupt Handling:** Mechanisms for efficient interrupt-driven processing and power management.
- **Temperature Sensor:** Internal temperature sensor with configurable data rates and data-ready signals.
- **Self-Test Functions:** Embedded self-test procedures for accelerometer and gyroscope to verify sensor functionality.

Refer to the AN5850 document for comprehensive details on device features, registers, and usage recommendations.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [ISM330IS Datasheet](https://www.st.com/resource/en/datasheet/ism330is.pdf)
- [LIS2MDL Datasheet](https://www.st.com/resource/en/datasheet/lis2mdl.pdf)
- [LPS22DF Datasheet](https://www.st.com/resource/en/datasheet/lps22df.pdf)
- [Embassy Embedded Rust Framework](https://embassy.dev/)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)
- [Application Note: AN5850](https://www.st.com/resource/en/application_note/an5850-ism330is-alwayson-3axis-accelerometer-and-3axis-gyroscope-with-ispu--intelligent-sensor-processing-unit-stmicroelectronics.pdf)

---

*This README provides a detailed explanation of the embedded Rust program for sensor hub data acquisition on STM32F401RE using the ISM330IS sensor with LIS2MDL and LPS22DF external sensors, integrating insights from the STMicroelectronics AN5850 application note.*
