use tauri::AppHandle;

use crate::store::get_device_name;

#[tauri::command]
pub fn get_hostname(app_handle: AppHandle) -> String {
    get_device_name(&app_handle)
}
