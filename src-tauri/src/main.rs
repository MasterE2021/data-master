#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

// 仅在生产环境下才导入 ShellExt，消除开发环境下的 warning
#[cfg(not(debug_assertions))]
use tauri_plugin_shell::ShellExt;

enum BackendProcess {
    #[cfg(debug_assertions)]
    Dev(std::process::Child),

    #[cfg(not(debug_assertions))]
    Prod(tauri_plugin_shell::process::CommandChild),
}

fn main() {
    let python_process: Arc<Mutex<Option<BackendProcess>>> = Arc::new(Mutex::new(None));
    let python_process_clone = python_process.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // 参数改为 _app 消除 unused warning
        .setup(move |_app| {
            #[cfg(debug_assertions)]
            {
                println!("====== 运行在 DEV 模式：直接启动 Python 源码 ======");
                let cmd_name = if cfg!(target_os = "windows") {
                    "python"
                } else {
                    "python3"
                };

                // 注意这里：加上 "../" 返回上一级目录，指向根目录的 server.py
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
                println!("====== 运行在 PROD 模式：启动 Sidecar (EXE) ======");
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
        .on_window_event(move |_window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                let mut process_guard = python_process_clone.lock().unwrap();
                if let Some(process) = process_guard.take() {
                    #[cfg(debug_assertions)]
                    {
                        // 强制消除 irrefutable_let_patterns warning
                        #[allow(irrefutable_let_patterns)]
                        if let BackendProcess::Dev(mut child) = process {
                            let _ = child.kill();
                            println!("开发环境 Python 已关闭。");
                        }
                    }

                    #[cfg(not(debug_assertions))]
                    {
                        #[allow(irrefutable_let_patterns)]
                        if let BackendProcess::Prod(child) = process {
                            let _ = child.kill();
                            println!("生产环境 Python (EXE) 已关闭。");
                        }
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
