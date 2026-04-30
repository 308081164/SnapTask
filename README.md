# SnapTask

<p align="center">
  <strong>Screenshot to Task with AI</strong><br>
  <em>截屏即落库 -- AI 智能任务管理系统</em>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-blue" alt="Platform" />
  <img src="https://img.shields.io/badge/Tauri-2.x-orange" alt="Tauri" />
  <img src="https://img.shields.io/badge/React-18-blue" alt="React" />
  <img src="https://img.shields.io/badge/Rust-1.70+-rust" alt="Rust" />
  <img src="https://img.shields.io/badge/License-MIT-green" alt="License" />
</p>

---

## English

SnapTask is an AI-powered task management desktop application that turns screenshots into actionable tasks. Simply take a screenshot, and the AI will automatically analyze the content, extract key information, and create a structured task entry -- all in one seamless workflow.

## 中文简介

SnapTask 是一款 AI 驱动的桌面任务管理应用，实现"截屏即落库"的智能工作流。只需截取屏幕内容，AI 会自动分析截图、提取关键信息，并创建结构化的任务条目。支持多设备云端同步，让您的任务管理随时随地保持最新。

---

## Features / 功能特性

- **Screenshot Capture** -- Global hotkey screenshot capture, supports full screen, window, and region selection
- **AI Analysis** -- Powered by Qwen Vision (qwen2.5-vl-plus), automatically extracts tasks, deadlines, priorities from screenshots
- **Smart Task Management** -- Create, edit, categorize, and track tasks with rich metadata
- **Reminder System** -- Built-in notification system for task deadlines and reminders
- **Local-First Storage** -- SQLite-based local storage, works fully offline
- **Cloud Sync** -- Optional cloud synchronization across multiple devices
- **Auto Update** -- Built-in Tauri updater for seamless application updates
- **Cross-Platform** -- Native desktop experience on macOS, Windows, and Linux
- **Privacy First** -- All data processed locally, cloud sync is optional

---

## Tech Stack / 技术栈

| Layer | Technology |
|-------|-----------|
| Desktop Framework | [Tauri 2.x](https://tauri.app/) |
| Frontend | [React 18](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/) |
| Styling | CSS Modules |
| State Management | [Zustand](https://zustand-demo.pmnd.rs/) |
| Backend (Rust) | Tauri Commands + SQLite |
| AI Engine | [Qwen Vision (qwen2.5-vl-plus)](https://dashscope.aliyuncs.com/) via DashScope API |
| Sync Server | Node.js + Express + better-sqlite3 |
| CI/CD | GitHub Actions |
| Auto Update | tauri-plugin-updater |

---

## Quick Start / 快速开始

### Prerequisites / 环境要求

- [Node.js](https://nodejs.org/) >= 20
- [Rust](https://www.rust-lang.org/tools/install) >= 1.70
- npm >= 9

### Installation / 安装步骤

```bash
git clone https://github.com/308081164/SnapTask.git
cd SnapTask

# Install dependencies
npm install

# Copy environment configuration
cp .env.example .env

# Edit .env and fill in your AI API key
# VITE_AI_API_KEY=your-dashscope-api-key

# Start development server
npm run tauri dev
```

> **Note:** SnapTask works fully offline without configuring the sync server.

---

## License

This project is licensed under the MIT License.

---

<p align="center">
  Built with ❤️ by <a href="https://github.com/308081164">SnapTask Team</a>
</p>