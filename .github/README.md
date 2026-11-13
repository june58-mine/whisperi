<div align="center">
  <img src="assets/app-icon.png" alt="Whisperi" width="96"/>
  <h1>Whisperi (한국어 커스텀 포크)</h1>
  <p><strong>Windows 환경을 위한 초고속 클라우드 음성 받아쓰기 & AI 정제 앱</strong></p>
  <p>
    <strong>🇰🇷 한국어</strong> | <a href="README.en.md">🇺🇸 English</a>
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
  <p><strong>Tauri 2.x</strong>로 제작된 빠르고 가벼운 Windows 데스크톱 음성 받아쓰기(Dictation) 앱입니다. 평소 말투 그대로 말하면 음성을 텍스트로 변환하고 AI로 깔끔하게 다듬어, 일반 메신저·문서 편집기는 물론 <strong>Claude Code</strong>, <strong>Codex CLI</strong> 같은 터미널/CLI 창에도 바로 붙여넣어 줍니다.</p>
</p>

## 커스텀 포크 주요 개선 사항 (`june58-mine/whisperi`)

원본 프로젝트([xarthurx/whisperi](https://github.com/xarthurx/whisperi))를 바탕으로 한국어 사용자와 Windows 환경의 편의성을 크게 개선한 포크입니다.

- **한국어 전용 AI 정제 프롬프트 내장 (`Light` / `Standard` / `Full`)**
  - 한국어(`ko`), 한영 이중언어(`ko + en`), 또는 한국어 UI 자동 감지 시 한국어에 최적화된 전용 프롬프트가 자동 적용됩니다.
  - 고유 IT 용어·프레임워크·코드 심볼(`React`, `API`, `PR`, `LLM`, `GitHub`, `Claude Code` 등)은 정확한 영어 표기로 복원하고, 일상 외래어(`리뷰`, `머지`, `에러`, `로딩`, `버튼` 등)는 한글 그대로 유지하며 화자의 말투·종결 어미를 최대한 보존합니다.
- **한국어 전각 문장부호(`。`, `？`) 버그 완전 수정**
  - 이중언어(`Bilingual`) 모드 등에서 한국어 문장 끝에 중국어·일본어 전각 부호(`。`, `？`)가 찍히던 문제를 시스템 프롬프트 + 언어 규칙 + Rust 후처리 안전장치(`normalize.rs`) 3단계로 원천 차단해 항상 표준 반각 부호(`. , ? !`)만 출력합니다.
- **앱 내 시스템 프롬프트 직접 편집 & 원클릭 기본값 복원**
  - **설정 > AI 향상 > 시스템 프롬프트**에서 `라이트(Light)` / `표준(Standard)` / `풀(Full)` 프리셋 본문을 프로그램 안에서 바로 수정하고 저장할 수 있습니다.
  - 수정된 상태(`수정됨` 배지 표시)에서 **`기본값으로 복원`** 버튼 한 번만 누르면 언제든 원래 기본 프롬프트로 즉시 되돌릴 수 있습니다.
- **트레이 `Show` 토글 상태 영구 유지 (`showOverlay`)**
  - 시스템 트레이에서 `Show` 체크를 해제해 플로팅 마이크 버튼을 숨기면, 앱을 껐다 켜도 숨김 상태가 그대로 유지됩니다. 숨김 상태에서 앱을 실행하면 오버레이를 강제로 띄우지 않고 설정 창을 엽니다.
- **Windows 모든 단축키·보조키 단독 지원 (`WH_KEYBOARD_LL`)**
  - 네이티브 Windows 저수준 키보드 훅을 사용해 `Ctrl + Win`, 단일 보조키(`Right Ctrl`, `Right Alt`, `Win` 등), 한국어 키(`한/영`, `한자`) 등 원하는 모든 키 조합을 **Tap** 및 **Push-to-Talk** 모드에서 시작 메뉴 오작동 없이 사용할 수 있습니다.
- **자동 업데이트 비활성화 (커스텀 빌드 보호)**
  - 원본 업스트림의 자동 업데이트가 커스텀 빌드를 덮어쓰지 않도록 차단되어 있습니다.

## 설치 방법 (이 포크 버전)

[Releases 페이지](https://github.com/june58-mine/whisperi/releases/latest)에서 최신 Windows 설치 파일(`Whisperi_*_x64-setup.exe`) 또는 무설치 단일 실행 파일(`whisperi.exe`)을 받아 바로 실행하세요.

*(원본 업스트림 버전을 설치하려면: `winget install xarthurx.Whisperi` 또는 [원본 Releases](https://github.com/xarthurx/whisperi/releases/latest))*

## 왜 클라우드 우선(Cloud-First)인가요?

Whisperi는 로컬 `whisper.cpp` 모델 대신 OpenAI, Groq, Mistral 등 **클라우드 음성 인식 서비스**를 중심으로 동작합니다. 일반 PC에서 로컬 모델로 충분한 속도와 한국어/한영 혼용 인식 정확도를 내기 어렵지만, 클라우드 API는 **거의 실시간에 가까운 속도와 높은 정확도**를 제공합니다.

자세한 모델 목록과 추천 조합은 아래 [지원 제공업체](#지원-제공업체)를 참고하세요.

## 주요 기능

<p align="center">
  <img src="assets/overlay-states.svg" alt="Overlay button states: Idle → Recording → Processing" width="700"/>
</p>

- **음성 인식 (Voice Transcription)** — OpenAI, Groq, Mistral, Qwen 등 모델 선택 지원
- **AI 텍스트 다듬기 (Text Enhancement)** — GPT, Claude, Gemini, Groq, Qwen, OpenRouter 모델로 필러 제거·문장부호 복원·서식 정리
- **자동 붙여넣기 (Auto-Paste)** — 일반 앱은 물론 [터미널/CLI 도구](#어디서나-붙여넣기--cli-터미널-포함)에도 직접 입력
- **라이브 받아쓰기 (Beta)** — OpenAI Realtime 또는 Qwen3-ASR-Flash-Realtime으로 말하는 즉시 입력
- **사용자 사전 (Custom Dictionary)** — 자주 쓰는 고유명사·전문 용어 등록으로 인식률 향상
- **받아쓰기 & AI 챗 모드 전환** — 에이전트 이름을 부르면 대화형 AI 비서로 동작
- **유연한 단축키 지원** — 한 번 눌러 토글(Tap) / 누르는 동안 녹음(Push) 모드 모두 지원

## 언어 및 번역 설정

Whisperi의 언어 설정(**설정 > 일반 > 출력 언어**)은 입력 언어가 아니라 **출력 언어**를 제어합니다.

- **자동 감지 (Auto-detect)** — 말한 언어 그대로 출력
- **단일 언어 (Single)** — 예: 한국어 또는 영어로 고정
- **이중 언어 (Bilingual)** — 예: **한국어 + 영어**처럼 한 문장 안에서 두 언어를 섞어 말해도 번역하지 않고 말한 언어 그대로 유지하며 문장부호만 정확히 정리

> **언어 규칙:** 시스템 프롬프트 언어와 무관하게 설정된 출력 언어 규칙이 최우선으로 적용됩니다.

<p align="center">
  <img src="setting_window.png" alt="Whisperi Settings Window" width="700"/>
</p>

## 어디서나 붙여넣기 — CLI 터미널 포함

대부분의 받아쓰기 앱은 일반 GUI 입력창에서만 동작하지만, Whisperi는 네이티브 Win32 `SendInput`으로 실제 키 입력을 시뮬레이션하므로 **명령줄 인터페이스(CLI)**와 **터미널 에뮬레이터**에도 바로 입력할 수 있습니다.

- **Claude Code** — CLI 프롬프트를 손으로 치지 않고 말로 바로 지시
- **Codex CLI** — 코딩 요청 사항을 음성으로 입력
- **모든 터미널 지원** — PowerShell, Windows Terminal, cmd.exe, WSL 터미널

## 지원 제공업체

### 추천 조합

| 단계 | 제공업체 | 모델 | 추천 이유 |
| --- | --- | --- | --- |
| **음성 인식** | Groq | Whisper Large v3 | 실시간의 299배 속도와 뛰어난 한국어/영어 인식 정확도 |
| **텍스트 정제** | Groq | GPT-OSS 120B / LLaMA 3.3 70B | 빠른 응답 속도와 자연스러운 후처리 품질 |

Groq API 키 하나만 있으면 음성 인식 + AI 정제 전체가 **보통 2초 이내**에 완료됩니다.

### 음성 인식 (Voice Transcription)

| 제공업체 | 모델 |
| --- | --- |
| OpenAI | GPT-4o Mini Transcribe, GPT-4o Transcribe, Whisper |
| Groq | Whisper Large v3, Whisper Large v3 Turbo |
| Mistral | Voxtral Mini |
| Qwen | Qwen3 ASR Flash |
| OpenRouter | 모든 모델 — `provider/model-name` 직접 입력 |

### 텍스트 정제 (Text Enhancement)

| 제공업체 | 모델 |
| --- | --- |
| OpenAI | GPT-5.2, GPT-5.2 Pro, GPT-5 Mini, GPT-5 Nano, GPT-4.1 계열 |
| Anthropic | Claude Opus 4.6, Claude Sonnet 4.5, Claude Haiku 4.5 |
| Google Gemini | Gemini 3 Pro, Gemini 3 Flash, Gemini 2.5 Pro, Gemini 2.5 Flash |
| Groq | LLaMA 4 Maverick, LLaMA 4 Scout, Qwen3 32B, GPT-OSS 120B/20B, LLaMA 3.3 70B |
| Qwen | Qwen3 235B (MoE), Qwen3 32B |
| OpenRouter | 모든 모델 — `provider/model-name` 직접 입력 (예: `anthropic/claude-sonnet-4`) |

## 프롬프트 커스터마이징 & 예제

**설정 > AI 향상 > 시스템 프롬프트**에서:
1. `라이트` / `표준` / `풀` 각 강도별 기본 프롬프트를 **직접 클릭해서 바로 수정**할 수 있습니다.
2. 수정 후 원래 상태로 돌아가고 싶을 때는 **`기본값으로 복원`** 버튼 한 번으로 초기화할 수 있습니다.
3. [`examples/prompts/`](../examples/prompts/) 폴더에 언어별 참고용 프롬프트도 제공됩니다:
   - [`custom-prompt-ko.txt`](../examples/prompts/custom-prompt-ko.txt) — 한국어 (Korean)
   - [`custom-prompt-en.txt`](../examples/prompts/custom-prompt-en.txt) — English
   - [`custom-prompt-zh.txt`](../examples/prompts/custom-prompt-zh.txt) — Chinese (中文)

## 알려진 이슈

- **원격 데스크톱 연결 후 전역 단축키가 멈출 때** — RustDesk, Windows RDP, AnyDesk 같은 원격 데스크톱 연결/해제 시 Windows 전역 키보드 훅이 끊길 수 있습니다. 트레이/오버레이를 한 번 클릭해 포커스를 주거나 환경설정에서 단축키를 다시 확인하세요.

## 직접 빌드 및 개발 참여 (Contributing)

**사전 요구 사항:** [Rust](https://rustup.rs/) (stable), [bun](https://bun.sh/), Windows 10/11

```bash
bun install                  # 의존성 설치
bun run tauri dev            # 개발 모드 실행 (Vite + Tauri)
bun run typecheck            # TypeScript 타입 체크
cd src-tauri && cargo test   # Rust 단위 테스트
bun run tauri build          # 프로덕션 빌드
```

## 라이선스 (License)

이 프로젝트는 MIT 라이선스를 따릅니다 — 자세한 내용은 [LICENSE](../LICENSE) 파일을 참고하세요.
