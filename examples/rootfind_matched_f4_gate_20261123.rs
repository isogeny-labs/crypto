//! Matched genuine-F4 arm for the program-B chart contract
//! (research/rootfind_matched_f4_gate_20261123/PROTOCOL.md).
//!
//! Program B's exact solver (`primal_parabola_20261109_v5/native.cpp`) takes a
//! public chart `(p, M, t0..t2, u0..u2)` and returns every ordered integer
//! triple `(x, y, z) in [0, M]^3` with
//!   e2(x,y,z) = t0*S^2 + t1*S + t2  (mod p),
//!   e3(x,y,z) = u0*S^2 + u1*S + u2  (mod p),   S = x + y + z.
//! For `p > M` this is exactly the F_p-variety of the same two polynomials
//! restricted to the domain {0..M}^3.  Two genuine F4 encodings:
//!   ideal: `f4_fp::solve` on {f1, f2, D(x), D(y), D(z)}, D(v)=prod_{j<=M}(v-j)
//!   visit: `f4_fp::solve_visit` on {f1, f2} with domains [0..M] per variable.
//!
//! Usage: rootfind_matched_f4_gate_20261123 <ideal|visit> <cases.txt> <copies> <budget_s>
//! cases.txt lines: `id p M t0 t1 t2 u0 u1 u2` (decimal).  Each case is solved
//! `copies` times (to match program B's five-case batch); all copies must agree.
//! Output: one JSON object per case on stdout.
use crypto_lib::cryptanalysis::f4_fp::{normalise, solve_visit, F4Options, Ordering, Poly};
use std::time::{Duration, Instant};

fn md(x: i128, p: u64) -> u64 {
    let p = p as i128;
    (((x % p) + p) % p) as u64
}

fn mono(e: [u32; 3]) -> Vec<u32> {
    e.to_vec()
}

fn chart_polys(p: u64, t: [u64; 3], u: [u64; 3]) -> (Poly, Poly) {
    let sq = [[2, 0, 0], [0, 2, 0], [0, 0, 2]];
    let cross = [[1, 1, 0], [1, 0, 1], [0, 1, 1]];
    let lin = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    let (t0, t1, t2) = (t[0] as i128, t[1] as i128, t[2] as i128);
    let (u0, u1, u2) = (u[0] as i128, u[1] as i128, u[2] as i128);
    // f1 = e2 - t0 S^2 - t1 S - t2
    let mut f1: Vec<(Vec<u32>, u64)> = Vec::new();
    for e in sq {
        f1.push((mono(e), md(-t0, p)));
    }
    for e in cross {
        f1.push((mono(e), md(1 - 2 * t0, p)));
    }
    for e in lin {
        f1.push((mono(e), md(-t1, p)));
    }
    f1.push((mono([0, 0, 0]), md(-t2, p)));
    // f2 = e3 - u0 S^2 - u1 S - u2
    let mut f2: Vec<(Vec<u32>, u64)> = vec![(mono([1, 1, 1]), 1)];
    for e in sq {
        f2.push((mono(e), md(-u0, p)));
    }
    for e in cross {
        f2.push((mono(e), md(-2 * u0, p)));
    }
    for e in lin {
        f2.push((mono(e), md(-u1, p)));
    }
    f2.push((mono([0, 0, 0]), md(-u2, p)));
    (
        normalise(&f1, p, Ordering::Grevlex),
        normalise(&f2, p, Ordering::Grevlex),
    )
}

/// prod_{j=0}^{M} (v - j) in variable `var`.
fn domain_poly(p: u64, m: u64, var: usize) -> Poly {
    let mut c: Vec<u64> = vec![1]; // coefficients, index = degree
    for j in 0..=m {
        let mut n = vec![0u64; c.len() + 1];
        for (d, &a) in c.iter().enumerate() {
            n[d + 1] = (n[d + 1] + a) % p;
            n[d] = md(n[d] as i128 - (a as i128) * (j as i128), p);
        }
        c = n;
    }
    let terms: Vec<(Vec<u32>, u64)> = c
        .iter()
        .enumerate()
        .map(|(d, &a)| {
            let mut e = vec![0u32; 3];
            e[var] = d as u32;
            (e, a)
        })
        .collect();
    normalise(&terms, p, Ordering::Grevlex)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert!(
        args.len() == 5,
        "usage: <ideal|visit> <cases> <copies> <budget_s>"
    );
    let mode = args[1].clone();
    assert!(mode == "ideal" || mode == "visit");
    let text = std::fs::read_to_string(&args[2]).expect("cases");
    let copies: usize = args[3].parse().unwrap();
    let budget: f64 = args[4].parse().unwrap();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let f: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(f.len(), 9, "case line");
        let id = f[0];
        let v: Vec<u64> = f[1..].iter().map(|s| s.parse().unwrap()).collect();
        let (p, m) = (v[0], v[1]);
        assert!(p < (1u64 << 32) && m < p);
        let t = [v[2] % p, v[3] % p, v[4] % p];
        let u = [v[5] % p, v[6] % p, v[7] % p];
        let mut first: Option<Vec<[u64; 3]>> = None;
        let mut secs = Vec::new();
        let mut complete = true;
        let (mut ops, mut deg, mut rows, mut cols, mut runs) = (0u64, 0u32, 0usize, 0usize, 0usize);
        for _ in 0..copies {
            let t_start = Instant::now();
            let (f1, f2) = chart_polys(p, t, u);
            let mut roots: Vec<[u64; 3]> = Vec::new();
            if mode == "ideal" {
                let mut sys = vec![f1, f2];
                for var in 0..3 {
                    sys.push(domain_poly(p, m, var));
                }
                let dmax = 3 * (m as u32 + 1) + 3;
                // Grevlex as frozen; F4_IDEAL_ORDER=lex is a diagnostic only.
                let ord = if std::env::var("F4_IDEAL_ORDER").as_deref() == Ok("lex") {
                    Ordering::Lex
                } else {
                    Ordering::Grevlex
                };
                let sys: Vec<Poly> = sys.iter().map(|f| normalise(f, p, ord)).collect();
                let opts = F4Options::new(ord, dmax).with_budget(Duration::from_secs_f64(budget));
                // DEVIATION 2 (PROTOCOL.md): `solve` finds univariate roots by scanning all
                // of F_p (O(p) at p ~ 2^32); `solve_visit` with no domains is the same F4
                // with Frobenius root extraction. The domain is imposed only by D(x),D(y),D(z).
                let r = solve_visit(&sys, 3, p, &opts, &[None, None, None], |s| {
                    roots.push([s[0], s[1], s[2]]);
                    false
                })
                .expect("valid system");
                if !r.complete || r.timed_out {
                    complete = false;
                }
                ops = r.field_ops
                    + r.root_field_multiplications
                    + r.domain_root_field_multiplications;
                deg = r.solving_degree;
                rows = r.max_rows;
                cols = r.max_cols;
                runs = r.f4_runs;
            } else {
                let dom: Vec<u64> = (0..=m).collect();
                let domains = vec![Some(dom.clone()), Some(dom.clone()), Some(dom)];
                let opts = F4Options::new(Ordering::Grevlex, 12)
                    .with_budget(Duration::from_secs_f64(budget));
                let r = solve_visit(&[f1, f2], 3, p, &opts, &domains, |s| {
                    roots.push([s[0], s[1], s[2]]);
                    false
                })
                .expect("valid system");
                if !r.complete || r.timed_out {
                    complete = false;
                }
                ops = r.field_ops
                    + r.root_field_multiplications
                    + r.domain_root_field_multiplications;
                deg = r.solving_degree;
                rows = r.max_rows;
                cols = r.max_cols;
                runs = r.f4_runs;
            }
            secs.push(t_start.elapsed().as_secs_f64());
            roots.sort();
            roots.dedup();
            // exact replay of every root against the integer chart contract
            for r in &roots {
                let (x, y, z) = (r[0] as i128, r[1] as i128, r[2] as i128);
                assert!(r.iter().all(|&a| a <= m), "domain");
                let s = x + y + z;
                let e2 = x * y + x * z + y * z;
                let e3 = x * y * z;
                let pp = p as i128;
                let tt = (t[0] as i128 * s * s + t[1] as i128 * s + t[2] as i128) % pp;
                let uu = (u[0] as i128 * s * s + u[1] as i128 * s + u[2] as i128) % pp;
                assert!(md(e2 - tt, p) == 0 && md(e3 - uu, p) == 0, "root replay");
            }
            match &first {
                None => first = Some(roots),
                Some(f0) => assert!(f0 == &roots, "copies disagree"),
            }
        }
        let roots = first.unwrap_or_default();
        let rs: Vec<String> = roots
            .iter()
            .map(|r| format!("[{},{},{}]", r[0], r[1], r[2]))
            .collect();
        let ss: Vec<String> = secs.iter().map(|s| format!("{s:.6}")).collect();
        println!(
            "{{\"id\":\"{id}\",\"mode\":\"{mode}\",\"M\":{m},\"complete\":{complete},\"roots\":[{}],\"solve_seconds\":[{}],\"field_ops\":{ops},\"solving_degree\":{deg},\"max_rows\":{rows},\"max_cols\":{cols},\"f4_runs\":{runs}}}",
            rs.join(","),
            ss.join(",")
        );
    }
}
