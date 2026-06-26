// 隐藏 Windows 上的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::sync::{Arc, Mutex};

fn main() {
    // 使用 Rust 标准库的 Child 进程
    let python_process: Arc<Mutex<Option<std::process::Child>>> = Arc::new(Mutex::new(None));
    let python_process_clone = python_process.clone();

    tauri::Builder::default()
        .setup(move |_app| {
            // 在 Tauri 启动时，拉起 Python FastAPI 服务器
            let cmd_name = if cfg!(target_os = "windows") {
                "python"
            } else {
                "python3"
            };
            let script_path = "python/server.py";

            // 使用标准库 Command 启动子进程
            match Command::new(cmd_name).arg(script_path).spawn() {
                Ok(child) => {
                    println!("Python backend started successfully.");
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
            // 当主窗口关闭时，杀掉 Python 后端进程
            if let tauri::WindowEvent::Destroyed = event {
                let mut process_guard = python_process_clone.lock().unwrap();
                // take() 会取出子进程，并把原来的位置置为 None
                if let Some(mut child) = process_guard.take() {
                    let _ = child.kill();
                    println!("Python backend stopped.");
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
