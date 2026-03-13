//! Integration tests for rqr
//!
//! Tests the complete encode-decode workflow and CLI functionality.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a temporary directory for tests
fn temp_dir() -> TempDir {
    TempDir::new().unwrap()
}

/// Helper function to get the binary command
fn cmd() -> Command {
    Command::cargo_bin("rqr").unwrap()
}

#[test]
fn test_cli_encode_help() {
    let mut command = cmd();
    command.arg("encode").arg("--help");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Encode a QR code"));
}

#[test]
fn test_cli_decode_help() {
    let mut command = cmd();
    command.arg("decode").arg("--help");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Decode a QR code"));
}

#[test]
fn test_cli_main_help() {
    let mut command = cmd();
    command.arg("--help");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("rqr"));
}

#[test]
fn test_cli_version() {
    let mut command = cmd();
    command.arg("--version");
    command
        .assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_encode_decode_roundtrip() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("roundtrip.png");

    // Encode
    let mut encode_cmd = cmd();
    encode_cmd
        .arg("encode")
        .arg("Hello, Integration Test!")
        .arg("-o")
        .arg(&output_path);
    encode_cmd.assert().success();

    assert!(output_path.exists());

    // Decode
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Integration Test!"));
}

#[test]
fn test_encode_with_custom_size() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("custom_size.png");

    let mut command = cmd();
    command
        .arg("encode")
        .arg("Size test")
        .arg("-o")
        .arg(&output_path)
        .arg("-s")
        .arg("400");

    command.assert().success();
    assert!(output_path.exists());

    // Verify image size
    let img = image::open(&output_path).unwrap();
    assert_eq!(img.width(), img.height());
}

#[test]
fn test_encode_with_all_options() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("all_options.png");

    let mut command = cmd();
    command
        .arg("encode")
        .arg("All options test")
        .arg("-o")
        .arg(&output_path)
        .arg("-s")
        .arg("300")
        .arg("-e")
        .arg("H")
        .arg("-m")
        .arg("15");

    command.assert().success();
    assert!(output_path.exists());
}

#[test]
fn test_encode_all_error_correction_levels() {
    let temp_dir = temp_dir();

    for level in ["L", "M", "Q", "H"] {
        let output_path = temp_dir.path().join(format!("ec_{}.png", level));

        let mut command = cmd();
        command
            .arg("encode")
            .arg("Error correction test")
            .arg("-o")
            .arg(&output_path)
            .arg("-e")
            .arg(level);

        command.assert().success();
        assert!(output_path.exists(), "Failed for level {}", level);
    }
}

#[test]
fn test_encode_terminal_output() {
    let mut command = cmd();
    command.arg("encode").arg("Terminal output test").arg("-t");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("Terminal output test"));
}

#[test]
fn test_encode_invalid_error_correction() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("invalid.png");

    let mut command = cmd();
    command
        .arg("encode")
        .arg("Test")
        .arg("-o")
        .arg(&output_path)
        .arg("-e")
        .arg("X");

    command.assert().failure();
}

#[test]
fn test_decode_file_not_found() {
    let mut command = cmd();
    command.arg("decode").arg("/nonexistent/path/file.png");

    command.assert().failure();
}

#[test]
fn test_decode_invalid_file() {
    let temp_dir = temp_dir();
    let invalid_file = temp_dir.path().join("not_an_image.txt");
    fs::write(&invalid_file, "This is not an image").unwrap();

    let mut command = cmd();
    command.arg("decode").arg(&invalid_file);

    command.assert().failure();
}

#[test]
fn test_encode_unicode_content() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("unicode.png");

    let content = "你好世界 🌍 Привет мир";

    // Encode
    let mut encode_cmd = cmd();
    encode_cmd
        .arg("encode")
        .arg(content)
        .arg("-o")
        .arg(&output_path);
    encode_cmd.assert().success();

    // Decode and verify
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(content));
}

#[test]
fn test_encode_url_content() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("url.png");

    let url = "https://example.com/path?query=value&foo=bar";

    // Encode
    let mut encode_cmd = cmd();
    encode_cmd
        .arg("encode")
        .arg(url)
        .arg("-o")
        .arg(&output_path);
    encode_cmd.assert().success();

    // Decode and verify
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(url));
}

#[test]
fn test_encode_long_content() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("long.png");

    let long_content = "a".repeat(500);

    let mut command = cmd();
    command
        .arg("encode")
        .arg(&long_content)
        .arg("-o")
        .arg(&output_path)
        .arg("-s")
        .arg("400");

    command.assert().success();
    assert!(output_path.exists());
}

#[test]
fn test_encode_special_characters() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("special.png");

    let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";

    // Encode
    let mut encode_cmd = cmd();
    encode_cmd
        .arg("encode")
        .arg(special)
        .arg("-o")
        .arg(&output_path);
    encode_cmd.assert().success();

    // Decode and verify
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains(special));
}

#[test]
fn test_encode_empty_content() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("empty.png");

    let mut command = cmd();
    command.arg("encode").arg("").arg("-o").arg(&output_path);

    command.assert().success();
    assert!(output_path.exists());
}

#[test]
fn test_multiple_encode_decode_cycles() {
    let temp_dir = temp_dir();

    for i in 0..5 {
        let output_path = temp_dir.path().join(format!("cycle_{}.png", i));
        let content = format!("Cycle number {}", i);

        // Encode
        let mut encode_cmd = cmd();
        encode_cmd
            .arg("encode")
            .arg(&content)
            .arg("-o")
            .arg(&output_path);
        encode_cmd.assert().success();

        // Decode
        let mut decode_cmd = cmd();
        decode_cmd.arg("decode").arg(&output_path);
        decode_cmd
            .assert()
            .success()
            .stdout(predicate::str::contains(&content));
    }
}

#[test]
fn test_encode_different_margins() {
    let temp_dir = temp_dir();

    for margin in [0, 5, 10, 20] {
        let output_path = temp_dir.path().join(format!("margin_{}.png", margin));

        let mut command = cmd();
        command
            .arg("encode")
            .arg("Margin test")
            .arg("-o")
            .arg(&output_path)
            .arg("-m")
            .arg(margin.to_string());

        command.assert().success();
        assert!(output_path.exists(), "Failed with margin {}", margin);
    }
}

#[test]
fn test_decode_no_qr_code() {
    let temp_dir = temp_dir();
    let blank_image = temp_dir.path().join("blank.png");

    // Create a blank image
    let img = image::ImageBuffer::from_pixel(100, 100, image::Luma([255u8]));
    img.save(&blank_image).unwrap();

    let mut command = cmd();
    command.arg("decode").arg(&blank_image);

    command.assert().failure();
}

#[test]
fn test_encode_overwrite_existing_file() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("overwrite.png");

    // First encode
    let mut command1 = cmd();
    command1
        .arg("encode")
        .arg("First content")
        .arg("-o")
        .arg(&output_path);
    command1.assert().success();

    let first_modified = fs::metadata(&output_path).unwrap().modified().unwrap();

    // Second encode (overwrite)
    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut command2 = cmd();
    command2
        .arg("encode")
        .arg("Second content")
        .arg("-o")
        .arg(&output_path);
    command2.assert().success();

    let second_modified = fs::metadata(&output_path).unwrap().modified().unwrap();

    // File should have been modified
    assert!(second_modified >= first_modified);

    // Verify new content
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Second content"));
}

#[test]
fn test_cli_missing_subcommand() {
    let mut command = cmd();
    command.assert().failure();
}

#[test]
fn test_encode_with_relative_path() {
    let temp_dir = temp_dir();
    let original_dir = std::env::current_dir().unwrap();

    // Change to temp directory
    std::env::set_current_dir(&temp_dir).unwrap();

    let mut command = cmd();
    command
        .arg("encode")
        .arg("Relative path test")
        .arg("-o")
        .arg("relative.png");
    command.assert().success();

    assert!(temp_dir.path().join("relative.png").exists());

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_decode_with_relative_path() {
    let temp_dir = temp_dir();
    let original_dir = std::env::current_dir().unwrap();

    // Create QR code
    let output_path = temp_dir.path().join("for_decode.png");
    let mut encode_cmd = cmd();
    encode_cmd
        .arg("encode")
        .arg("Decode relative")
        .arg("-o")
        .arg(&output_path);
    encode_cmd.assert().success();

    // Change to temp directory
    std::env::set_current_dir(&temp_dir).unwrap();

    // Decode using relative path
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg("for_decode.png");
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Decode relative"));

    // Restore original directory
    std::env::set_current_dir(original_dir).unwrap();
}

#[test]
fn test_encode_multiline_content() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("multiline.png");

    let content = "Line 1\nLine 2\nLine 3";

    let mut command = cmd();
    command
        .arg("encode")
        .arg(content)
        .arg("-o")
        .arg(&output_path);
    command.assert().success();

    // Decode and verify
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Line 1"));
}

#[test]
fn test_encode_with_tabs() {
    let temp_dir = temp_dir();
    let output_path = temp_dir.path().join("tabs.png");

    let content = "Column1\tColumn2\tColumn3";

    let mut command = cmd();
    command
        .arg("encode")
        .arg(content)
        .arg("-o")
        .arg(&output_path);
    command.assert().success();

    // Decode and verify
    let mut decode_cmd = cmd();
    decode_cmd.arg("decode").arg(&output_path);
    decode_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Column1"));
}
