// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use md5::Md5;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{self, Read};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

#[derive(serde::Serialize)]
struct HashResult {
    md5: String,
    sha1: String,
    sha256: String,
    sha512: String,
    file_size: u64,
    modified: String,
    created: String,
}

fn calculate_file_hash(path: &str) -> io::Result<HashResult> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Get file metadata
    let file_size = metadata.len();
    let modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let created = metadata
        .created()?
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Calculate MD5
    let mut md5_hasher = Md5::new();
    md5_hasher.update(&buffer);
    let md5_result = md5_hasher.finalize();
    let md5_hex = format!("{:x}", md5_result);

    // Calculate SHA1
    let mut sha1_hasher = Sha1::new();
    sha1_hasher.update(&buffer);
    let sha1_result = sha1_hasher.finalize();
    let sha1_hex = format!("{:x}", sha1_result);

    // Calculate SHA256
    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(&buffer);
    let sha256_result = sha256_hasher.finalize();
    let sha256_hex = format!("{:x}", sha256_result);

    // Calculate SHA512
    let mut sha512_hasher = Sha512::new();
    sha512_hasher.update(&buffer);
    let sha512_result = sha512_hasher.finalize();
    let sha512_hex = format!("{:x}", sha512_result);

    Ok(HashResult {
        md5: md5_hex,
        sha1: sha1_hex,
        sha256: sha256_hex,
        sha512: sha512_hex,
        file_size,
        modified: modified.to_string(),
        created: created.to_string(),
    })
}

#[tauri::command]
async fn calculate_checksum(path: String) -> Result<HashResult, String> {
    calculate_file_hash(&path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Set up window close handler
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        window_clone.hide().unwrap();
                        api.prevent_close();
                    }
                });
            }

            // Position and show the main window on launch
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                tauri::async_runtime::spawn(async move {
                    // First center the window
                    let _ = window_clone.center();
                    // Then move it up by 15% of the screen height
                    if let Some(monitor) = window_clone.current_monitor().ok().flatten() {
                        if let Ok(position) = window_clone.outer_position() {
                            let monitor_size = monitor.size();
                            let offset_y = (monitor_size.height as f64 * 0.20) as i32;
                            let new_position = tauri::Position::Physical(tauri::PhysicalPosition {
                                x: position.x,
                                y: position.y - offset_y,
                            });
                            let _ = window_clone.set_position(new_position);
                        }
                    }
                    let _ = window_clone.show();
                    let _ = window_clone.set_focus();
                });
            }

            // Create menu items
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            // Create the menu
            let menu = Menu::with_items(app, &[&quit_i])?;

            // Build the tray
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                            }
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![calculate_checksum])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(content: &[u8]) -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content).unwrap();
        file.sync_all().unwrap();
        (temp_dir, file_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_calculate_empty_file() {
        let (_temp_dir, file_path) = create_test_file(b"");
        let result = calculate_file_hash(&file_path).unwrap();

        assert_eq!(result.md5, "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(result.sha1, "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(
            result.sha256,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(result.sha512, "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e");
        assert_eq!(result.file_size, 0);
    }

    #[test]
    fn test_calculate_known_content() {
        let content = b"The quick brown fox jumps over the lazy dog";
        let (_temp_dir, file_path) = create_test_file(content);
        let result = calculate_file_hash(&file_path).unwrap();

        assert_eq!(result.md5, "9e107d9d372bb6826bd81d3542a419d6");
        assert_eq!(result.sha1, "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
        assert_eq!(
            result.sha256,
            "d7a8fbb307d7809469ca9abcb0082e4f8d5651e46d3cdb762d02d0bf37c9e592"
        );
        assert_eq!(result.sha512, "07e547d9586f6a73f73fbac0435ed76951218fb7d0c8d788a309d785436bbb642e93a252a954f23912547d1e8a3b5ed6e1bfd7097821233fa0538f3db854fee6");
        assert_eq!(result.file_size, 43);
    }

    #[test]
    fn test_calculate_file_metadata() {
        let content = b"Test content";
        let (_temp_dir, file_path) = create_test_file(content);
        let result = calculate_file_hash(&file_path).unwrap();

        assert_eq!(result.file_size, 12);
        assert!(result.modified.parse::<u64>().unwrap() > 0);
        assert!(result.created.parse::<u64>().unwrap() > 0);
    }

    #[test]
    fn test_calculate_nonexistent_file() {
        let result = calculate_file_hash("/nonexistent/file/path.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_format() {
        let content = b"Format test";
        let (_temp_dir, file_path) = create_test_file(content);
        let result = calculate_file_hash(&file_path).unwrap();

        // Verify hash formats (lowercase hex)
        assert_eq!(result.md5.len(), 32);
        assert!(result.md5.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(result
            .md5
            .chars()
            .filter(|c| c.is_alphabetic())
            .all(|c| c.is_lowercase()));

        assert_eq!(result.sha1.len(), 40);
        assert!(result.sha1.chars().all(|c| c.is_ascii_hexdigit()));

        assert_eq!(result.sha256.len(), 64);
        assert!(result.sha256.chars().all(|c| c.is_ascii_hexdigit()));

        assert_eq!(result.sha512.len(), 128);
        assert!(result.sha512.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_identical_content_identical_hash() {
        let content = b"Consistency test";
        let (_temp_dir1, file_path1) = create_test_file(content);
        let (_temp_dir2, file_path2) = create_test_file(content);

        let result1 = calculate_file_hash(&file_path1).unwrap();
        let result2 = calculate_file_hash(&file_path2).unwrap();

        assert_eq!(result1.md5, result2.md5);
        assert_eq!(result1.sha1, result2.sha1);
        assert_eq!(result1.sha256, result2.sha256);
        assert_eq!(result1.sha512, result2.sha512);
    }

    #[test]
    fn test_different_content_different_hash() {
        let (_temp_dir1, file_path1) = create_test_file(b"Content A");
        let (_temp_dir2, file_path2) = create_test_file(b"Content B");

        let result1 = calculate_file_hash(&file_path1).unwrap();
        let result2 = calculate_file_hash(&file_path2).unwrap();

        assert_ne!(result1.md5, result2.md5);
        assert_ne!(result1.sha1, result2.sha1);
        assert_ne!(result1.sha256, result2.sha256);
        assert_ne!(result1.sha512, result2.sha512);
    }

    #[test]
    fn test_binary_content() {
        let content: Vec<u8> = (0..=255).collect();
        let (_temp_dir, file_path) = create_test_file(&content);
        let result = calculate_file_hash(&file_path).unwrap();

        assert_eq!(result.file_size, 256);
        assert_eq!(result.md5.len(), 32);
        assert_eq!(result.sha256.len(), 64);
    }

    #[test]
    fn test_large_file() {
        let content = vec![0xAB; 1024 * 1024]; // 1MB
        let (_temp_dir, file_path) = create_test_file(&content);
        let result = calculate_file_hash(&file_path).unwrap();

        assert_eq!(result.file_size, 1024 * 1024);
        assert!(result.md5.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_unicode_content() {
        let content = "Hello, 世界! 🌍".as_bytes();
        let (_temp_dir, file_path) = create_test_file(content);
        let result = calculate_file_hash(&file_path).unwrap();

        assert!(result.file_size > 0);
        assert_eq!(result.md5.len(), 32);
        assert_eq!(result.sha256.len(), 64);
    }

    #[tokio::test]
    async fn test_calculate_checksum_command() {
        let content = b"Command test";
        let (_temp_dir, file_path) = create_test_file(content);
        let result = calculate_checksum(file_path).await;

        assert!(result.is_ok());
        let hash_result = result.unwrap();
        assert_eq!(hash_result.file_size, 12);
    }

    #[tokio::test]
    async fn test_calculate_checksum_command_error() {
        let result = calculate_checksum("/nonexistent/file.txt".to_string()).await;
        assert!(result.is_err());
    }
}
