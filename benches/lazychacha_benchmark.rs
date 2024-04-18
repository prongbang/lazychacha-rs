use criterion::{Criterion, criterion_group, criterion_main};
use lazychacha::lazychacha::LazyChaCha;

fn criterion_benchmark(c: &mut Criterion) {
    // Given
    let lazychacha = LazyChaCha::new();
    let shared_key = "5330d653f2d3f33b393ca5a88bacb3ac502e01b07b4fa063ebf77654937e1013";
    let plaintext = r#"{"token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxIn0.rTCH8cLoGxAm_xw68z-zXVKi9ie6xJn9tnVWjd_9ftE"}"#;
    let ciphertext = "7581ef119758e7830ef23b3b0b5034a75d891df8e27d1cb59ab16d9a1ae9174f36a577e6aa88e67b113872007f5343ffd4a1f14a14fb20b55b69866fa111d43707a41623d803c0a6e1639838f488760839bf935a752d1a5e94ae1e3d451422d032e0bad5e1dac1f8e8bf2f008a0a960c625262bff8b88400826d88a3da347381c9e7549e040b42d51c";

    c.bench_function("encrypt", |b| b.iter(|| {
        let actual = lazychacha.encrypt(plaintext, shared_key);
        if actual.is_empty() {
            eprintln!("error: is empty")
        }
    }));

    c.bench_function("decrypt", |b| b.iter(|| {
        let actual = lazychacha.decrypt(ciphertext, shared_key);
        if actual.is_empty() {
            eprintln!("error: is empty")
        }
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);