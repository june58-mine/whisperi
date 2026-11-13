<div align="center">
  <img src="assets/app-icon.png" alt="Whisperi" width="96"/>
  <h1>Whisperi</h1>
  <p><strong>Built on Windows, for Windows.</strong></p>
  <p>
    <a href="README.md">🇰🇷 한국어</a> | <strong>🇺🇸 English</strong>
  </p>
  <p>
    <a href="https://github.com/june58-mine/whisperi/releases/latest"><img src="https://img.shields.io/github/v/release/june58-mine/whisperi?color=88C0D0" alt="Release"></a>
    <a href="https://github.com/xarthurx/whisperi/blob/main/LICENSE"><img src="https://img.shields.io/github/license/xarthurx/whisperi?color=A3BE8C" alt="License"></a>
    <img src="https://img.shields.io/badge/platform-Windows-5E81AC" alt="Platform">
    <img src="https://img.shields.io/badge/built%20with-Tauri%202.x-EBCB8B" alt="Tauri">
  </p>
</div>

<p align="center">
  <img src="assets/flow-diagram.svg" alt="Speech → Voice Transcription → Text Enhancement → Output" width="800"/>
  <p>A fast, modern desktop dictation app built with <strong>Tauri 2.x</strong>. Speak naturally and have your words transcribed, cleaned up, and pasted into any application — including CLI tools like <strong>Claude Code</strong> and <strong>Codex</strong>.</p>
</p>

## Custom Fork Changes (`june58-mine/whisperi`)

This fork of [xarthurx/whisperi](https://github.com/xarthurx/whisperi) adds Windows usability enhancements and dedicated Korean dictation support:

- **Dedicated Korean Prompt Presets (`Light` / `Standard` / `Full`)** — Automatically selects Korean-tailored cleanup prompts when Korean (`ko`), Korean bilingual (`ko + en`), or Korean UI auto-mode is active. Restores proper IT/framework/code terms (`React`, `API`, `PR`, `LLM`, `GitHub`, `Claude Code`) to standard English while keeping everyday Korean loanwords natural in Hangul and preserving speaker tone/endings.
- **Korean Half-Width Punctuation Enforcement (Fixes `。` / `？` Issue)** — Fixes `BILINGUAL_INSTRUCTION`, `INTERNAL_SYSTEM_PROMPT`, `PUNCTUATION_RESTORATION_PROMPT`, and `languageRegistry.json`, backed by a deterministic Rust safety net in `normalize.rs`, so Korean and mixed Korean-English text always uses ASCII half-width `. , ? ! : ;` instead of full-width CJK marks.
- **In-App Editable Default Prompts + 1-Click Reset** — Edit `Light`, `Standard`, and `Full` default prompts directly in **Settings > AI Enhancement > System Prompt** with per-intensity persistence and a one-click **Reset to Default** button.
- **Persistent Tray `Show` Toggle (`showOverlay`)** — Unchecking `Show` in the system tray hides the floating microphone button and remembers your preference across app restarts. When hidden, launching the app again opens Preferences instead of forcing the overlay back on screen.
- **Arbitrary & Modifier-Only Hotkeys (`WH_KEYBOARD_LL`)** — Uses a native Windows low-level keyboard hook so combinations like `Ctrl + Win`, single modifier keys (`Right Ctrl`, `Right Alt`, `Win`, etc.), Korean IME keys (`한/영`, `한자`), and any chord work in both **Tap** and **Push** modes without accidentally triggering the Start Menu.
- **Custom Build Protection** — Automatic upstream updater checks are disabled so official releases do not overwrite this custom build.

## Installation (This Fork)

Download the latest Windows installer (`Whisperi_*_x64-setup.exe`) or standalone `whisperi.exe` from the [Fork Releases](https://github.com/june58-mine/whisperi/releases/latest) page.

*(To install the original upstream version instead: `winget install xarthurx.Whisperi` or [upstream Releases](https://github.com/xarthurx/whisperi/releases/latest).)*

## Why Cloud-First?

Whisperi primarily relies on **cloud transcription services** (OpenAI, Groq, Mistral) rather than local models. While local speech-to-text models like `whisper.cpp` exist, they require significant computational resources to achieve acceptable speed and accuracy. For most users, cloud APIs deliver **near-instant, high-quality transcription** that local models on consumer hardware simply cannot match.

See [Supported Providers](#supported-providers) for the full list of models and our recommended setup.

## Features

<p align="center">
  <img src="assets/overlay-states.svg" alt="Overlay button states: Idle → Recording → Processing" width="700"/>
</p>

- **Voice Transcription** — OpenAI, Groq, Mistral, and Qwen with model selection
- **Text Enhancement** — Post-process transcriptions with GPT, Claude, Gemini, Groq, Qwen, or any model via OpenRouter
- **Auto-Paste** — Transcribed text is automatically pasted into the active window, including [CLI tools](#paste-anywhere--including-cli-tools)
- **Live dictation (Beta)** — stream text into your focused window as you speak, with OpenAI Realtime or Qwen3-ASR-Flash-Realtime
- **Custom Dictionary** — Add names, jargon, and technical terms to improve accuracy
- **Transcribe & Chat Modes** — Cleans up speech by default; say the agent name to switch to a conversational AI chatbot
- **Hotkey Support** — Tap-to-toggle or push-to-talk activation modes

## Language & Translation

Whisperi's language selector (**Settings > General > Language**) controls the **output language**, not the input language. This means you can speak in one language and have the output automatically produced in another.

- **Auto-detect** — output matches whatever language you speak
- **Specific language** (e.g., "English") — output is always in the selected language, regardless of what language you speak
- **Bilingual** (e.g., "Korean + English") — keeps code-switched speech in its original language without translating while applying each language's native punctuation conventions

> The language setting overrides the system prompt language — even if your custom prompt is written in Chinese or Korean, selecting "English" as the output language will produce English output.

<p align="center">
  <img src="setting_window.png" alt="Whisperi Settings Window" width="700"/>
</p>

## Paste Anywhere — Including CLI Tools

Most dictation apps can only paste into standard GUI text fields. Whisperi uses native Win32 `SendInput` to simulate real keystrokes, which means it can paste directly into **command-line interfaces** and **terminal emulators** — something most competitors simply cannot do.

This makes Whisperi especially useful for developers who work with AI coding assistants in the terminal:

- **Claude Code** — dictate prompts and instructions directly into the Claude Code CLI
- **Codex CLI** — speak your coding requests instead of typing them
- **Any terminal** — PowerShell, Windows Terminal, cmd.exe, WSL terminals

> No need to type out long prompts manually. Just press the hotkey, speak, and your words appear right in the terminal input.

## Supported Providers

### Recommended Models

| Stage             | Provider | Model                        | Why                                                         |
| ----------------- | -------- | ---------------------------- | ----------------------------------------------------------- |
| **Transcription** | Groq     | Whisper Large v3             | Highest accuracy cloud transcription at 299x real-time speed |
| **Enhancement**   | Groq     | GPT-OSS 120B / LLaMA 3.3 70B | Best speed-to-quality ratio for text cleanup                |

Both models run on Groq's inference engine, so you only need **a single API key**. Transcription + enhancement typically completes in **under 2 seconds** end-to-end.

### Voice Transcription

| Provider   | Models                                             |
| ---------- | -------------------------------------------------- |
| OpenAI     | GPT-4o Mini Transcribe, GPT-4o Transcribe, Whisper |
| Groq       | Whisper Large v3, Whisper Large v3 Turbo           |
| Mistral    | Voxtral Mini                                       |
| Qwen       | Qwen3 ASR Flash                                    |
| OpenRouter | Any model — enter `provider/model-name`            |

### Text Enhancement

| Provider      | Models                                                                      |
| ------------- | --------------------------------------------------------------------------- |
| OpenAI        | GPT-5.2, GPT-5.2 Pro, GPT-5 Mini, GPT-5 Nano, GPT-4.1 family                |
| Anthropic     | Claude Opus 4.6, Claude Sonnet 4.5, Claude Haiku 4.5                        |
| Google Gemini | Gemini 3 Pro, Gemini 3 Flash, Gemini 2.5 Pro, Gemini 2.5 Flash              |
| Groq          | LLaMA 4 Maverick, LLaMA 4 Scout, Qwen3 32B, GPT-OSS 120B/20B, LLaMA 3.3 70B |
| Qwen          | Qwen3 235B (MoE), Qwen3 32B                                                  |
| OpenRouter    | Any model — enter `provider/model-name` (e.g. `anthropic/claude-sonnet-4`)   |

## Example Prompts

Whisperi lets you edit the built-in `Light` / `Standard` / `Full` prompts directly or use the Custom Prompt tab. Example prompts are available in [`examples/prompts/`](../examples/prompts/):

- [`custom-prompt-ko.txt`](../examples/prompts/custom-prompt-ko.txt) — Korean (한국어)
- [`custom-prompt-en.txt`](../examples/prompts/custom-prompt-en.txt) — English
- [`custom-prompt-zh.txt`](../examples/prompts/custom-prompt-zh.txt) — Chinese (中文)

## Known Issues

- **Global hotkey may stop working after a remote desktop session** — If you use remote desktop software (RustDesk, Windows RDP, AnyDesk, etc.), the OS-level global hotkey registration can be disrupted when the remote session connects or disconnects. Click the Whisperi overlay/tray once to restore focus, or re-open Preferences if needed.

## Other Platforms

Whisperi currently targets **Windows only**, built with [Tauri](https://tauri.app/).

## Contributing

**Prerequisites:** [Rust](https://rustup.rs/) (stable), [bun](https://bun.sh/), Windows 10/11

```bash
bun install              # install dependencies
bun run tauri dev        # dev mode (Vite + Tauri)
bun run typecheck        # TypeScript check
cd src-tauri && cargo test   # Rust tests
bun run tauri build      # production build
```

## License

This project is licensed under the MIT License — see the [LICENSE](../LICENSE) file for details.
