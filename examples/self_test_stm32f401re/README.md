# ISM330IS Accelerometer and Gyroscope Self-Test on STM32F401RE Nucleo

This example demonstrates how to perform **self-test** procedures for the accelerometer and gyroscope sensors embedded in the **ISM330IS** inertial measurement unit (IMU) using an **STM32F401RE** microcontroller board. The self-test applies electrostatic test forces to the sensors and verifies the output changes against expected ranges to ensure sensor functionality.

The program performs both **positive** and **negative** self-tests for each sensor, averages multiple samples, compares the results against defined thresholds, and reports pass/fail status over UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** ISM330IS 3-axis accelerometer and 3-axis gyroscope
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |

The ISM330IS sensor is connected to the STM32F401RE via the I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes STM32F401RE peripherals including clocks, GPIO pins, I2C, and UART.
- I2C1 is configured for 100 kHz Standard Mode with open-drain pins PB8 and PB9.
- USART2 is configured on PA2 at 115200 baud for serial output.

### Self-Test Procedure

- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration.
- For each sensor (accelerometer and gyroscope), the program performs:
  - **Positive self-test:**
    - Enables sensor with BDU enabled, appropriate full scale, and ODR.
    - Waits for sensor stabilization.
    - Averages 5 samples of sensor output (no self-test).
    - Enables positive self-test mode.
    - Waits for stabilization.
    - Averages 5 samples of sensor output (self-test active).
    - Compares the difference between self-test and no-self-test averages against defined thresholds.
    - Reports pass or fail over UART.
  - **Negative self-test:**
    - Similar procedure with negative self-test mode enabled.
- After each self-test, the sensor is disabled or reset as needed.

### Helper Functions

- `avg_5_xl_samples` and `avg_5_gy_samples` read and average 5 samples from accelerometer and gyroscope respectively, waiting for data-ready flags.

### Result Reporting

- Pass or fail results for each test and axis are printed over UART with indication of positive or negative test.

---

## Usage

1. Connect the ISM330IS sensor to the STM32F401RE Nucleo board via I2C on pins PB8 (SCL) and PB9 (SDA).
2. Build and flash the Rust firmware onto the STM32F401RE.
3. Open a serial terminal at 115200 baud on the USART2 TX pin (PA2).
4. Observe self-test results for accelerometer and gyroscope printed over UART.

---

## Notes

- The self-test thresholds are based on STMicroelectronics specifications for acceptable sensor response.
- The program uses blocking polling for data-ready flags.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`panic_halt`).
- The example uses the `libm` crate for floating-point absolute value function.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [ISM330IS Datasheet](https://www.st.com/resource/en/datasheet/ism330is.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README provides a detailed explanation of the embedded Rust program for accelerometer and gyroscope self-test on STM32F401RE using the ISM330IS sensor.*
