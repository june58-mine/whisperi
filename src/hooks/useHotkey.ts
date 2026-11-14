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
  ready?: boolean;
}

export function useHotkey({
  shortcut,
  activationMode,
  onToggle,
  onPushStart,
  onPushEnd,
  enabled = true,
  ready = true,
}: UseHotkeyOptions) {
  const onToggleRef = useRef(onToggle);
  const onPushStartRef = useRef(onPushStart);
  const onPushEndRef = useRef(onPushEnd);
  const activationModeRef = useRef(activationMode);
  const configuredRef = useRef(false);

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
    if (!ready) return;
    configuredRef.current = true;
    const activeEnabled = Boolean(shortcut && enabled);
    invoke("configure_hotkey", {
      shortcut: shortcut || "",
      enabled: activeEnabled,
    }).catch((e) => {
      console.warn("Failed to configure native hotkey:", shortcut, e);
    });
  }, [ready, shortcut, enabled]);

  useEffect(() => {
    return () => {
      if (!configuredRef.current) return;
      configuredRef.current = false;
      invoke("configure_hotkey", {
        shortcut: "",
        enabled: false,
      }).catch(() => {});
    };
  }, []);
}
