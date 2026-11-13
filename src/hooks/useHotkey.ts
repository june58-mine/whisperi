import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface UseHotkeyOptions {
  shortcut: string;
  activationMode: "tap" | "push";
  onToggle: () => void;
  onPushStart?: () => void;
  onPushEnd?: () => void;
  enabled?: boolean;
}

export function useHotkey({
  shortcut,
  activationMode,
  onToggle,
  onPushStart,
  onPushEnd,
  enabled = true,
}: UseHotkeyOptions) {
  const onToggleRef = useRef(onToggle);
  const onPushStartRef = useRef(onPushStart);
  const onPushEndRef = useRef(onPushEnd);
  const activationModeRef = useRef(activationMode);

  onToggleRef.current = onToggle;
  onPushStartRef.current = onPushStart;
  onPushEndRef.current = onPushEnd;
  activationModeRef.current = activationMode;

  useEffect(() => {
    let active = true;
    const unlistenPressed = listen("hotkey-pressed", () => {
      if (!active) return;
      if (activationModeRef.current === "tap") {
        onToggleRef.current();
      } else {
        onPushStartRef.current?.();
      }
    });

    const unlistenReleased = listen("hotkey-released", () => {
      if (!active) return;
      if (activationModeRef.current === "push") {
        onPushEndRef.current?.();
      }
    });

    return () => {
      active = false;
      unlistenPressed.then((fn) => fn());
      unlistenReleased.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    const activeEnabled = Boolean(shortcut && enabled);
    invoke("configure_hotkey", {
      shortcut: shortcut || "",
      enabled: activeEnabled,
    }).catch((e) => {
      console.warn("Failed to configure native hotkey:", shortcut, e);
    });

    return () => {
      invoke("configure_hotkey", {
        shortcut: "",
        enabled: false,
      }).catch(() => {});
    };
  }, [shortcut, enabled]);
}
