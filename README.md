# rqr - QR Code CLI Tool & Library

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org) [![License](https://img.shields.io/badge/license-MIT-blue.svg)]()

一个简洁、快速的二维码生成和解析工具，同时提供 Rust 库供其他项目使用。

# ✨ 特性

- **双重用途**：既是命令行工具，也是 Rust 库
- **功能完善**：支持二维码生成与解析，多种图片格式
- **终端显示**：可以直接在终端中显示二维码
- **网络支持**：支持从 URL 直接解析二维码
- **高度可定制**：自定义尺寸、错误纠正级别、边距等参数
- **库 API**：提供简洁的库接口，易于集成

# 安装

## 从源码编译

```bash
# 克隆项目
git clone https://github.com/StudentWeis/rqr.git
cd rqr

# 编译项目
cargo build --release

# 安装到系统路径（可选）
cargo install --path .
```

## 作为依赖使用

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
rqr = { git = "https://github.com/StudentWeis/rqr" }
```

## 从脚本安装

## 🚀 使用方法

### 命令行工具

#### 生成二维码

```bash
# 基本用法 - 生成并保存为 PNG 文件
rqr encode "Hello, World!" --output hello.png

# 在终端中显示二维码
rqr encode "终端显示测试" --terminal

# 自定义参数生成
rqr encode "https://rust-lang.org" \
  --output rust.png \
  --size 300 \
  --error-correction H \
  --margin 5
```

#### 解析二维码

```bash
# 从图片文件解析二维码
rqr decode qr-image.png

# 从网络 URL 解析二维码
rqr decode "https://example.com/qr-code.png"

# 解析刚生成的二维码
rqr decode hello.png
```

### Rust 库使用

#### 基本用法

```rust
use rqr::{encode, decode, Result};

fn main() -> Result<()> {
    // 生成二维码并保存为文件
    encode(
        "Hello, World!".to_string(),
        "hello.png".into(),
        200,
        "M".to_string(),
        10,
        false
    )?;

    // 从文件解析二维码
    decode("hello.png".to_string())?;

    // 从 URL 解析二维码
    decode("https://example.com/qr-code.png".to_string())?;

    Ok(())
}
```

#### 高级用法

```rust
use rqr::{encode, decode, Result};

fn main() -> Result<()> {
    // 生成终端显示的二维码
    encode(
        "Terminal QR".to_string(),
        "dummy.png".into(), // 不会实际保存
        200,
        "H".to_string(),    // 高错误纠正
        5,
        true                // 终端显示
    )?;

    // 解析多个二维码
    decode("multi_qr.png".to_string())?;

    Ok(())
}
```

## 📖 详细说明

### Encode 命令参数

| 参数                 | 短参数 | 默认值   | 描述                     |
| -------------------- | ------ | -------- | ------------------------ |
| `--output`           | `-o`   | `qr.png` | 输出文件路径             |
| `--size`             | `-s`   | `200`    | 二维码尺寸（像素）       |
| `--error-correction` | `-e`   | `M`      | 错误纠正级别 (L/M/Q/H)   |
| `--margin`           | `-m`   | `10`     | 边距大小（模块数）       |
| `--terminal`         | `-t`   | -        | 在终端中显示而非保存文件 |

### 错误纠正级别

- **L (Low)**：约 7% 的错误纠正能力
- **M (Medium)**：约 15% 的错误纠正能力（默认）
- **Q (Quartile)**：约 25% 的错误纠正能力
- **H (High)**：约 30% 的错误纠正能力

## 🌟 使用示例

### 1. 网址二维码

```bash
rqr encode "https://github.com/rust-lang/rust" --output github.png --size 400
```

### 2. 联系信息二维码

```bash
rqr encode "联系人：张三
电话：138-0000-0000
邮箱：zhangsan@example.com" --output contact.png
```

### 3. WiFi 连接二维码

```bash
rqr encode "WIFI:T:WPA;S:MyNetwork;P:password123;;" --output wifi.png
```

### 4. 在终端快速预览

```bash
rqr encode "快速测试" --terminal
```

### 5. 高容错二维码

```bash
rqr encode "重要信息" --error-correction H --output important.png
```

### 6. 从网络解析二维码

```bash
# 从网络图片直接解析二维码
rqr encode "网络测试" --output test.png
rqr decode "https://example.com/test.png"
```

## 🛠️ 技术栈

- **[clap](https://crates.io/crates/clap)** - 命令行参数解析
- **[qrcode](https://crates.io/crates/qrcode)** - 二维码生成
- **[rqrr](https://crates.io/crates/rqrr)** - 二维码识别
- **[image](https://crates.io/crates/image)** - 图像处理
- **[thiserror](https://crates.io/crates/thiserror)** - 错误处理

## 🧪 测试

```bash
# 运行测试
cargo test

# 生成测试二维码
cargo run -- encode "测试内容" --output test.png

# 解析测试二维码
cargo run -- decode test.png

# 从 URL 解析测试二维码
cargo run -- decode "https://example.com/test.png"
```

## 🔧 开发

### 本地开发环境

```bash
# 克隆项目
git clone <your-repo-url>
cd rqr

# 安装依赖并编译
cargo build

# 运行开发版本
cargo run -- encode "开发测试" --terminal

# 测试解码功能
cargo run -- decode "test.png"
cargo run -- decode "https://example.com/qr.png"
```

### 项目结构

```
rqr/
├── src/
│   ├── lib.rs          # 库入口，公开 API
│   ├── main.rs         # CLI 工具入口
│   ├── commands/       # 命令处理模块
│   │   ├── mod.rs
│   │   ├── encode.rs   # 编码命令
│   │   └── decode.rs   # 解码命令
│   ├── qr/             # 核心 QR 功能（内部模块）
│   │   ├── mod.rs
│   │   ├── encoder.rs  # QR 编码器
│   │   ├── decoder.rs  # QR 解码器
│   │   └── output.rs   # 输出格式处理
│   └── utils/          # 工具模块
│       ├── mod.rs
│       └── error.rs    # 错误类型定义
├── examples/           # 使用示例
└── tests/              # 单元测试
```

## 📊 性能

- **生成速度**：毫秒级生成小型二维码
- **内存使用**：低内存占用，适合批量处理
- **文件大小**：生成的二进制文件约 2-3MB（release 模式）

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本项目
2. 创建功能分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add some amazing feature'`
4. 推送到分支：`git push origin feature/amazing-feature`
5. 提交 Pull Request

## 📝 TODO

- [x] 压缩可执行文件大小
- [x] 支持处理图片链接
- [ ] 自定义颜色主题
- [ ] Logo 嵌入功能
- [ ] 更多输出格式（PDF、JPEG 等）
- [ ] 批量处理功能
- [ ] WebAssembly 支持
- [ ] 性能优化和基准测试

## 📄 许可证

本项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [qrcode-rust](https://github.com/kennytm/qrcode-rust) - 优秀的 QR 码生成库
- [rqrr](https://github.com/WanzenBug/rqrr) - 高效的 QR 码识别库
- [clap](https://github.com/clap-rs/clap) - 强大的命令行参数解析库

---

**如果这个项目对你有帮助，请给它一个 ⭐！**
