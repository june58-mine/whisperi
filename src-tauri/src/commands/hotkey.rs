use crate::hotkey::HotkeyManager;
use tauri::State;

#[tauri::command]
pub fn configure_hotkey(
    manager: State<'_, HotkeyManager>,
    shortcut: String,
    enabled: bool,
) -> Result<(), String> {
    manager.configure(&shortcut, enabled);
    Ok(())
}

#[tauri::command]
pub fn set_hotkey_capturing(
    manager: State<'_, HotkeyManager>,
    capturing: bool,
) -> Result<(), String> {
    manager.set_capturing(capturing);
    Ok(())
}
