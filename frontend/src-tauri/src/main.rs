// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv::dotenv;
use std::env;
use std::process::Command;
mod cryption;

#[tauri::command]
fn encrypt(input: &str, salt: &str) -> String {
    let key = cryption::convert_to_aes_key(salt);
    let encrypted = cryption::encrypt_string(input, &key);
    hex::encode(&encrypted)
}

#[tauri::command]
fn decrypt(input: &str, salt: &str) -> String {
    let key = cryption::convert_to_aes_key(salt);

    let input_hex = match hex::decode(input) {
        Ok(decoded) => decoded,
        Err(_) => return String::from("解密失败：无效的十六进制字符串"),
    };

    match cryption::decrypt_string(&input_hex, &key) {
        Ok(decrypted) => decrypted,
        Err(err_msg) => err_msg.to_string(),
    }
}

#[tauri::command]
fn add_new_env(key: &str) -> u8 {
    if env::var(key).is_ok() {
        return 2;
    }
    
    // 调用 setx 命令设置环境变量
    let output = Command::new("setx")
        .arg("/M")  // 添加此参数以设置系统环境变量
        .arg(key)
        .arg(key)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            std::env::set_var(key, key);
            0
        },
        Ok(_) => 1,
        Err(_) => 2,
    }
}

#[tauri::command]
fn del_env(key: &str) -> u8 {
    // 删除环境变量
    env::remove_var(key);
    match env::var(key) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

#[tauri::command]
fn update_env(key: &str, new_value: &str) -> u8 {
    // 调用 setx 命令更新系统环境变量
    let output = Command::new("setx")
        .arg("/M")  // 设置为系统环境变量
        .arg(key)
        .arg(new_value)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            std::env::set_var(key, new_value);
            0
        },
        Ok(_) => 1,
        Err(_) => 2,
    }
}

#[tauri::command]
fn get_all_env() -> Vec<(String, String)> {
    env::vars().collect()
}

fn main() {
    dotenv().ok(); // 加载 .env 文件
    // 启用完整回溯信息
    unsafe { env::set_var("RUST_BACKTRACE", "full"); }
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![encrypt, decrypt, get_all_env, add_new_env, del_env, update_env])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
