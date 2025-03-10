# Rustic Notification

![Rust Version](https://img.shields.io/badge/rust-stable-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A robust monitoring tool built in Rust that watches database instances and alerts you when proxies become inactive, and monitors GUPSHUP wallet balance.

## Overview

Rustic Notification helps system administrators and DevOps engineers maintain system reliability by providing immediate notifications when critical services go offline and when wallet balances are running low.

## Features

- **Database Monitoring**: Seamlessly connects to PostgreSQL databases
- **Proxy Status Tracking**: Continuously checks and reports on proxy instance activity
- **Desktop Notifications**: Delivers instant alerts when issues are detected
- **Email Alerts**: Sends email notifications for low balance conditions
- **Financial Monitoring**: Tracks GUPSHUP wallet balance to prevent service interruptions
- **Automated Checking**: Periodic monitoring with configurable intervals

## Configuration

Create a `.env` file with the following variables:

```
DATABASE_URL=postgres://user:password@localhost/dbname
APIKEY=your_gupshup_api_key
GOOGLE_KEY=your_google_app_password
EMAIL=your_notification_email@gmail.com
```

## Usage

```bash
cargo run
```

## Completed Features

- ✅ Email notification support
- ✅ GUPSHUP balance monitoring
- ✅ Desktop notifications
- ✅ Proxy status monitoring

## Roadmap

- 📋 Additional notification channels (SMS, Slack, Teams)
- 📋 Enhanced error handling and recovery
- 📋 Configurable check intervals via config file
- 📋 Comprehensive documentation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
