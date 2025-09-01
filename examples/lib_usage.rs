use rqr::{QrDecoder, QrEncoder, Result};
use std::path::Path;

fn main() -> Result<()> {
    println!("rqr Library Usage Example");
    println!("=========================");

    // 创建编码器
    let encoder = QrEncoder::new(200, 10, "M")?;

    // 生成二维码
    let content = "Library Example - 库使用示例";
    let qr_code = encoder.encode(content)?;

    // 保存为文件
    let output_path = Path::new("examples/lib-example.png");
    encoder.save_to_file(&qr_code, output_path)?;
    println!("✅ Encoded QR code: {}", output_path.display());

    // 在终端显示
    println!("\n📟 Terminal output:");
    let terminal_output = encoder.to_terminal_string(&qr_code);
    println!("{}", terminal_output);

    // 解析二维码
    println!("\n🔍 Decoding the encoded QR code:");
    let decoder = QrDecoder::new();
    let decoded_content = decoder.decode_from_file(output_path)?;

    for (i, content) in decoded_content.iter().enumerate() {
        println!("QR Code #{}: {}", i + 1, content);
    }

    Ok(())
}
