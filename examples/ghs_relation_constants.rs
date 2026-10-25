//! Measured constants for a Gaudry-style relation search on the genus-16
//! Jacobian a GHS descent of the Oakley curves would give, priced in the
//! SAME generic F2mElement arithmetic on both sides (not optimized code).
//!
//! 1. The probability that a random monic degree-g polynomial over F_(2^k)
//!    splits into distinct linear factors, measured for small g and compared
//!    with 1/g!. This is the smoothness probability of a random reduced
//!    divisor's Mumford u-polynomial.
//! 2. Wall-clock per trial (root finding, degree 16, F_(2^31)) and per
//!    affine EC addition on F_(2^155) (1 inversion, 2 mul, 1 squaring) as the
//!    rho step.
//! Output is a measurement of two inner loops. It omits the linear algebra,
//! the factor-base build and divisor arithmetic, so the relation-phase total
//! below is a LOWER bound on the Gaudry route, not its cost.
use crypto_lib::binary_ecc::{F2mElement, F2mPoly, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_isogeny::find_roots_in_f2m;
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use std::time::Instant;

fn rand_elt(rng: &mut SmallRng, m: u32) -> F2mElement {
    let bits: Vec<u32> = (0..m).filter(|_| rng.next_u32() & 1 == 1).collect();
    F2mElement::from_bit_positions(&bits, m)
}

fn rand_monic(rng: &mut SmallRng, g: usize, m: u32) -> F2mPoly {
    let mut c: Vec<F2mElement> = (0..g).map(|_| rand_elt(rng, m)).collect();
    c.push(F2mElement::one(m));
    F2mPoly::from_coeffs(c, m)
}

/// u splits into distinct linear factors over F_(2^k) iff u | x^(2^k) - x,
/// i.e. x^(2^k) = x (mod u): k modular squarings, no root finding.
fn splits(u: &F2mPoly, k: u32, irr: &IrreduciblePoly) -> bool {
    let x = F2mPoly::x(k);
    let mut r = x.rem(u, irr);
    for _ in 0..k {
        r = r.square_mod(u, irr);
    }
    r.add(&x.rem(u, irr)).is_zero()
}

fn main() {
    let mut rng = SmallRng::seed_from_u64(0x6e75_7265);
    // 1. split probability, q = 2^8 so brute-force root finding is cheap.
    let m = 8u32;
    let irr = IrreduciblePoly { degree: 8, low_terms: vec![0, 1, 3, 4] };
    println!("split probability over F_(2^{m}), trials per g:");
    for (g, trials) in [(2usize, 200_000u32), (3, 200_000), (4, 200_000), (5, 400_000), (6, 800_000)] {
        let mut split = 0u32;
        for _ in 0..trials {
            let p = rand_monic(&mut rng, g, m);
            if find_roots_in_f2m(&p, m, &irr).len() == g { split += 1; }
        }
        let fact: f64 = (1..=g).map(|k| k as f64).product();
        let phat = split as f64 / trials as f64;
        println!("  g={g} measured={phat:.6} (n_split={split}) 1/g!={:.6} ratio={:.3}", 1.0 / fact, phat * fact);
    }
    // Agreement of the modular-squaring test with root finding.
    let mut disagree = 0u32;
    for _ in 0..5000 {
        let p = rand_monic(&mut rng, 4, m);
        if splits(&p, m, &irr) != (find_roots_in_f2m(&p, m, &irr).len() == 4) { disagree += 1; }
    }
    println!("splits() vs root finding on 5000 random degree-4 polys over F_(2^8): {disagree} disagreements");
    // 2a. per-trial root finding, degree 16 over F_(2^31).
    let m31 = 31u32;
    let irr31 = IrreduciblePoly { degree: 31, low_terms: vec![0, 3] };
    let reps = 30;
    let polys: Vec<F2mPoly> = (0..reps).map(|_| rand_monic(&mut rng, 16, m31)).collect();
    let t = Instant::now();
    let mut nroots = 0usize;
    for p in &polys { nroots += find_roots_in_f2m(p, m31, &irr31).len(); }
    let t_trial = t.elapsed().as_nanos() as f64 / reps as f64;
    println!("deg-16 root finding over F_(2^31): {t_trial:.0} ns/trial ({reps} trials, {nroots} roots seen)");
    let t = Instant::now();
    let mut nsplit = 0usize;
    for p in &polys { if splits(p, m31, &irr31) { nsplit += 1; } }
    let t_split = t.elapsed().as_nanos() as f64 / reps as f64;
    println!("deg-16 splitting test (x^q mod u) over F_(2^31): {t_split:.0} ns/trial ({nsplit} of {reps} split)");
    // 2b. affine EC add cost proxy on F_(2^155): 1 inversion + 2 mul + 1 square.
    let irr155 = IrreduciblePoly { degree: 155, low_terms: vec![0, 62] };
    let xs: Vec<F2mElement> = (0..200).map(|_| rand_elt(&mut rng, 155)).collect();
    let t = Instant::now();
    let mut sink = F2mElement::zero(155);
    for x in &xs {
        let inv = x.flt_inverse(&irr155).unwrap_or_else(|| F2mElement::one(155));
        let a = inv.mul(x, &irr155);
        let b = a.mul(&inv, &irr155);
        sink = sink.add(&b.square(&irr155));
    }
    let t_step = t.elapsed().as_nanos() as f64 / xs.len() as f64;
    println!("affine-add proxy over F_(2^155): {t_step:.0} ns/step (sink={})", sink.is_zero());
    // Model: relation phase only (>= q relations, 1/16! per trial) vs rho.
    let g = 16.0f64;
    let log2_fact: f64 = (1..=16).map(|k| (k as f64).log2()).sum();
    let l = 31.0;
    let log2_trials = l + log2_fact; // q relations / (1/g!)
    let log2_gaudry_ns = log2_trials + t_split.min(t_trial).log2();
    let log2_rho_ns = 76.5 + t_step.log2();
    println!("g={g}: relation-phase trials 2^{log2_trials:.1} x {:.0} ns = 2^{log2_gaudry_ns:.1} ns (LOWER bound, no LA)", t_split.min(t_trial));
    println!("Oakley 3 rho 2^76.5 x {t_step:.0} ns = 2^{log2_rho_ns:.1} ns; gap = {:+.1} bits (positive = Gaudry relation phase slower)", log2_gaudry_ns - log2_rho_ns);
}
