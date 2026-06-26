#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_shell::ShellExt;
use std::sync::{Arc, Mutex};
use tauri_plugin_shell::process::CommandChild;

fn main() {
    let python_process: Arc<Mutex<Option<CommandChild>>> = Arc::new(Mutex::new(None));
    let python_process_clone = python_process.clone();

    tauri::Builder::default()
        // 注册 shell 插件
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            // 使用 sidecar 启动打包后的 Python EXE
            // "server" 对应 tauri.conf.json -> externalBin 中的 "bin/server"
            let sidecar_command = app.shell().sidecar("server").unwrap();
            
            match sidecar_command.spawn() {
                Ok((_rx, child)) => {
                    println!("Python Standalone Backend started successfully.");
                    let mut process_guard = python_process.lock().unwrap();
                    *process_guard = Some(child);
                }
                Err(e) => {
                    eprintln!("Failed to start Python backend: {}", e);
                }
            }
            Ok(())
        })
        .on_window_event(move |_window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let mut process_guard = python_process_clone.lock().unwrap();
                if let Some(child) = process_guard.take() {
                    let _ = child.kill();
                    println!("Python Standalone Backend stopped.");
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}