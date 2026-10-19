#![allow(dead_code)]
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
use serde_json::json;

#[path = "../../../../src/binary_ecc/mod.rs"]
#[allow(unused_imports)]
mod binary_ecc;
// Link the existing library's optional rho helper; experiments below use
// geometric relations, not this helper.
mod utils {
    pub fn mod_inverse(
        a: &num_bigint::BigUint,
        modulus: &num_bigint::BigUint,
    ) -> Option<num_bigint::BigUint> {
        use num_integer::Integer;
        use num_traits::One;
        let m = num_bigint::BigInt::from(modulus.clone());
        let witness = num_bigint::BigInt::from(a.clone()).extended_gcd(&m);
        if witness.gcd.is_one() {
            witness.x.mod_floor(&m).to_biguint()
        } else {
            None
        }
    }
}

#[path = "../../../../src/cryptanalysis/curve_construction/gf2k.rs"]
mod gf2k;
#[path = "../../../../src/cryptanalysis/curve_construction/modular.rs"]
mod modular;
#[path = "../../../../src/hash/sha256.rs"]
mod sha256_impl;
#[path = "../../../../src/cryptanalysis/curve_construction/weil.rs"]
mod weil;
mod hash {
    pub mod sha256 {
        pub use crate::sha256_impl::*;
    }
}
mod census;
mod cofactor07;
mod controls09;
mod delpezzo;
mod delpezzo_conic;
mod forward07;
mod gluing;
#[path = "../../../../src/cryptanalysis/identity_certificate.rs"]
mod identity_certificate;
mod klein_transfer;
mod kummer08;
mod large_field;
#[path = "../../../../src/cryptanalysis/curve_construction/lpoly.rs"]
mod lpoly;
mod modular_geometry;
mod modular_screen;
mod n5_relations;
mod odd07;
mod polarized;
mod quotient_geometry;
mod quotients06;
mod quotients07;
mod relations08;
mod reverse06;
mod round02;
mod round03_record;
mod round04;
mod round04_record;
mod round04_verify;
mod round05;
mod round05_record;
mod round06;
mod round06_record;
mod round07_record;
mod round08_record;
mod round09_record;
mod round_record;

fn pow(mut a: u64, mut k: u64, p: u64) -> u64 {
    let mut b = 1;
    while k > 0 {
        if k & 1 != 0 {
            b = b * a % p;
        }
        a = a * a % p;
        k >>= 1;
    }
    b
}
fn prime(p: u64) -> bool {
    p >= 2 && (2..).take_while(|d| d * d <= p).all(|d| p % d != 0)
}
fn primitive_root(p: u64) -> u64 {
    let factors = modular::factorize(p - 1);
    (2..p)
        .find(|g| factors.iter().all(|(l, _)| pow(*g, (p - 1) / l, p) != 1))
        .unwrap()
}
fn log_two(g: u64, p: u64) -> u64 {
    let mut x = 1;
    for k in 0..p - 1 {
        if x == 2 {
            return k;
        }
        x = x * g % p;
    }
    panic!("primitive root must reach 2")
}
fn genus(n: u64, ell: u64, base: u64) -> i64 {
    let level = base * ell * ell;
    let g = modular::gamma0(level);
    // All cusps split: for every c|N, ell divides c*N/gcd(c^2,N).
    // Elliptic stabilizers map to ker(chi^2) iff the corresponding
    // root d of d^2+1=0 or d^2+d+1=0 mod ell is in that kernel.
    let in_kernel = |d| pow(d, (ell - 1) / n, ell) == 1;
    let e2_ok = (0..ell).filter(|d| (d * d + 1) % ell == 0).all(in_kernel);
    let e3_ok = (0..ell)
        .filter(|d| (d * d + d + 1) % ell == 0)
        .all(in_kernel);
    let e2 = if e2_ok { n * g.e2 } else { 0 };
    let e3 = if e3_ok { n * g.e3 } else { 0 };
    let numerator = 12 + n * g.mu0 - 3 * e2 - 4 * e3 - 6 * n * g.cusps0;
    assert_eq!(numerator % 12, 0);
    (numerator / 12) as i64
}

// Complete coset enumeration with arbitrary order-n kernel, rather than
// the historical +/-1 kernel which applies only when ell-1=2n.
fn coset_genus(n: u64, ell: u64, level: u64) -> (i64, u64, u64) {
    let units: Vec<u64> = (1..level)
        .filter(|u| u.gcd(&level) == 1 && pow(u % ell, (ell - 1) / n, ell) == 1)
        .collect();
    let mut ids = vec![usize::MAX; (level * level) as usize];
    let mut representatives = Vec::new();
    for c in 0..level {
        for d in 0..level {
            if c.gcd(&d).gcd(&level) != 1 || ids[(c * level + d) as usize] != usize::MAX {
                continue;
            }
            let id = representatives.len();
            for u in &units {
                ids[((u * c % level) * level + u * d % level) as usize] = id;
            }
            representatives.push((c, d));
        }
    }
    let idx = |c, d| ids[(c * level + d) as usize];
    let mut e2 = 0;
    let mut e3 = 0;
    let mut perm = Vec::new();
    for (id, (c, d)) in representatives.iter().copied().enumerate() {
        e2 += u64::from(idx(d, (level - c) % level) == id);
        e3 += u64::from(idx(d, (d + level - c) % level) == id);
        perm.push(idx(c, (c + d) % level));
    }
    let mut seen = vec![false; perm.len()];
    let mut cusps = 0;
    for i in 0..perm.len() {
        if !seen[i] {
            cusps += 1;
            let mut j = i;
            while !seen[j] {
                seen[j] = true;
                j = perm[j];
            }
        }
    }
    let mu = perm.len() as u64;
    let numerator = 12 + mu - 3 * e2 - 4 * e3 - 6 * cusps;
    assert_eq!(numerator % 12, 0);
    ((numerator / 12) as i64, mu, cusps)
}

fn norm_poly(t: i64, n: u32) -> Vec<BigInt> {
    // alpha^2=t alpha-2. Pair (a,b) represents a+b alpha.
    let mut powers = vec![(BigInt::one(), BigInt::zero())];
    for _ in 1..n {
        let (a, b) = powers.last().unwrap();
        powers.push((-2 * b, a + t * b));
    }
    let mut norm = vec![(BigInt::zero(), BigInt::zero()); (2 * n - 1) as usize];
    for (i, (a, b)) in powers.iter().rev().enumerate() {
        for (j, (c0, d0)) in powers.iter().rev().enumerate() {
            let c = c0 + t * d0;
            let d = -d0;
            norm[i + j].0 += a * &c - 2 * b * &d;
            norm[i + j].1 += a * &d + b * &c + t * b * &d;
        }
    }
    norm.into_iter()
        .map(|(a, b)| {
            assert!(b.is_zero());
            a
        })
        .collect()
}
fn klein_counts() -> Vec<i64> {
    (1..=8)
        .map(|k| {
            let f = gf2k::SmallField::new(k);
            let cube = |x| f.mul(f.mul(x, x), x);
            let mut count = 2;
            for x in 0..f.q as u16 {
                for y in 0..f.q as u16 {
                    if f.mul(cube(x), y) ^ cube(y) ^ x == 0 {
                        count += 1;
                    }
                }
            }
            let tr = if k % 3 == 0 {
                3 * weil::frobenius_trace(1, k)
            } else {
                BigInt::zero()
            };
            assert_eq!(BigInt::from(count), BigInt::from(f.q + 1) - tr);
            count
        })
        .collect()
}

fn certificate(n: u32, t: i64) -> identity_certificate::IdentityCertificate {
    use identity_certificate::{Expr as E, IdentityStatement};
    let poly = weil::trace_zero_poly(t, n);
    let as_i64 = |x: &BigInt| i64::try_from(x).unwrap();
    let terms = poly
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.is_zero())
        .map(|(i, c)| {
            let mut p = vec![E::Const(as_i64(c))];
            p.extend((0..i).map(|_| E::var("X")));
            E::product(p)
        })
        .collect();
    let power = |k: u32| E::product((0..k).map(|_| E::var("X")).collect());
    let quadratic = E::sum(vec![
        power(2),
        E::times(E::Const(-t), power(1)),
        E::Const(2),
    ]);
    let statement=IdentityStatement {
        name:format!("trace-zero quotient n={n} t={t}"),
        source:"research/koblitz_modular_ic_20261018/native/src/main.rs; coefficientwise integer checks in results.json".into(),
        variables:vec!["X".into()],degree_bound:2*n,
        lhs:E::times(quadratic,E::sum(terms)),
        rhs:E::sum(vec![power(2*n),E::times(E::Const(-as_i64(&weil::frobenius_trace(t,n))),power(n)),E::Const(1i64<<n)]),
    };
    let cert = identity_certificate::issue(
        statement,
        20261018,
        64,
        "Same-agent native issuance and extra-point replay; not independent-host validation",
        None,
    )
    .unwrap();
    let verdict = identity_certificate::check_with_extra(&cert, Some((2026101802, 128))).unwrap();
    assert!(matches!(
        verdict,
        identity_certificate::Verdict::Accept { .. }
    ));
    cert
}

fn run(out: &std::path::Path) {
    let mut scans = Vec::new();
    let mut best = Vec::new();
    let mut polynomials = Vec::new();
    for n in [3u64, 5, 7, 11, 13, 41, 53, 61, 83, 131] {
        for t in [-1, 1] {
            let a = weil::trace_zero_poly(t, n as u32);
            let b = norm_poly(t, n as u32);
            assert_eq!(a, b);
            polynomials.push(json!({"n":n,"trace":t,"coefficientwise_equal":true,"coefficients_ascending":a.iter().map(ToString::to_string).collect::<Vec<_>>(),"order_at_one":weil::eval(&a,&BigInt::one()).to_string()}));
        }
        let mut accepted = Vec::new();
        for ell in (3..=10_000u64)
            .step_by(2)
            .filter(|ell| *ell != 7 && (ell - 1) % n == 0 && prime(*ell))
        {
            let g = primitive_root(ell);
            let k = log_two(g, ell);
            for m in [n, 2 * n] {
                if (ell - 1) % m != 0 {
                    continue;
                }
                let order = m / k.gcd(&m);
                let row = json!({"n":n,"ell":ell,"generator":g,"log_two":k,"character_order":m,"order_at_two":order,"character_odd":((ell-1)/2)%m!=0,"accepted":order==m,"level":49*ell*ell,"genus":genus(n,ell,49)});
                scans.push(row.clone());
                if order == m {
                    accepted.push(row);
                }
            }
        }
        for m in [n, 2 * n] {
            let row = accepted
                .iter()
                .filter(|v| v["character_order"] == m)
                .min_by_key(|v| v["genus"].as_i64().unwrap())
                .unwrap();
            best.push(row.clone());
        }
    }
    let mut controls = Vec::new();
    for (n, ell, level) in [
        (3, 7, 49),
        (3, 7, 441),
        (3, 13, 169),
        (5, 11, 121),
        (5, 31, 961),
        (7, 29, 841),
    ] {
        let (g, mu, c) = coset_genus(n, ell, level);
        let base = level / (ell * ell);
        let formula = genus(n, ell, base);
        assert_eq!(g, formula);
        assert_eq!(c, n * modular::gamma0(level).cusps0);
        controls.push(
            json!({"n":n,"ell":ell,"level":level,"genus":g,"mu":mu,"cusps":c,"formula_equal":true}),
        );
    }
    // n=9: removing the n=3 trace-zero factor leaves the order-9 part.
    let a9 = weil::trace_zero_poly(-1, 9);
    let a3 = weil::trace_zero_poly(-1, 3);
    let (primitive9, rem) = weil::divide_monic(&a9, &a3);
    assert!(rem.iter().all(Zero::is_zero));
    let baseline = modular::modular_row(131, 0, 263, 0);
    let signed = best
        .iter()
        .find(|r| r["n"] == 131 && r["character_order"] == 262)
        .unwrap();
    let result = json!({"schema":"koblitz.modular-carriers/v1","base_revision":"4fdf4f39d","conductor_bound":10000,"classification":"structural exact arithmetic; no performance measurement","baseline_131":baseline,"best":best,"scan":scans,"polynomials":polynomials,"coset_controls":controls,"klein_counts":klein_counts(),"composite_control":{"n":9,"trace_zero_dimension":8,"primitive_order_dimension":(primitive9.len()-1)/2,"missing_order3_dimension":2},"signed_131_genus_ratio":signed["genus"].as_f64().unwrap()/baseline.genus_split_cusps.unwrap() as f64,"executable_transfer":false,"ic_cost":null,"rho_comparison":null});
    std::fs::write(
        out.join("results.json"),
        serde_json::to_string_pretty(&result).unwrap() + "\n",
    )
    .unwrap();
    for n in [3, 5, 7, 11, 13] {
        for t in [-1, 1] {
            let cert = certificate(n, t);
            std::fs::write(
                out.join(format!("identity-n{n}-t{t}.json")),
                serde_json::to_string_pretty(&cert).unwrap() + "\n",
            )
            .unwrap();
        }
    }
    println!(
        "{}",
        json!({"status":"PASS","best_131":signed,"baseline_genus":baseline.genus_split_cusps,"polynomial_checks":polynomials.len(),"coset_controls":controls.len(),"results":out.join("results.json")})
    );
}
fn main() {
    let args09: Vec<String> = std::env::args().collect();
    if args09.get(1).map(String::as_str) == Some("--record-round09") {
        round09_record::record(std::path::Path::new(
            args09.get(2).expect("fresh round records"),
        ));
        return;
    }
    if args09.get(1).map(String::as_str) == Some("--round09-invariants") {
        kummer08::descent09::traces(std::path::Path::new(args09.get(2).expect("fresh outputs")));
        return;
    }
    if args09.get(1).map(String::as_str) == Some("--verify-round09-descent") {
        kummer08::descent09::verify(std::path::Path::new(args09.get(2).expect("fresh outputs")));
        return;
    }
    if args09.get(1).map(String::as_str) == Some("--round09-descent") {
        kummer08::descent09::run(std::path::Path::new(args09.get(2).expect("fresh outputs")));
        return;
    }
    if args09.get(1).map(String::as_str) == Some("--round09-controls") {
        let dir = std::path::Path::new(args09.get(2).expect("fresh outputs"));
        controls09::twists(dir);
        controls09::coupled(dir);
        return;
    }
    let args08: Vec<String> = std::env::args().collect();
    if args08.get(1).map(String::as_str) == Some("--record-round08") {
        round08_record::record(std::path::Path::new(
            args08.get(2).expect("fresh round records"),
        ));
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--verify-round08-kummer-glue") {
        kummer08::verify_glue(std::path::Path::new(args08.get(2).expect("fresh outputs")));
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--round08-kummer-glue") {
        kummer08::glue(std::path::Path::new(args08.get(2).expect("fresh outputs")));
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--verify-round08-relations") {
        let dir = std::path::Path::new(args08.get(2).expect("fresh outputs"));
        relations08::verify(dir, false);
        relations08::verify(dir, true);
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--verify-round08-kummer") {
        kummer08::verify(std::path::Path::new(args08.get(2).expect("fresh outputs")));
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--round08-infinity") {
        relations08::infinity(std::path::Path::new(args08.get(2).expect("fresh outputs")));
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--round08-kummer") {
        kummer08::run(std::path::Path::new(args08.get(2).expect("fresh outputs")));
        return;
    }
    if args08.get(1).map(String::as_str) == Some("--round08-relations") {
        relations08::run(std::path::Path::new(args08.get(2).expect("fresh outputs")));
        return;
    }
    let args07: Vec<String> = std::env::args().collect();
    if args07.get(1).map(String::as_str) == Some("--verify-round07-next") {
        forward07::verify_next(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        forward07::verify_bases(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--verify-round07-bases") {
        forward07::verify_bases(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--verify-round07-odd") {
        odd07::verify(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-odd-targets") {
        odd07::target_norms(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-odd-divisor") {
        odd07::divisor(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-cofactor-equations") {
        cofactor07::run(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-relations-next") {
        forward07::relations_next(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--verify-round07-relations") {
        forward07::verify(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-quotients") {
        quotients07::run(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-odd-screen") {
        odd07::screen(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-odd-algebras") {
        odd07::algebras(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-forward") {
        forward07::even(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-relations") {
        forward07::relations(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    if args07.get(1).map(String::as_str) == Some("--round07-norm-seeds") {
        forward07::seeds(std::path::Path::new(args07.get(2).expect("fresh outputs")));
        return;
    }
    let round06args: Vec<String> = std::env::args().collect();
    if round06args.get(1).map(String::as_str) == Some("--record-round07") {
        round07_record::record(std::path::Path::new(
            round06args.get(2).expect("fresh round records"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--record-round06") {
        round06_record::record(std::path::Path::new(
            round06args.get(2).expect("fresh round records"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--verify-round06") {
        let p = std::path::Path::new(round06args.get(2).expect("fresh checker outputs"));
        quotients06::verify(p);
        reverse06::verify(p);
        round06::matrix_check(p);
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-reverse-divisor") {
        reverse06::divisor(std::path::Path::new(
            round06args.get(2).expect("fresh outputs"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-split-fiber") {
        reverse06::split_fiber(
            std::path::Path::new(round06args.get(2).expect("fresh outputs")),
            false,
        );
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-split-fiber-negative") {
        reverse06::split_fiber(
            std::path::Path::new(round06args.get(2).expect("fresh outputs")),
            true,
        );
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-lean") {
        quotients06::lean(std::path::Path::new(
            round06args.get(2).expect("fresh outputs"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-reverse-screen") {
        reverse06::screen(
            std::path::Path::new(round06args.get(2).expect("fresh outputs")),
            false,
        );
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-reverse-screen-negative") {
        reverse06::screen(
            std::path::Path::new(round06args.get(2).expect("fresh outputs")),
            true,
        );
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-reverse-algebras") {
        reverse06::algebras(
            std::path::Path::new(round06args.get(2).expect("fresh outputs")),
            false,
        );
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-reverse-algebras-negative") {
        reverse06::algebras(
            std::path::Path::new(round06args.get(2).expect("fresh outputs")),
            true,
        );
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-seven-fibers") {
        round06::seven_fibers(std::path::Path::new(
            round06args.get(2).expect("fresh outputs"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-check-images") {
        round06::check_images(std::path::Path::new(
            round06args.get(2).expect("fresh outputs"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-quotients") {
        quotients06::run(std::path::Path::new(
            round06args.get(2).expect("fresh outputs"),
        ));
        return;
    }
    if round06args.get(1).map(String::as_str) == Some("--round06-images") {
        round06::images(std::path::Path::new(
            round06args.get(2).expect("fresh outputs"),
        ));
        return;
    }
    let newargs: Vec<String> = std::env::args().collect();
    if newargs.get(1).map(String::as_str) == Some("--round05-poles") {
        round05::poles(std::path::Path::new(
            newargs.get(2).expect("fresh pole/relation output"),
        ));
        return;
    }
    if newargs.get(1).map(String::as_str) == Some("--record-round05") {
        round05_record::record(std::path::Path::new(
            newargs.get(2).expect("new round directory"),
        ));
        return;
    }
    if newargs.get(1).map(String::as_str) == Some("--round05-lean") {
        round05::lean(std::path::Path::new(
            newargs.get(2).expect("fresh proof output"),
        ));
        return;
    }
    if newargs.get(1).map(String::as_str) == Some("--round05-extend") {
        round05::extend(std::path::Path::new(
            newargs.get(2).expect("fresh extended outputs"),
        ));
        return;
    }
    if newargs.get(1).map(String::as_str) == Some("--round05-resume") {
        round05::resume(std::path::Path::new(
            newargs.get(2).expect("fresh remaining outputs"),
        ));
        return;
    }
    if newargs.get(1).map(String::as_str) == Some("--round05") {
        round05::run(std::path::Path::new(
            newargs.get(2).expect("fresh run directory"),
        ));
        return;
    }
    if newargs.get(1).map(String::as_str) == Some("--verify-round05") {
        round05::verify(std::path::Path::new(newargs.get(2).expect("run directory")));
        return;
    }
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--quotient-lattices") {
        quotient_geometry::lattices(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--natural-quotient") {
        quotient_geometry::natural(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--record-round04") {
        round04_record::record(std::path::Path::new(args.get(2).expect("round")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--verify-round04") {
        round04_verify::run(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--modular-interior-test") {
        modular_geometry::interior_test(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--direct-modular-relations") {
        modular_geometry::relations(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--modular-transport-paired") {
        modular_geometry::transport_paired(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--modular-transport") {
        modular_geometry::transport(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--modular-geometry") {
        modular_geometry::run(std::path::Path::new(args.get(2).expect("output")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--genus4-bounds") {
        census::genus_four_bounds(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--record-round03") {
        round03_record::record(std::path::Path::new(args.get(2).expect("round directory")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--verify-n5") {
        n5_relations::verify(std::path::Path::new(
            args.get(2).expect("n5 relation record"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--polarization-vectors") {
        polarized::short_vectors(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--verify-polarized") {
        polarized::verify(std::path::Path::new(
            args.get(2).expect("polarization record"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--n5-hermite-relations") {
        n5_relations::run_hermite(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--n5-quartic-relations") {
        n5_relations::run_quartic(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--genus6-census") {
        census::run_six(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--n5-vertical-relations") {
        n5_relations::run_vertical(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--n5-relations") {
        n5_relations::run(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--genus5-census") {
        census::run(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--determinant-screen") {
        modular_screen::run(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--polarized-extended") {
        polarized::run_extended(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--polarized") {
        polarized::run(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--conic-twists") {
        delpezzo_conic::run(std::path::Path::new(
            args.get(2).expect("round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--record-round") {
        round_record::record(std::path::Path::new(args.get(2).expect("round directory")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--verify-klein-relations") {
        klein_transfer::verify_relations(
            std::path::Path::new(args.get(2).expect("relation record")),
            std::path::Path::new(args.get(3).expect("geometric point record")),
        );
        return;
    }
    if matches!(
        args.get(1).map(String::as_str),
        Some("--klein-lines" | "--klein-lines-wide")
    ) {
        klein_transfer::line_relations(
            std::path::Path::new(args.get(2).expect("round directory")),
            args[1] == "--klein-lines-wide",
        );
        return;
    }
    if args.get(1).map(String::as_str) == Some("--gluing-modules") {
        gluing::run(std::path::Path::new(args.get(2).expect("round directory")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--unbalanced") {
        round02::unbalanced(std::path::Path::new(
            args.get(2).expect("fresh output path"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--verify-klein") {
        klein_transfer::verify(std::path::Path::new(args.get(2).expect("certificate path")));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--klein-transfer-wide") {
        klein_transfer::run_wide(std::path::Path::new(
            args.get(2).expect("fresh output JSON path"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--delpezzo") {
        delpezzo::run(std::path::Path::new(
            args.get(2).expect("fresh output path"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--klein-transfer") {
        klein_transfer::run(std::path::Path::new(
            args.get(2).expect("fresh output JSON path"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--round02") {
        round02::run(std::path::Path::new(
            args.get(2).expect("fresh round output directory"),
        ));
        return;
    }
    if args.get(1).map(String::as_str) == Some("--large-identity") {
        use identity_certificate::{Expr as E, IdentityStatement};
        fn constant(c: &BigInt) -> E {
            let digits = c.to_string();
            let negative = digits.starts_with('-');
            let digits = digits.trim_start_matches('-');
            let mut expr = E::Const(0);
            let first = if digits.len() % 9 == 0 {
                9
            } else {
                digits.len() % 9
            };
            let mut offset = 0;
            while offset < digits.len() {
                let width = if offset == 0 { first } else { 9 };
                let chunk = digits[offset..offset + width].parse::<i64>().unwrap();
                expr = E::plus(
                    E::times(expr, E::Const(if offset == 0 { 1 } else { 1_000_000_000 })),
                    E::Const(chunk),
                );
                offset += width;
            }
            if negative {
                E::minus(expr)
            } else {
                expr
            }
        }
        fn polynomial(coeffs: &[BigInt]) -> E {
            fn power(k: usize) -> E {
                if k == 0 {
                    E::Const(1)
                } else if k == 1 {
                    E::var("X")
                } else {
                    let a = power(k / 2);
                    let b = E::times(a.clone(), a);
                    if k % 2 == 0 {
                        b
                    } else {
                        E::times(b, E::var("X"))
                    }
                }
            }
            if coeffs.len() == 1 {
                return constant(&coeffs[0]);
            }
            let split = coeffs.len() / 2;
            E::plus(
                polynomial(&coeffs[..split]),
                E::times(power(split), polynomial(&coeffs[split..])),
            )
        }
        let path = std::path::Path::new(args.get(2).expect("fresh JSON path"));
        assert!(!path.exists(), "do not overwrite frozen output");
        let a = weil::trace_zero_poly(-1, 131);
        assert_eq!(a, norm_poly(-1, 131));
        let statement=IdentityStatement {
            name:"Exact ECC2K-130 trace-zero quotient over integers".into(),
            source:"research/koblitz_modular_ic_20261018/results.json; norm and monic-division coefficient checks".into(),
            variables:vec!["X".into()],degree_bound:264,
            lhs:E::times(polynomial(&weil::curve_poly(-1)),polynomial(&a)),
            rhs:polynomial(&weil::weil_restriction_poly(-1,131)),
        };
        let cert=identity_certificate::issue(statement,20261018,64,"Same-agent native issue and 128 additional checker points; exact integer norm and division also agree",None).unwrap();
        let verdict =
            identity_certificate::check_with_extra(&cert, Some((2026101803, 128))).unwrap();
        assert!(matches!(
            verdict,
            identity_certificate::Verdict::Accept { .. }
        ));
        let encoded = serde_json::to_string(&cert).unwrap();
        let decoded: identity_certificate::IdentityCertificate =
            serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, cert);
        assert!(matches!(
            identity_certificate::check(&decoded).unwrap(),
            identity_certificate::Verdict::Accept { .. }
        ));
        std::fs::write(path, encoded + "\n").unwrap();
        println!(
            "{}",
            json!({"id":cert.id,"verdict":format!("{verdict:?}"),"integer_coefficients_equal":true})
        );
        return;
    }
    if args.get(1).map(String::as_str) == Some("--cyclic-only") {
        let path = std::path::Path::new(args.get(2).expect("fresh JSON path"));
        assert!(!path.exists(), "do not overwrite frozen output");
        let mut rh_spectral = 0;
        let mut surviving = Vec::new();
        for g in 0i64..=393 {
            for b in 0..=g {
                for d in [1, 2, 5, 10, 13, 26, 65, 130] {
                    if 130 * d > g - b {
                        continue;
                    }
                    let delta = g - 131 * (b - 1) - 1;
                    if delta < 0 || delta % 65 != 0 {
                        continue;
                    }
                    let branches = delta / 65;
                    rh_spectral += 1;
                    assert!(b <= 3 && d <= 2);
                    // J(B) and its quadratic twist have <=5^b<=125 F_2 points,
                    // so neither F_2 nor F_4 supplies unramified 131-torsion.
                    assert!(5i64.pow(b as u32) < 131);
                    let min_branches = 130 / d;
                    if branches >= min_branches {
                        surviving.push(json!({"g":g,"base_genus":b,"d":d,"branches":branches}));
                    }
                }
            }
        }
        assert!(surviving.is_empty());
        let record = json!({"schema":"koblitz.cyclic-window/v1","parameter_bound_g":393,"degree":131,"frobenius_action_orders":[1,2,5,10,13,26,65,130],"rh_and_spectral_candidates":rh_spectral,"survivors":surviving,"derived_minimum_genus":394,"abelian_order_bound_f2_dim3":125,"scope":"Arithmetic replay conditional on the geometric and class-field-theory lemmas in REPORT.md; not curve enumeration; no noncyclic exclusion"});
        std::fs::write(path, serde_json::to_string_pretty(&record).unwrap() + "\n").unwrap();
        println!("{record}");
        return;
    }
    let out = std::path::Path::new(args.get(1).expect("output directory"));
    assert!(out.is_dir());
    assert!(
        !out.join("results.json").exists(),
        "do not overwrite frozen output"
    );
    run(out);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_character_is_full_order_at_two() {
        let g = primitive_root(787);
        let k = log_two(g, 787);
        assert_eq!(262 / k.gcd(&262), 262);
        assert_eq!(131 / k.gcd(&131), 131);
    }
    #[test]
    fn conductor263_cannot_supply_negative_sign() {
        let k = log_two(primitive_root(263), 263);
        assert_eq!(262 / k.gcd(&262), 131);
    }
    #[test]
    fn norm_rejects_wrong_sign() {
        assert_ne!(norm_poly(1, 5), weil::trace_zero_poly(-1, 5));
    }
    #[test]
    fn exact_norm_matches_large_target() {
        assert_eq!(norm_poly(-1, 131), weil::trace_zero_poly(-1, 131));
    }
    #[test]
    fn full_size_order_matches_registered_subgroup() {
        let order = weil::eval(&weil::trace_zero_poly(-1, 131), &BigInt::one());
        assert_eq!(order.to_string(), "680564733841876926932320129493409985129");
        assert_eq!(weil::curve_order(-1, 131), 4 * order);
    }
    #[test]
    fn elliptic_ramification_is_not_silently_ignored() {
        assert_eq!(coset_genus(3, 13, 169).0, genus(3, 13, 1));
    }
}
