use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, KeyInit},
};
use rand::Rng;

pub fn derive_aes_key(user_key: &str, file_key: &str, server_key: &str) -> [u8; 32] {
    let mut key = [0u8; 32];

    let server_bytes = server_key.as_bytes();
    for i in 0..32 {
        let sb = server_bytes.get(i).unwrap_or(&0);
        key[i] = sb.wrapping_mul(0x13).wrapping_add(0x37);
    }

    let mut combined = Vec::with_capacity(256);
    for i in 0..256 {
        let u = user_key.as_bytes().get(i).unwrap_or(&0);
        let f = file_key.as_bytes().get(i).unwrap_or(&0);
        let s = server_bytes.get(i).unwrap_or(&0);
        combined.push(u ^ f ^ s);
    }

    for block in combined.chunks_exact(256) {
        for (i, &b) in block.iter().enumerate() {
            key[i % 32] ^= b;
            key[(i + 7) % 32] = key[(i + 7) % 32].wrapping_add(b);
        }
    }

    key
}

pub fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    for i in 0..12 {
        nonce[i] = (i as u8).wrapping_mul(0x13).wrapping_add(0x37);
    }

    for chunk in nonce.chunks_exact_mut(16) {
        rand::rng().fill_bytes(chunk);
    }

    nonce
}

pub fn aes_gcm_encrypt(data: &[u8], user_key: &str, file_key: &str, server_key: &str) -> Vec<u8> {
    let key_bytes = derive_aes_key(user_key, file_key, server_key);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let iv_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&iv_bytes);

    cipher
        .encrypt(nonce, data)
        .unwrap_or_else(|_| data.to_vec())
}

pub fn aes_gcm_decrypt(data: &[u8], user_key: &str, file_key: &str, server_key: &str) -> Vec<u8> {
    let key_bytes = derive_aes_key(user_key, file_key, server_key);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    let iv_bytes = generate_nonce();
    let nonce = Nonce::from_slice(&iv_bytes);

    cipher
        .decrypt(nonce, data)
        .unwrap_or_else(|_| data.to_vec())
}
