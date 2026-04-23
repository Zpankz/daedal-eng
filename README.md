# gurim (그림)

OpenAI **`gpt-image-2`** 모델로 이미지를 뚝딱 만드는 작은 Rust CLI.
단일 정적 바이너리, Python·Node.js 불필요.

[![Rust](https://img.shields.io/badge/Rust-stable-orange)](https://rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

## 왜?

기존 OpenAI 이미지 클라이언트는 대부분 Node.js / Python SDK 로 묵직합니다.
`gurim` 은 **2.3MB 정적 바이너리** 하나로 딱 한 가지만 합니다 —
프롬프트를 OpenAI 공식 `/v1/images/generations` 엔드포인트에 보내고,
base64 PNG 를 디코드해서 파일로 저장.

- SDK 의존성 0
- 공식 REST API 직접 호출 (`reqwest` + `rustls`)
- Linux · macOS · Windows · Android(Termux) 모두 지원
- 설정 파일 없음. 환경변수 하나 (`OPENAI_API_KEY`) 만 있으면 끝

## 준비물

- Rust stable toolchain (빌드용) — `rustup` 로 설치
- `OPENAI_API_KEY` 환경변수 — `gpt-image-2` 사용 가능한 계정

## 설치

### 방법 1. cargo install (추천)

```bash
cargo install --git https://github.com/Hostingglobal-Tech/gurim --locked
```

바이너리는 `~/.cargo/bin/gurim` 에 생성됩니다. `PATH` 에 포함돼 있는지 확인하세요.

### 방법 2. 소스 빌드

```bash
git clone https://github.com/Hostingglobal-Tech/gurim
cd gurim
cargo build --release
# target/release/gurim 을 PATH 안 아무 곳이나 복사 (예: ~/bin/gurim)
```

### 방법 3. Claude Code 로 설치

[Claude Code](https://www.claude.com/claude-code) 를 쓰고 있으면 이렇게 시켜보세요:

> `github.com/Hostingglobal-Tech/gurim` 에서 gurim CLI 를 설치해줘. cargo 로 빌드하고 PATH 에 넣어줘.

## 설정

OpenAI API 키를 한 번만 셸 rc 파일에 등록:

```bash
# bash / zsh
echo 'export OPENAI_API_KEY="sk-..."' >> ~/.bashrc
source ~/.bashrc
```

```powershell
# PowerShell
setx OPENAI_API_KEY "sk-..."
```

## 사용

```bash
gurim "흰 배경에 빨간 큐브"
gurim "유화풍으로 달 위에 앉은 파란 고양이" --quality high
gurim "벚꽃 핀 한옥 마당" --size 1024x1536 -o hanok.png
gurim "로고 시안 3가지" -n 3
gurim "스크립트용" --quiet -o out.png   # stdout 에 파일 경로만 출력
```

### 옵션

| Flag | 값 | 기본 |
|---|---|---|
| `--size` | `1024x1024` · `1024x1536` · `1536x1024` · `auto` | `1024x1024` |
| `--quality` | `low` · `medium` · `high` · `auto` | `auto` |
| `-n` | `1..=10` 장 | `1` |
| `-o, --out` | 저장 경로 | 아래 기본 경로 표 참조 |
| `--quiet` | — | off |

### 기본 저장 경로

`--out` 을 생략하면:

| 플랫폼 | 경로 |
|---|---|
| Android (Termux) | `/sdcard/DCIM/gurim-<epoch>.png` (갤러리 자동 등록) |
| Windows | `%USERPROFILE%\Pictures\gurim\gurim-<epoch>.png` |
| macOS / Linux | `$HOME/Pictures/gurim/gurim-<epoch>.png` |
| 직접 지정 | `export GURIM_OUT_DIR=/원하는/경로` |

폴더가 없으면 자동 생성됩니다.

## 예제

프롬프트: *"실사풍 한국 전통 한옥 마당에 벚꽃이 만발한 봄날 오후, 따뜻한 햇빛, 기와 지붕 디테일 정교, 고해상도 사진"*
크기: `1024x1536` · 품질: `high`

![한옥 벚꽃](examples/hanok-blossom.png)

프롬프트: *"a futuristic seoul skyline at sunset, photorealistic"*
크기: `1024x1024` · 품질: `low`

![서울 스카이라인](examples/seoul-skyline.png)

## 모델

**`gpt-image-2`** 가 코드에 고정돼 있습니다. CLI 플래그로 바꿀 수 없습니다.
다른 모델을 쓰려면 fork 해서 `src/main.rs` 의 `MODEL` 상수를 수정하세요.

## 비용

호출마다 usage 가 stderr 에 출력됩니다:

```
[gurim] usage: {"total_tokens":211, ...}
```

상세 요금은 [OpenAI pricing](https://openai.com/api/pricing/) 에서 `gpt-image-2` 항목 참조.

## 보안

- 소스에 API 키 하드코딩 없음. `OPENAI_API_KEY` 환경변수에서만 읽음.
- TLS 는 `rustls` 사용 (OpenSSL 의존 X).
- 텔레메트리·분석·에러 리포팅 없음.
- 외부 통신: `api.openai.com` 하나뿐.

## 라이선스

MIT — [LICENSE](LICENSE) 참조.

## 기여

바이너리는 작게, 코드는 지루하게 유지합니다. 기능 과잉·플러그인 시스템 없음.
한 파일, 한 목적.
