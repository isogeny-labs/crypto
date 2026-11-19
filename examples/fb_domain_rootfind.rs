//! # Factor-base domain root extraction in the `f4_fp` visitor
//!
//! The recursive visitor ([`f4_fp::solve_visit`]) finds the roots of a
//! univariate whose variable has a finite domain `D` (a factor-base
//! coordinate) by evaluating it at every element of `D`.  The opt-in
//! [`DomainRootMode::Frobenius`] finds the univariate's `F_p`-roots with the
//! counted `gcd(g, x^p − x)` root finder and keeps those in `D` through an
//! index built once per traversal; [`DomainRootMode::Auto`] picks the cheaper
//! of the two per univariate.  This example runs the index-calculus
//! decomposition systems the visitor is used for, in every mode, on the same
//! targets, and
//!
//! * **asserts** that every mode visits the identical solution sequence,
//!   and that every planted target is decomposed;
//! * reports, per configuration, the root-step multiplications of each mode
//!   (the scan's are exact: `f4_fp::domain_scan_cost` per element evaluated),
//!   the F4 row-reduction multiplications (`field_ops`, identical across
//!   modes by construction) and local wall time.
//!
//! Systems (`S₃` in the symmetric odd-characteristic expansion used by the
//! prime-chain builder of `ml_dataset::solvers`):
//!
//! * `m = 2`: `S₃(x₀, x₁, X_R) = 0`, `x₀, x₁ ∈ D`.
//! * `m = 3` chain: `S₃(x₀, x₁, u) = 0`, `S₃(u, x₂, X_R) = 0`, `x₀, x₁, x₂ ∈ D`,
//!   `u ∈ F_p` free.
//!
//! `D` is the small-abscissa factor base: the smallest `|D|` values `x ≥ 1`
//! with `x³ + ax + b` a square.  Half the targets are planted
//! (`R = P_i + P_j`, resp. `P_i + P_j + P_k`), half random curve abscissae.
//!
//! Wall times are single-host, unisolated (L0) diagnostics.  The
//! multiplication counts are deterministic.  Nothing here is a ranking
//! against rho or a complete-solve result.
//!
//! ```text
//! cargo run --release --example fb_domain_rootfind
//! FBDR_OUT=dir cargo run --release --example fb_domain_rootfind   # JSON lines
//! ```

use std::io::Write;
use std::time::Instant;

use crypto_lib::cryptanalysis::f4_fp::{
    self, normalise, DomainRootMode, F4Options, Ordering, Poly, VisitReport,
};

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(p)) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    ((u128::from(a) + u128::from(b)) % u128::from(p)) as u64
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    addm(a, p - b % p, p)
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn inv(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'witness: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}
/// The largest prime `≡ 3 (mod 4)` below `2^bits` (square roots are one power).
fn prime_below(bits: u32) -> u64 {
    let mut p = (1u64 << bits) - 1;
    while !(p % 4 == 3 && is_prime(p)) {
        p -= 1;
    }
    p
}

#[derive(Clone, Copy, PartialEq)]
struct Pt {
    x: u64,
    y: u64,
    inf: bool,
}

struct Curve {
    p: u64,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p), self.b, p)
    }
    fn lift(&self, x: u64) -> Option<Pt> {
        let r = self.rhs(x);
        let y = powm(r, (self.p + 1) / 4, self.p);
        (mulm(y, y, self.p) == r).then_some(Pt { x, y, inf: false })
    }
    fn add(&self, s: Pt, t: Pt) -> Pt {
        let p = self.p;
        if s.inf {
            return t;
        }
        if t.inf {
            return s;
        }
        let lam = if s.x == t.x {
            if addm(s.y, t.y, p) == 0 {
                return Pt { x: 0, y: 0, inf: true };
            }
            let num = addm(mulm(3, mulm(s.x, s.x, p), p), self.a, p);
            mulm(num, inv(mulm(2, s.y, p), p), p)
        } else {
            mulm(subm(t.y, s.y, p), inv(subm(t.x, s.x, p), p), p)
        };
        let x = subm(subm(mulm(lam, lam, p), s.x, p), t.x, p);
        let y = subm(mulm(lam, subm(s.x, x, p), p), s.y, p);
        Pt { x, y, inf: false }
    }
}

/// `S₃` over `n` variables; `None` puts the target abscissa in that slot.
fn s3(slots: [Option<usize>; 3], n: usize, target_x: u64, c: &Curve) -> Poly {
    let p = c.p;
    let neg = |v: u64| (p - v % p) % p;
    let raw: [([u32; 3], u64); 13] = [
        ([2, 2, 0], 1),
        ([2, 0, 2], 1),
        ([0, 2, 2], 1),
        ([2, 1, 1], neg(2)),
        ([1, 2, 1], neg(2)),
        ([1, 1, 2], neg(2)),
        ([1, 1, 0], neg(mulm(2, c.a, p))),
        ([1, 0, 1], neg(mulm(2, c.a, p))),
        ([0, 1, 1], neg(mulm(2, c.a, p))),
        ([1, 0, 0], neg(mulm(4, c.b, p))),
        ([0, 1, 0], neg(mulm(4, c.b, p))),
        ([0, 0, 1], neg(mulm(4, c.b, p))),
        ([0, 0, 0], mulm(c.a, c.a, p)),
    ];
    let terms: Vec<(Vec<u32>, u64)> = raw
        .iter()
        .map(|(powers, coefficient)| {
            let mut e = vec![0u32; n];
            let mut coefficient = *coefficient;
            for (axis, &power) in powers.iter().enumerate() {
                match slots[axis] {
                    Some(v) => e[v] += power,
                    None => coefficient = mulm(coefficient, powm(target_x, u64::from(power), p), p),
                }
            }
            (e, coefficient)
        })
        .collect();
    normalise(&terms, p, Ordering::Grevlex)
}

struct Row {
    mode: DomainRootMode,
    solutions: Vec<Vec<u64>>,
    report: VisitReport,
    seconds: f64,
}

fn run(system: &[Poly], n: usize, p: u64, domains: &[Option<Vec<u64>>], mode: DomainRootMode) -> Row {
    let opts = F4Options::new(Ordering::Grevlex, 12).with_domain_roots(mode);
    let mut solutions = Vec::new();
    let t = Instant::now();
    let report = f4_fp::solve_visit(system, n, p, &opts, domains, |s| {
        solutions.push(s.to_vec());
        false
    })
    .expect("valid system");
    Row {
        mode,
        solutions,
        report,
        seconds: t.elapsed().as_secs_f64(),
    }
}

fn root_muls(r: &VisitReport) -> u64 {
    r.domain_scan_field_multiplications + r.domain_root_field_multiplications
}

fn main() {
    let out = std::env::var("FBDR_OUT").ok().map(|d| {
        std::fs::create_dir_all(&d).unwrap();
        std::fs::File::create(format!("{d}/fb_domain_rootfind.jsonl")).unwrap()
    });
    let mut out = out;
    // (m, field bits, |D|, targets)
    let configs: &[(usize, u32, usize, usize)] = &[
        (2, 16, 64, 16),
        (2, 16, 256, 16),
        (2, 20, 256, 16),
        (2, 20, 1024, 8),
        (2, 24, 1024, 8),
        (2, 31, 1024, 8),
        (2, 31, 4096, 4),
        (3, 16, 16, 8),
        (3, 20, 32, 6),
        (3, 31, 32, 6),
        (3, 31, 64, 4),
    ];
    println!(
        "{:>2} {:>4} {:>6} {:>4} | {:>14} {:>14} {:>7} | {:>14} {:>9} | {:>9} {:>9} {:>7} | {}",
        "m", "bits", "|D|", "tgt", "root mul scan", "root mul frob", "ratio",
        "F4 field_ops", "F4 share", "wall scan", "wall frob", "ratio", "auto/scan wall"
    );
    for &(m, bits, size, targets) in configs {
        let p = prime_below(bits);
        let curve = Curve { p, a: p - 3, b: 0x5a5a_1234 % p };
        let base: Vec<Pt> = (1..p).filter_map(|x| curve.lift(x)).take(size).collect();
        let domain: Vec<u64> = base.iter().map(|q| q.x).collect();
        let n = if m == 2 { 2 } else { 4 };
        let domains: Vec<Option<Vec<u64>>> = (0..n)
            .map(|v| (m == 2 || v < 3).then(|| domain.clone()))
            .collect();
        let mut state = 0x9E37_79B9_7F4A_7C15u64 ^ (bits as u64) << 32 ^ size as u64;
        let mut next = |bound: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % bound
        };
        let (mut scan_root, mut frob_root, mut f4_ops) = (0u64, 0u64, 0u64);
        let (mut scan_wall, mut frob_wall, mut auto_wall) = (0f64, 0f64, 0f64);
        let (mut planted_found, mut visited) = (0usize, 0usize);
        for t in 0..targets {
            let planted = t % 2 == 0;
            let target_x = if planted {
                let mut r = Pt { x: 0, y: 0, inf: true };
                for _ in 0..m {
                    r = curve.add(r, base[next(size as u64) as usize]);
                }
                if r.inf {
                    continue;
                }
                r.x
            } else {
                loop {
                    let x = next(p);
                    if curve.lift(x).is_some() {
                        break x;
                    }
                }
            };
            let system: Vec<Poly> = if m == 2 {
                vec![s3([Some(0), Some(1), None], n, target_x, &curve)]
            } else {
                vec![
                    s3([Some(0), Some(1), Some(3)], n, target_x, &curve),
                    s3([Some(3), Some(2), None], n, target_x, &curve),
                ]
            };
            let scan = run(&system, n, p, &domains, DomainRootMode::Scan);
            let frob = run(&system, n, p, &domains, DomainRootMode::Frobenius);
            let auto = run(&system, n, p, &domains, DomainRootMode::Auto);
            for other in [&frob, &auto] {
                assert_eq!(
                    other.solutions, scan.solutions,
                    "{:?} visited a different solution sequence (m {m}, p {p}, |D| {size})",
                    other.mode
                );
                assert_eq!(other.report.field_ops, scan.report.field_ops);
                assert_eq!(other.report.f4_runs, scan.report.f4_runs);
                assert_eq!(other.report.complete, scan.report.complete);
            }
            assert!(scan.report.complete, "incomplete traversal");
            if planted {
                assert!(!scan.solutions.is_empty(), "planted target not decomposed");
                planted_found += 1;
            }
            visited += scan.solutions.len();
            scan_root += root_muls(&scan.report);
            frob_root += root_muls(&frob.report);
            f4_ops += scan.report.field_ops;
            scan_wall += scan.seconds;
            frob_wall += frob.seconds;
            auto_wall += auto.seconds;
            if let Some(f) = out.as_mut() {
                for row in [&scan, &frob, &auto] {
                    writeln!(
                        f,
                        "{{\"m\":{m},\"p\":{p},\"bits\":{bits},\"domain\":{size},\"target\":{t},\"planted\":{planted},\"mode\":\"{:?}\",\"solutions\":{},\"field_ops\":{},\"f4_runs\":{},\"domain_evaluations\":{},\"domain_scan_field_multiplications\":{},\"domain_root_field_multiplications\":{},\"domain_frobenius_univariates\":{},\"domain_index_entries\":{},\"full_field_branches\":{},\"wall_s\":{:.9}}}",
                        row.mode,
                        row.solutions.len(),
                        row.report.field_ops,
                        row.report.f4_runs,
                        row.report.domain_evaluations,
                        row.report.domain_scan_field_multiplications,
                        row.report.domain_root_field_multiplications,
                        row.report.domain_frobenius_univariates,
                        row.report.domain_index_entries,
                        row.report.full_field_branches,
                        row.seconds
                    )
                    .unwrap();
                }
            }
        }
        let share = scan_root as f64 / (scan_root + f4_ops).max(1) as f64;
        println!(
            "{:>2} {:>4} {:>6} {:>4} | {:>14} {:>14} {:>6.1}x | {:>14} {:>8.1}% | {:>8.3}s {:>8.3}s {:>6.1}x | {:.2}  (planted found {planted_found}, solutions {visited})",
            m, bits, size, targets, scan_root, frob_root,
            scan_root as f64 / frob_root.max(1) as f64,
            f4_ops, 100.0 * (1.0 - share),
            scan_wall, frob_wall, scan_wall / frob_wall.max(1e-12),
            auto_wall / scan_wall.max(1e-12),
        );
    }
    println!("every mode visited the identical solution sequence on every target");
}
