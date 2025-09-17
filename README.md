# PortScan

A Rust-based port scanning tool designed for both beginners and professionals, offering customizable options from simple TCP scans to more advanced techniques.

## Features

- Basic TCP connection scanning
- Support for multiple targets (IPs or CIDR)
- Concurrent scanning using async or parallel processing
- Customizable output formats (standard, quiet, JSON)
- Modular design for easy extension and maintenance

## Requirements

- Rust stable toolchain
- Optional: sudo/root access for SYN scan support

## Table of Contents

- [PortScan](#portscan)
  - [Features](#features)
  - [Requirements](#requirements)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Basic Usage](#basic-usage)
  - [License](#license)

## Getting Started

### Installation

```bash
cargo install --git https://github.com/HuRuilizhen/portscan.git
```

### Basic Usage

```bash
portscan --target github.com --ports 1-100,3000 --timeout 500
```

To get more information, please run `portscan --help`.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
