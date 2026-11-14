use crate::hotkey::HotkeyManager;
use tauri::{State, WebviewWindow};

#[tauri::command]
pub fn configure_hotkey(
    window: WebviewWindow,
    manager: State<'_, HotkeyManager>,
    shortcut: String,
    enabled: bool,
) -> Result<(), String> {
    if window.label() != "main" {
        return Ok(());
    }
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
