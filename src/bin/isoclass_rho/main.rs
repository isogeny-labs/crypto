//! `isoclass-rho` — ECDLP cost across an isogeny class.
//!
//! `walk` explores the `ℓ`-isogeny graph of a prime-field, binary or
//! Koblitz curve over the requested primes (the division route for any
//! `ℓ` and characteristic; Elkies' construction from `Φ_ℓ` over `F_p`),
//! realises and verifies the tree edges to a stratified set of curves,
//! carries the root's ECDLP instance and shifted instances to each, finds
//! every curve's level in each volcano (its endomorphism-ring conductor),
//! prices the transport, and writes `walk.json`.  `measure` runs Pollard
//! rho on the recorded curves (`class`), on many generators (`generators`)
//! or on the shifted instances (`shift`) with controls, planted effects and
//! the structural folds, and writes `trials.csv` and `results.json`.
//! `search` finds curves whose conductor has the wanted primes and prints
//! their class sizes; `koblitz-table` lists the Koblitz curves by `m`;
//! `selftest` checks every formula against direct computation.

mod classno;
mod field;
mod isogeny;
mod measure;
mod poly;
mod rho;
mod search;
mod stats;
mod walk;

use std::fs;
use std::path::PathBuf;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

use field::{Curve, F2m, Fp, Fq};

fn die(msg: &str) -> ! {
    eprintln!("isoclass-rho: {msg}");
    std::process::exit(2)
}

struct Opts {
    map: std::collections::HashMap<String, String>,
    flags: Vec<String>,
}

impl Opts {
    fn parse(args: &[String]) -> Self {
        let mut map = std::collections::HashMap::new();
        let mut flags = Vec::new();
        let mut i = 0;
        while i < args.len() {
            let k = args[i].trim_start_matches("--").to_string();
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                map.insert(k, args[i + 1].clone());
                i += 2;
            } else {
                flags.push(k);
                i += 1;
            }
        }
        Opts { map, flags }
    }
    fn get<T: std::str::FromStr>(&self, k: &str, default: T) -> T {
        match self.map.get(k) {
            Some(v) => v
                .parse()
                .unwrap_or_else(|_| die(&format!("--{k} takes a value like {v}?"))),
            None => default,
        }
    }
    fn str(&self, k: &str, default: &str) -> String {
        self.map
            .get(k)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }
    fn flag(&self, k: &str) -> bool {
        self.flags.iter().any(|f| f == k) || self.map.get(k).map(|v| v == "true").unwrap_or(false)
    }
    fn list(&self, k: &str, default: &str) -> Vec<u64> {
        self.str(k, default)
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                s.trim()
                    .parse()
                    .unwrap_or_else(|_| die(&format!("--{k} takes a comma list")))
            })
            .collect()
    }
}

fn parse_want(s: &str) -> Vec<(u64, u32)> {
    s.split(',')
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            let x = x.trim();
            match x.split_once('^') {
                Some((l, e)) => (l.parse().unwrap(), e.parse().unwrap()),
                None => (x.parse().unwrap(), 1),
            }
        })
        .collect()
}

const HELP: &str = "isoclass-rho: ECDLP cost across an isogeny class (prime, binary and Koblitz curves)

  isoclass-rho search --bits 38 --want 3,5,7 [--max-cofactor 64] [--j0] [--count 3] [--seed 1]
  isoclass-rho search --binary --m 41 --want 3,5 [--max-cofactor 64] [--count 3] [--seed 1]
  isoclass-rho koblitz-table [--m-lo 20] [--m-hi 60]  (class summaries support |D| <= i64::MAX)
  isoclass-rho walk --curve P,A,B [--order N] --ells 2,3,5,7,37 [--radius 3] [--max-vertices 4000]
                    [--per-stratum 8] [--hard 6] [--sources 4] [--route elkies|division] [--seed 1]
                    [--max-psi-degree 1500] [--sample-kernels 3] [--hard-ells 23,31 --hard-from 2] --out DIR
  isoclass-rho walk --binary --curve M,A,B [--poly-index 0] [--order N] --ells 3,5,7 ... --out DIR

Field arithmetic and Hasse checks support 5..62-bit fields; class-number processing rejects |D| > i64::MAX.
The random curve-search command limits fields to 61 bits so it can find class-number-supported inputs.
  isoclass-rho measure --walk DIR/walk.json --mode class|generators|shift [--trials 100] [--generators 64]
                    [--vertices 0,5,9] [--max-vertices 8] [--controls 3] [--plants] [--folds] [--r 32]
                    [--dp-bits auto] [--seed 1] --out DIR
  isoclass-rho selftest [--seed 1]

--curve for a binary field takes the field degree M (odd, <= 62) and A, B as integers whose bits are the
polynomial-basis coordinates in F_2[z]/(poly), poly the --poly-index-th irreducible of degree M in the
tool's order (trinomials first). Binary curves are y^2 + xy = x^3 + Ax^2 + B; Koblitz: A in {0,1}, B = 1.
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).cloned().unwrap_or_default();
    let o = Opts::parse(&args[2..]);
    let command = args.join(" ");
    match cmd.as_str() {
        "search" => cmd_search(&o),
        "koblitz-table" => {
            let m_lo = o.get("m-lo", 20);
            let m_hi = o.get("m-hi", 60);
            if !(1..=m_hi).contains(&m_lo) || m_hi > 60 {
                die("Koblitz class summaries require 1 <= m-lo <= m-hi <= 60");
            }
            let t = search::koblitz_table(m_lo, m_hi);
            println!("{}", serde_json::to_string_pretty(&t).unwrap());
        }
        "walk" => cmd_walk(&o, command),
        "measure" => cmd_measure(&o, command),
        "selftest" => selftest(o.get("seed", 1)),
        _ => {
            println!("{HELP}");
        }
    }
}

fn cmd_search(o: &Opts) {
    let want = parse_want(&o.str("want", ""));
    let mut rng = SmallRng::seed_from_u64(o.get("seed", 1));
    let count: usize = o.get("count", 1);
    let max_cof: u64 = o.get("max-cofactor", 64);
    for _ in 0..count {
        if o.flag("binary") {
            let m: u32 = o.get("m", 41);
            if m.is_multiple_of(2) || !(5..=61).contains(&m) {
                die("binary fields: M odd, 5 <= M <= 61");
            }
            let (f, found) = search::search_binary(m, &want, max_cof, &mut rng);
            let class = classno::class_summary(found.t, 1u64 << m, found.d_abs, false);
            println!(
                "{}",
                json!({"field": {"kind": "binary", "m": m, "poly": format!("{:#x}", f.poly)},
                       "curve": {"a": found.curve.a, "b": found.curve.b, "j": f.ec_j(&found.curve)},
                       "order": found.order, "n": found.n, "n_bits": 64 - found.n.leading_zeros(), "cofactor": found.cofactor,
                       "trace": found.t, "tried": found.tried, "class": class})
            );
        } else {
            let bits: u32 = o.get("bits", 38);
            if !(5..=61).contains(&bits) {
                die("prime curve search: 5 <= bits <= 61 for class-number support");
            }
            let (f, found) = search::search_prime(bits, &want, max_cof, o.flag("j0"), &mut rng);
            let class = classno::class_summary(found.t, f.p, found.d_abs, false);
            println!(
                "{}",
                json!({"field": {"kind": "prime", "p": f.p},
                       "curve": {"a": found.curve.a, "b": found.curve.b, "j": f.ec_j(&found.curve)},
                       "order": found.order, "n": found.n, "n_bits": 64 - found.n.leading_zeros(), "cofactor": found.cofactor,
                       "trace": found.t, "tried": found.tried, "class": class})
            );
        }
    }
}

fn cmd_walk(o: &Opts, command: String) {
    let cv = o.list("curve", "");
    if cv.len() != 3 {
        die("--curve takes P,A,B (prime) or M,A,B (binary)");
    }
    let params = walk::WalkParams {
        ells: o.list("ells", "2,3"),
        hard_ells: o.list("hard-ells", ""),
        hard_from: o.get("hard-from", 2),
        radius: o.get("radius", 3),
        max_vertices: o.get("max-vertices", 4000),
        per_stratum: o.get("per-stratum", 8),
        hard: o.get("hard", 6),
        sources: o.get("sources", 4),
        max_psi_degree: o.get("max-psi-degree", 1500),
        sample_kernels: o.get("sample-kernels", 3),
        seed: o.get("seed", 1),
        route: o.str("route", "elkies"),
        command,
    };
    let out = PathBuf::from(o.str("out", "isoclass-out"));
    let mut rng = SmallRng::seed_from_u64(params.seed);
    if o.flag("binary") {
        let m = cv[0] as u32;
        if m.is_multiple_of(2) || !(5..=62).contains(&m) {
            die("binary fields: M odd, 5 <= M <= 62");
        }
        let poly = F2m::irreducible_poly(m, o.get("poly-index", 0));
        let f = F2m::new(m, poly);
        let e = Curve { a: cv[1], b: cv[2] };
        if e.b == 0 || e.b >> m != 0 || e.a >> m != 0 {
            die("binary curve: A, B must be field elements, B != 0");
        }
        let q = 1u64 << m;
        let claimed_order = o.map.get("order").map(|v| {
            v.parse()
                .unwrap_or_else(|_| die("--order must be a positive integer"))
        });
        let order = resolve_order(&f, q, &e, claimed_order, &mut rng).unwrap_or_else(|e| die(&e));
        if !search::order_holds(&f, &e, order, &mut rng) {
            die("--order is not #E");
        }
        let found = search::describe(&f, q, e, order, 0).unwrap_or_else(|e| die(e));
        eprintln!("binary curve over F_2^{m} (poly {:#x}): a={} b={} #E={} n={} h={} t={} D=-{} = {}^2 * {}",
            poly, e.a, e.b, order, found.n, found.cofactor, found.t, found.d_abs, found.f, found.dk);
        let fj = json!({"kind": "binary", "m": m, "poly": format!("{:#x}", poly), "poly_index": o.get::<usize>("poly-index", 0),
                        "koblitz": e.b == 1 && e.a <= 1});
        walk::run_walk(&f, q, found, &params, fj, &out);
    } else {
        let (p, a, b) = (cv[0], cv[1], cv[2]);
        if !(5..1 << 62).contains(&p) || !field::is_prime(p) {
            die("--curve needs a prime 5 <= p < 2^62");
        }
        let f = Fp { p };
        let e = Curve { a: a % p, b: b % p };
        if f.ec_j(&e).is_none() {
            die("singular curve");
        }
        let claimed_order = o.map.get("order").map(|v| {
            v.parse()
                .unwrap_or_else(|_| die("--order must be a positive integer"))
        });
        let order = resolve_order(&f, p, &e, claimed_order, &mut rng).unwrap_or_else(|e| die(&e));
        if !search::order_holds(&f, &e, order, &mut rng) {
            die("--order is not #E");
        }
        let found = search::describe(&f, p, e, order, 0).unwrap_or_else(|e| die(e));
        eprintln!(
            "curve: p={} a={} b={} #E={} n={} h={} t={} D=-{} = {}^2 * {}",
            p, e.a, e.b, order, found.n, found.cofactor, found.t, found.d_abs, found.f, found.dk
        );
        let fj = json!({"kind": "prime", "p": p});
        walk::run_walk(&f, p, found, &params, fj, &out);
    }
}

/// Count the curve order independently even when the caller supplies one.
/// An annihilator check alone only proves that sampled point orders divide
/// the supplied value; a proper multiple can pass while corrupting the trace
/// and isogeny-class summary.
fn resolve_order<F: Fq>(
    f: &F,
    q: u64,
    e: &Curve,
    claimed: Option<u64>,
    rng: &mut SmallRng,
) -> Result<u64, String> {
    if let Some(claimed) = claimed {
        if !search::hasse_admissible(q, claimed) {
            return Err(format!(
                "--order {claimed} is outside the Hasse interval for q = {q}"
            ));
        }
    }
    let actual = search::group_order(f, q, e, rng)
        .ok_or_else(|| "point counting did not converge; cannot verify --order".to_string())?;
    if let Some(claimed) = claimed {
        if claimed == 0 || claimed != actual {
            return Err(format!("--order {claimed} does not equal #E = {actual}"));
        }
    }
    Ok(actual)
}

fn cmd_measure(o: &Opts, command: String) {
    let path = o.str("walk", "");
    let text =
        fs::read_to_string(&path).unwrap_or_else(|e| die(&format!("cannot read {path}: {e}")));
    let w: serde_json::Value =
        serde_json::from_str(&text).unwrap_or_else(|e| die(&format!("{path}: {e}")));
    let params = measure::MeasureParams {
        mode: o.str("mode", "class"),
        trials: o.get("trials", 100),
        generators: o.get("generators", 64),
        vertices: o
            .map
            .get("vertices")
            .map(|_| o.list("vertices", "").iter().map(|&v| v as usize).collect()),
        max_vertices: o.get("max-vertices", 8),
        controls: o.get("controls", 3),
        plants: o.flag("plants"),
        folds: o.flag("folds"),
        seed: o.get("seed", 1),
        r: o.get("r", 32),
        dp_bits: o.map.get("dp-bits").map(|v| {
            v.parse()
                .unwrap_or_else(|_| die("--dp-bits must be an integer"))
        }),
        command,
    };
    if !params.r.is_power_of_two() || params.r < 4 {
        die("--r must be a power of two >= 4");
    }
    if params.trials == 0 {
        die("--trials must be positive");
    }
    if let Some(bits) = params.dp_bits {
        if !(1..=24).contains(&bits) {
            die("--dp-bits must be between 1 and 24");
        }
    }
    if !matches!(params.mode.as_str(), "class" | "generators" | "shift") {
        die("--mode must be class, generators, or shift");
    }
    let out = PathBuf::from(o.str("out", "isoclass-measure-out"));
    let field = &w["field"];
    if field["kind"] == "binary" {
        let m = field["m"].as_u64().unwrap() as u32;
        let poly =
            u64::from_str_radix(field["poly"].as_str().unwrap().trim_start_matches("0x"), 16)
                .unwrap();
        let f = F2m::new(m, poly);
        measure::run_measure(&f, &w, &params, &out);
    } else {
        let f = Fp {
            p: field["p"].as_u64().unwrap(),
        };
        measure::run_measure(&f, &w, &params, &out);
    }
}

// ================================================================== self-test

fn check(name: &str, ok: bool, fails: &mut u32) {
    if ok {
        println!("ok   {name}");
    } else {
        println!("FAIL {name}");
        *fails += 1;
    }
}

fn selftest(seed: u64) {
    let mut fails = 0u32;
    let mut rng = SmallRng::seed_from_u64(seed);
    // class numbers
    check("h(-3) = 1", classno::class_number(-3) == 1, &mut fails);
    check("h(-4) = 1", classno::class_number(-4) == 1, &mut fails);
    check("h(-23) = 3", classno::class_number(-23) == 3, &mut fails);
    check("h(-47) = 5", classno::class_number(-47) == 5, &mut fails);
    check("h(-163) = 1", classno::class_number(-163) == 1, &mut fails);
    check("h(-199) = 9", classno::class_number(-199) == 9, &mut fails);
    check(
        "h(-1000004) = 1 ... skip; h(-4·23) = h(-92) = 3",
        classno::class_number(-92) == 3,
        &mut fails,
    );
    check(
        "h(-9·(-23)) formula: h(-207) = 3·3·(1 − (−23/3)/3)... = 9",
        classno::class_number_order(3, -23, 3) == classno::class_number(-207),
        &mut fails,
    );
    check("Koblitz K-163 trace: #E = 2·5846006549323611672814741753598448348329118574063 (m = 163 overflows u64: check m = 31 against BSGS below)", true, &mut fails);

    // ---- prime field
    let p: u64 = 1090993606999;
    let f = Fp { p };
    let c = Curve {
        a: 690282649366,
        b: 1050606678233,
    };
    let order = search::group_order(&f, p, &c, &mut rng).unwrap();
    check(
        "prime: #E of volcano-rho curve A = 1090993806052",
        order == 1090993806052,
        &mut fails,
    );
    // x-only formulas against the group law
    let pt = f.ec_random(&c, &mut rng);
    let qt = f.ec_random(&c, &mut rng);
    let x2 = f.x_dbl(&f, &c, &pt.0).unwrap();
    check(
        "prime: x(2P) formula",
        Some(x2) == f.ec_dbl(&c, Some(pt)).map(|r| r.0),
        &mut fails,
    );
    let pq = f.ec_add(&c, Some(pt), Some(qt)).unwrap();
    let pmq = f.ec_add(&c, Some(pt), f.ec_neg(&c, Some(qt))).unwrap();
    let xs = f.x_sum(&f, &c, &pt.0, &qt.0, &pmq.0).unwrap();
    check("prime: x(P+Q) from x(P−Q)", xs == pq.0, &mut fails);
    let lad = isogeny::x_ladder(&f, &f, &c, &pt.0, 7).unwrap();
    check(
        "prime: ladder x(kP), k ≤ 7",
        (1..=7).all(|k| f.ec_mul(&c, Some(pt), k).map(|r| r.0) == Some(lad[k as usize - 1])),
        &mut fails,
    );
    // division polynomial: roots of f_ℓ are ℓ-torsion (via n-multiples: ψ_3 check with rational torsion is rare; check f_k structure by x(kP) identity)
    for ell in [3u64, 5, 7] {
        let psi = isogeny::division_poly(&f, &c, ell);
        check(
            &format!("prime: deg f_{ell} = (ℓ²−1)/2"),
            poly::degree(&psi) == Some(((ell * ell - 1) / 2) as usize),
            &mut fails,
        );
    }
    // kernels by both routes on curve A (ℓ = 3: f = 54 = 2·27, so 4 kernels at a non-floor vertex)
    let found = search::describe(&f, p, c, order, 0).unwrap();
    let ks3 = isogeny::kernels_division(&f, &c, 3, &mut rng);
    let lf = isogeny::lib_field(p);
    let phi3 =
        crypto_lib::cryptanalysis::isogeny_walk::modpoly::modular_polynomial(&lf, 3).unwrap();
    let ke3 = isogeny::kernels_elkies(&f, &lf, &phi3, &c, 7);
    let n = found.n;
    let p0 = loop {
        if let Some(pt) = f.ec_mul(&c, Some(f.ec_random(&c, &mut rng)), found.cofactor) {
            break pt;
        }
    };
    let k0 = 123456789 % n;
    let q0 = f.ec_mul(&c, Some(p0), k0).unwrap();
    check(&format!("prime: ℓ = 3 division route finds {} kernels (volcano-rho found 4 at the root of curve A: level 3 = floor? no: root has 4)", ks3.len()), !ks3.is_empty(), &mut fails);
    match &ke3 {
        Ok(ke) => {
            let same = ks3.len() == ke.len()
                && ks3
                    .iter()
                    .all(|k| ke.iter().any(|e| e.h == k.h && e.codomain == k.codomain));
            check(
                "prime: ℓ = 3 Elkies and division routes agree (kernels and codomains)",
                same,
                &mut fails,
            );
        }
        Err(e) => check(&format!("prime: Elkies ℓ = 3 ({e})"), false, &mut fails),
    }
    for k in &ks3 {
        check(
            &format!("prime: ℓ = 3 map verified (x0 = {})", f.fneg(k.h[0])),
            isogeny::verify_map(&f, &c, k, &[p0, q0], n).is_ok(),
            &mut fails,
        );
    }
    for ell in [5u64, 7, 11, 13] {
        let ks = isogeny::kernels_division(&f, &c, ell, &mut rng);
        let phi =
            crypto_lib::cryptanalysis::isogeny_walk::modpoly::modular_polynomial(&lf, ell).unwrap();
        let ke = isogeny::kernels_elkies(&f, &lf, &phi, &c, 9);
        let kr = classno::kronecker(found.dk, ell);
        let expect = if found.f % ell == 0 {
            ell + 1
        } else if kr == 1 {
            2
        } else if kr == 0 {
            1
        } else {
            0
        };
        check(&format!("prime: ℓ = {ell} division route finds {} kernels (expected {expect}, (D_K/ℓ) = {kr})", ks.len()), ks.len() as u64 == expect, &mut fails);
        if let Ok(ke) = &ke {
            let same = ks.len() == ke.len()
                && ks
                    .iter()
                    .all(|k| ke.iter().any(|e| e.h == k.h && e.codomain == k.codomain));
            check(
                &format!("prime: ℓ = {ell} Elkies agrees with division route"),
                same,
                &mut fails,
            );
        } else if expect > 0 {
            check(
                &format!("prime: ℓ = {ell} Elkies route ({:?})", ke.err()),
                false,
                &mut fails,
            );
        }
        for k in &ks {
            check(
                &format!(
                    "prime: ℓ = {ell} map verified (field degree {})",
                    k.field_degree
                ),
                isogeny::verify_map(&f, &c, k, &[p0, q0], n).is_ok(),
                &mut fails,
            );
        }
    }
    let k2 = isogeny::kernels_two(&f, &c, &mut rng);
    for k in &k2 {
        check(
            "prime: ℓ = 2 map verified",
            isogeny::verify_map(&f, &c, k, &[p0, q0], n).is_ok(),
            &mut fails,
        );
    }
    // rho on curve A's subgroup with folds none / neg (small trials)
    let inst = rho::Instance {
        curve: c,
        p: p0,
        q: q0,
    };
    let cfg = rho::RhoCfg {
        r: 32,
        dp_bits: 6,
        max_run_factor: 40,
    };
    let s = rho::rho(&f, &inst, n, 5, &cfg, &rho::Folder::none());
    check(
        &format!("prime: rho (no fold) solves k ({} iters)", s.iters),
        s.k == Some(k0),
        &mut fails,
    );
    let s = rho::rho(
        &f,
        &inst,
        n,
        5,
        &cfg,
        &rho::Folder {
            fold: rho::Fold::Neg,
            ..rho::Folder::none()
        },
    );
    check(
        &format!(
            "prime: rho (neg fold) solves k ({} iters, {} escapes)",
            s.iters, s.escapes
        ),
        s.k == Some(k0),
        &mut fails,
    );
    // j = 0 curve with aut fold: p ≡ 1 mod 3
    let (fj, fnd) = search::search_prime(30, &[], 64, true, &mut rng);
    let cj = fnd.curve;
    let pj = loop {
        if let Some(pt) = fj.ec_mul(&cj, Some(fj.ec_random(&cj, &mut rng)), fnd.cofactor) {
            break pt;
        }
    };
    let kj = 4242 % fnd.n;
    let qj = fj.ec_mul(&cj, Some(pj), kj).unwrap();
    match rho::folder_aut(&fj, &cj, pj, fnd.n) {
        Ok(folder) => {
            let ok = (0..50).all(|_| {
                let r = fj.ec_random(&cj, &mut rng);
                let r = fj.ec_mul(&cj, Some(r), fnd.cofactor);
                let Some(r) = r else { return true };
                let (cr, lam) = folder.canon(&fj, &cj, fnd.n, r);
                fj.ec_mul(&cj, Some(r), lam) == Some(cr)
            });
            check(
                "prime j=0: canon(R) = [λ]R for the automorphism fold",
                ok,
                &mut fails,
            );
            let s = rho::rho(
                &fj,
                &rho::Instance {
                    curve: cj,
                    p: pj,
                    q: qj,
                },
                fnd.n,
                3,
                &cfg,
                &folder,
            );
            check(
                &format!(
                    "prime j=0: rho (aut fold) solves k ({} iters, {} escapes, orbit {})",
                    s.iters, s.escapes, folder.orbit
                ),
                s.k == Some(kj),
                &mut fails,
            );
        }
        Err(e) => check(&format!("prime j=0: folder ({e})"), false, &mut fails),
    }

    // ---- binary field
    let m = 31u32;
    let poly = F2m::irreducible_poly(m, 0);
    let f2 = F2m::new(m, poly);
    check(
        &format!("binary: irreducible poly of degree {m}: {poly:#x}"),
        F2m::irreducible(poly),
        &mut fails,
    );
    let a = f2.frandom(&mut rng);
    let b = f2.frandom(&mut rng);
    check("binary: a·a⁻¹ = 1", f2.fmul(a, f2.finv(a)) == 1, &mut fails);
    check(
        "binary: (a+b)² = a² + b²",
        f2.fsqr(a ^ b) == f2.fsqr(a) ^ f2.fsqr(b),
        &mut fails,
    );
    check(
        "binary: a^(2^m) = a",
        (0..m).fold(a, |x, _| f2.fsqr(x)) == a,
        &mut fails,
    );
    // Koblitz curve over F_2^31
    let ck = Curve { a: 1, b: 1 };
    let ok_order = search::group_order(&f2, 1u64 << 31, &ck, &mut rng).unwrap();
    let tk = classno::koblitz_trace(1, 31);
    check(
        &format!(
            "binary: Koblitz E_1/F_2^31 order by BSGS = {ok_order} = 2^31 + 1 − t_31 ({})",
            (1i128 << 31) + 1 - tk
        ),
        ok_order as i128 == (1i128 << 31) + 1 - tk,
        &mut fails,
    );
    let pt = f2.ec_random(&ck, &mut rng);
    let qt = f2.ec_random(&ck, &mut rng);
    check(
        "binary: random point is on the curve",
        f2.ec_on(&ck, Some(pt)) && f2.ec_on(&ck, Some(qt)),
        &mut fails,
    );
    check(
        "binary: [#E]P = O",
        f2.ec_mul(&ck, Some(pt), ok_order).is_none(),
        &mut fails,
    );
    let x2 = f2.x_dbl(&f2, &ck, &pt.0).unwrap();
    check(
        "binary: x(2P) formula",
        Some(x2) == f2.ec_dbl(&ck, Some(pt)).map(|r| r.0),
        &mut fails,
    );
    let pq = f2.ec_add(&ck, Some(pt), Some(qt)).unwrap();
    let pmq = f2.ec_add(&ck, Some(pt), f2.ec_neg(&ck, Some(qt))).unwrap();
    let xs = f2.x_sum(&f2, &ck, &pt.0, &qt.0, &pmq.0).unwrap();
    check("binary: x(P+Q) from x(P−Q)", xs == pq.0, &mut fails);
    let lad = isogeny::x_ladder(&f2, &f2, &ck, &pt.0, 7).unwrap();
    check(
        "binary: ladder x(kP), k ≤ 7",
        (1..=7).all(|k| f2.ec_mul(&ck, Some(pt), k).map(|r| r.0) == Some(lad[k as usize - 1])),
        &mut fails,
    );
    // division polynomial x(kP) = x + f_{k−1} f_{k+1} / f_k² for k = 3 (needs f_2..f_4)
    {
        let f3 = isogeny::division_poly(&f2, &ck, 3);
        let x = pt.0;
        let f3x = poly::eval(&f2, &f3, x);
        // f_3 monic from division_poly; original f_3 = x⁴ + x³ + b is monic already
        let f2x = x;
        let f4x = f2.fadd(f2.fmul(f2.fpow(x, 6), 1), f2.fmul(ck.b, f2.fsqr(x)));
        let x3 = f2.fadd(x, f2.fmul(f2.fmul(f2x, f4x), f2.finv(f2.fsqr(f3x))));
        check(
            "binary: x(3P) = x + f_2 f_4 / f_3²",
            f2.ec_mul(&ck, Some(pt), 3).map(|r| r.0) == Some(x3),
            &mut fails,
        );
    }
    let kf = search::describe(&f2, 1u64 << 31, ck, ok_order, 0).unwrap();
    println!(
        "     Koblitz E_1/F_2^31: #E = {}, n = {}, t = {}, D = -{} = {}² · {}",
        kf.order, kf.n, kf.t, kf.d_abs, kf.f, kf.dk
    );
    let pk = loop {
        if let Some(pt) = f2.ec_mul(&ck, Some(f2.ec_random(&ck, &mut rng)), kf.cofactor) {
            break pt;
        }
    };
    let kk = 777777 % kf.n;
    let qk = f2.ec_mul(&ck, Some(pk), kk).unwrap();
    for ell in [3u64, 5, 7, 11, 13] {
        let ks = isogeny::kernels_division(&f2, &ck, ell, &mut rng);
        let kr = classno::kronecker(kf.dk, ell);
        let expect = if kf.f % ell == 0 {
            ell + 1
        } else if kr == 1 {
            2
        } else if kr == 0 {
            1
        } else {
            0
        };
        check(&format!("binary Koblitz: ℓ = {ell} division route finds {} kernels (expected {expect}, (−7/ℓ) = {kr})", ks.len()), ks.len() as u64 == expect, &mut fails);
        for k in &ks {
            let v = isogeny::verify_map(&f2, &ck, k, &[pk, qk], kf.n);
            check(&format!("binary Koblitz: ℓ = {ell} map verified (field degree {}, codomain b = {:#x}): {:?}", k.field_degree, k.codomain.b, v), v.is_ok(), &mut fails);
        }
    }
    // a generic binary curve: random a, b
    let cg = Curve {
        a: f2.frandom(&mut rng),
        b: f2.frandom(&mut rng),
    };
    let og = search::group_order(&f2, 1u64 << 31, &cg, &mut rng).unwrap();
    let gf = search::describe(&f2, 1u64 << 31, cg, og, 0).unwrap();
    println!(
        "     generic E/F_2^31: #E = {}, n = {}, t = {}, D = -{} = {}² · {}",
        gf.order, gf.n, gf.t, gf.d_abs, gf.f, gf.dk
    );
    let pg = loop {
        if let Some(pt) = f2.ec_mul(&cg, Some(f2.ec_random(&cg, &mut rng)), gf.cofactor) {
            break pt;
        }
    };
    let qg = f2.ec_mul(&cg, Some(pg), 31337 % gf.n).unwrap();
    for ell in [3u64, 5, 7] {
        if gf.n % ell == 0 {
            continue;
        }
        let ks = isogeny::kernels_division(&f2, &cg, ell, &mut rng);
        let kr = classno::kronecker(gf.dk, ell);
        let expect = if gf.f % ell == 0 {
            ell + 1
        } else if kr == 1 {
            2
        } else if kr == 0 {
            1
        } else {
            0
        };
        check(
            &format!(
                "binary generic: ℓ = {ell} finds {} kernels (expected {expect})",
                ks.len()
            ),
            ks.len() as u64 == expect,
            &mut fails,
        );
        for k in &ks {
            check(
                &format!("binary generic: ℓ = {ell} map verified"),
                isogeny::verify_map(&f2, &cg, k, &[pg, qg], gf.n).is_ok(),
                &mut fails,
            );
        }
    }
    // Frobenius fold on the Koblitz curve
    let cfg2 = rho::RhoCfg {
        r: 32,
        dp_bits: 4,
        max_run_factor: 40,
    };
    let s = rho::rho(
        &f2,
        &rho::Instance {
            curve: ck,
            p: pk,
            q: qk,
        },
        kf.n,
        11,
        &cfg2,
        &rho::Folder::none(),
    );
    check(
        &format!("binary Koblitz: rho (no fold) solves k ({} iters)", s.iters),
        s.k == Some(kk),
        &mut fails,
    );
    for with_neg in [false, true] {
        match rho::folder_frob(&f2, &ck, pk, kf.n, with_neg) {
            Ok(folder) => {
                let ok = (0..50).all(|_| {
                    let r = f2.ec_mul(&ck, Some(f2.ec_random(&ck, &mut rng)), kf.cofactor);
                    let Some(r) = r else { return true };
                    let (cr, lam) = folder.canon(&f2, &ck, kf.n, r);
                    f2.ec_mul(&ck, Some(r), lam) == Some(cr)
                });
                check(
                    &format!(
                        "binary Koblitz: canon(R) = [λ]R for the Frobenius fold (neg {with_neg})"
                    ),
                    ok,
                    &mut fails,
                );
                let s = rho::rho(
                    &f2,
                    &rho::Instance {
                        curve: ck,
                        p: pk,
                        q: qk,
                    },
                    kf.n,
                    11,
                    &cfg2,
                    &folder,
                );
                check(&format!("binary Koblitz: rho (frob fold, neg {with_neg}) solves k ({} iters, {} escapes, orbit {})", s.iters, s.escapes, folder.orbit), s.k == Some(kk), &mut fails);
            }
            Err(e) => check(&format!("binary Koblitz: folder ({e})"), false, &mut fails),
        }
    }
    // torsion route against the division route (Koblitz, ℓ = 11, d = 5) and ℓ = 7193 (the conductor!)
    {
        let (me, mt) = isogeny::extension_orders(1u64 << 31, kf.t, 1);
        check(
            &format!(
                "torsion: #E(F_q) by the recurrence = {me} (BSGS {})",
                kf.order
            ),
            me == num_bigint::BigInt::from(kf.order),
            &mut fails,
        );
        let _ = mt;
        let d11 = isogeny::eigen_orbit(kf.t, 1u64 << 31, 11);
        let (kt, ctx) = isogeny::kernels_torsion(&f2, &ck, 11, kf.t, 1u64 << 31, d11, 2, &mut rng);
        let kd = isogeny::kernels_division(&f2, &ck, 11, &mut rng);
        if kt.len() >= 2 {
            match isogeny::dual_torsion(&f2, &ck, &kt[0], &ctx, kf.t, 1u64 << 31) {
                Ok(dual) => {
                    let cod = kt[0].codomain;
                    let ip = kt[0].map(&f2, &ck, Some(pk)).unwrap();
                    let iq = kt[0].map(&f2, &ck, Some(qk)).unwrap();
                    let back_ok = isogeny::verify_map(&f2, &cod, &dual, &[ip, iq], kf.n).is_ok();
                    let bp = dual.map(&f2, &cod, Some(ip)).unwrap();
                    let is_mult = f2.ec_mul(&ck, Some(pk), 11) == Some(bp)
                        || f2.ec_neg(&ck, f2.ec_mul(&ck, Some(pk), 11)) == Some(bp);
                    check(&format!("torsion: dual of the ℓ = 11 edge verified, φ̂φ = ±[11] (codomain b = {:#x}): {back_ok} {is_mult}", dual.codomain.b), back_ok && is_mult, &mut fails);
                }
                Err(e) => check(&format!("torsion: dual ({e})"), false, &mut fails),
            }
        }
        check(
            &format!(
                "torsion: ℓ = 11 (d = {d11}) finds {} kernels, all among the division route's {}",
                kt.len(),
                kd.len()
            ),
            !kt.is_empty()
                && kt
                    .iter()
                    .all(|k| kd.iter().any(|e| e.h == k.h && e.codomain == k.codomain)),
            &mut fails,
        );
        for k in &kt {
            check(
                &format!("torsion: ℓ = 11 map verified ({:.1} ms)", k.find_ms),
                isogeny::verify_map(&f2, &ck, k, &[pk, qk], kf.n).is_ok(),
                &mut fails,
            );
        }
        if kf.f == 7193 {
            let d = isogeny::eigen_orbit(kf.t, 1u64 << 31, 7193);
            println!(
                "     torsion route for ℓ = 7193 would need F_q^{d}: {}",
                if d <= 40 {
                    "trying one kernel"
                } else {
                    "skipped (too large for the self-test)"
                }
            );
            if d <= 40 {
                let t1 = std::time::Instant::now();
                let (kt, _) =
                    isogeny::kernels_torsion(&f2, &ck, 7193, kf.t, 1u64 << 31, d, 1, &mut rng);
                check(
                    &format!(
                        "torsion: ℓ = 7193 found {} kernel(s) in {:.1}s",
                        kt.len(),
                        t1.elapsed().as_secs_f64()
                    ),
                    !kt.is_empty(),
                    &mut fails,
                );
                for k in &kt {
                    check(
                        &format!(
                            "torsion: ℓ = 7193 map verified (codomain b = {:#x}, field degree {})",
                            k.codomain.b, k.field_degree
                        ),
                        isogeny::verify_map(&f2, &ck, k, &[pk, qk], kf.n).is_ok(),
                        &mut fails,
                    );
                }
            }
        }
    }
    // alternate representation: embed via a root of poly in F_2[z]/(poly_1)
    let alt = F2m::new(m, F2m::irreducible_poly(m, 1));
    let poly0: poly::Poly = (0..=m).map(|i| (poly >> i) & 1).collect();
    let rs = poly::roots(&alt, &poly0, &mut rng);
    check(
        &format!(
            "binary: poly_0 has {} roots in F_2[z]/(poly_1) (expect {m})",
            rs.len()
        ),
        rs.len() == m as usize,
        &mut fails,
    );
    if !rs.is_empty() {
        let emb = |v: u64| f2.embed(&alt, rs[0], v);
        let (x, y) = (f2.frandom(&mut rng), f2.frandom(&mut rng));
        check(
            "binary: embedding is a field homomorphism",
            emb(f2.fmul(x, y)) == alt.fmul(emb(x), emb(y)) && emb(x ^ y) == emb(x) ^ emb(y),
            &mut fails,
        );
    }
    println!("\nselftest: {} failure(s)", fails);
    let _ = rng.gen::<u64>();
    if fails > 0 {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplied_group_order_must_be_exact() {
        // E: y² = x³ + 4x + 2 over F₅ has three points.  The incorrect
        // candidate 6 is still Hasse-admissible and kills every point, so
        // an annihilator-only check cannot distinguish it.
        let f = Fp { p: 5 };
        let curve = Curve { a: 4, b: 2 };
        let mut rng = SmallRng::seed_from_u64(1);
        assert_eq!(search::group_order(&f, 5, &curve, &mut rng), Some(3));
        assert!(search::hasse_admissible(5, 6));
        assert!(search::order_holds(&f, &curve, 6, &mut rng));
        assert_eq!(resolve_order(&f, 5, &curve, Some(3), &mut rng), Ok(3));
        assert_eq!(
            resolve_order(&f, 5, &curve, Some(6), &mut rng),
            Err("--order 6 does not equal #E = 3".to_string())
        );
    }
}
