//! Single solve CLI: rho <arm> <a> <b> <n> <gx> <gy> <qx> <qy> <seed> <W> <dp> [beta lambda]
use m31_structure_rho::*;
fn arg(v: &[String], i: usize) -> u64 {
    let s = &v[i];
    match s.strip_prefix("0x") {
        Some(h) => u64::from_str_radix(h, 16).unwrap(),
        None => s.parse().unwrap(),
    }
}
fn main() {
    let v: Vec<String> = std::env::args().collect();
    let job = Job {
        arm: parse_arm(&v[1]),
        a: arg(&v, 2),
        b: arg(&v, 3),
        n: arg(&v, 4),
        g: (arg(&v, 5), arg(&v, 6)),
        q: (arg(&v, 7), arg(&v, 8)),
        seed: arg(&v, 9),
        walks: arg(&v, 10) as usize,
        dp_bits: arg(&v, 11) as u32,
        beta: if v.len() > 13 { arg(&v, 12) } else { 0 },
        lambda: if v.len() > 13 { arg(&v, 13) } else { 0 },
    };
    let s = solve(&job);
    println!("{}", s.json);
    assert!(s.verified);
}
