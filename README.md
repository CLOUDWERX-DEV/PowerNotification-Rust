# Power Notification

**By CLOUDWERX LAB** | *Digital Food for the Analog Soul*

A lightweight Rust application that monitors AC power status and displays desktop notifications on Linux.

---

**Contact:** mail@cloudwerx.dev  
**Website:** http://cloudwerx.dev

## Features
- System tray icon that reflects power status
- Desktop notifications with sound when power is plugged/unplugged
- Right-click menu with status display and controls
- Minimal resource usage
- Silent background monitoring

## Requirements
- Linux with `/sys/class/power_supply` support
- D-Bus notification daemon
- System tray (tested on Cinnamon, should work on GNOME, KDE, XFCE)

## Installation

### Prerequisites

You'll need to install Rust if you don't have it already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then activate the Rust environment:

```bash
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

### From Source
```bash
cargo build --release
mkdir -p ~/.local/bin
cp target/release/power-notification ~/.local/bin/
```

### Auto-start on Login
Create `~/.config/autostart/power-notification.desktop`:
```ini
[Desktop Entry]
Type=Application
Name=Power Notification
Exec=/home/YOUR_USERNAME/.local/bin/power-notification
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
```
Replace `YOUR_USERNAME` with your actual username.

## Usage

Run the application:
```bash
power-notification
```

Or run in background:
```bash
power-notification &
```

### System Tray Menu
Right-click the tray icon to:
- View current power status
- Test notifications
- Exit the application

## Build from Source
```bash
cargo build --release
```

## Technical Details
- Monitors `/sys/class/power_supply` for AC adapter status changes
- Polls every 2 seconds for power state changes
- Uses D-Bus for desktop notifications
- Graceful error handling if power supply interface is unavailable
- Works without icon themes (uses generic system icons)

## License

Apache License 2.0

Copyright © 2026 CLOUDWERX LAB

For license details, see the [LICENSE](LICENSE) file.
