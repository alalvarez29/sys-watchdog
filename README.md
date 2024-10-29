# System-Watchdog

System-Watchdog aka sys-watchdog. This Rust program monitors CPU usage, RAM and network interface speed in real-time, updating and displaying the data in the console.

## Contents

- [Dependencies](#dependencies)
    - [sysinfo](#sysinfo)
- [Architecture](#architecture)
    - [NetworkInterface struct](#networkinterface-struct)
    - [SystemMetrics struct](#systemmetrics-struct)
- [Functions](#functions)
    - [main](#main)
    - [update](#update)
    - [display](#display)
    - [format_bytes](#format_bytes)
- [Real-time monitoring](#real-time-monitoring)
- [Usage](#usage)
- [Conclusions](#conclusions)

## Dependencies

### sysinfo

`sysinfo` is a Rust crate used to get a system's information which currently supports all the known operating systems.

## Architecture

This section describes the main components of sys-watchdog.

### NetworkInterface struct

NetworkInterface struct tracks and calculate the current speed of data transmission and reception for each network interface on the system, allowing for real-time monitoring of network performance.

### SystemMetrics struct

SystemMetrics struct provides a structured way to collect, store, and update system performance data, making it easier to monitor and display this information in real time.

## Functions

This section describes the key functions used by sys-watchdog

### main

The `main()` function essentially coordinates the entire monitoring process, handling the setup, continuous updating of metrics, and displaying the results in a user-friendly format.

### update

The `update()` function effectively gathers and refreshes real-time data on system performance. It ensures that the metrics displayed to the user are current and reflect the actual state of the system's resources.

### display

The `display()` function provides a real-time visualization of critical system performance metrics, including CPU, RAM, and network interface speed, enabling users to quickly assess the status of their system's resources.

### format_bytes

The `format_bytes()` function makes it easier for users to understand network speeds or data sizes by converting raw byte counts into a more digestible format. This enhances the usability of the program by providing clear and concise performance metrics.

## Real-time monitoring

## Usage

## Conclusions
