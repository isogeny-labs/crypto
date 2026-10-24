//! Which GHS magic numbers can exist at all for a tower F_2 <= k <= K,
//! [K:k] = n odd?  MMT (LMS JCM 5, 2002) Thm 5: m(b) = deg Ord_b + c, where
//! Ord_b | x^n - 1 over F_2 is the sigma-annihilator of sqrt(b) and c = 1
//! iff (x+1) does not divide Ord_b.  K is a free F_2[x]/(x^n-1)-module
//! (normal basis theorem), so every divisor of x^n-1 other than 1 is the
//! annihilator of some sqrt(b), b != 0.  For odd n the irreducible factors
//! of x^n-1 are the cyclotomic cosets of 2 mod n, of size = their degree.
//!
//! Usage: ghs_magic_spectrum   (prints spectra and a brute-force check)
use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::ec_trapdoor::{magic_number, FieldTower};
use std::collections::BTreeSet;

fn coset_sizes(n: u32) -> (Vec<u32>, bool) {
    // sizes of cyclotomic cosets of 2 mod n; the coset {0} has size 1.
    let mut seen = vec![false; n as usize];
    let mut sizes = Vec::new();
    for s in 0..n {
        if seen[s as usize] { continue; }
        let (mut c, mut k) = (s, 0u32);
        while !seen[c as usize] { seen[c as usize] = true; c = (c * 2) % n; k += 1; }
        sizes.push(k);
    }
    (sizes, true)
}

/// All m = deg d + [(x+1) does not divide d] over divisors d != 1 of x^n-1.
fn spectrum(n: u32) -> BTreeSet<u32> {
    let (sizes, _) = coset_sizes(n); // sizes[0] is the (x+1) factor, size 1
    let mut sums: BTreeSet<u32> = BTreeSet::new(); // degree sums of subsets of the non-trivial factors
    sums.insert(0);
    for &s in &sizes[1..] {
        let cur: Vec<u32> = sums.iter().cloned().collect();
        for c in cur { sums.insert(c + s); }
    }
    let mut out = BTreeSet::new();
    for &d in &sums {
        out.insert(d + 1);              // (x+1) not included, c = 1 (d = 0 gives Ord = 1: excluded below)
        if d > 0 || true { out.insert(d + 1); } // with (x+1): deg = d+1, c = 0
    }
    // Ord = 1 means sqrt(b) = 0; with (x+1) only: m = 1 (subfield). The
    // subsets-without-(x+1) case with d = 0 is Ord = 1: not allowed, but its
    // value d+1 = 1 coincides with the subfield value, so the set is unchanged.
    out
}

fn main() {
    for n in [3u32, 5, 7, 31, 37, 155, 185] {
        let sp = spectrum(n);
        let mut it = sp.iter().filter(|&&m| m > 1);
        println!("n={n:>3}: cosets={:?}  smallest m>1 = {:?}  (genus 2^(m-1) or 2^(m-1)-1)  count of m values = {}",
            coset_sizes(n).0.iter().take(12).collect::<Vec<_>>(), it.next(), sp.len());
        if n <= 7 { println!("      m spectrum = {:?}", sp); }
    }
    // Brute-force check on all b != 0 of small fields (normal basis theorem
    // predicts the spectrum is attained exactly).
    for (big_n, n, low) in [(9u32, 3u32, vec![0u32, 4]), (10, 5, vec![0, 3]), (15, 5, vec![0, 1]), (14, 7, vec![0, 5])] {
        let irr = IrreduciblePoly { degree: big_n, low_terms: low };
        let tower = FieldTower::new(big_n, n, big_n / n, irr);
        let mut seen = BTreeSet::new();
        for v in 1u64..(1u64 << big_n) {
            let bits: Vec<u32> = (0..big_n).filter(|i| (v >> i) & 1 == 1).collect();
            let b = F2mElement::from_bit_positions(&bits, big_n);
            seen.insert(magic_number(&tower, &F2mElement::zero(big_n), &b));
        }
        let pred = spectrum(n);
        println!("brute N={big_n} n={n}: observed {:?} predicted {:?} {}", seen, pred, if seen == pred { "MATCH" } else { "MISMATCH" });
    }
}
