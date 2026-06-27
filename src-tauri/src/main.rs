// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 这里的名字 data_master_lib 是在 Cargo.toml 里自动定义的
    data_master_lib::run()
}