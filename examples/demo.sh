#!/bin/bash

# rqr 使用示例脚本

echo "=== rqr CLI Tool 示例 ==="
echo ""

# 基本生成示例
echo "1. 生成基本二维码..."
./target/debug/rqr encode "Hello, RQR!" --output examples/basic.png
echo "✅ 生成完成: examples/basic.png"
echo ""

# 大尺寸高容错示例
echo "2. 生成大尺寸高容错二维码..."
./target/debug/rqr encode "https://rust-lang.org" \
  --output examples/rust-website.png \
  --size 400 \
  --error-correction H \
  --margin 8
echo "✅ 生成完成: examples/rust-website.png"
echo ""

# 中文内容示例
echo "3. 生成中文内容二维码..."
./target/debug/rqr encode "你好，世界！🦀 Rust QR 工具" \
  --output examples/chinese.png \
  --size 300
echo "✅ 生成完成: examples/chinese.png"
echo ""

# 终端显示示例
echo "4. 在终端显示二维码..."
./target/debug/rqr encode "终端显示" --terminal
echo ""

# 解析示例
echo "5. 解析二维码..."
echo "解析基本二维码:"
./target/debug/rqr decode examples/basic.png
echo ""

echo "解析中文二维码:"
./target/debug/rqr decode examples/chinese.png
echo ""

echo "6. 从 URL 解析二维码..."
echo "解析网络图片中的二维码:"
./target/debug/rqr decode "https://s2.loli.net/2025/09/10/mv4ewox82dHQLYV.png"
echo ""

echo "=== 所有示例完成 ==="
echo "生成的文件位于 examples/ 目录中"
