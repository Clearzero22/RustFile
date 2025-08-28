# Rust 终端常用库

在开发 Rust 终端应用程序时，有许多优秀的 crates 可以帮助你提升开发效率和用户体验。以下是一些常用的 Rust 终端库，按功能分类列出：

## 🖨️ 终端输出与美化

1. **[colored](https://crates.io/crates/colored)** - 简单易用的颜色输出库，为终端文字添加颜色。
2. **[console](https://crates.io/crates/console)** - 一个跨平台的终端库，支持颜色、样式、进度条、输入等。
3. **[crossterm](https://crates.io/crates/crossterm)** - 跨平台终端库，支持颜色、光标控制、事件监听、窗口大小调整等。
4. **[termcolor](https://crates.io/crates/termcolor)** - 一个简单、低级别的终端颜色输出库，支持 Windows 和 Unix。
5. **[nu-ansi-term](https://crates.io/crates/nu-ansi-term)** - ANSI 终端颜色库，支持真彩色和样式。

## 📊 进度条与加载动画

1. **[indicatif](https://crates.io/crates/indicatif)** - 功能强大的进度条和状态提示库，支持多种样式和多线程。
2. **[spinner](https://crates.io/crates/spinner)** - 简单的加载动画库，提供多种旋转动画效果。
3. **[tqdm](https://crates.io/crates/tqdm)** - 类似 Python 的 tqdm，提供简洁的进度条实现。

## 🎯 命令行解析

1. **[clap](https://crates.io/crates/clap)** - 功能强大且流行的命令行参数解析库，支持自动生成帮助信息。
2. **[structopt](https://crates.io/crates/structopt)** - 基于结构体的命令行解析库，现已并入 clap v3。
3. **[argh](https://crates.io/crates/argh)** - 受 Go 语言启发的命令行参数解析库，注重类型安全。
4. **[pico-args](https://crates.io/crates/pico-args)** - 轻量级命令行参数解析库，适用于简单场景。

## 🧭 TUI (终端用户界面)

1. **[tui-rs](https://crates.io/crates/tui-rs)** - 一个功能齐全的 TUI 库，支持多种组件（表格、图表、列表等）。
2. **[ratatui](https://crates.io/crates/ratatui)** - tui-rs 的继任者，提供现代化的 TUI 开发体验。
3. **[cursive](https://crates.io/crates/cursive)** - 一个高级 TUI 库，支持多种后端（如 termion、crossterm）。
4. **[tuikit](https://crates.io/crates/tuikit)** - 一个轻量级 TUI 库，专注于灵活性和性能。

## 📝 日志与调试

1. **[log](https://crates.io/crates/log)** - Rust 生态中的标准日志接口，需要配合具体实现（如 env_logger）。
2. **[env_logger](https://crates.io/crates/env_logger)** - 基于环境变量的日志实现，简单易用。
3. **[tracing](https://crates.io/crates/tracing)** - 下一代日志、指标和追踪库，功能强大且灵活。
4. **[fern](https://crates.io/crates/fern)** - 高度可配置的日志库，支持多种输出格式和过滤规则。
5. **[pretty_env_logger](https://crates.io/crates/pretty_env_logger)** - 带颜色输出的 env_logger，提升可读性。

## 🔧 系统交互与实用工具

1. **[dirs](https://crates.io/crates/dirs)** - 跨平台获取标准目录路径（如配置目录、缓存目录等）。
2. **[which](https://crates.io/crates/which)** - 查找可执行文件的路径，类似于 Unix 的 `which` 命令。
3. **[home](https://crates.io/crates/home)** - 获取用户主目录。
4. **[shell-escape](https://crates.io/crates/shell-escape)** - 安全地转义 shell 命令参数。
5. **[tempfile](https://crates.io/crates/tempfile)** - 创建临时文件和目录。
6. **[ctrlc](https://crates.io/crates/ctrlc)** - 处理 Ctrl+C 信号，优雅退出程序。
7. **[signal-hook](https://crates.io/crates/signal-hook)** - 高级信号处理库，支持多种 Unix 信号。

## 🧪 测试与模拟

1. **[assert_cmd](https://crates.io/crates/assert_cmd)** - 断言命令行程序的行为，简化 CLI 测试。
2. **[predicates](https://crates.io/crates/predicates)** - 提供丰富的断言宏，配合 assert_cmd 使用。
3. **[rexpect](https://crates.io/crates/rexpect)** - 类似 expect 的库，用于测试交互式程序。
4. **[tempfile](https://crates.io/crates/tempfile)** - 创建临时文件和目录，用于测试。

## 🌐 网络与异步

1. **[tokio](https://crates.io/crates/tokio)** - 异步运行时和生态系统，支持网络编程和并发任务。
2. **[async-std](https://crates.io/crates/async-std)** - 另一个异步运行时，提供类似标准库的 API。
3. **[reqwest](https://crates.io/crates/reqwest)** - 异步 HTTP 客户端，支持多种功能（如 JSON 解析、重定向等）。
4. **[hyper](https://crates.io/crates/hyper)** - 低级别的 HTTP 实现，适合构建高性能服务。

## 📦 数据序列化与解析

1. **[serde](https://crates.io/crates/serde)** - 强大的序列化和反序列化框架，支持多种格式（JSON、YAML、TOML 等）。
2. **[toml](https://crates.io/crates/toml)** - TOML 格式的序列化和反序列化库。
3. **[serde_json](https://crates.io/crates/serde_json)** - JSON 格式的序列化和反序列化库。
4. **[ron](https://crates.io/crates/ron)** - Rust 对象表示法，类似 JSON 但更贴近 Rust 语法。

## 🧠 其他实用库

1. **[anyhow](https://crates.io/crates/anyhow)** - 简单易用的错误处理库，适合应用程序。
2. **[thiserror](https://crates.io/crates/thiserror)** - 定义错误类型，适合库开发。
3. **[lazy_static](https://crates.io/crates/lazy_static)** - 延迟初始化静态变量。
4. **[parking_lot](https://crates.io/crates/parking_lot)** - 更高效的同步原语（如 Mutex、RwLock）。
5. **[chrono](https://crates.io/crates/chrono)** - 日期和时间处理库。
6. **[regex](https://crates.io/crates/regex)** - 正则表达式库，功能强大且安全。

---

> 📌 **提示**：选择库时，建议优先考虑生态成熟、文档完善、社区活跃的 crates。你可以通过 [crates.io](https://crates.io) 查看库的下载量、版本更新频率和用户评价。