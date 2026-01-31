<div align="center">

# 🧠 cotask — Versioned Task Manager
### A Git-like task manager built in Rust with full version control mechanics

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Serde](https://img.shields.io/badge/serde-JSON-orange?style=for-the-badge)](https://serde.rs/)
[![CLI](https://img.shields.io/badge/CLI-Task_Manager-blue?style=for-the-badge)](https://github.com)

</div>

---

## 📖 Overview

**cotask** is a command-line task manager built in Rust that works like a simplified Git. Instead of tracking files, it tracks **tasks with full version history**. Every change creates a new snapshot (commit). You can branch, merge, rebase, tag releases, stash changes, and even export the entire repository state.


## 📁 Repository Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── x.sh                          # Installation script
├── src
│   ├── bin
│   │   └── cli.rs                # CLI entry point
│   ├── lib.rs                    # Library root
│   ├── main.rs                   # Main binary entry
│   ├── logic                     # Core business logic
│   │   ├── add_task.rs
│   │   ├── branch.rs
│   │   ├── checkout.rs
│   │   ├── diff.rs
│   │   ├── export.rs
│   │   ├── gc.rs
│   │   ├── import.rs
│   │   ├── init_repo.rs
│   │   ├── list_task.rs
│   │   ├── merge.rs
│   │   ├── mod.rs
│   │   ├── rand_hash.rs
│   │   ├── rebase.rs
│   │   ├── resolve.rs
│   │   ├── revert.rs
│   │   ├── show_help.rs
│   │   ├── show_log.rs
│   │   ├── stash.rs
│   │   └── tag.rs
│   ├── models                    # Data structures
│   │   ├── commit_model.rs
│   │   ├── export_model.rs
│   │   ├── mod.rs
│   │   └── task_model.rs
│   └── storage                   # Persistence layer
│       ├── commit.rs
│       ├── head.rs
│       └── mod.rs
```

### .cotask Directory Structure

```
.cotask/
├── commits/      # Commit snapshots (JSON)
├── refs/         # Branch pointers
├── tags/         # Release tags
├── stash/        # Temporary saved states
└── HEAD          # Current branch or detached commit
```

---

## 🚀 Features

### ✅ Core Task Tracking
- Add tasks
- Mark tasks complete
- Snapshot history automatically

### 🕓 History System
- Commit log
- Checkout commit, branch, or tag
- Diff between commits

### 🌿 Branching & Versioning
- Create/delete branches
- Merge branches
- Rebase branches

### 🏷 Release Management
- Tags (fixed references to commits)

### ⚠️ Conflict Handling
- Detect merge conflicts
- Manual resolution

### 🧹 Maintenance Tools
- Garbage collection (remove unreachable commits)
- Stash system (temporary state save)
- Export/import full repository state

---

## 📦 Installation

### Prerequisites
- Rust toolchain (rustc, cargo)
- Unix-like system (Linux, macOS) for the installation script

### Quick Install

```bash
git clone https://github.com/BartSimpson2911/CoTask
cd CoTask
chmod +x x.sh
./x.sh
```

**What the script does:**
- Builds the project in release mode (`cargo build --release`)
- Installs the binary globally (`cargo install --path .`)
- Makes `cotask` available system-wide

---

## 🛠 Commands Reference

| Command | Description |
|---------|-------------|
| `cotask init` | Initialize a new task repository |
| `cotask export` | Backup repository state to JSON |
| `cotask import <file>` | Restore repository from backup |
| `cotask gc` | Clean unused commits |

| Command | Description |
|---------|-------------|
| `cotask add "task"` | Add a new task |
| `cotask list` | List all current tasks |
| `cotask done <id>` | Mark a task as complete |

| Command | Description |
|---------|-------------|
| `cotask log` | Show commit history |
| `cotask checkout <ref>` | Switch branch/tag/commit |
| `cotask diff <c1> <c2>` | Compare two commits |
| `cotask revert <commit>` | Revert state |

| Command | Description |
|---------|-------------|
| `cotask branch <name>` | Create branch |
| `cotask merge <branch>` | Merge |
| `cotask rebase <branch>` | Rebase |
| `cotask branches` | List branches |
| `cotask branch-delete <name>` | Delete branch |

| Command | Description |
|---------|-------------|
| `cotask tag <name>` | Create tag |
| `cotask tags` | List tags |

| Command | Description |
|---------|-------------|
| `cotask stash` | Save state |
| `cotask stash-pop` | Restore stash |

---

## 💡 Quick Start

```bash
cotask init
cotask add "Learn Rust"
cotask add "Build CLI"
cotask done 1
cotask log
```

---

## 🧠 How It Works

- **Commits** → Snapshot of task state  
- **Branches** → Pointers in `.cotask/refs/`  
- **Tags** → Fixed commit references  
- **HEAD** → Current position  
- **DAG** → Commit graph like Git  

---

## 🦀 Built With

- Rust  
- Serde  
- Standard Library  

---

## 📌 Future Ideas

- Bisect  
- Commit signing  
- Storage compression  
- Remote sync  
- Interactive rebase  
- Hooks  
- Web UI  

---

## 🤝 Contributing

1. Fork  
2. Create branch  
3. Make changes  
4. Submit PR  

---

## 📄 License
MIT LICENSE

---

<div align="center">

### **cotask = Learn Rust + Learn Git Internals + Build a Real System** 🦀

Made with ❤️ using Rust and Serde

</div>
