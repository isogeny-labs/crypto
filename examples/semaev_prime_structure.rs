//! L4-B step 1 of `research/prime_s5_small_root_leads_20261028/`, natively:
//! for seeded random `z`, is the Newton polytope of `S_{m+1}(·; z)` the full
//! box, and is the polynomial irreducible over `F_p`?  The Rust port of
//! `l4b_step1.gp` (m = 3) and `l4b_step1_s5.gp` (m = 4); see
//! `crypto_lib::cryptanalysis::semaev_prime_poly` for the method and its
//! limits.  One JSON line per `z`.
//!
//! ```text
//! cargo run --release --example semaev_prime_structure -- --m 3,4 --zs 20 --lines 40
//! ```

use crypto_lib::cryptanalysis::semaev_prime_poly::{structure_check, Curve};

fn arg<'a>(args: &'a [String], name: &str, default: &'a str) -> &'a str {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p: u64 = arg(&args, "--p", "1099511627563").parse().unwrap();
    let a: i64 = arg(&args, "--a", "-3").parse().unwrap();
    let b: u64 = arg(&args, "--b", "56").parse().unwrap();
    let zs: usize = arg(&args, "--zs", "20").parse().unwrap();
    let lines: usize = arg(&args, "--lines", "40").parse().unwrap();
    let seed: u64 = arg(&args, "--seed", "20261028").parse().unwrap();
    let curve = Curve::new(p, a.rem_euclid(p as i64) as u64, b);
    for m in arg(&args, "--m", "3,4")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
    {
        let t = std::time::Instant::now();
        for row in structure_check(&curve, m, zs, lines, seed + m as u64) {
            println!(
                "{{\"p\":{p},\"m\":{m},\"z\":{},\"box_vertices_nonzero\":{},\"support_size\":{},\"support_max\":{},\"irreducible_lines\":{}}}",
                row.z,
                row.box_vertices_nonzero,
                row.support_size,
                ((1usize << (m - 1)) + 1).pow(m as u32),
                row.irreducible_line.map_or("null".to_string(), |l| l.to_string())
            );
        }
        eprintln!("m = {m}: {} ms", t.elapsed().as_millis());
    }
}
