# ✅ rtodo — Terminal To-Do App in Rust

**`rtodo`** is a feature-rich terminal-based To-Do list manager written in Rust using the `ratatui`, `crossterm`, and `serde` ecosystems. It offers a clean and productive TUI interface with support for task creation, editing, sorting, status tracking, and persistent storage.

---

## ✨ Features

- 🧾 **Task management** with title, description, target date, and completion status
- ✅ **Mark tasks complete** with color-coded status (✓ Green for complete, 🔴 Red for overdue)
- 📅 **Sort tasks** by creation date, target date, or completion status
- 🖋 **Add/Edit/Delete** tasks using a dynamic form popup
- 📊 **Live progress bar** showing task completion status
- 💾 **Persistent local storage** using `todos.json`
- 🎨 **Modern TUI** interface using `ratatui` and keyboard navigation
- 🦀 Built in Rust for performance and safety

---

## 📦 Installation

You must have Rust installed:  
➡️ [Install Rust](https://www.rust-lang.org/tools/install)

Then, run:

```bash
cargo install --git https://github.com/Bearcry55/rtodo.git




Or clone manually:

git clone https://github.com/Bearcry55/rtodo.git
cd rtodo
cargo install --path .

Make sure $HOME/.cargo/bin is in your $PATH to run rtodo globally.


🧠 Usage

Run the app:

rtodo

All data is saved to todos.json in the working directory.
🎮 Keyboard Controls
Key	Action
↑ / ↓	Navigate tasks
Space	Toggle complete/incomplete
N	Add new task
E	Edit selected task
D	Delete selected task
S	Sort by created date
T	Sort by target date
C	Sort by completion status
Enter	Submit form (when adding/editing)
Tab / Shift+Tab	Navigate between fields
Esc / Q	Cancel form or exit app
📁 Project Structure

rtodo/
├── src/
│   └── main.rs        # Main app logic
├── Cargo.toml         # Dependencies and metadata
└── todos.json         # Saved tasks (auto-created)

📷 Screenshots

    Coming soon...

🧑‍💻 Author

Deep Narayan Banerjee
GitHub: @Bearcry55
📄 License

Licensed under the MIT License.
Feel free to fork, contribute, and build upon it!
