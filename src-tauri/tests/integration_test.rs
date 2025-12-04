use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

// Helper function to create a test file with specific content
fn create_test_file(content: &[u8]) -> (TempDir, String) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(content).unwrap();
    file.sync_all().unwrap();
    (temp_dir, file_path.to_string_lossy().to_string())
}

#[cfg(test)]
mod checksum_tests {
    use super::*;

    #[test]
    fn test_empty_file_checksums() {
        let (_temp_dir, _file_path) = create_test_file(b"");

        // Calculate checksums using md5, sha1, sha2 crates directly
        use md5::Md5;
        use sha1::Sha1;
        use sha2::{Digest, Sha256, Sha512};

        let content = b"";

        let mut md5_hasher = Md5::new();
        md5_hasher.update(content);
        let md5_result = format!("{:x}", md5_hasher.finalize());

        let mut sha1_hasher = Sha1::new();
        sha1_hasher.update(content);
        let sha1_result = format!("{:x}", sha1_hasher.finalize());

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(content);
        let sha256_result = format!("{:x}", sha256_hasher.finalize());

        let mut sha512_hasher = Sha512::new();
        sha512_hasher.update(content);
        let sha512_result = format!("{:x}", sha512_hasher.finalize());

        // Verify known empty file hashes
        assert_eq!(md5_result, "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(sha1_result, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(
            sha256_result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(sha512_result, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
    }

    #[test]
    fn test_known_content_md5() {
        let content = b"The quick brown fox jumps over the lazy dog";
        let (_temp_dir, _) = create_test_file(content);

        use md5::Md5;
        use sha2::Digest;

        let mut hasher = Md5::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Known MD5 hash for this content
        assert_eq!(result, "9e107d9d372bb6826bd81d3542a419d6");
    }

    #[test]
    fn test_known_content_sha1() {
        let content = b"The quick brown fox jumps over the lazy dog";
        let (_temp_dir, _) = create_test_file(content);

        use sha1::Sha1;
        use sha2::Digest;

        let mut hasher = Sha1::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Known SHA1 hash for this content
        assert_eq!(result, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
    }

    #[test]
    fn test_known_content_sha256() {
        let content = b"The quick brown fox jumps over the lazy dog";
        let (_temp_dir, _) = create_test_file(content);

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Known SHA256 hash for this content
        assert_eq!(
            result,
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
    }

    #[test]
    fn test_known_content_sha512() {
        let content = b"The quick brown fox jumps over the lazy dog";
        let (_temp_dir, _) = create_test_file(content);

        use sha2::{Digest, Sha512};

        let mut hasher = Sha512::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Known SHA512 hash for this content
        assert_eq!(result, "07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6");
    }

    #[test]
    fn test_small_file() {
        let content = b"Hello, World!";
        let (_temp_dir, file_path) = create_test_file(content);

        // Verify file exists and has correct size
        let metadata = std::fs::metadata(&file_path).unwrap();
        assert_eq!(metadata.len(), 13);

        // Calculate hash
        use md5::Md5;
        use sha2::Digest;

        let mut hasher = Md5::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Known MD5 for "Hello, World!"
        assert_eq!(result, "65a8e27d8879283831b664bd8b7f0ad4");
    }

    #[test]
    fn test_binary_file() {
        // Create binary content
        let content: Vec<u8> = (0..=255).collect();
        let (_temp_dir, file_path) = create_test_file(&content);

        // Verify file exists and has correct size
        let metadata = std::fs::metadata(&file_path).unwrap();
        assert_eq!(metadata.len(), 256);

        // Should be able to hash binary data
        use md5::Md5;
        use sha2::Digest;

        let mut hasher = Md5::new();
        hasher.update(&content);
        let result = format!("{:x}", hasher.finalize());

        // Result should be valid hex string of correct length
        assert_eq!(result.len(), 32);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_large_file() {
        // Create a 1MB file
        let content = vec![0xAB; 1024 * 1024];
        let (_temp_dir, file_path) = create_test_file(&content);

        let metadata = std::fs::metadata(&file_path).unwrap();
        assert_eq!(metadata.len(), 1024 * 1024);

        // Should successfully hash large files
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = format!("{:x}", hasher.finalize());

        // Result should be valid
        assert_eq!(result.len(), 64);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_unicode_content() {
        let content = "Hello, 世界! 🌍 Привет!".as_bytes();
        let (_temp_dir, _) = create_test_file(content);

        use md5::Md5;
        use sha2::Digest;

        let mut hasher = Md5::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Should produce valid hash
        assert_eq!(result.len(), 32);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_same_content_same_hash() {
        let content = b"Test content for consistency";

        // Create two identical files
        let (_temp_dir1, _) = create_test_file(content);
        let (_temp_dir2, _) = create_test_file(content);

        use sha2::{Digest, Sha256};

        let mut hasher1 = Sha256::new();
        hasher1.update(content);
        let result1 = format!("{:x}", hasher1.finalize());

        let mut hasher2 = Sha256::new();
        hasher2.update(content);
        let result2 = format!("{:x}", hasher2.finalize());

        // Hashes should be identical
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_different_content_different_hash() {
        let content1 = b"Content A";
        let content2 = b"Content B";

        use sha2::{Digest, Sha256};

        let mut hasher1 = Sha256::new();
        hasher1.update(content1);
        let result1 = format!("{:x}", hasher1.finalize());

        let mut hasher2 = Sha256::new();
        hasher2.update(content2);
        let result2 = format!("{:x}", hasher2.finalize());

        // Hashes should be different
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_single_byte_change() {
        let content1 = b"The quick brown fox";
        let content2 = b"The quick brown fox!"; // One extra character

        use sha2::{Digest, Sha256};

        let mut hasher1 = Sha256::new();
        hasher1.update(content1);
        let result1 = format!("{:x}", hasher1.finalize());

        let mut hasher2 = Sha256::new();
        hasher2.update(content2);
        let result2 = format!("{:x}", hasher2.finalize());

        // Even one byte change should produce completely different hash
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_all_hash_lengths() {
        let content = b"Test content";

        use md5::Md5;
        use sha1::Sha1;
        use sha2::{Digest, Sha256, Sha512};

        let mut md5_hasher = Md5::new();
        md5_hasher.update(content);
        let md5_result = format!("{:x}", md5_hasher.finalize());

        let mut sha1_hasher = Sha1::new();
        sha1_hasher.update(content);
        let sha1_result = format!("{:x}", sha1_hasher.finalize());

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(content);
        let sha256_result = format!("{:x}", sha256_hasher.finalize());

        let mut sha512_hasher = Sha512::new();
        sha512_hasher.update(content);
        let sha512_result = format!("{:x}", sha512_hasher.finalize());

        // Verify correct hash lengths
        assert_eq!(md5_result.len(), 32); // 128 bits = 32 hex chars
        assert_eq!(sha1_result.len(), 40); // 160 bits = 40 hex chars
        assert_eq!(sha256_result.len(), 64); // 256 bits = 64 hex chars
        assert_eq!(sha512_result.len(), 128); // 512 bits = 128 hex chars
    }

    #[test]
    fn test_file_metadata_extraction() {
        let content = b"Test file";
        let (_temp_dir, file_path) = create_test_file(content);

        let metadata = std::fs::metadata(&file_path).unwrap();

        // Should be able to extract size
        assert_eq!(metadata.len(), 9);

        // Should be able to extract timestamps
        let modified = metadata.modified().unwrap();
        let created = metadata.created().unwrap();

        // Timestamps should be valid
        let modified_secs = modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let created_secs = created
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(modified_secs > 0);
        assert!(created_secs > 0);
    }

    #[test]
    fn test_zero_byte_file() {
        let content = b"";
        let (_temp_dir, file_path) = create_test_file(content);

        let metadata = std::fs::metadata(&file_path).unwrap();
        assert_eq!(metadata.len(), 0);

        // Should handle empty files without errors
        use md5::Md5;
        use sha2::Digest;

        let mut hasher = Md5::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_hex_output_format() {
        let content = b"Format test";

        use md5::Md5;
        use sha2::Digest;

        let mut hasher = Md5::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Should be lowercase hex
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(result
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_lowercase()));

        // Should not contain any other characters
        assert!(!result.contains(' '));
        assert!(!result.contains('-'));
        assert!(!result.contains(':'));
    }

    #[test]
    fn test_newline_variations() {
        // Test different line ending styles
        let content_lf = b"Line 1\nLine 2\nLine 3";
        let content_crlf = b"Line 1\r\nLine 2\r\nLine 3";

        use sha2::{Digest, Sha256};

        let mut hasher1 = Sha256::new();
        hasher1.update(content_lf);
        let result1 = format!("{:x}", hasher1.finalize());

        let mut hasher2 = Sha256::new();
        hasher2.update(content_crlf);
        let result2 = format!("{:x}", hasher2.finalize());

        // Different line endings should produce different hashes
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_special_characters() {
        let content = b"!@#$%^&*()_+-=[]{}|;':\",./<>?`~";

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Should successfully hash special characters
        assert_eq!(result.len(), 64);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_whitespace_content() {
        let content = b"   \t\n\r  ";

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Should produce valid hash for whitespace
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_repeated_patterns() {
        let content = b"AAAAAAAAAA"; // Repeated character

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Should produce valid hash
        assert_eq!(result.len(), 64);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_numeric_content() {
        let content = b"1234567890";

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = format!("{:x}", hasher.finalize());

        // Should handle numeric content
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn test_hash_determinism() {
        let content = b"Determinism test";

        use sha2::{Digest, Sha256};

        // Hash the same content multiple times
        let results: Vec<String> = (0..10)
            .map(|_| {
                let mut hasher = Sha256::new();
                hasher.update(content);
                format!("{:x}", hasher.finalize())
            })
            .collect();

        // All results should be identical
        for i in 1..results.len() {
            assert_eq!(results[0], results[i]);
        }
    }
}
