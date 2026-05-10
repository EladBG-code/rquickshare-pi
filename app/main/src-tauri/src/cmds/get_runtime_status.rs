use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeIssue {
    title: String,
    message: String,
    command: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeStatus {
    ok: bool,
    issues: Vec<RuntimeIssue>,
}

#[tauri::command]
pub fn get_runtime_status() -> RuntimeStatus {
    let mut issues = Vec::new();

    #[cfg(target_os = "linux")]
    {
        if !command_success("systemctl", &["is-active", "--quiet", "bluetooth"]) {
            issues.push(RuntimeIssue {
                title: "Bluetooth service is not running".into(),
                message: "RQuickShare Pi needs the BlueZ Bluetooth service for nearby discovery."
                    .into(),
                command: Some("sudo systemctl enable --now bluetooth".into()),
            });
        }

        if !command_success("systemctl", &["is-active", "--quiet", "avahi-daemon"]) {
            issues.push(RuntimeIssue {
                title: "mDNS service is not running".into(),
                message: "RQuickShare Pi needs Avahi/mDNS so Android can resolve this Pi on the local network.".into(),
                command: Some("sudo systemctl enable --now avahi-daemon".into()),
            });
        }

        if let Some(output) = command_output("rfkill", &["list", "bluetooth"]) {
            if output.contains("Hard blocked: yes") {
                issues.push(RuntimeIssue {
                    title: "Bluetooth is hard-blocked".into(),
                    message: "The Pi reports Bluetooth as blocked by hardware or firmware. RQuickShare Pi cannot fix that without a system-level change.".into(),
                    command: None,
                });
            } else if output.contains("Soft blocked: yes") {
                issues.push(RuntimeIssue {
                    title: "Bluetooth is soft-blocked".into(),
                    message: "Bluetooth is disabled at the OS radio layer. Unblock it once, then RQuickShare Pi can manage visibility automatically.".into(),
                    command: Some("sudo rfkill unblock bluetooth".into()),
                });
            }
        }

        if let Some(output) = command_output("bluetoothctl", &["show"]) {
            if output.contains("Powered: no") {
                issues.push(RuntimeIssue {
                    title: "Bluetooth adapter is powered off".into(),
                    message: "RQuickShare Pi will try to power the adapter automatically, but this session currently reports it as off.".into(),
                    command: Some("bluetoothctl power on".into()),
                });
            }
        }
    }

    RuntimeStatus {
        ok: issues.is_empty(),
        issues,
    }
}

#[cfg(target_os = "linux")]
fn command_success(program: &str, args: &[&str]) -> bool {
    std::process::Command::new(program)
        .args(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "linux")]
fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = std::process::Command::new(program)
        .args(args)
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&output.stdout).to_string())
}
