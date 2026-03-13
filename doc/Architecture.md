# rqr 架构设计文档

## 概述

**rqr** 是一个用 Rust 编写的二维码生成与解析命令行工具，采用分层架构设计，将 CLI 接口、业务逻辑和核心 QR 功能清晰分离。

## 架构原则

- **单一职责**：每个模块专注于特定功能
- **错误统一**：使用 `thiserror` 定义统一的错误类型
- **可测试性**：核心逻辑与 CLI 分离，便于单元测试
- **可扩展性**：模块化设计支持未来功能扩展

## 项目结构

```
rqr/
├── src/
│   ├── main.rs           # CLI 入口点
│   ├── commands/         # 命令处理层
│   │   ├── mod.rs
│   │   ├── encode.rs     # encode 命令实现
│   │   └── decode.rs     # decode 命令实现
│   ├── qr/               # 核心 QR 功能层
│   │   ├── mod.rs
│   │   ├── encoder.rs    # QR 编码器
│   │   ├── decoder.rs    # QR 解码器
│   │   └── output.rs     # 输出格式处理
│   └── utils/            # 工具层
│       ├── mod.rs
│       └── error.rs      # 错误类型定义
└── tests/                # 集成测试
    └── integration_test.rs
```

## 分层架构

### 1. 表现层（CLI Layer）

**文件**: `src/main.rs`

职责：
- 定义命令行接口（使用 `clap` derive 宏）
- 解析用户输入参数
- 调用对应的命令处理函数

设计要点：
- 使用 `clap` 的 derive 模式定义 `Cli` 和 `Commands` 枚举
- 支持 `encode` 和 `decode` 两个子命令
- 参数验证在命令层完成

```rust
#[derive(Parser)]
#[command(name = "rqr", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

### 2. 命令层（Command Layer）

**文件**: `src/commands/encode.rs`, `src/commands/decode.rs`

职责：
- 编排业务逻辑流程
- 协调核心模块完成具体任务
- 处理用户输出（打印结果）

设计要点：
- `encode` 命令：创建编码器 → 生成 QR 码 → 输出到文件或终端
- `decode` 命令：创建解码器 → 从文件或 URL 加载 → 解析并显示结果
- 每个命令函数返回 `Result<()>`，错误向上传播

### 3. 核心层（Core Layer）

**文件**: `src/qr/encoder.rs`, `src/qr/decoder.rs`, `src/qr/output.rs`

这是系统的核心，封装了 QR 码的生成和识别能力。

#### 3.1 编码器（QrEncoder）

```rust
pub struct QrEncoder {
    size: u32,              // 输出图像尺寸
    margin: u32,            // 边距大小
    error_correction: EcLevel,  // 错误纠正级别
}
```

核心方法：
- `new()` - 创建配置好的编码器实例
- `encode()` - 将文本编码为 QR 码
- `to_image()` - 将 QR 码渲染为图像
- `save_to_file()` - 保存为 PNG 文件
- `to_terminal_string()` - 生成终端显示字符串

依赖外部库：
- `qrcode` - QR 码生成算法
- `image` - 图像处理和保存

#### 3.2 解码器（QrDecoder）

```rust
pub struct QrDecoder;
```

核心方法：
- `new()` - 创建解码器实例
- `decode_from_file()` - 从本地文件解码
- `decode_from_url()` - 从网络 URL 解码
- `decode_from_image()` - 从图像缓冲区解码

设计特点：
- 支持多 QR 码检测（一张图可能包含多个码）
- 自动判断输入类型（本地文件 vs URL）
- 非 UTF-8 内容自动转为十六进制显示

依赖外部库：
- `rqrr` - QR 码识别算法
- `image` - 图像加载
- `reqwest` - HTTP 请求（用于 URL 解码）

#### 3.3 输出格式（OutputFormat）

```rust
pub enum OutputFormat {
    Png,      // PNG 图像文件
    Terminal, // 终端 ASCII 显示
    // TODO: Future formats: SVG, JPEG, etc.
}
```

职责：
- 根据文件扩展名确定输出格式
- 当前仅支持 PNG 和 Terminal 两种格式
- 代码中已标记未来扩展点（SVG、JPEG 等），待实现

### 4. 工具层（Utility Layer）

**文件**: `src/utils/error.rs`

#### 错误处理（RqrError）

使用 `thiserror` 定义统一的错误类型：

```rust
#[derive(Error, Debug)]
pub enum RqrError {
    #[error("QR code encoding failed: {0}")]
    EncodingError(String),
    
    #[error("QR code decoding failed: {0}")]
    DecodingError(String),
    
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("File I/O error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Output format not supported: {0}")]
    UnsupportedFormat(String),
}
```

设计优势：
- 统一错误类型简化错误处理
- 自动转换底层库错误（`#[from]`）
- 清晰的错误信息便于调试

## 数据流

### 编码流程

```
用户输入文本
    ↓
main.rs 解析参数
    ↓
commands/encode.rs 接收参数
    ↓
QrEncoder::new() 创建编码器
    ↓
QrEncoder::encode() 生成 QR 码
    ↓
判断输出格式
    ├── Terminal → to_terminal_string() → 终端显示
    └── PNG → to_image() → save_to_file() → 文件保存
```

### 解码流程

```
用户输入路径/URL
    ↓
main.rs 解析参数
    ↓
commands/decode.rs 接收输入
    ↓
判断输入类型
    ├── URL → decode_from_url() → HTTP 获取图像
    └── 文件路径 → decode_from_file() → 本地读取图像
    ↓
decode_from_image() 统一处理
    ↓
rqrr 检测并解码 QR 码
    ↓
输出解码结果
```

## 依赖关系

### 外部依赖

| 依赖 | 用途 | 版本 |
|------|------|------|
| `clap` | 命令行参数解析 | 4.6 |
| `qrcode` | QR 码生成 | 0.14 |
| `rqrr` | QR 码识别 | 0.10 |
| `image` | 图像处理 | 0.25 |
| `thiserror` | 错误处理 | 2 |
| `reqwest` | HTTP 客户端 | 0.13 |
| `hex` | 十六进制编码 | 0.4 |

### 内部模块依赖

```
main.rs
  ├── commands/
  │     ├── encode.rs → qr::encoder, qr::output
  │     └── decode.rs → qr::decoder
  ├── qr/
  │     ├── encoder.rs → utils::error
  │     ├── decoder.rs → utils::error
  │     └── output.rs → utils::error
  └── utils/
        └── error.rs (无内部依赖)
```

## 扩展性设计

### 添加新输出格式

1. 在 `OutputFormat` 枚举中添加新变体
2. 实现 `from_path()` 中的格式识别
3. 在 `encoder.rs` 中添加对应的渲染方法

### 添加新命令

1. 在 `Commands` 枚举中添加新子命令
2. 在 `commands/` 目录创建新模块
3. 在 `main.rs` 的 `match` 中添加处理分支

### 支持新图像格式

- 解码：依赖 `image`  crate 自动支持
- 编码：在 `OutputFormat` 和 `encoder.rs` 中扩展

## 测试策略

- **单元测试**：每个模块的 `#[cfg(test)]` 模块
- **集成测试**：`tests/integration_test.rs`
- **测试工具**：使用 `tempfile` 创建临时文件，使用 `assert_cmd` 测试 CLI

## 最佳实践

1. **错误处理**：所有可失败操作返回 `Result<T>`，使用 `?` 传播错误
2. **参数验证**：在编码器/解码器构造时验证参数有效性
3. **资源管理**：使用 Rust 的所有权系统自动管理资源
4. **类型安全**：利用 Rust 类型系统避免运行时错误
