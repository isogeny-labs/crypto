// Replica of ic_boundary::calibrate_word_xor's loop, run `reps` times; instruction count via `time -l`.
fn mix(mut z: u64) -> u64 { z = z.wrapping_add(0x9e3779b97f4a7c15); z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9); z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb); z ^ (z >> 31) }
fn main() {
    let reps: u64 = std::env::args().nth(1).unwrap().parse().unwrap();
    let words = 4096usize;
    let mut a: Vec<u64> = (0..words as u64).map(mix).collect();
    let b: Vec<u64> = (0..words as u64).map(|i| mix(i + 77)).collect();
    let t = std::time::Instant::now();
    for k in 0..reps { let shift = (k % 7) as usize; for j in 0..words { a[j] ^= b[(j + shift) % words]; } }
    eprintln!("ns/xor {:.4}", t.elapsed().as_nanos() as f64 / (reps * words as u64) as f64);
    std::hint::black_box(&a);
}
