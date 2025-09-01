# rqr - Rust QR Code CLI Tool

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org) [![License](https://img.shields.io/badge/license-MIT-blue.svg)]()

一个简洁、快速的二维码生成和解析命令行工具，使用纯 Rust 开发。

## 功能特性

- 功能完善：可以在终端完成二维码生成与解析。
- 终端显示：直接在终端中显示二维码。
- 高度可定制：自定义尺寸、错误纠正级别、边距等参数。

## 安装

### 从源码编译

```bash
# 克隆项目
git clone <your-repo-url>
cd rqr

# 编译项目
cargo build --release

# 安装到系统路径（可选）
cargo install --path .
```

## 🚀 使用方法

### 生成二维码

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

### 解析二维码

```bash
# 从图片文件解析二维码
rqr decode qr-image.png

# 解析刚生成的二维码
rqr decode hello.png
```

### 查看帮助

```bash
# 查看总体帮助
rqr --help

# 查看生成命令帮助
rqr encode --help

# 查看解析命令帮助
rqr decode --help
```

## 📖 详细说明

### Encode 命令参数

| 参数 | 短参数 | 默认值 | 描述 |
|------|--------|--------|------|
| `--output` | `-o` | `qr.png` | 输出文件路径 |
| `--size` | `-s` | `200` | 二维码尺寸（像素） |
| `--error-correction` | `-e` | `M` | 错误纠正级别 (L/M/Q/H) |
| `--margin` | `-m` | `10` | 边距大小（模块数） |
| `--terminal` | `-t` | - | 在终端中显示而非保存文件 |

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

## 🛠️ 技术栈

- **[clap](https://crates.io/crates/clap)** - 命令行参数解析
- **[qrcode](https://crates.io/crates/qrcode)** - 二维码生成
- **[rqrr](https://crates.io/crates/rqrr)** - 二维码识别
- **[image](https://crates.io/crates/image)** - 图像处理
- **[thiserror](https://crates.io/crates/thiserror)** - 错误处理
- **[anyhow](https://crates.io/crates/anyhow)** - 错误上下文

## 🧪 测试

```bash
# 运行测试
cargo test

# 生成测试二维码
cargo run -- encode "测试内容" --output test.png

# 解析测试二维码
cargo run -- decode test.png
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
```

### 添加新功能

项目采用模块化设计，易于扩展：

1. **新的输出格式**：在 `qr/output.rs` 中添加新格式支持
2. **批量处理**：在 `commands/` 中添加批量命令
3. **新的解析源**：扩展 `qr/decoder.rs` 支持更多输入格式

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

## 📝 待办功能

- [ ] SVG 输出格式支持
- [ ] 批量处理多个文件
- [ ] 自定义颜色主题
- [ ] Logo 嵌入功能
- [ ] 更多输出格式（PDF、JPEG 等）
- [ ] Web 界面版本

## 📄 许可证

本项目基于 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [qrcode-rust](https://github.com/kennytm/qrcode-rust) - 优秀的 QR 码生成库
- [rqrr](https://github.com/WanzenBug/rqrr) - 高效的 QR 码识别库
- [clap](https://github.com/clap-rs/clap) - 强大的命令行参数解析库

---

**如果这个项目对你有帮助，请给它一个 ⭐！**
