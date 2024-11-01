# System-Watchdog

```
                                  _       _         _             
 ___ _   _ ___     __      ____ _| |_ ___| |__   __| | ___   __ _ 
/ __| | | / __|____\ \ /\ / / _` | __/ __| '_ \ / _` |/ _ \ / _` |
\__ \ |_| \__ \_____\ V  V / (_| | || (__| | | | (_| | (_) | (_| |
|___/\__, |___/      \_/\_/ \__,_|\__\___|_| |_|\__,_|\___/ \__, |
     |___/                                                  |___/ 
```

System-Watchdog aka sys-watchdog. This Rust program monitors CPU usage, RAM and network interface speed in real-time, updating and displaying the data in the console.

## Features
- Real-time CPU monitoring
- Monitoring of RAM usage
- Tracking of network traffic (incoming/outgoing)
- Automatic updates every second
- Command-line interface with readable formatting

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

### 1. Install Rust and Cargo
```bash
# Install rustup (Rust installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rust-lang.org | sh

# Verify installation
rustc --version
cargo --version
```

### 2. Clone the Repository
```bash
# Using HTTPS
git clone https://github.com/alalvarez29/sys-watchdog.git

# Using SSH
git clone git@github.com:alalvarez29/sys-watchdog.git

# Navigate to project directory
cd sys-watchdog
```

### 3. Build the Project
```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### 4. Run the Program
```bash
# Run development version
cargo run

# Run release version
./target/release/sys-watchdog
```

## Platform-Specific Instructions

### Linux
```bash
# Install build essentials
sudo apt update
sudo apt install build-essential

# Install required system libraries
sudo apt install libssl-dev pkg-config
```

### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew (if needed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Windows
```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.BuildTools
```

## Permissions

### Linux
```bash
# If needed, add user to necessary groups for system monitoring
sudo usermod -a -G systemd-journal $USER
sudo usermod -a -G netdev $USER
```

### macOS
- System Monitoring permissions might need to be granted in System Preferences

### Windows
- Run as Administrator might be required for full functionality

## Conclusions

1. **Design Efficiency**
   - The use of Rust ensures optimal performance and memory safety
   - Modular structure allows for easy code maintainability
   - The program achieves real-time monitoring with minimal system impact

2. **Robust Functionality**
   - Effective monitoring of critical metrics (CPU, RAM, Network)
   - Real-time updates without screen flicker
   - Intelligent data formatting for better readability
   - Efficient handling of multiple network interfaces

3. **Technical Implementation**
   - Effective use of data structures (HashMap for interfaces)
   - Proper error handling using Result
   - Consistent metric updates every second
   - Clean and well-organized code
