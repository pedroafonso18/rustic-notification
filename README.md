# Rustic Notification

![Rust Version](https://img.shields.io/badge/rust-stable-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

A robust monitoring tool built in Rust that watches database instances, campaign numbers, and alerts you when proxies become inactive or when wallet balances are running low.

## Overview

Rustic Notification helps system administrators and DevOps engineers maintain system reliability by providing immediate notifications when critical services go offline, campaigns run low on numbers, or wallet balances are running low.

## Features

- **Database Monitoring**: Seamlessly connects to PostgreSQL databases
- **Proxy Status Tracking**: Continuously checks and reports on proxy instance activity
- **Campaign Monitoring**: Tracks remaining campaign numbers and alerts when running low
- **Desktop Notifications**: Delivers instant alerts when issues are detected
- **Email Alerts**: Sends email notifications for critical conditions
- **WhatsApp Notifications**: Sends alerts via WhatsApp API for immediate attention
- **Financial Monitoring**: Tracks GUPSHUP wallet balance to prevent service interruptions
- **Automated Checking**: Periodic monitoring with configurable intervals

## Configuration

Create a `.env` file with the following variables:

```
DATABASE_URL=postgres://user:password@localhost/dbname
CAMPANHAS_URL=postgres://user:password@localhost/campaigns_db
APIKEY=your_gupshup_api_key
GOOGLE_KEY=your_google_app_password
EMAIL=your_notification_email@gmail.com
EVO_APIKEY=your_whatsapp_api_key
EVO_URL=https://api.whatsapp.service/endpoint
NUM_SEND_TO=whatsapp_recipient_number
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
- ✅ WhatsApp notification integration
- ✅ Campaign number monitoring

## Roadmap

- 📋 Additional notification channels (SMS, Slack, Teams)
- 📋 Enhanced error handling and recovery
- 📋 Configurable check intervals via config file
- 📋 Comprehensive documentation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.