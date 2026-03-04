# Examples for ISM330IS Sensor

This project abstracts board-specific details, enabling easy selection and setup of supported boards and frameworks while keeping examples consistent.

## Supported Boards and Frameworks

| Board-id               | stm32-rs Framework | Embassy Framework |
|------------------------|--------------------|-------------------|
| nucleo-f401re          | ✓                  |                   |
| nucleo-f401re-embassy  |                    | ✓                 |

## Quickstart

To run an example, first select the appropriate board-id from the table above. Then execute the example with Cargo by specifying the example as a feature.

### Example command syntax:

```sh
cargo <board_id> --features <example_name>
```

### Example usage

Run the *read_irq* example on the Nucleo F401RE using the stm32-rs framework:
```sh
cargo nucleo-f401re --features read_irq
```

Run the same example using the Embassy async framework:
```sh
cargo nucleo-f401re-embassy --features read_irq
```

## Board Configuration
Board-specific configurations such as pins for I2C (SDA, SCL), UART, and interrupt lines are abstracted via macros in the BSP source code. These macros simplify adapting examples to different hardware setups.

### Configuration Macros in bsp/src/main.rs
For the **Embassy** framework (**nucleo-f401re-embassy** board-id):
```rust
define_embassy_with_st_link! {
    i2c = {
        address: I2CAddress::I2cAddL as u8,
        periph: I2C1,
        scl: PB8,
        sda: PB9,
        ev_irq: I2C1_EV,
        er_irq: I2C1_ER,
        dma_tx: DMA1_CH7,
        dma_rx: DMA1_CH0,
    },
    uart = {
        periph: USART2,
        tx: PA2,
        dma_tx: DMA1_CH6,
        baud: 115200,
    },
    int_pin = {
        pin: PC0,
        exti_line: EXTI0,
        exti_mux: EXTI0,
    }
}
```
For the **stm32-rs** HAL framework (**nucleo-f401re** board-id):
```rust
define_stm32_rs_with_st_link!(
    i2c = {
        address: I2CAddress::I2cAddL as u8,
        periph: I2C1,
        scl: (port_b, pb8),
        sda: (port_b, pb9),
    },
    uart = {
        periph: USART2,
        tx: (port_a, pa2),
    },
    interrupt = {
        pin: (port_c, pc0),
        exti_irq: EXTI0,
    }
);
```
*Note*:

- The sensor I2C address can be changed depending on the SD0 pin state.

## Available Examples
Examples are located in the [src/examples](src/examples) directory. Below is a list of available examples:

- ispu_norm.rs — Norm computed inside the ISPU.
- read_irq.rs — Data reading using interrupts.
- read_polling.rs — Data reading using polling.
- self_test.rs — Sensor self-test example.
- sensor_hub.rs — Sensor hub example.

## FSM and MLC Configuration Files

Some examples use configuration files for the Finite State Machine (FSM) and Machine Learning Core (MLC). These JSON files are located in [src/config](src/config):

- norm.json

These configuration files are automatically included and converted to Rust code for examples that require them, via the [build.rs](build.rs) build script.
