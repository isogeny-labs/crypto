//! Columns of a degree-<=D Boolean Macaulay matrix in N unknowns,
//! cols(N, D) = sum_{k<=D} C(N, k), and the bytes of a dense GF(2) echelon
//! holding `rank` rows of that width: rank * cols / 8. Prints CSV for the
//! memory figure of lead L2 (`../README.md`). Derived arithmetic only.
//!   rustc -O columns.rs && ./columns > columns.csv
fn binom(n: u64, k: u64) -> u128 {
    let mut r: u128 = 1;
    for i in 0..k {
        r = r * (n - i) as u128 / (i + 1) as u128;
    }
    r
}
fn main() {
    println!("N,D,cols,dense_full_bytes,rank_0.25_bytes,rank_0.10_bytes");
    for d in [4u64, 5, 6] {
        for n in (30u64..=52).step_by(2) {
            let cols: u128 = (0..=d).map(|k| binom(n, k)).sum();
            let full = cols * cols / 8;
            println!("{},{},{},{},{},{}", n, d, cols, full, full / 4, full / 10);
        }
    }
}
