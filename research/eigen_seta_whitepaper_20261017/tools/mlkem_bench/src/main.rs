use ml_kem::kem::{Decapsulate, Encapsulate};
use ml_kem::{KemCore, MlKem512, MlKem768, MlKem1024, EncodedSizeUser};
use std::time::Instant;
fn bench<K: KemCore>(name: &str) {
    let mut rng = rand::thread_rng();
    let n = 2000;
    let t = Instant::now();
    let mut kp = None;
    for _ in 0..n { kp = Some(K::generate(&mut rng)); }
    let kg = t.elapsed().as_secs_f64() / n as f64;
    let (dk, ek) = kp.unwrap();
    let t = Instant::now();
    let mut last = None;
    for _ in 0..n { last = Some(ek.encapsulate(&mut rng).unwrap()); }
    let en = t.elapsed().as_secs_f64() / n as f64;
    let (ct, k1) = last.unwrap();
    let t = Instant::now();
    let mut k2 = None;
    for _ in 0..n { k2 = Some(dk.decapsulate(&ct).unwrap()); }
    let de = t.elapsed().as_secs_f64() / n as f64;
    assert_eq!(k1, k2.unwrap());
    println!("{name}\tkeygen_us={:.1}\tencaps_us={:.1}\tdecaps_us={:.1}\tek_bytes={}\tct_bytes={}", kg*1e6, en*1e6, de*1e6, ek.as_bytes().len(), ct.len());
}
fn main() { for _ in 0..3 { bench::<MlKem512>("ML-KEM-512"); bench::<MlKem768>("ML-KEM-768"); bench::<MlKem1024>("ML-KEM-1024"); } }
