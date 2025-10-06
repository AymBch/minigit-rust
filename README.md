# MiniGit 🐙

MiniGit is a **minimal Git commit tracker** built in Rust.  
It provides basic commit tracking for your project without the complexity of full Git.

---

## 🚀 Features

- **`init`** — Initialize a MiniGit repository (`.minigit` folder).
- **`commit <message>`** — Save a snapshot of your project files with a commit message.
- **`log`** — View commit history with timestamps and messages.

---

## 🔧 How It Works

MiniGit scans your project folder (including subfolders), hashes file contents, and stores commits in `.minigit/commits.json`.  
Each commit includes:
- Commit ID
- Commit message
- Timestamp
- File hashes

---

## ⚡ Installation

Clone the repository:
```bash
git clone https://github.com/<your-username>/minigit.git
cd minigit
./install.sh

📜 Usage

# Initialize repository
mingit init

# Commit changes with a message
mingit commit "Initial commit"

# View commit log
mingit log

🛠 Built With
	•	Rust
	•	serde_json
	•	sha2
	•	chrono

  
📄 License

MIT License — see the LICENSE file for details.
