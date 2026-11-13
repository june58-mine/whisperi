import { useState, useRef, useEffect, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

interface HotkeyInputProps {
  value: string;
  onChange: (hotkey: string) => void;
  disabled?: boolean;
}

const DISPLAY_TOKEN_MAP: Record<string, string> = {
  CommandOrControl: "Ctrl",
  Control: "Ctrl",
  Super: "Win",
  Meta: "Win",
  LeftCtrl: "Left Ctrl",
  RightCtrl: "Right Ctrl",
  LeftAlt: "Left Alt",
  RightAlt: "Right Alt",
  LeftShift: "Left Shift",
  RightShift: "Right Shift",
  LeftWin: "Left Win",
  RightWin: "Right Win",
  Hangul: "한/영",
  Hanja: "한자",
};

function formatHotkeyDisplay(hotkey: string): { parts: string[] } {
  if (!hotkey) return { parts: [] };

  return {
    parts: hotkey
      .split("+")
      .map((part) => part.trim())
      .filter(Boolean)
      .map((part) => DISPLAY_TOKEN_MAP[part] ?? part),
  };
}

export function HotkeyInput({ value, onChange, disabled }: HotkeyInputProps) {
  const { t } = useTranslation();
  const [listening, setListening] = useState(false);
  const [preview, setPreview] = useState("");
  const containerRef = useRef<HTMLDivElement>(null);

  const toggleListening = useCallback(() => {
    if (disabled) return;
    setPreview("");
    setListening((prev) => !prev);
  }, [disabled]);

  // Notify overlay & Rust keyboard hook when capture mode changes
  useEffect(() => {
    emit("hotkey-capturing", { capturing: listening });
    invoke("set_hotkey_capturing", { capturing: listening }).catch(() => {});
  }, [listening]);

  // Ensure Rust hook exits capture mode if component unmounts while listening
  useEffect(() => {
    return () => {
      emit("hotkey-capturing", { capturing: false });
      invoke("set_hotkey_capturing", { capturing: false }).catch(() => {});
    };
  }, []);

  // Listen for native low-level hook capture events so modifier-only chords
  // like Ctrl+Win, single modifiers, and reserved OS combos work seamlessly.
  useEffect(() => {
    if (!listening) return;

    const unlistenPreview = listen<{ hotkey: string }>(
      "hotkey-capture-preview",
      (event) => {
        setPreview(event.payload.hotkey);
      },
    );

    const unlistenCaptured = listen<{ hotkey: string }>(
      "hotkey-captured",
      (event) => {
        if (event.payload.hotkey) {
          onChange(event.payload.hotkey);
        }
        setPreview("");
        setListening(false);
      },
    );

    const unlistenCancelled = listen("hotkey-capture-cancelled", () => {
      setPreview("");
      setListening(false);
    });

    const handleClickOutside = (e: MouseEvent) => {
      if (
        containerRef.current &&
        !containerRef.current.contains(e.target as Node)
      ) {
        setPreview("");
        setListening(false);
      }
    };

    window.addEventListener("mousedown", handleClickOutside);
    return () => {
      window.removeEventListener("mousedown", handleClickOutside);
      unlistenPreview.then((fn) => fn());
      unlistenCaptured.then((fn) => fn());
      unlistenCancelled.then((fn) => fn());
    };
  }, [listening, onChange]);

  const { parts } = formatHotkeyDisplay(listening && preview ? preview : value);

  return (
    <div
      ref={containerRef}
      onClick={toggleListening}
      className={`flex items-center justify-center gap-2 px-4 py-3 min-h-[48px] rounded-control border cursor-pointer transition-all duration-150 ${
        listening
          ? "border-primary bg-surface-2 ring-1 ring-primary/30"
          : disabled
            ? "border-border-subtle bg-surface-1 opacity-50 cursor-not-allowed"
            : "border-border-subtle bg-surface-1 hover:border-border-hover hover:bg-surface-2"
      }`}
    >
      {listening ? (
        preview ? (
          <div className="flex items-center gap-1.5">
            {parts.map((part, i) => (
              <span key={i} className="flex items-center gap-1.5">
                {i > 0 && <span className="text-xs text-muted-foreground/50">+</span>}
                <kbd className="inline-flex items-center justify-center min-w-[28px] h-7 px-2 rounded-inner bg-primary/15 border border-primary/40 text-sm font-medium text-primary shadow-sm">
                  {part}
                </kbd>
              </span>
            ))}
          </div>
        ) : (
          <span className="text-sm text-muted-foreground animate-pulse">
            {t("hotkey.listening")}
          </span>
        )
      ) : parts.length > 0 ? (
        <div className="flex items-center gap-1.5">
          {parts.map((part, i) => (
            <span key={i} className="flex items-center gap-1.5">
              {i > 0 && <span className="text-xs text-muted-foreground/50">+</span>}
              <kbd className="inline-flex items-center justify-center min-w-[28px] h-7 px-2 rounded-inner bg-surface-2 border border-border text-sm font-medium text-foreground shadow-sm">
                {part}
              </kbd>
            </span>
          ))}
        </div>
      ) : (
        <span className="text-sm text-muted-foreground/60">
          {t("hotkey.empty")}
        </span>
      )}
    </div>
  );
}
