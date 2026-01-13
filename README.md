# silk2mp3

[![Rust](https://img.shields.io/badge/built_with-Rust-dca282.svg)](https://www.rust-lang.org/)

**silk2mp3** 是一个基于 Rust 编写的高效命令行工具，用于将微信使用的 **Silk v3** 音频文件直接解码并转换为通用的 **MP3** 格式。

本项目基于 [geniusnut/silk2wav](https://github.com/geniusnut/silk2wav) 重构，移除了中间的 WAV 转换步骤，利用 `lame` 编码器直接输出 MP3 文件。

## ✨ 主要特性

- **直接转码**：Silk 解码后直接编码为 MP3，无需生成巨大的临时 WAV 文件。
- **跨平台**：支持 Windows、macOS 和 Linux (需满足编译环境要求)。
- **高性能**：基于 Rust 和 C FFI (Silk SDK & LAME) 实现。
- **可配置**：支持自定义输出音频的采样率。

## 🛠️ 环境要求与编译指南

由于本项目依赖 `silk-rs`（需要 C++ 绑定生成），在编译前必须配置好 LLVM 环境。

### 1. 安装 Rust
如果您尚未安装 Rust，请访问 [rustup.rs](https://rustup.rs/) 进行安装。

### 2. 配置编译环境 (Windows)

在 Windows 上编译此项目，**必须安装 LLVM** 以支持 `bindgen` 生成 C 语言绑定。

#### 方法 A: 使用 Winget 安装 (推荐)
打开 PowerShell 或 CMD，运行：
```powershell
winget install -e --id LLVM.LLVM
```

#### 方法 B: 手动安装
前往 [LLVM Releases](https://github.com/llvm/llvm-project/releases) 下载 Windows 安装包（例如 `LLVM-xx.x.x-win64.exe`）并安装。**安装时请勾选 "Add LLVM to the system PATH"**。

#### 关键步骤：设置环境变量
安装完成后，必须设置 `LIBCLANG_PATH` 环境变量，否则编译会报错。

1. 找到 LLVM 安装目录下的 `bin` 文件夹（通常为 `C:\Program Files\LLVM\bin`）。
2. 设置环境变量：
   - **临时设置** (PowerShell):
     ```powershell
     $env:LIBCLANG_PATH="C:\Program Files\LLVM\bin"
     ```
   - **永久设置**: 在“系统属性” -> “环境变量”中，新建系统变量 `LIBCLANG_PATH`，值为 `C:\Program Files\LLVM\bin`。

### 3. 编译项目

```bash
# 克隆项目
git clone https://github.com/xmind555/silk2mp3.git
cd silk2mp3

# 编译 Release 版本 (推荐)
cargo build --release
```

编译成功后，可执行文件位于 `target/release/silk2mp3.exe`。

## 🚀 使用方法

### 基本用法

```powershell
# 将单个 input.silk 文件转换为 input.mp3
.\target\release\silk2mp3.exe input.silk

# 转换指定目录 (包括子目录) 下所有 .silk 文件为同名 .mp3 文件
.\target\release\silk2mp3.exe C:\path\to\your\silk_files_folder
```

### 常用选项

```powershell
# 指定采样率进行转换 (默认 16000)
# 支持的采样率: 8000, 16000, 24000, 32000, 44100, 48000
.\target\release\silk2mp3.exe input.silk --sample-rate 24000
```

### 帮助信息

```text
Usage: silk2mp3.exe [OPTIONS] <INPUT_PATH>

Arguments:
  <INPUT_PATH>  输入路径 (.silk 文件或包含 .silk 文件的目录)

Options:
  -s, --sample-rate <SAMPLE_RATE>  设置采样率 [default: 16000] [possible values: 8000, 16000, 24000, 32000, 44100, 48000]
  -h, --help                       打印帮助信息
  -V, --version                    打印版本信息
```

## 📄 许可证

本项目基于 MIT License 开源。

## 🙏 致谢

- [geniusnut/silk2wav](https://github.com/geniusnut/silk2wav): 本项目的灵感来源及基础代码。
- [silk-rs](https://crates.io/crates/silk-rs): Silk 解码库。
- [mp3lame-encoder](https://crates.io/crates/mp3lame-encoder): MP3 编码库。
