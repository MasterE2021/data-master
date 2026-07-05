// src-tauri/src/lib.rs

use std::sync::{Arc, Mutex};

use tauri::Manager;
#[cfg(not(debug_assertions))]
use tauri_plugin_shell::ShellExt; // 引入 Manager 用于多窗口判断

enum BackendProcess {
    #[cfg(debug_assertions)]
    Dev(std::process::Child),

    #[cfg(not(debug_assertions))]
    Prod(tauri_plugin_shell::process::CommandChild),
}

#[tauri::command]
fn backend_log(level: String, timestamp: String, message: String) {
    let color_code = match level.as_str() {
        "DEBUG" => "\x1b[90m",
        "INFO" => "\x1b[36m",
        "WARN" => "\x1b[33m",
        "ERROR" => "\x1b[31m",
        _ => "\x1b[0m",
    };
    let reset_code = "\x1b[0m";

    let formatted_log = format!(
        "{}[FRONTEND] {} [{}] {}{}",
        color_code, timestamp, level, message, reset_code
    );

    if level == "ERROR" {
        eprintln!("{}", formatted_log);
    } else {
        println!("{}", formatted_log);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let python_process: Arc<Mutex<Option<BackendProcess>>> = Arc::new(Mutex::new(None));
    let python_process_clone = python_process.clone();

    tauri::Builder::default()
        // 1. 注册插件
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        // 2. 注册前端调用的日志命令
        .invoke_handler(tauri::generate_handler![backend_log])
        // 3. 启动 Python 后端
        .setup(move |_app| {
            #[cfg(debug_assertions)]
            {
                let cmd_name = if cfg!(target_os = "windows") {
                    "python"
                } else {
                    "python3"
                };
                match std::process::Command::new(cmd_name)
                    .arg("../server.py")
                    .spawn()
                {
                    Ok(child) => {
                        let mut process_guard = python_process.lock().unwrap();
                        *process_guard = Some(BackendProcess::Dev(child));
                    }
                    Err(e) => eprintln!("开发环境 Python 启动失败: {}", e),
                }
            }

            #[cfg(not(debug_assertions))]
            {
                let sidecar_command = _app.shell().sidecar("server").unwrap();
                match sidecar_command.spawn() {
                    Ok((_rx, child)) => {
                        let mut process_guard = python_process.lock().unwrap();
                        *process_guard = Some(BackendProcess::Prod(child));
                    }
                    Err(e) => eprintln!("生产环境 Python 启动失败: {}", e),
                }
            }
            Ok(())
        })
        // 4. 监听窗口关闭，销毁 Python 进程
        .on_window_event(move |window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                // 【优化建议】判断是否是主窗口，或者是否是最后一个窗口
                // 避免在多窗口应用中，关闭一个子窗口就杀死了后端进程
                let windows = window.app_handle().webview_windows();
                let is_last_window = windows.len() <= 1; // 如果只剩 1 个或 0 个窗口了

                if is_last_window {
                    let mut process_guard = python_process_clone.lock().unwrap();
                    if let Some(process) = process_guard.take() {
                        #[cfg(debug_assertions)]
                        {
                            #[allow(irrefutable_let_patterns)]
                            if let BackendProcess::Dev(mut child) = process {
                                let _ = child.kill();
                            }
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            #[allow(irrefutable_let_patterns)]
                            if let BackendProcess::Prod(child) = process {
                                let _ = child.kill();
                            }
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
