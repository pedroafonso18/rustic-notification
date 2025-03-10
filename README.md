# Rustic Notification

![Rust Version](https://img.shields.io/badge/rust-stable-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A robust monitoring tool built in Rust that watches database instances and alerts you when proxies become inactive.

## Overview

Rustic Notification helps system administrators and DevOps engineers maintain system reliability by providing immediate notifications when critical services go offline.

## Features

- **Database Monitoring**: Seamlessly connects to PostgreSQL databases
- **Proxy Status Tracking**: Continuously checks and reports on proxy instance activity
- **Desktop Notifications**: Delivers instant alerts when issues are detected
- **Automated Checking**: Configurable periodic monitoring
- **Financial Monitoring**: Tracks GUPSHUP wallet balance to prevent service interruptions

## Installation

```bash
cargo install rustic-notification
```

## Usage

```bash
rustic-notification --config config.yml
```

## Completed Features

- ✅ Email notification support
- ✅ GUPSHUP balance monitoring
- ✅ Desktop notifications

## Roadmap

- 📋 Additional notification channels (SMS, Slack, Teams)
- 📋 Enhanced error handling and recovery
- 📋 Configurable check intervals
- 📋 Comprehensive documentation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
