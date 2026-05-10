use crate::{store::set_device_name, AppState};

#[tauri::command]
pub fn change_device_name(
    message: String,
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    info!("change_device_name: {message}");

    let normalized = set_device_name(&app_handle, &message).map_err(|e| e.to_string())?;
    state
        .rqs
        .lock()
        .unwrap()
        .change_device_name(normalized.clone());

    Ok(normalized)
}
