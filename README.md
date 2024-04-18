# lazychacha-rs

Lazy ChaCha20-Poly1305 in Rust base on RustCrypto: ChaCha20Poly1305.

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/prongbang)

### Algorithm details

- Key exchange: X25519
- Encryption: ChaCha20
- Authentication: Poly1305

### Benchmark

```shell
Gnuplot not found, using plotters backend
encrypt                 time:   [1.2343 µs 1.2372 µs 1.2407 µs]
                        change: [-1.9100% -1.5520% -1.2222%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
Benchmarking decrypt: Warming up for 3.0000 s
decrypt                 time:   [1.0420 µs 1.0439 µs 1.0462 µs]
                        change: [-16.591% -14.215% -12.124%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
```

### How to use

- Generate KeyPair

```rust
let keypair = KeyPair::new();
```

- Key Exchange & Shared Key

```rust
let client_kp = KeyPair::new();
let server_kp = KeyPair::new();
let server_pk = server_kp.pk_string();

let shared_key = SharedKey::new(server_pk, client_kp.sk);
```

- Encrypt

```rust
let lazychacha = LazyChaCha::new();
let shared_key = SharedKey::new(server_pk, client_kp.sk);
let plaintext = r#"{"message": "hi"}"#;

let ciphertext = lazychacha.encrypt(plaintext, shared_key);
```

- Decrypt

```rust
let lazychacha = LazyChaCha::new();
let shared_key = SharedKey::new(server_pk, client_kp.sk);
let ciphertext = "58b99ca4a7";

let plaintext = lazychacha.decrypt(ciphertext, shared_key);
```
