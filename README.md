# ✅ rtodo — Terminal To-Do App in Rust

**`rtodo`** is a feature-rich terminal-based To-Do list manager written in Rust using the `ratatui`, `crossterm`, and `serde` ecosystems. It offers a clean and productive TUI interface with support for task creation, editing, sorting, status tracking, and persistent storage.

---

## ✨ Features

- 🧾 Task management with title, description, target date, and completion status  
- ✅ Mark tasks complete with color-coded status  
- 🔴 Overdue tasks shown in red  
- 📅 Sort tasks by created date, target date, or completion status  
- 🖋 Add, edit, delete tasks with an interactive popup form  
- 📊 Live progress bar showing task completion status  
- 💾 Tasks are saved to `todos.json` locally  
- 🎨 Powered by `ratatui` for a smooth terminal UI  
- 🦀 Written in Rust for speed, safety, and fun!

---

## 📦 Installation 

Make sure [Rust](https://www.rust-lang.org/tools/install) is installed.

Then install with:

```bash
cargo install --git https://github.com/Bearcry55/rtodo.git

```
Or clone manually:
```bash
git clone https://github.com/Bearcry55/rtodo.git
cd rtodo
cargo install --path .

```
## Running the program 
```bash
rtodo 
```


## 🧠 Usage



- All data is saved to todos.json in the working directory.
- 🎮 Keyboard Controls
- key	Action
- ↑ / ↓	Navigate tasks
- Space	Toggle complete/incomplete
- N	Add new task
- E	Edit selected task
- D	Delete selected task
- S	Sort by created date
- T	Sort by target date
- C	Sort by completion status
-  Enter	Submit form (when adding/editing)
- Tab / Shift+Tab	Navigate between fields
- Esc / Q	Cancel form or exit app

---
## 📁 Project Structure

- rtodo/
- ├── src/
- │   └── main.rs        # Main app logic
- ├── Cargo.toml         # Dependencies and metadata
- └── todos.json         # Saved tasks (auto-created)

📷 Screenshots

    Coming soon...

## 🧑‍💻 Author

Deep Narayan Banerjee
GitHub: @Bearcry55
📄 License

Licensed under the MIT License.
Feel free to fork, contribute, and build upon it!
