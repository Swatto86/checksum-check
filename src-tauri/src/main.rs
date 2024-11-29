// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{self, Read};
use sha2::{Sha256, Sha512, Digest};
use sha1::Sha1;
use md5::Md5;
use tauri::{
    Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}
};

#[derive(serde::Serialize)]
struct HashResult {
    md5: String,
    sha1: String,
    sha256: String,
    sha512: String,
}

fn calculate_file_hash(path: &str) -> io::Result<HashResult> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

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
    })
}

#[tauri::command]
async fn calculate_checksum(path: String) -> Result<HashResult, String> {
    calculate_file_hash(&path).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Create menu items
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_i = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            
            // Create the menu
            let menu = Menu::with_items(app, &[&show_i, &hide_i, &quit_i])?;

            // Build the tray
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                        }
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
                            let _ = window.show();
                            let _ = window.set_focus();
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