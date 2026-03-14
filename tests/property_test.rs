//! Property-based tests for rqr
//!
//! These tests use proptest to verify that encoding and decoding
//! work correctly for arbitrary inputs.

use proptest::prelude::*;
use rqr::qr::{decoder::QrDecoder, encoder::QrEncoder};

proptest! {
    /// Property: Any string that can be encoded should be decodable
    /// and return the original content
    #[test]
    fn encode_decode_roundtrip(content in ".*") {
        let encoder = QrEncoder::new(400, 10, "M").expect("Failed to create encoder");
        let qr_code = encoder.encode(&content).expect("Failed to encode");

        let image = encoder.to_image(&qr_code).expect("Failed to render");

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_image(image).expect("Failed to decode");

        prop_assert_eq!(results.len(), 1);
        prop_assert_eq!(&results[0], &content);
    }

    /// Property: Different error correction levels should all work
    #[test]
    fn all_error_correction_levels_work(
        content in "[a-zA-Z0-9]{1,100}",
        level in "[LMQH]"
    ) {
        let encoder = QrEncoder::new(300, 10, &level).expect("Failed to create encoder");
        let qr_code = encoder.encode(&content).expect("Failed to encode");

        let image = encoder.to_image(&qr_code).expect("Failed to render");

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_image(image).expect("Failed to decode");

        prop_assert_eq!(&results[0], &content);
    }

    /// Property: Various sizes should produce valid images
    #[test]
    fn various_sizes_produce_valid_images(
        content in "[a-zA-Z0-9]{1,50}",
        size in 100u32..800
    ) {
        let encoder = QrEncoder::new(size, 10, "M").expect("Failed to create encoder");
        let qr_code = encoder.encode(&content).expect("Failed to encode");

        let image = encoder.to_image(&qr_code).expect("Failed to render");

        // Image should be square
        prop_assert_eq!(image.width(), image.height());
        // Image should have reasonable dimensions
        prop_assert!(image.width() > 0);
    }

    /// Property: Various margins should work
    #[test]
    fn various_margins_work(
        content in "[a-zA-Z0-9]{1,50}",
        margin in 0u32..30
    ) {
        let encoder = QrEncoder::new(400, margin, "M").expect("Failed to create encoder");
        let qr_code = encoder.encode(&content).expect("Failed to encode");

        let image = encoder.to_image(&qr_code);

        // Should either succeed or fail gracefully with size too small
        match image {
            Ok(img) => {
                prop_assert!(img.width() > 0u32);
            }
            Err(e) => {
                // Only acceptable error is size too small
                let err_msg = e.to_string();
                prop_assert!(err_msg.contains("Size too small"));
            }
        }
    }

    /// Property: Unicode content roundtrip
    #[test]
    fn unicode_content_roundtrip(content in "\\PC{0,50}") {
        let encoder = QrEncoder::new(400, 10, "M").expect("Failed to create encoder");
        let qr_code = encoder.encode(&content).expect("Failed to encode");

        let image = encoder.to_image(&qr_code).expect("Failed to render");

        let decoder = QrDecoder::new();
        let results = decoder.decode_from_image(image).expect("Failed to decode");

        prop_assert_eq!(&results[0], &content);
    }
}
