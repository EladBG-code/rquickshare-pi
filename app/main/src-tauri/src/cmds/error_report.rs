use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn report_dir() -> Result<PathBuf, String> {
    let home = std::env::var_os("HOME").ok_or_else(|| "HOME is not set".to_string())?;
    let dir = PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("dev.eladbg.rquickshare-pi")
        .join("error-reports");

    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn write_native_error_report(kind: &str, message: &str) -> Result<PathBuf, String> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();
    let path = report_dir()?.join(format!("rquickshare-pi-{kind}-{now}.txt"));
    let body = format!(
        "RQuickShare Pi error report\n\nKind: {kind}\nTimestamp: {now}\nOS: {}\nArch: {}\n\nMessage:\n{message}\n",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    fs::write(&path, body).map_err(|e| e.to_string())?;
    Ok(path)
}

#[tauri::command]
pub fn write_error_report(message: String) -> Result<String, String> {
    write_native_error_report("client", &message).map(|path| path.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn get_latest_error_report() -> Result<Option<String>, String> {
    let dir = report_dir()?;
    let mut latest: Option<(SystemTime, PathBuf)> = None;

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("txt") {
            continue;
        }

        let modified = entry
            .metadata()
            .and_then(|metadata| metadata.modified())
            .map_err(|e| e.to_string())?;

        if latest
            .as_ref()
            .map(|(latest_modified, _)| modified > *latest_modified)
            .unwrap_or(true)
        {
            latest = Some((modified, path));
        }
    }

    Ok(latest.map(|(_, path)| path.to_string_lossy().into_owned()))
}
