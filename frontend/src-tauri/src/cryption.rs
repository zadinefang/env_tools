use aes::Aes256;
use block_modes::{BlockMode, Cfb};
use sha2::{Sha256, Digest};
use rand::RngCore;
use rand::rngs::OsRng;
use block_modes::block_padding::Pkcs7;

type Aes256Cfb = Cfb<Aes256, Pkcs7>;

pub fn convert_to_aes_key(key_string: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    key
}

pub fn encrypt_string(input_string: &str, key: &[u8; 32]) -> Vec<u8> {
    // 生成随机的 16 字节初始化向量（IV）
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut iv);
    // 创建 CFB 模式的 AES-256 加密器
    let cipher = Aes256Cfb::new_from_slices(key, &iv).unwrap();
    // 加密数据
    let encrypted_data = cipher.encrypt_vec(input_string.as_bytes());
    // 将 IV 和加密数据组合返回
    [iv.to_vec(), encrypted_data].concat()
}

pub fn decrypt_string(encrypted_string: &[u8], key: &[u8; 32]) -> Result<String, String> {
    let iv = &encrypted_string[0..16]; // 提取前16字节的IV

    let cipher = Aes256Cfb::new_from_slices(key, iv)
        .map_err(|_| "Failed to create cipher")?;

    // 解密并处理潜在错误
    let decrypted_bytes = cipher
        .decrypt_vec(&encrypted_string[16..])
        .map_err(|_| "Decryption failed")?;

    // 将解密后的字节转换为字符串
    Ok(String::from_utf8(decrypted_bytes)
        .map_err(|_| "Failed to convert decrypted bytes to String")?)
}

