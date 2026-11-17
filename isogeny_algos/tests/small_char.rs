//! Isogenies between given curves in small characteristic (p <= 4l): the p-adic lift and
//! Couveignes' 2-torsion algorithm against the independent reference (division-polynomial
//! factoring with the doubling test), in characteristic 2, 3 and small odd p.
//!
//! The lift needs j~ to be a simple root of Phi_l(j, Y) mod p (Hensel); for multiple roots it
//! may return None, so those cases are counted apart. Two rational l-isogenies can reach curves
//! with the same j-invariant, so a result is accepted when it is any reference kernel with that
//! codomain j-invariant.
use isogeny_algos::binary::{kernel_polys, kohel_codomain, phi_mod2, BinCurve};
use isogeny_algos::binary_volcano::roots_with_multiplicity;
use isogeny_algos::field::{Field, Rng, Zp};
use isogeny_algos::find::modpoly::Phi;
use isogeny_algos::find::p_torsion::couveignes_binary;
use isogeny_algos::find::small_char::{binary_kernel, generic_kernel, ternary_kernel, Residue};
use isogeny_algos::fpr::{FpR, RMAX};
use isogeny_algos::gf2n::GF2n;
use isogeny_algos::gf3n::GF3n;
use isogeny_algos::poly::Poly;
use isogeny_algos::weier::{self, GWCurve};

fn simple_root<F: Field>(f: &F, phi: &Phi<F>, j: F::E, jt: F::E) -> bool {
    !f.is_zero(phi.grad(f, j, jt).1)
}

/// Over a general Weierstrass curve: for every rational l-isogeny of the reference, run `solve`
/// on (j~, l); returns the number of simple-root cases (each must be recovered).
fn check_gw<F: Field>(
    f: &F,
    e: &GWCurve<F::E>,
    ells: &[usize],
    rng: &mut Rng,
    mut solve: impl FnMut(F::E, usize, &mut Rng) -> Option<Poly<F>>,
) -> usize {
    let mut simple = 0;
    for &ell in ells {
        let phi = Phi::via_crt(f, ell);
        let all = weier::kernel_polys(f, e, ell as u64, rng);
        for h in &all {
            let jt = weier::kohel_codomain(f, e, h).j(f);
            let got = solve(jt, ell, rng);
            if let Some(g) = &got {
                assert!(
                    all.iter()
                        .any(|a| a == g && weier::kohel_codomain(f, e, a).j(f) == jt),
                    "l = {ell}: a kernel the reference does not list"
                );
            }
            if simple_root(f, &phi, e.j(f), jt) {
                simple += 1;
                assert!(got.is_some(), "l = {ell}: simple root not recovered");
            }
        }
    }
    simple
}

#[test]
fn characteristic_two_lift_recovers_every_simple_root_isogeny() {
    let f = GF2n::new(9);
    let mut rng = Rng::new(2);
    let mut simple = 0;
    for trial in 0..6 {
        let e = BinCurve::new(trial & 1, 0, 1 + rng.below((1 << 9) - 1));
        for ell in [3usize, 5, 7, 11, 13] {
            let phi = phi_mod2(&f, ell);
            let all = kernel_polys(&f, &e, ell as u64, &mut rng);
            for h in &all {
                let jt = kohel_codomain(&f, &e, h).j(&f);
                let got = binary_kernel(&f, &e, jt, ell, &mut rng);
                if let Some((g, out)) = &got {
                    assert!(
                        all.iter()
                            .any(|a| a == g && kohel_codomain(&f, &e, a).j(&f) == jt),
                        "l = {ell}, precision {}",
                        out.precision
                    );
                }
                if simple_root(&f, &phi, e.j(&f), jt) {
                    simple += 1;
                    assert!(got.is_some(), "l = {ell}: simple root not recovered");
                }
            }
        }
    }
    assert!(
        simple >= 6,
        "too few simple-root isogenies in the sample ({simple})"
    );
}

#[test]
fn characteristic_three_lift_recovers_every_simple_root_isogeny() {
    let f = GF3n::new(6);
    let mut rng = Rng::new(3);
    let mut simple = 0;
    for _ in 0..8 {
        // ordinary in characteristic 3: a2 != 0
        let a2 = 1 + rng.below(f.size() as u64 - 1);
        let a6 = 1 + rng.below(f.size() as u64 - 1);
        let e = GWCurve::new([0, a2, 0, 0, a6]);
        simple += check_gw(&f, &e, &[5, 7, 11], &mut rng, |jt, ell, rng| {
            ternary_kernel(&f, &e, jt, ell, rng).map(|r| r.0)
        });
    }
    assert!(
        simple >= 4,
        "too few simple-root isogenies in the sample ({simple})"
    );
}

/// p = 53 < 4l for l = 17, 19, 23: Elkies/BMSS over F_p would divide by p; the lift does not.
#[test]
fn small_prime_field_lift_recovers_every_simple_root_isogeny() {
    let p = 53u64;
    let f = Zp::new(p);
    let res = Residue {
        p,
        modulus: vec![0, 1],
    };
    let mut rng = Rng::new(5);
    let mut simple = 0;
    for _ in 0..60 {
        let (a, b) = (rng.below(p), rng.below(p));
        let e = GWCurve::new([0, 0, 0, a, b]);
        if f.is_zero(e.disc(&f)) {
            continue;
        }
        simple += check_gw(&f, &e, &[17, 19, 23], &mut rng, |jt, ell, rng| {
            generic_kernel(&f, &res, &e, jt, ell, |x| vec![x], |d| d[0] % p, rng).map(|r| r.0)
        });
        if simple >= 4 {
            break;
        }
    }
    assert!(
        simple >= 3,
        "too few simple-root isogenies in the sample ({simple})"
    );
}

/// F_{7^4} through `FpR`: the lift over the unramified extension of degree 4.
#[test]
fn small_prime_extension_lift_recovers_every_simple_root_isogeny() {
    let f = FpR::new(7, 4);
    let res = Residue {
        p: 7,
        modulus: f.modulus(),
    };
    let mut rng = Rng::new(11);
    let mut simple = 0;
    for _ in 0..30 {
        let (a, b) = (f.random(&mut rng), f.random(&mut rng));
        let e = GWCurve::new([f.zero(), f.zero(), f.zero(), a, b]);
        if f.is_zero(e.disc(&f)) {
            continue;
        }
        let digits = |x: [u64; RMAX]| x[..4].to_vec();
        let undigits = |d: &[u64]| {
            let mut x = f.zero();
            x[..4].copy_from_slice(&d[..4]);
            x
        };
        simple += check_gw(&f, &e, &[5, 11, 13], &mut rng, |jt, ell, rng| {
            generic_kernel(&f, &res, &e, jt, ell, digits, undigits, rng).map(|r| r.0)
        });
        if simple >= 4 {
            break;
        }
    }
    assert!(
        simple >= 3,
        "too few simple-root isogenies in the sample ({simple})"
    );
}

/// Couveignes' 2-torsion algorithm needs no simple root: it recovers every rational isogeny.
#[test]
fn couveignes_two_torsion_matches_the_reference() {
    let f = GF2n::new(8);
    let mut rng = Rng::new(21);
    let mut cases = 0;
    for trial in 0..6 {
        let e = BinCurve::new(trial & 1, 0, 1 + rng.below((1 << 8) - 1));
        for ell in [3usize, 5, 7, 11, 13] {
            let all = kernel_polys(&f, &e, ell as u64, &mut rng);
            for h in &all {
                let jt = kohel_codomain(&f, &e, h).j(&f);
                let (g, out) = couveignes_binary(&f, &e, jt, ell, &mut rng)
                    .unwrap_or_else(|| panic!("C2 failed, l = {ell}"));
                assert!(
                    all.iter()
                        .any(|a| *a == g && kohel_codomain(&f, &e, a).j(&f) == jt),
                    "l = {ell}, k = {}",
                    out.k
                );
                cases += 1;
            }
        }
    }
    assert!(cases >= 6, "too few rational isogenies ({cases})");
}

/// Past the reach of the 2-adic lift at 126 bits (l > 31): C2 finds the isogeny to every
/// rational neighbour (roots of Phi_l(j, Y) mod 2), each verified inside.
#[test]
fn couveignes_two_torsion_beyond_the_lift() {
    let f = GF2n::new(9);
    let mut rng = Rng::new(23);
    for ell in [41usize, 61] {
        let phi = phi_mod2(&f, ell);
        let mut done = 0;
        for _ in 0..40 {
            let e = BinCurve::new(rng.below(2), 0, 1 + rng.below((1 << 9) - 1));
            let roots = roots_with_multiplicity(&f, &phi.y_poly(&f, e.j(&f)), &mut rng);
            for jt in roots {
                assert!(
                    couveignes_binary(&f, &e, jt, ell, &mut rng).is_some(),
                    "C2 failed, l = {ell}"
                );
                done += 1;
            }
            if done >= 2 {
                break;
            }
        }
        assert!(done >= 1, "no rational {ell}-isogeny in the sample");
    }
}
