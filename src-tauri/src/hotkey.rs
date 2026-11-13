use serde::Serialize;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyToken {
    AnyCtrl,
    LeftCtrl,
    RightCtrl,
    AnyAlt,
    LeftAlt,
    RightAlt,
    AnyShift,
    LeftShift,
    RightShift,
    AnyWin,
    LeftWin,
    RightWin,
    Vk(u32),
}

impl KeyToken {
    pub fn matches_vk(&self, vk: u32) -> bool {
        match self {
            Self::AnyCtrl => matches!(vk, 0x11 | 0xA2 | 0xA3),
            Self::LeftCtrl => vk == 0xA2,
            Self::RightCtrl => vk == 0xA3,
            Self::AnyAlt => matches!(vk, 0x12 | 0xA4 | 0xA5),
            Self::LeftAlt => vk == 0xA4,
            Self::RightAlt => vk == 0xA5,
            Self::AnyShift => matches!(vk, 0x10 | 0xA0 | 0xA1),
            Self::LeftShift => vk == 0xA0,
            Self::RightShift => vk == 0xA1,
            Self::AnyWin => matches!(vk, 0x5B | 0x5C),
            Self::LeftWin => vk == 0x5B,
            Self::RightWin => vk == 0x5C,
            Self::Vk(code) => vk == *code,
        }
    }

    pub fn is_win(&self) -> bool {
        matches!(self, Self::AnyWin | Self::LeftWin | Self::RightWin)
    }
}

pub fn parse_shortcut(shortcut: &str) -> Vec<KeyToken> {
    let mut tokens = Vec::new();
    for raw in shortcut.split('+') {
        let part = raw.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(tok) = parse_token(part) {
            if !tokens.contains(&tok) {
                tokens.push(tok);
            }
        }
    }
    tokens
}

fn parse_token(part: &str) -> Option<KeyToken> {
    let lower = part.to_ascii_lowercase();
    let compact: String = lower
        .chars()
        .filter(|c| *c != ' ' && *c != '_' && *c != '-')
        .collect();

    match compact.as_str() {
        "ctrl" | "control" | "commandorcontrol" | "cmdorctrl" | "command" => {
            Some(KeyToken::AnyCtrl)
        }
        "leftctrl" | "lctrl" | "leftcontrol" | "lcontrol" => Some(KeyToken::LeftCtrl),
        "rightctrl" | "rctrl" | "rightcontrol" | "rcontrol" => Some(KeyToken::RightCtrl),
        "alt" | "option" | "menu" => Some(KeyToken::AnyAlt),
        "leftalt" | "lalt" | "leftoption" | "lmenu" => Some(KeyToken::LeftAlt),
        "rightalt" | "ralt" | "rightoption" | "rmenu" | "altgr" => Some(KeyToken::RightAlt),
        "shift" => Some(KeyToken::AnyShift),
        "leftshift" | "lshift" => Some(KeyToken::LeftShift),
        "rightshift" | "rshift" => Some(KeyToken::RightShift),
        "win" | "windows" | "super" | "meta" | "cmd" | "os" => Some(KeyToken::AnyWin),
        "leftwin" | "lwin" | "osleft" | "metaleft" | "leftmeta" | "leftsuper" => {
            Some(KeyToken::LeftWin)
        }
        "rightwin" | "rwin" | "osright" | "metaright" | "rightmeta" | "rightsuper" => {
            Some(KeyToken::RightWin)
        }
        "space" => Some(KeyToken::Vk(0x20)),
        "enter" | "return" | "numpadenter" => Some(KeyToken::Vk(0x0D)),
        "backspace" => Some(KeyToken::Vk(0x08)),
        "tab" => Some(KeyToken::Vk(0x09)),
        "escape" | "esc" => Some(KeyToken::Vk(0x1B)),
        "delete" | "del" => Some(KeyToken::Vk(0x2E)),
        "insert" | "ins" => Some(KeyToken::Vk(0x2D)),
        "home" => Some(KeyToken::Vk(0x24)),
        "end" => Some(KeyToken::Vk(0x23)),
        "pageup" | "pgup" => Some(KeyToken::Vk(0x21)),
        "pagedown" | "pgdn" => Some(KeyToken::Vk(0x22)),
        "up" | "arrowup" => Some(KeyToken::Vk(0x26)),
        "down" | "arrowdown" => Some(KeyToken::Vk(0x28)),
        "left" | "arrowleft" => Some(KeyToken::Vk(0x25)),
        "right" | "arrowright" => Some(KeyToken::Vk(0x27)),
        "capslock" => Some(KeyToken::Vk(0x14)),
        "numlock" => Some(KeyToken::Vk(0x90)),
        "scrolllock" => Some(KeyToken::Vk(0x91)),
        "pause" => Some(KeyToken::Vk(0x13)),
        "printscreen" | "prtsc" => Some(KeyToken::Vk(0x2C)),
        "apps" | "contextmenu" => Some(KeyToken::Vk(0x5D)),
        "hangul" | "kana" => Some(KeyToken::Vk(0x15)),
        "hanja" | "kanji" => Some(KeyToken::Vk(0x19)),
        "numpadmultiply" => Some(KeyToken::Vk(0x6A)),
        "numpadadd" => Some(KeyToken::Vk(0x6B)),
        "numpadsubtract" => Some(KeyToken::Vk(0x6D)),
        "numpaddecimal" => Some(KeyToken::Vk(0x6E)),
        "numpaddivide" => Some(KeyToken::Vk(0x6F)),
        "backquote" => Some(KeyToken::Vk(0xC0)),
        "minus" => Some(KeyToken::Vk(0xBD)),
        "equal" | "plus" => Some(KeyToken::Vk(0xBB)),
        "bracketleft" => Some(KeyToken::Vk(0xDB)),
        "bracketright" => Some(KeyToken::Vk(0xDD)),
        "backslash" => Some(KeyToken::Vk(0xDC)),
        "semicolon" => Some(KeyToken::Vk(0xBA)),
        "quote" => Some(KeyToken::Vk(0xDE)),
        "comma" => Some(KeyToken::Vk(0xBC)),
        "period" => Some(KeyToken::Vk(0xBE)),
        "slash" => Some(KeyToken::Vk(0xBF)),
        _ => {
            // Single character punctuation
            match part {
                "`" => return Some(KeyToken::Vk(0xC0)),
                "-" => return Some(KeyToken::Vk(0xBD)),
                "=" => return Some(KeyToken::Vk(0xBB)),
                "[" => return Some(KeyToken::Vk(0xDB)),
                "]" => return Some(KeyToken::Vk(0xDD)),
                "\\" => return Some(KeyToken::Vk(0xDC)),
                ";" => return Some(KeyToken::Vk(0xBA)),
                "'" => return Some(KeyToken::Vk(0xDE)),
                "," => return Some(KeyToken::Vk(0xBC)),
                "." => return Some(KeyToken::Vk(0xBE)),
                "/" => return Some(KeyToken::Vk(0xBF)),
                _ => {}
            }

            // Letter A-Z
            if compact.len() == 1 {
                let b = compact.as_bytes()[0];
                if b.is_ascii_alphabetic() {
                    return Some(KeyToken::Vk(b.to_ascii_uppercase() as u32));
                }
                if b.is_ascii_digit() {
                    return Some(KeyToken::Vk(b as u32));
                }
            }

            // Numpad 0-9
            if let Some(rest) = compact.strip_prefix("numpad") {
                if rest.len() == 1 && rest.as_bytes()[0].is_ascii_digit() {
                    let digit = (rest.as_bytes()[0] - b'0') as u32;
                    return Some(KeyToken::Vk(0x60 + digit));
                }
            }

            // F1..F24
            if let Some(rest) = compact.strip_prefix('f') {
                if let Ok(num) = rest.parse::<u32>() {
                    if (1..=24).contains(&num) {
                        return Some(KeyToken::Vk(0x70 + (num - 1)));
                    }
                }
            }

            // Hex VK fallback: VK_0xXX or vk0xxx
            if let Some(hex) = compact.strip_prefix("vk0x") {
                if let Ok(vk) = u32::from_str_radix(hex, 16) {
                    return Some(KeyToken::Vk(vk));
                }
            }

            None
        }
    }
}

pub fn vk_to_capture_name(vk: u32, generic_modifiers: bool) -> String {
    if generic_modifiers {
        match vk {
            0x11 | 0xA2 | 0xA3 => return "Ctrl".to_string(),
            0x12 | 0xA4 | 0xA5 => return "Alt".to_string(),
            0x10 | 0xA0 | 0xA1 => return "Shift".to_string(),
            0x5B | 0x5C => return "Win".to_string(),
            _ => {}
        }
    } else {
        match vk {
            0x11 => return "Ctrl".to_string(),
            0xA2 => return "LeftCtrl".to_string(),
            0xA3 => return "RightCtrl".to_string(),
            0x12 => return "Alt".to_string(),
            0xA4 => return "LeftAlt".to_string(),
            0xA5 => return "RightAlt".to_string(),
            0x10 => return "Shift".to_string(),
            0xA0 => return "LeftShift".to_string(),
            0xA1 => return "RightShift".to_string(),
            0x5B => return "LeftWin".to_string(),
            0x5C => return "RightWin".to_string(),
            _ => {}
        }
    }

    match vk {
        0x20 => "Space".to_string(),
        0x0D => "Enter".to_string(),
        0x08 => "Backspace".to_string(),
        0x09 => "Tab".to_string(),
        0x1B => "Escape".to_string(),
        0x2E => "Delete".to_string(),
        0x2D => "Insert".to_string(),
        0x24 => "Home".to_string(),
        0x23 => "End".to_string(),
        0x21 => "PageUp".to_string(),
        0x22 => "PageDown".to_string(),
        0x26 => "Up".to_string(),
        0x28 => "Down".to_string(),
        0x25 => "Left".to_string(),
        0x27 => "Right".to_string(),
        0x14 => "CapsLock".to_string(),
        0x90 => "NumLock".to_string(),
        0x91 => "ScrollLock".to_string(),
        0x13 => "Pause".to_string(),
        0x2C => "PrintScreen".to_string(),
        0x5D => "ContextMenu".to_string(),
        0x15 => "Hangul".to_string(),
        0x19 => "Hanja".to_string(),
        0x6A => "NumpadMultiply".to_string(),
        0x6B => "NumpadAdd".to_string(),
        0x6D => "NumpadSubtract".to_string(),
        0x6E => "NumpadDecimal".to_string(),
        0x6F => "NumpadDivide".to_string(),
        0xC0 => "`".to_string(),
        0xBD => "-".to_string(),
        0xBB => "=".to_string(),
        0xDB => "[".to_string(),
        0xDD => "]".to_string(),
        0xDC => "\\".to_string(),
        0xBA => ";".to_string(),
        0xDE => "'".to_string(),
        0xBC => ",".to_string(),
        0xBE => ".".to_string(),
        0xBF => "/".to_string(),
        0x30..=0x39 => ((vk as u8) as char).to_string(),
        0x41..=0x5A => ((vk as u8) as char).to_string(),
        0x60..=0x69 => format!("Numpad{}", vk - 0x60),
        0x70..=0x87 => format!("F{}", vk - 0x70 + 1),
        _ => format!("VK_0x{:02X}", vk),
    }
}

fn modifier_sort_rank(vk: u32) -> (u8, String) {
    match vk {
        0x11 | 0xA2 | 0xA3 => (0, String::new()),
        0x12 | 0xA4 | 0xA5 => (1, String::new()),
        0x10 | 0xA0 | 0xA1 => (2, String::new()),
        0x5B | 0x5C => (3, String::new()),
        _ => (4, vk_to_capture_name(vk, true)),
    }
}

pub fn format_captured_vks(vks: &HashSet<u32>) -> String {
    if vks.is_empty() {
        return String::new();
    }
    // Single modifier keeps left/right distinction so RightCtrl can be bound
    // without colliding with LeftCtrl+C. Multi-key chords collapse modifiers
    // generically (e.g. Ctrl+Win) so either Ctrl/Win works naturally.
    let generic_modifiers = vks.len() > 1;
    let mut ordered: Vec<u32> = vks.iter().copied().collect();
    ordered.sort_by_key(|&vk| modifier_sort_rank(vk));

    let mut parts: Vec<String> = Vec::new();
    for vk in ordered {
        let name = vk_to_capture_name(vk, generic_modifiers);
        if !parts.contains(&name) {
            parts.push(name);
        }
    }
    parts.join("+")
}

pub fn all_tokens_pressed(tokens: &[KeyToken], pressed_vks: &HashSet<u32>) -> bool {
    !tokens.is_empty()
        && tokens
            .iter()
            .all(|token| pressed_vks.iter().any(|&vk| token.matches_vk(vk)))
}

pub fn exact_tokens_match(tokens: &[KeyToken], pressed_vks: &HashSet<u32>) -> bool {
    all_tokens_pressed(tokens, pressed_vks)
        && pressed_vks
            .iter()
            .all(|&vk| tokens.iter().any(|token| token.matches_vk(vk)))
}

#[derive(Debug, Default)]
pub struct HookRuntimeState {
    pub tokens: Vec<KeyToken>,
    pub enabled: bool,
    pub capturing: bool,
    pub pressed_vks: HashSet<u32>,
    pub active: bool,
    pub need_win_menu_mask: bool,
    pub capture_pressed: HashSet<u32>,
    pub capture_peak: HashSet<u32>,
    pub capture_swallowed: HashSet<u32>,
}

#[derive(Clone)]
pub struct HotkeyManager {
    state: Arc<Mutex<HookRuntimeState>>,
}

#[derive(Clone, Serialize)]
struct CapturePayload {
    hotkey: String,
}

enum OutboundEvent {
    HotkeyPressed,
    HotkeyReleased,
    CapturePreview(String),
    Captured(String),
    CaptureCancelled,
}

impl HotkeyManager {
    pub fn start(app: AppHandle) -> Self {
        let state = Arc::new(Mutex::new(HookRuntimeState::default()));

        #[cfg(windows)]
        {
            let (tx, rx) = std::sync::mpsc::sync_channel::<OutboundEvent>(64);
            let app_clone = app.clone();
            std::thread::Builder::new()
                .name("whisperi-hotkey-emitter".into())
                .spawn(move || {
                    while let Ok(ev) = rx.recv() {
                        match ev {
                            OutboundEvent::HotkeyPressed => {
                                let _ = app_clone.emit("hotkey-pressed", ());
                            }
                            OutboundEvent::HotkeyReleased => {
                                let _ = app_clone.emit("hotkey-released", ());
                            }
                            OutboundEvent::CapturePreview(hotkey) => {
                                let _ = app_clone
                                    .emit("hotkey-capture-preview", CapturePayload { hotkey });
                            }
                            OutboundEvent::Captured(hotkey) => {
                                let _ = app_clone
                                    .emit("hotkey-captured", CapturePayload { hotkey });
                            }
                            OutboundEvent::CaptureCancelled => {
                                let _ = app_clone.emit("hotkey-capture-cancelled", ());
                            }
                        }
                    }
                })
                .ok();

            win_hook::install_hook_thread(state.clone(), tx);
        }

        Self { state }
    }

    pub fn configure(&self, shortcut: &str, enabled: bool) {
        if let Ok(mut guard) = self.state.lock() {
            guard.tokens = parse_shortcut(shortcut);
            guard.enabled = enabled && !guard.tokens.is_empty();
            guard.active = false;
            guard.need_win_menu_mask = false;
        }
    }

    pub fn set_capturing(&self, capturing: bool) {
        if let Ok(mut guard) = self.state.lock() {
            guard.capturing = capturing;
            guard.active = false;
            guard.capture_pressed.clear();
            guard.capture_peak.clear();
            if !capturing {
                guard.capture_swallowed.clear();
            }
        }
    }
}

#[cfg(windows)]
mod win_hook {
    use super::{
        all_tokens_pressed, exact_tokens_match, format_captured_vks, HookRuntimeState,
        OutboundEvent,
    };
    use std::sync::mpsc::SyncSender;
    use std::sync::{Arc, Mutex, OnceLock};
    use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput, VIRTUAL_KEY,
    };
    use windows::Win32::UI::WindowsAndMessaging::{
        CallNextHookEx, DispatchMessageW, GetMessageW, HHOOK, KBDLLHOOKSTRUCT, LLKHF_INJECTED,
        MSG, SetWindowsHookExW, TranslateMessage, UnhookWindowsHookEx, WH_KEYBOARD_LL,
        WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
    };

    struct HookContext {
        state: Arc<Mutex<HookRuntimeState>>,
        tx: SyncSender<OutboundEvent>,
    }

    static HOOK_CTX: OnceLock<HookContext> = OnceLock::new();

    fn send_win_menu_mask_key() {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0xFF),
                    wScan: 0,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe {
            let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
        }
    }

    unsafe extern "system" fn ll_keyboard_proc(
        n_code: i32,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        if n_code >= 0 && l_param.0 != 0 {
            let kb = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
            let msg = w_param.0 as u32;

            let is_down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
            let is_up = msg == WM_KEYUP || msg == WM_SYSKEYUP;
            let is_injected = (kb.flags.0 & LLKHF_INJECTED.0) != 0;
            let vk = kb.vkCode;

            // Ignore synthetic/Unicode keystrokes (such as Whisperi's Ctrl+V or typing).
            if (is_down || is_up) && !is_injected && vk != 0xE7 && vk != 0xFF {
                if let Some(ctx) = HOOK_CTX.get() {
                    let mut mask_win_release = false;
                    let mut swallow = false;

                    if let Ok(mut guard) = ctx.state.lock() {
                        if is_down {
                            guard.pressed_vks.insert(vk);
                        } else if is_up {
                            guard.pressed_vks.remove(&vk);
                        }

                        if guard.capturing {
                            if is_down {
                                if vk == 0x1B {
                                    // Escape cancels shortcut capture.
                                    guard.capturing = false;
                                    guard.capture_pressed.clear();
                                    guard.capture_peak.clear();
                                    guard.capture_swallowed.insert(vk);
                                    swallow = true;
                                    let _ = ctx.tx.try_send(OutboundEvent::CaptureCancelled);
                                } else {
                                    guard.capture_pressed.insert(vk);
                                    guard.capture_peak.insert(vk);
                                    guard.capture_swallowed.insert(vk);
                                    swallow = true;
                                    let preview = format_captured_vks(&guard.capture_peak);
                                    let _ =
                                        ctx.tx.try_send(OutboundEvent::CapturePreview(preview));
                                }
                            } else if is_up {
                                guard.capture_pressed.remove(&vk);
                                if guard.capture_pressed.is_empty()
                                    && !guard.capture_peak.is_empty()
                                {
                                    let captured = format_captured_vks(&guard.capture_peak);
                                    guard.capture_peak.clear();
                                    guard.capturing = false;
                                    let _ = ctx.tx.try_send(OutboundEvent::Captured(captured));
                                }
                                if guard.capture_swallowed.remove(&vk) {
                                    swallow = true;
                                }
                            }
                        } else if guard.enabled && !guard.tokens.is_empty() {
                            if is_down && !guard.active {
                                if exact_tokens_match(&guard.tokens, &guard.pressed_vks) {
                                    guard.active = true;
                                    if guard.tokens.iter().any(|t| t.is_win()) {
                                        guard.need_win_menu_mask = true;
                                    }
                                    let _ = ctx.tx.try_send(OutboundEvent::HotkeyPressed);
                                }
                            } else if is_up && guard.active {
                                if !all_tokens_pressed(&guard.tokens, &guard.pressed_vks) {
                                    guard.active = false;
                                    let _ = ctx.tx.try_send(OutboundEvent::HotkeyReleased);
                                }
                            }

                            if is_up && (vk == 0x5B || vk == 0x5C) && guard.need_win_menu_mask {
                                guard.need_win_menu_mask = false;
                                mask_win_release = true;
                            }
                        } else if guard.active {
                            guard.active = false;
                        }
                    }

                    if mask_win_release {
                        send_win_menu_mask_key();
                    }

                    if swallow {
                        return LRESULT(1);
                    }
                }
            }
        }

        unsafe { CallNextHookEx(HHOOK::default(), n_code, w_param, l_param) }
    }

    pub fn install_hook_thread(
        state: Arc<Mutex<HookRuntimeState>>,
        tx: SyncSender<OutboundEvent>,
    ) {
        let _ = HOOK_CTX.set(HookContext { state, tx });
        std::thread::Builder::new()
            .name("whisperi-hotkey-hook".into())
            .spawn(|| unsafe {
                let Ok(hook) = SetWindowsHookExW(WH_KEYBOARD_LL, Some(ll_keyboard_proc), None, 0)
                else {
                    log::error!("Failed to install Windows low-level keyboard hook");
                    return;
                };

                let mut msg = MSG::default();
                while GetMessageW(&mut msg, None, 0, 0).as_bool() {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }

                let _ = UnhookWindowsHookEx(hook);
            })
            .ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ctrl_win_and_single_modifiers() {
        assert_eq!(
            parse_shortcut("Ctrl+Win"),
            vec![KeyToken::AnyCtrl, KeyToken::AnyWin]
        );
        assert_eq!(
            parse_shortcut("CommandOrControl+Shift+Space"),
            vec![KeyToken::AnyCtrl, KeyToken::AnyShift, KeyToken::Vk(0x20)]
        );
        assert_eq!(parse_shortcut("RightCtrl"), vec![KeyToken::RightCtrl]);
        assert_eq!(parse_shortcut("F13"), vec![KeyToken::Vk(0x7C)]);
    }

    #[test]
    fn formats_multi_key_generically_and_single_modifier_specifically() {
        let ctrl_win: HashSet<u32> = [0xA2, 0x5B].into_iter().collect();
        assert_eq!(format_captured_vks(&ctrl_win), "Ctrl+Win");

        let right_ctrl: HashSet<u32> = [0xA3].into_iter().collect();
        assert_eq!(format_captured_vks(&right_ctrl), "RightCtrl");
    }

    #[test]
    fn exact_match_prevents_extra_keys() {
        let tokens = parse_shortcut("Ctrl+Win");
        let pressed_ok: HashSet<u32> = [0xA2, 0x5B].into_iter().collect();
        let pressed_extra: HashSet<u32> = [0xA2, 0x5B, 0xA0].into_iter().collect();

        assert!(exact_tokens_match(&tokens, &pressed_ok));
        assert!(!exact_tokens_match(&tokens, &pressed_extra));
    }
}
