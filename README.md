# 📁 File Manager (fmg)

A command-line file manager written in Rust with tree view and detailed listing capabilities.

## 🌟 Features

- 🎨 List directory contents with colors
- 🌲 Tree view of directory structure
- 📋 Long format listing with file details
- 🖥️ Cross-platform support (Windows, macOS, Linux)

## 🚀 Installation

### From source

```bash
git clone https://github.com/yourusername/file-manage.git
cd file-manage
cargo install --path .
```

### From crates.io (when published)

```bash
cargo install file-manage
```

## 📖 Usage

```bash
# List current directory
fmg list

# List with tree view
fmg list --tree

# List with long format
fmg list --long

# List specific directory
fmg list /path/to/directory

# List with tree view and depth limit
fmg list --tree --depth 2

# Alias for list
fmg ls
```

## 🛠️ Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

## ⚙️ Automation

This project uses GitHub Actions for continuous integration:
- ✅ Code checking on multiple platforms
- 🧪 Automated testing
- 📏 Code formatting validation
- 🧹 Clippy linting

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.