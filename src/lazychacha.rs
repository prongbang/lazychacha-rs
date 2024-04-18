use std::sync::Arc;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use chacha20poly1305::aead::generic_array::GenericArray;
use crate::hex;

pub trait Cryptography {
    fn encrypt(&self, plaintext: &str, key: &str) -> String;
    fn decrypt(&self, ciphertext: &str, key: &str) -> String;
}

pub struct LazyChaCha {}

impl LazyChaCha {
    pub fn new() -> Arc<dyn Cryptography> {
        Arc::new(LazyChaCha {})
    }
}

impl Cryptography for LazyChaCha {
    fn encrypt(&self, plaintext: &str, key: &str) -> String {
        let key = hex::decode(key);
        let key = GenericArray::from_slice(key.as_slice());
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref()).unwrap_or(Vec::new());
        let mut combined = Vec::<u8>::with_capacity(nonce.len() + ciphertext.len());
        combined.extend_from_slice(&nonce);
        combined.extend_from_slice(&ciphertext);

        hex::encode(combined)
    }

    fn decrypt(&self, ciphertext: &str, key: &str) -> String {
        let key = hex::decode(key);
        let key = GenericArray::from_slice(key.as_slice());
        let cipher = ChaCha20Poly1305::new(key);

        let ciphertext = hex::decode(ciphertext);
        let nonce = &ciphertext[0..12];
        let nonce = GenericArray::from_slice(nonce);
        let ciphertext = &ciphertext[12..];
        let plaintext = cipher.decrypt(nonce, ciphertext).unwrap_or(Vec::new());

        String::from_utf8(plaintext).unwrap_or("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt() {
        // Given
        let lazychacha = LazyChaCha::new();
        let shared_key = "edf9d004edae8335f095bb8e01975c42cf693ea60322b75cb7c6667dc836fd7e";
        let plaintext = r#"{"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxIn0.rTCH8cLoGxAm_xw68z-zXVKi9ie6xJn9tnVWjd_9ftE"}"#;

        // When
        let actual = lazychacha.encrypt(plaintext, shared_key);

        // Then
        println!("{}", &actual);
        assert_eq!(actual.is_empty(), false)
    }

    #[test]
    fn decrypt() {
        // Given
        let lazychacha = LazyChaCha::new();
        let shared_key = "5330d653f2d3f33b393ca5a88bacb3ac502e01b07b4fa063ebf77654937e1013";
        let ciphertext = "7581ef119758e7830ef23b3b0b5034a75d891df8e27d1cb59ab16d9a1ae9174f36a577e6aa88e67b113872007f5343ffd4a1f14a14fb20b55b69866fa111d43707a41623d803c0a6e1639838f488760839bf935a752d1a5e94ae1e3d451422d032e0bad5e1dac1f8e8bf2f008a0a960c625262bff8b88400826d88a3da347381c9e7549e040b42d51c";
        let plaintext = r#"{"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxIn0.rTCH8cLoGxAm_xw68z-zXVKi9ie6xJn9tnVWjd_9ftE"}"#;

        // When
        let actual = lazychacha.decrypt(ciphertext, shared_key);

        // Then
        assert_eq!(actual, plaintext)
    }
}
