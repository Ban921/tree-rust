# tree-rust 🌲

A fast, feature-rich implementation of the Linux `tree` command written in Rust.

**[繁體中文](#繁體中文)**

---

## Features

- 📁 Display directory structure in a tree format
- 🎨 Colorized output (auto-detects terminal)
- 📊 Multiple output formats: Text, JSON, TOON
- 🔍 Pattern matching with `-P` and `-I` options
- 📏 Depth limiting with `-L`
- 🔧 File permissions, sizes, and timestamps display
- ⚡ Fast and memory-efficient

## Installation

### From Source

```bash
git clone https://github.com/Ban921/tree-rust.git
cd tree-rust
cargo install --path .
```

### Using Cargo

```bash
cargo install tree-rust
```

## Usage

```bash
# Basic usage
tree-rust

# Show hidden files
tree-rust -a

# Directories only
tree-rust -d

# Limit depth
tree-rust -L 2

# Show permissions and type indicators
tree-rust -pF

# JSON output
tree-rust -J

# TOON output (Token-Oriented Object Notation)
tree-rust -T
```

## Options

| Option | Description |
|--------|-------------|
| `-a, --all` | Show hidden files |
| `-d, --dirs-only` | List directories only |
| `-L, --level <N>` | Limit display depth |
| `-f, --full-path` | Print full path prefix |
| `-p, --perm` | Show file permissions |
| `-s, --size` | Show file sizes |
| `-h, --human` | Human-readable sizes |
| `-D, --date` | Show modification date |
| `-F, --classify` | Append type indicator |
| `-t, --sort-time` | Sort by modification time |
| `-r, --reverse` | Reverse sort order |
| `--dirsfirst` | List directories first |
| `-P, --pattern` | Include pattern |
| `-I, --ignore` | Exclude pattern |
| `-C, --color` | Force colorization |
| `-n, --nocolor` | Disable colorization |
| `-J, --json` | JSON output |
| `-T, --toon` | TOON output |

## Output Formats

### Text (Default)
```
project
├── src
│   ├── main.rs
│   └── lib.rs
└── Cargo.toml

1 directory, 3 files
```

### JSON (`-J`)
```json
[{"type": "directory", "name": "project", "contents": [...]}]
```

### TOON (`-T`)
```
# TOON - Tree Output
d:project
  d:src
    f:main.rs
    f:lib.rs
  f:Cargo.toml
```

### TOON with details (`-TphD`, like `ls -la`)
```
# TOON - Tree Output
d:drwxr-xr-x:4.0K:Jan 08 23:50:project
  d:drwxr-xr-x:128B:Jan 08 23:49:src
    f:-rw-r--r--:1.2K:Jan 08 23:49:main.rs
    f:-rw-r--r--:512B:Jan 08 23:49:lib.rs
  f:-rw-r--r--:256B:Jan 08 23:50:Cargo.toml
```

Format: `type:permissions:size:date:name`

TOON (Token-Oriented Object Notation) is optimized for LLMs with minimal token usage.

## License

MIT License

---

# 繁體中文

## tree-rust 🌲

用 Rust 編寫的快速、功能豐富的 Linux `tree` 命令實現。

## 功能特色

- 📁 以樹狀格式顯示目錄結構
- 🎨 彩色輸出（自動偵測終端）
- 📊 多種輸出格式：文字、JSON、TOON
- 🔍 使用 `-P` 和 `-I` 進行模式匹配
- 📏 使用 `-L` 限制深度
- 🔧 顯示檔案權限、大小和時間戳
- ⚡ 快速且記憶體效率高

## 安裝

### 從原始碼安裝

```bash
git clone https://github.com/Ban921/tree-rust.git
cd tree-rust
cargo install --path .
```

### 使用 Cargo

```bash
cargo install tree-rust
```

## 使用方式

```bash
# 基本用法
tree-rust

# 顯示隱藏檔案
tree-rust -a

# 僅顯示目錄
tree-rust -d

# 限制深度
tree-rust -L 2

# 顯示權限和類型指示器
tree-rust -pF

# JSON 輸出
tree-rust -J

# TOON 輸出（Token 導向物件表示法）
tree-rust -T

# TOON 輸出（含詳細資訊，類似 ls -la）
tree-rust -TphD
```

## 選項

| 選項 | 說明 |
|------|------|
| `-a, --all` | 顯示隱藏檔案 |
| `-d, --dirs-only` | 僅列出目錄 |
| `-L, --level <N>` | 限制顯示深度 |
| `-f, --full-path` | 顯示完整路徑 |
| `-p, --perm` | 顯示檔案權限 |
| `-s, --size` | 顯示檔案大小 |
| `-h, --human` | 人類可讀大小 |
| `-D, --date` | 顯示修改日期 |
| `-F, --classify` | 附加類型指示器 |
| `-t, --sort-time` | 按修改時間排序 |
| `-r, --reverse` | 反向排序 |
| `--dirsfirst` | 目錄優先列出 |
| `-P, --pattern` | 包含模式 |
| `-I, --ignore` | 排除模式 |
| `-C, --color` | 強制彩色輸出 |
| `-n, --nocolor` | 停用彩色輸出 |
| `-J, --json` | JSON 輸出 |
| `-T, --toon` | TOON 輸出 |

## 授權條款

MIT 授權
