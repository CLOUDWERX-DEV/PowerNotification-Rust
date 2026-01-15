use notify_rust::{Notification, Timeout};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::process;
use ksni;

struct PowerTray {
    is_plugged: bool,
}

impl ksni::Tray for PowerTray {
    fn icon_name(&self) -> String {
        if self.is_plugged {
            "ac-adapter".to_string()
        } else {
            "battery".to_string()
        }
    }

    fn title(&self) -> String {
        "Power Monitor".to_string()
    }

    fn id(&self) -> String {
        "power-notification".to_string()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            StandardItem {
                label: if self.is_plugged { "Status: Plugged In" } else { "Status: On Battery" }.to_string(),
                enabled: false,
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "Test Notification".to_string(),
                activate: Box::new(|_| {
                    Notification::new()
                        .summary("Power Monitor")
                        .body("Test notification")
                        .show()
                        .ok();
                }),
                ..Default::default()
            }.into(),
            MenuItem::Separator,
            StandardItem {
                label: "Exit".to_string(),
                activate: Box::new(|_| process::exit(0)),
                ..Default::default()
            }.into(),
        ]
    }
}

fn is_ac_online() -> Option<bool> {
    let ac_path = Path::new("/sys/class/power_supply/AC/online");
    if ac_path.exists() {
        if let Ok(content) = fs::read_to_string(ac_path) {
            return Some(content.trim() == "1");
        }
    }
    
    let power_supply_dir = Path::new("/sys/class/power_supply");
    if !power_supply_dir.exists() {
        return None;
    }
    
    if let Ok(entries) = fs::read_dir(power_supply_dir) {
        for entry in entries.flatten() {
            let online_path = entry.path().join("online");
            if online_path.exists() {
                if let Ok(content) = fs::read_to_string(&online_path) {
                    if content.trim() == "1" {
                        return Some(true);
                    }
                }
            }
        }
    }
    Some(false)
}

fn show_notification(plugged: bool) {
    let (summary, body) = if plugged {
        ("Power Connected", "AC adapter has been plugged in")
    } else {
        ("Power Disconnected", "AC adapter has been unplugged")
    };

    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(Timeout::Milliseconds(3000))
        .sound_name(if plugged { "dialog-information" } else { "dialog-warning" })
        .show()
        .ok();
}

fn main() {
    let mut last_state = match is_ac_online() {
        Some(state) => state,
        None => {
            eprintln!("Error: Cannot access /sys/class/power_supply. This application requires Linux with power supply support.");
            process::exit(1);
        }
    };
    
    let tray_service = ksni::TrayService::new(PowerTray { is_plugged: last_state });
    let handle = tray_service.handle();
    tray_service.spawn();

    loop {
        thread::sleep(Duration::from_secs(2));
        
        if let Some(current_state) = is_ac_online() {
            if current_state != last_state {
                show_notification(current_state);
                last_state = current_state;
                handle.update(|tray: &mut PowerTray| {
                    tray.is_plugged = current_state;
                });
            }
        }
    }
}
