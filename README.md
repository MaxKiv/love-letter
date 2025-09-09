# Love Letter

A shared library crate defining the communication protocol for the HHH mockloop system.

## About the Holland Hybrid Heart Project

The [Holland Hybrid Heart](https://hollandhybridheart.nl/) project aims to develop a biocompatible artificial heart using soft robotics and tissue engineering. This innovative artificial heart is designed to help heart failure patients by improving life expectancy and quality of life. It reduces the dependency on donor hearts, addressing the shortage of available donor organs and improving patient outcomes.

## What is a Mockloop?

A mockloop (also called a mock circulatory loop) is a physical simulator that replicates the human cardiovascular system for testing cardiac devices. It consists of:

- **Pumps** that simulate heart ventricles
- **Pressure chambers** representing different parts of the circulatory system
- **Flow sensors** and **pressure sensors** for monitoring
- **Controllable resistances** to simulate blood vessel properties
- **Compliance chambers** to mimic arterial elasticity

Mockloops are essential for:

- Testing artificial hearts and ventricular assist devices
- Validating control algorithms in a controlled environment
- Collecting performance data before animal or clinical trials
- Training medical professionals on the artificial heart device operation

## Purpose

This crate serves as a common interface between different components of the mockloop system. By defining shared message structures and serialization methods in a single library, we ensure consistent communication protocols across all system components while avoiding code duplication and maintaining type safety.

## Communication Protocol

The system uses UART communication with the following stack:

- **Transport**: UART at 115200 baud
- **Framing**: COBS (Consistent Overhead Byte Stuffing) encoding
- **Serialization**: Postcard (compact binary format)
- **Message Types**: `Report` and `Setpoint` structs

### COBS Encoding

COBS is a framing algorithm that eliminates zero bytes from data packets, making it ideal for UART communication where zero bytes often serve as packet delimiters. It adds minimal overhead (typically 1 byte per 254 bytes of data) while guaranteeing that encoded packets contain no zero bytes, enabling reliable packet boundaries.

## Key Dependencies

### Core Serialization

- **`serde`**: Rust's de facto serialization framework, providing `Serialize` and `Deserialize` traits
- **`postcard`**: Compact, `no_std` binary serialization format optimized for embedded systems
- **`defmt`**: Efficient logging framework for embedded systems with compile-time format string optimization

### Domain-Specific

- **`uom`**: Units of measurement library providing type-safe physical quantities (pressure, frequency, flow rates)
- **`chrono`**: Date and time handling (currently commented out but available)

## Message Structures

### `Report`

System status message containing:

- Current setpoint configuration
- Application state (StandBy/Running/Fault)
- Real-time measurements (pressures, flow rates, timestamps)

### `Setpoint`

Control commands containing:

- System enable/disable flag
- Mockloop controller parameters (resistances, compliances)
- Heart controller parameters (rate, pressure, systole ratio)

## Usage

```rust
use love_letter::{Report, Setpoint, serialize_report, deserialize_setpoint};

// Serialize a report for transmission
let mut buffer = [0u8; 256];
let encoded = serialize_report(report, &mut buffer)?;

// Deserialize received setpoint
let setpoint: Setpoint = deserialize_setpoint(&mut received_data)?;
```

## Constants

- `BAUDRATE`: 115200 (standard UART baud rate)
- `REPORT_BYTES`: Size of `Report` struct in bytes
- `SETPOINT_BYTES`: Size of `Setpoint` struct in bytes

The crate is `no_std` compatible, making it suitable for embedded microcontroller environments.
