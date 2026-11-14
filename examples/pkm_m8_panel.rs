//! Native instrument for the m = 8 sparse PKM relation-finder panel (panel A).
//!
//! Protocol: research/pkm_m8_sparse_panel_20261114 (CONTINUATION_PROTOCOL.md section A).
//! Everything this tool writes is `[pcc:unverified]`; it is instrument construction.
//!
//! Subcommands (all outputs JSON with a sealed `sha256`, deterministic given seeds):
//!   cell         --t T --ratio R --seed S --out DIR [--no-l1-reuse]
//!   targets      --cell DIR --n-sat 4 --n-unsat 4 [--planted 1] --seed S [--m 8] [--out DIR]
//!                [--max-draws N] [--max-enum-entries N] [--max-alg-leaves N]
//!   export       --cell DIR --targets DIR --out DIR [--format ms|native|both] [--only NAME]
//!   verify       --cell DIR --targets DIR --msolve-out FILE --target NAME --out FILE
//!                [--seed S] [--deadline SECS]        (public data and solver output only)
//!   native-solve --system FILE --max-degree D --deadline SECS --out FILE [--extract-max-q Q]
//!                [--gb-out FILE] [--cell DIR --targets DIR --target NAME]
//!   rho          --cell DIR --targets DIR --seed S --out FILE [--seeds 3] [--threads 1]
//!   check        --hidden FILE --verify FILE [--out FILE]      (local, after fetch)
//!   compare-gb   --a FILE --b FILE --out FILE                  (reduced bases, msolve -g 2 vs native)
//!   export       --l1-replication --out DIR                      (natively reconstructed L1 decisive cell)
//! `--master-seed S` is accepted wherever `--seed S` is; `cell` takes `--min-prime N` to step over
//! primes on which msolve crashed (all primes below 2^18 did, see pilot notes).
//!
//! The arithmetic of the chain polynomials is `examples/pkm_oracle_program/algebra.rs`,
//! included unchanged by `#[path]`.

#[path = "pkm_oracle_program/algebra.rs"]
mod algebra;
#[path = "pkm_m8_panel/cell.rs"]
mod cell;
#[path = "pkm_m8_panel/check.rs"]
mod check;
#[path = "pkm_m8_panel/ec.rs"]
mod ec;
#[path = "pkm_m8_panel/native_solve.rs"]
mod native_solve;
#[path = "pkm_m8_panel/rho.rs"]
mod rho;
#[path = "pkm_m8_panel/system.rs"]
mod system;
#[path = "pkm_m8_panel/targets.rs"]
mod targets;
#[path = "pkm_m8_panel/util.rs"]
mod util;
#[path = "pkm_m8_panel/verify.rs"]
mod verify;

use serde_json::{json, Value};

pub fn arg_opt(args: &[String], name: &str) -> Option<String> {
    let find = |n: &str| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).cloned();
    find(name).or_else(|| if name == "--seed" { find("--master-seed") } else { None })
}
pub fn arg_req(args: &[String], name: &str) -> Result<String, String> {
    arg_opt(args, name).ok_or_else(|| format!("missing {name}"))
}
pub fn flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

/// Identity of this build: hash of its own sources and the git commit when there is one.
pub fn tool_info() -> Value {
    let sources = [
        include_str!("pkm_m8_panel.rs"),
        include_str!("pkm_m8_panel/cell.rs"),
        include_str!("pkm_m8_panel/check.rs"),
        include_str!("pkm_m8_panel/ec.rs"),
        include_str!("pkm_m8_panel/native_solve.rs"),
        include_str!("pkm_m8_panel/rho.rs"),
        include_str!("pkm_m8_panel/system.rs"),
        include_str!("pkm_m8_panel/targets.rs"),
        include_str!("pkm_m8_panel/util.rs"),
        include_str!("pkm_m8_panel/verify.rs"),
        include_str!("pkm_oracle_program/algebra.rs"),
    ];
    let mut all = String::new();
    for s in sources {
        all.push_str(&util::sha256_hex(s.as_bytes()));
    }
    let git = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .or_else(|| option_env!("PKM_M8_GIT_COMMIT").map(str::to_string));
    json!({
        "name": "pkm_m8_panel",
        "source_sha256": util::sha256_hex(all.as_bytes()),
        "git_commit": git,
        "git_note": "null when run outside a git checkout and PKM_M8_GIT_COMMIT was not set at build",
    })
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().cloned() else {
        eprintln!("usage: pkm_m8_panel <cell|targets|export|verify|native-solve|rho> ...");
        std::process::exit(2);
    };
    let rest = &args[1..];
    let res = match cmd.as_str() {
        "cell" => cell::run(rest),
        "targets" => targets::run(rest),
        "export" => system::run_export(rest),
        "verify" => verify::run(rest),
        "check" => check::run(rest),
        "compare-gb" => native_solve::compare_gb(rest),
        "native-solve" => native_solve::run(rest),
        "rho" => rho::run(rest),
        other => Err(format!("unknown subcommand {other}")),
    };
    if let Err(e) = res {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::Pol;
    use crate::cell::{derive, Cell};
    use crate::ec::*;
    use crate::system::{build, Spec};
    use crate::targets::*;
    use crate::util::*;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use std::collections::BTreeMap;

    #[test]
    fn mt19937_reference_vector() {
        let mut r = PyRandom::from_key(&[0x123, 0x234, 0x345, 0x456]);
        let got: Vec<u32> = (0..5).map(|_| r.next_u32()).collect();
        assert_eq!(got, vec![1067595299, 955945823, 477289528, 4107218783, 4228976476]);
    }

    #[test]
    fn eligible_prime_rule() {
        assert!(cell::is_fermat(65537));
        let p = cell::least_prime(2, 8).unwrap();
        assert!(p >= 65536);
        let d = derive(2, 8, 777, false).unwrap();
        assert!(d.cell.p >= 1 << 16 && !cell::is_fermat(d.cell.p));
        assert!(d.cell.real.len() >= 2);
        // determinism
        let d2 = derive(2, 8, 777, false).unwrap();
        assert_eq!(d.cell.sha256, d2.cell.sha256);
        assert!(check_seal(&d.json));
    }

    #[test]
    fn bsgs_count_matches_legendre_count() {
        use crypto_lib::cryptanalysis::ic_boundary::PrimeCurve;
        let p = cell::least_prime(2, 10).unwrap();
        let mut rng = StdRng::seed_from_u64(5);
        let mut checked = 0;
        for _ in 0..40 {
            let (a, b) = (rng.gen_range(1..p), rng.gen_range(1..p));
            let n = PrimeCurve { p, a, b }.point_count();
            let got = cell::prime_order(p, a, b, &mut rng);
            let want = is_prime_for_test(n).then_some(n);
            assert_eq!(got, want);
            checked += usize::from(got.is_some());
        }
        assert!(checked >= 1);
    }
    fn is_prime_for_test(n: u64) -> bool {
        crypto_lib::cryptanalysis::residual_walk::is_prime_u64(n)
    }

    // ---- cross-check against examples/pkm_tower_pilot.rs ----------------------
    // Verbatim copy of the pilot's `Pol`, `s3`, `summation` (Kummer arm, no cover map) and
    // `Tower::equations` (Square step), pkm_tower_pilot.rs lines ~204-320, ~1237-1268 and ~395-417.
    // The test below checks that the copied `s3` text is still the pilot's.
    mod pilot {
        use std::collections::BTreeMap;
        pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
            ((a as u128 * b as u128) % p as u128) as u64
        }
        fn addm(a: u64, b: u64, p: u64) -> u64 {
            (a + b) % p
        }
        #[derive(Clone)]
        pub struct Pol {
            pub n: usize,
            pub terms: BTreeMap<Vec<u32>, u64>,
        }
        impl Pol {
            pub fn zero(n: usize) -> Self {
                Pol { n, terms: BTreeMap::new() }
            }
            pub fn constant(n: usize, c: u64, p: u64) -> Self {
                let mut f = Pol::zero(n);
                if c % p != 0 {
                    f.terms.insert(vec![0; n], c % p);
                }
                f
            }
            pub fn var(n: usize, i: usize) -> Self {
                let mut e = vec![0; n];
                e[i] = 1;
                let mut f = Pol::zero(n);
                f.terms.insert(e, 1);
                f
            }
            pub fn add(&self, o: &Pol, p: u64) -> Pol {
                let mut f = self.clone();
                for (e, c) in &o.terms {
                    let v = f.terms.entry(e.clone()).or_insert(0);
                    *v = addm(*v, *c, p);
                }
                f.terms.retain(|_, c| *c != 0);
                f
            }
            pub fn scale(&self, k: u64, p: u64) -> Pol {
                let mut f = Pol::zero(self.n);
                for (e, c) in &self.terms {
                    let v = mulm(*c, k, p);
                    if v != 0 {
                        f.terms.insert(e.clone(), v);
                    }
                }
                f
            }
            pub fn sub(&self, o: &Pol, p: u64) -> Pol {
                self.add(&o.scale(p - 1, p), p)
            }
            pub fn mul(&self, o: &Pol, p: u64) -> Pol {
                let mut f = Pol::zero(self.n);
                for (e1, c1) in &self.terms {
                    for (e2, c2) in &o.terms {
                        let e: Vec<u32> = e1.iter().zip(e2).map(|(a, b)| a + b).collect();
                        let v = f.terms.entry(e).or_insert(0);
                        *v = addm(*v, mulm(*c1, *c2, p), p);
                    }
                }
                f.terms.retain(|_, c| *c != 0);
                f
            }
        }
        pub fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {
            let n = x1.n;
            let d = x1.sub(x2, p);
            let t1 = d.mul(&d, p).mul(x3, p).mul(x3, p);
            let s = x1.add(x2, p);
            let pr = x1.mul(x2, p);
            let inner = s
                .mul(&pr.add(&Pol::constant(n, a, p), p), p)
                .add(&Pol::constant(n, mulm(2, b, p), p), p);
            let t2 = inner.mul(x3, p).scale(p - 2, p);
            let q = pr.sub(&Pol::constant(n, a, p), p);
            let t3 = q.mul(&q, p).sub(&s.scale(mulm(4, b, p), p), p);
            t1.add(&t2, p).add(&t3, p)
        }
        /// pilot `summation` (Kummer) followed by the tower equations of every block.
        pub fn tower_system(m: usize, t: usize, g: u64, a: u64, b: u64, p: u64, x_r: u64) -> Vec<Pol> {
            let n = m * t + (m - 2);
            let x = |i: usize| Pol::var(n, i * t).scale(g, p);
            let xr = Pol::constant(n, x_r, p);
            let mut eqs = Vec::new();
            if m == 2 {
                eqs.push(s3(&x(0), &x(1), &xr, a, b, p));
            } else {
                let u = |k: usize| Pol::var(n, m * t + k);
                eqs.push(s3(&x(0), &x(1), &u(0), a, b, p));
                for k in 1..m - 2 {
                    eqs.push(s3(&u(k - 1), &x(k + 1), &u(k), a, b, p));
                }
                eqs.push(s3(&u(m - 3), &x(m - 1), &xr, a, b, p));
            }
            for i in 0..m {
                for j in 0..t {
                    let yj = Pol::var(n, i * t + j);
                    let next = if j + 1 < t { Pol::var(n, i * t + j + 1) } else { Pol::constant(n, 1, p) };
                    eqs.push(next.sub(&yj.mul(&yj, p), p));
                }
            }
            eqs
        }
    }

    fn fn_text<'a>(src: &'a str, head: &str, end: &str) -> &'a str {
        let s = src.find(head).expect("function present");
        let e = src[s..].find(end).expect("function end") + s + end.len();
        &src[s..e]
    }

    #[test]
    fn exporter_equals_pilot_tower_control() {
        let pilot_src = include_str!("pkm_tower_pilot.rs");
        let mine = include_str!("pkm_m8_panel.rs");
        // the verbatim copy still is the pilot's s3 (compare after stripping indentation)
        let norm = |s: &str| s.lines().map(str::trim).collect::<Vec<_>>().join("\n");
        let want = norm(fn_text(pilot_src, "fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {", "\n}\n"));
        let have = norm(fn_text(mine, "pub fn s3(x1: &Pol, x2: &Pol, x3: &Pol, a: u64, b: u64, p: u64) -> Pol {", "\n        }\n"));
        assert_eq!(want.replace("pub fn", "fn"), have.replace("pub fn", "fn"));
        // matching convention: same variable layout y_{i,j} at i*t+j then u_k, same equation
        // order (chain, then per block y_{j+1} - y_j^2 and 1 - y_{t-1}^2), same coefficients
        // (reduced mod p), no tower reduction in either; only the container differs.
        let (p, a, b, g, zeta) = (40961u64, 17u64, 29u64, 7u64, 3u64);
        for (m, t) in [(3usize, 2usize), (4, 2), (2, 2), (5, 2)] {
            let spec = Spec { p, a, b, g, zeta, m, t, x_r: 12345 };
            let (mine_polys, n) = build(&spec);
            let theirs = pilot::tower_system(m, t, g, a, b, p, 12345);
            assert_eq!(n, theirs[0].n);
            assert_eq!(mine_polys.len(), theirs.len(), "m={m}");
            for (k, (x, y)) in mine_polys.iter().zip(&theirs).enumerate() {
                let xm: BTreeMap<Vec<u32>, u64> = x.iter().cloned().collect();
                assert_eq!(xm, y.terms, "m={m} t={t} poly {k}");
            }
        }
    }

    fn small_cell() -> Cell {
        derive(2, 8, 777, false).unwrap().cell
    }

    fn eval_poly(f: &[(Vec<u32>, u64)], x: &[u64], p: u64) -> u64 {
        f.iter().fold(0, |acc, (e, c)| {
            let mut v = *c;
            for (i, &k) in e.iter().enumerate() {
                v = mulm(v, powm(x[i], u64::from(k), p), p);
            }
            addm(acc, v, p)
        })
    }

    #[test]
    fn planted_decomposition_zeroes_every_polynomial() {
        let cell = small_cell();
        let p = cell.p;
        let sym = symbols(&cell);
        let mut rng = StdRng::seed_from_u64(9);
        for m in [3usize, 4, 8] {
            let (s, _x_r, assignment, spec) = loop {
                let s: Vec<usize> = (0..m).map(|_| rng.gen_range(0..sym.len())).collect();
                let mut acc: Option<Aff> = None;
                let mut prefix = Vec::new();
                let mut ok = true;
                for &k in &s {
                    acc = add_opt(p, cell.a, acc, Some(sym[k]));
                    if acc.is_none() {
                        ok = false;
                        break;
                    }
                    prefix.push(acc.unwrap().0);
                }
                if !ok {
                    continue;
                }
                let x_r = acc.unwrap().0;
                let t = cell.t;
                let mut asg = vec![0u64; system::nvars(m, t)];
                for (i, &k) in s.iter().enumerate() {
                    let mut y = mulm(sym[k].0, invm(cell.g, p), p);
                    for j in 0..t {
                        asg[i * t + j] = y;
                        y = mulm(y, y, p);
                    }
                    assert_eq!(y, 1, "tower closes: y_i^(2^t) = 1");
                }
                for k in 0..m.saturating_sub(2) {
                    asg[m * t + k] = prefix[k + 1];
                }
                let spec = Spec { p, a: cell.a, b: cell.b, g: cell.g, zeta: cell.zeta, m, t, x_r };
                break (s, x_r, asg, spec);
            };
            let _ = s;
            let (polys, _) = build(&spec);
            for (k, f) in polys.iter().enumerate() {
                assert_eq!(eval_poly(f, &assignment, p), 0, "m={m} poly {k}");
            }
        }
    }

    #[test]
    fn s3_vanishes_on_collinear_sums() {
        let cell = small_cell();
        let p = cell.p;
        let sym = symbols(&cell);
        let mut rng = StdRng::seed_from_u64(3);
        let mut tested = 0;
        while tested < 20 {
            let (p1, p2) = (sym[rng.gen_range(0..sym.len())], sym[rng.gen_range(0..sym.len())]);
            for q in [p2, neg_u(p, p2)] {
                if let Some(s) = add_u(p, cell.a, p1, q) {
                    let f = crate::algebra::s3(
                        &Pol::c(1, p1.0, p),
                        &Pol::c(1, p2.0, p),
                        &Pol::c(1, s.0, p),
                        cell.a,
                        cell.b,
                        p,
                    );
                    assert_eq!(f.eval(&[0], p), 0);
                    tested += 1;
                }
            }
        }
        // and not on a wrong abscissa
        let f = crate::algebra::s3(&Pol::c(1, 3, p), &Pol::c(1, 5, p), &Pol::c(1, 7, p), cell.a, cell.b, p);
        assert_ne!(f.eval(&[0], p), 0);
    }

    fn combos(n: usize, m: usize, start: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == m {
            out.push(cur.clone());
            return;
        }
        for s in start..n {
            cur.push(s);
            combos(n, m, s, cur, out);
            cur.pop();
        }
    }

    #[test]
    fn classifier_matches_brute_force() {
        let cell = small_cell();
        let p = cell.p;
        let sym = symbols(&cell);
        for m in [3usize, 4] {
            let tab = enumerate_real(&cell, m, 1 << 30).unwrap();
            let alg = enumerate_alg(&cell, m, 1 << 30).unwrap();
            let mut all = Vec::new();
            combos(sym.len(), m, 0, &mut Vec::new(), &mut all);
            // brute force against every abscissa hit by some multiset, plus misses
            let mut rng = StdRng::seed_from_u64(2);
            let mut targets: Vec<Aff> = all
                .iter()
                .take(40)
                .filter_map(|c| c.iter().fold(None, |acc, &k| add_opt(p, cell.a, acc, Some(sym[k]))))
                .collect();
            for _ in 0..30 {
                let x = rng.gen_range(1..p);
                if let Some(pt) = lift_small(p, cell.a, cell.b, x) {
                    targets.push(pt);
                }
            }
            for r in targets {
                let mut brute: Vec<Vec<u8>> = all
                    .iter()
                    .filter(|c| c.iter().fold(None, |acc, &k| add_opt(p, cell.a, acc, Some(sym[k]))) == Some(r))
                    .map(|c| c.iter().map(|&k| k as u8).collect())
                    .collect();
                brute.sort();
                assert_eq!(decompositions_of(&cell, &sym, &tab, m, r), brute, "m={m}");
                // gen.py-style algebraic count: unsigned multisets of V, first sign +, sum x = x_R in F_p
                let f = F2::new(p);
                let vp: Vec<F2pt> = cell.v.iter().map(|&x| lift_f2(&f, cell.a, cell.b, x)).collect();
                let mut uc = Vec::new();
                combos(vp.len(), m, 0, &mut Vec::new(), &mut uc);
                let mut hits = 0usize;
                for ms in &uc {
                    'signs: for mask in 0..(1u32 << (m - 1)) {
                        let mut acc: Option<F2pt> = Some(vp[ms[0]]);
                        for (k, &i) in ms.iter().enumerate().skip(1) {
                            let mut q = vp[i];
                            if (mask >> (k - 1)) & 1 == 1 {
                                q = (q.0, f.neg(q.1));
                            }
                            acc = add_f2(&f, cell.a, acc, Some(q));
                        }
                        if let Some(s) = acc {
                            if s.0 == (r.0, 0) {
                                hits += 1;
                                break 'signs;
                            }
                        }
                    }
                }
                let (_, uns, ordered) = alg_info(&alg, cell.a, m, r.0);
                assert_eq!(uns.len(), hits, "algebraic count m={m}");
                assert!(ordered as usize >= uns.len() || uns.is_empty());
            }
        }
    }

    #[test]
    fn ordered_count_counts_sequences() {
        let cell = small_cell();
        let alg = enumerate_alg(&cell, 3, 1 << 30).unwrap();
        // three distinct, mutually non-cancelling symbols: 3! sequences
        assert_eq!(ordered_count(&alg, cell.a, &[0, 2, 4]), 6);
        // a repeated symbol: 3!/2!
        assert_eq!(ordered_count(&alg, cell.a, &[0, 0, 2]), 3);
        // P, -P, Q: the prefix P + (-P) = O is excluded: only P Q -P and -P Q P and Q-first orders
        // that never put P next to -P first, i.e. 4 of 6.
        assert_eq!(ordered_count(&alg, cell.a, &[0, 1, 2]), 4);
    }

    #[test]
    fn rank_mod_small() {
        let mut w = verify::Work::default();
        let rows = vec![vec![1, 2, 3], vec![2, 4, 6], vec![0, 1, 1]];
        assert_eq!(verify::rank_mod(&rows, 101, &mut w), 2);
    }

    #[test]
    fn msolve_grammar_parses() {
        let a = "[0, [10007, \n2, \n2, \n['x', 'y'],\n[0, 1],\n[1,\n[[2,\n[10005, 0, 1]],\n[0,\n[1]],\n[\n[[1,\n[0, 10006]]]\n]]]]]:\n";
        assert!(matches!(verify::parse_msolve(a), Ok(verify::Outcome::ZeroDim(_))));
        assert!(matches!(verify::parse_msolve("[-1]:"), Ok(verify::Outcome::NoSolution)));
        assert!(matches!(verify::parse_msolve("[1, 2, -1, []]:"), Ok(verify::Outcome::PosDim)));
    }

    #[test]
    fn analyze_recovers_a_planted_decomposition() {
        let cell = small_cell();
        let p = cell.p;
        let sym = symbols(&cell);
        let ec = cell.ec();
        let mut rng = StdRng::seed_from_u64(4);
        let m = 4;
        loop {
            let s: Vec<usize> = (0..m).map(|_| rng.gen_range(0..sym.len())).collect();
            let sum = s.iter().fold(None, |acc, &k| add_opt(p, cell.a, acc, Some(sym[k])));
            let Some(r) = sum else { continue };
            let mut ctx = verify::Ctx {
                cell: small_cell(),
                m,
                name: "t".into(),
                label: "sat".into(),
                x_r: r.0,
                ab: Some((5, 7)),
                r_point: Some(ec.pt(r.0, r.1)),
                counts: serde_json::Value::Null,
                work: verify::Work::default(),
            };
            let y0: Vec<u64> = s.iter().map(|&k| mulm(sym[k].0, invm(cell.g, p), p)).collect();
            let found = verify::analyze(&mut ctx, &[y0]);
            let mut want: Vec<u8> = s.iter().map(|&k| k as u8).collect();
            want.sort_unstable();
            assert!(found.decomps.contains(&want));
            assert!(ctx.work.ops.adds > 0);
            break;
        }
    }

    #[test]
    fn gb_text_round_trips_and_compare_gb_equal() {
        use crypto_lib::cryptanalysis::f4_fp::{f4, normalise, F4Options, Ordering};
        let cell = small_cell();
        let (m, t) = (2usize, 2usize);
        let spec = Spec { p: cell.p, a: cell.a, b: cell.b, g: cell.g, zeta: cell.zeta, m, t, x_r: 4242 };
        let (polys, n) = build(&spec);
        let input: Vec<_> = polys.iter().map(|f| normalise(f, cell.p, Ordering::Grevlex)).collect();
        let rep = f4(&input, n, cell.p, &F4Options::new(Ordering::Grevlex, 12));
        let names = system::var_names(m, t);
        let text = native_solve::gb_text(&rep.basis, &names, cell.p);
        let dir = std::env::temp_dir().join(format!("pkm_m8_gb_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (fa, fb, fo) = (dir.join("a.txt"), dir.join("b.txt"), dir.join("o.json"));
        std::fs::write(&fa, &text).unwrap();
        // a reordered copy of the same basis (element order and term order are not significant)
        std::fs::write(&fb, text.lines().rev().collect::<Vec<_>>().join("\n")).unwrap();
        native_solve::compare_gb(&[
            "--a".into(), fa.display().to_string(), "--b".into(), fb.display().to_string(), "--out".into(), fo.display().to_string(),
        ])
        .unwrap();
        let j = read_json(&fo).unwrap();
        assert_eq!(j["equal"], true);
        std::fs::remove_dir_all(&dir).ok();
    }
}
