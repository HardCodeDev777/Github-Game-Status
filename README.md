# 🎮 Github Game Status

**GitHub Game Status** is a simple desktop app built with Rust + Slint that automatically updates your GitHub status based on running games or processes.

## ✅ Features

- Save your GitHub CLI (gh) path and default status text + emoji.
- Add multiple games/processes with custom status and emoji.
- Live monitoring of running processes — updates GitHub status in real time.
- Supports clearing all saved data in one click.
- Lightweight, fully local — no internet services except GitHub CLI.

## 📁 Data Files
- setupdata.json — stores GitHub CLI path and default status.
- processesdata.json — stores list of games and their statuses.
  
Everything is saved locally in human-readable JSON files.

## ⚙️ How It Works

- You set up your GitHub CLI path and default status (text + emoji).
- You add game executables file names and status data for them(e.g., RDR2.exe → "Playing in the Red Dead Redemption 2 🤠").
- Start monitoring from the UI.
- When a listed process runs, the app updates your GitHub status automatically via gh status.
- Once closed, monitoring stops immediately.


## ❗ Dependencies

- `gh ext install vilmibm/gh-user-status`

---

## 📄 License

This project is licensed under the **MIT License**.  
See [LICENSE](LICENSE) for full terms.

---

> 💬 Got feedback, found a bug, or want to contribute? Open an issue or fork the repo!
