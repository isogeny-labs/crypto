//! Same-host timing of the post-quantum KEMs in liboqs (via the `oqs`
//! crate), for the comparison in the Eigen-Seta white paper.
//!
//! Each operation is timed individually; the run stops after 2 s or 1000
//! operations, whichever comes first, with at least 5. Prints median and
//! minimum per operation in microseconds, plus key and ciphertext sizes.
//! Every decapsulation is checked against the encapsulated secret.
use oqs::kem::{Algorithm, Kem};
use std::time::{Duration, Instant};

fn stats(mut v: Vec<f64>) -> (f64, f64, usize) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (v[v.len() / 2], v[0], v.len())
}

fn time<F: FnMut()>(mut f: F) -> (f64, f64, usize) {
    let mut v = Vec::new();
    let start = Instant::now();
    while v.len() < 5 || (v.len() < 1000 && start.elapsed() < Duration::from_secs(2)) {
        let t = Instant::now();
        f();
        v.push(t.elapsed().as_secs_f64() * 1e6);
    }
    stats(v)
}

fn main() {
    oqs::init();
    let algs = [
        ("ML-KEM-512", 1, Algorithm::MlKem512),
        ("ML-KEM-768", 3, Algorithm::MlKem768),
        ("ML-KEM-1024", 5, Algorithm::MlKem1024),
        ("HQC-128", 1, Algorithm::Hqc128),
        ("HQC-192", 3, Algorithm::Hqc192),
        ("HQC-256", 5, Algorithm::Hqc256),
        ("BIKE-L1", 1, Algorithm::BikeL1),
        ("BIKE-L3", 3, Algorithm::BikeL3),
        ("BIKE-L5", 5, Algorithm::BikeL5),
        ("Classic-McEliece-348864", 1, Algorithm::ClassicMcEliece348864),
        ("Classic-McEliece-460896", 3, Algorithm::ClassicMcEliece460896),
        ("Classic-McEliece-6688128", 5, Algorithm::ClassicMcEliece6688128),
        ("FrodoKEM-640-AES", 1, Algorithm::FrodoKem640Aes),
        ("FrodoKEM-976-AES", 3, Algorithm::FrodoKem976Aes),
        ("FrodoKEM-1344-AES", 5, Algorithm::FrodoKem1344Aes),
    ];
    for (name, cat, alg) in algs {
        if !alg.is_enabled() {
            println!("kem\t{name}\tcat={cat}\tenabled=false");
            continue;
        }
        let kem = Kem::new(alg).expect("kem");
        let kg = time(|| {
            kem.keypair().expect("keypair");
        });
        let (pk, sk) = kem.keypair().expect("keypair");
        let en = time(|| {
            kem.encapsulate(&pk).expect("encaps");
        });
        let (ct, ss) = kem.encapsulate(&pk).expect("encaps");
        let mut ok = true;
        let de = time(|| {
            ok &= kem.decapsulate(&sk, &ct).expect("decaps") == ss;
        });
        println!(
            "kem\t{name}\tcat={cat}\tkeygen_med_us={:.1}\tkeygen_min_us={:.1}\tkeygen_n={}\tencaps_med_us={:.1}\tencaps_min_us={:.1}\tencaps_n={}\tdecaps_med_us={:.1}\tdecaps_min_us={:.1}\tdecaps_n={}\tpk_bytes={}\tct_bytes={}\tok={ok}",
            kg.0, kg.1, kg.2, en.0, en.1, en.2, de.0, de.1, de.2,
            kem.length_public_key(), kem.length_ciphertext()
        );
    }
}
