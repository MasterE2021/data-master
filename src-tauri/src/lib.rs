use std::sync::{Arc, Mutex};

#[cfg(not(debug_assertions))]
use tauri_plugin_shell::ShellExt;

enum BackendProcess {
    #[cfg(debug_assertions)]
    Dev(std::process::Child), 
    
    #[cfg(not(debug_assertions))]
    Prod(tauri_plugin_shell::process::CommandChild), 
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let python_process: Arc<Mutex<Option<BackendProcess>>> = Arc::new(Mutex::new(None));
    let python_process_clone = python_process.clone();

    tauri::Builder::default()
        // 注册刚才安装的两个插件
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(move |_app| {
            #[cfg(debug_assertions)]
            {
                let cmd_name = if cfg!(target_os = "windows") { "python" } else { "python3" };
                match std::process::Command::new(cmd_name).arg("../server.py").spawn() {
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
        .on_window_event(move |_window, event| {
            if let tauri::WindowEvent::Destroyed = event {
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
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}