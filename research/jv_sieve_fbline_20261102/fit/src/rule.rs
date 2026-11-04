//! The frozen rule that chooses n_H and n_T (PROTOCOL.md "Choosing n").
//! Cost model, per prime p (all counts are means over V2's 16 sparse records):
//!   route(n; w) = C_P(w) * (1 + k(n)/sqrt p)^(-GAMMA),  C_P(w) = M_P + w*GAE*N_P
//!   horner(n; w) = n*p*(17 + w*GAE*16)
//!   table(n; w)  = 38 q + w*GAE*(30 q + 3 n p),  q = p^2, n capped at L = p(p+1)/2
//! J(n; w) = (route + search) / C_P(w); the rule minimises the geometric mean of J over
//! the three primes on the grid n in {1, 2, 4, ..., 2^20} U {exhaustive}.
use serde_json::Value;

pub const GAMMA: f64 = 5.0;
pub const GAE: f64 = 331.0;
pub const PRIMES: [u64; 3] = [1009, 1511, 1823];

fn erfc(x: f64) -> f64 {
    // Numerical Recipes erfcc, fractional error < 1.2e-7 everywhere
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t
        * (-z * z - 1.26551223
            + t * (1.00002368
                + t * (0.37409196
                    + t * (0.09678418
                        + t * (-0.18628806
                            + t * (0.27886807
                                + t * (-1.13520398
                                    + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
            .exp();
    if x >= 0.0 { r } else { 2.0 - r }
}
/// Upper tail Q(x) = P(Z > x).
fn qtail(x: f64) -> f64 { 0.5 * erfc(x / std::f64::consts::SQRT_2) }

/// E[max of n standard normals] by quadrature of x * n * phi(x) * Phi(x)^(n-1).
pub fn kmax(n: f64) -> f64 {
    if n <= 1.0 { return 0.0; }
    let (lo, hi, h) = (-9.0f64, 12.0f64, 0.0005f64);
    let mut s = 0.0;
    let mut x = lo;
    while x < hi {
        let xm = x + h / 2.0;
        let phi = (-xm * xm / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let lg = (n - 1.0) * (-qtail(xm)).ln_1p();
        s += xm * n * phi * lg.exp() * h;
        x += h;
    }
    s
}

pub fn read_v2(dir: &str, p: u64) -> (f64, f64) {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(format!("{dir}/ic_{p}_sparse.json")).unwrap()).unwrap();
    let a = v.as_array().unwrap();
    let m = a.iter().map(|r| r["total_muls"].as_u64().unwrap() as f64).sum::<f64>() / a.len() as f64;
    let n = a.iter().map(|r| (r["sieve_adds"].as_u64().unwrap() + r["sieve_lookups"].as_u64().unwrap() + r["enum_adds"].as_u64().unwrap()) as f64).sum::<f64>() / a.len() as f64;
    (m, n)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode { Horner, Table }

/// J(n; w) / 1 for prime p; n = f64::INFINITY means exhaustive.
pub fn j(mode: Mode, n: f64, w: f64, p: u64, m: f64, nat: f64) -> f64 {
    let pf = p as f64;
    let q = pf * pf;
    let lines = pf * (pf + 1.0) / 2.0;
    let cp = m + w * GAE * nat;
    let ne = n.min(lines);
    let route = cp * (1.0 + kmax(ne) / pf.sqrt()).powf(-GAMMA);
    let search = match mode {
        Mode::Horner => n.min(1e12) * pf * (17.0 + w * GAE * 16.0),
        Mode::Table => 38.0 * q + w * GAE * (30.0 * q + 3.0 * ne * pf),
    };
    (route + search) / cp
}

pub fn grid() -> Vec<f64> {
    let mut g: Vec<f64> = (0..=20).map(|e| (1u64 << e) as f64).collect();
    g.push(f64::INFINITY);
    g
}

pub fn best(mode: Mode, w: f64, data: &[(u64, f64, f64)]) -> (f64, f64) {
    let mut bn = (f64::NAN, f64::INFINITY);
    for n in grid() {
        if mode == Mode::Horner && n.is_infinite() { continue; }
        let g = (data.iter().map(|&(p, m, nat)| j(mode, n, w, p, m, nat).ln()).sum::<f64>() / data.len() as f64).exp();
        if g < bn.1 { bn = (n, g); }
    }
    bn
}

pub fn run(dir: &str) {
    let data: Vec<(u64, f64, f64)> = PRIMES.iter().map(|&p| { let (m, n) = read_v2(dir, p); (p, m, n) }).collect();
    println!("# n-selection rule (model; GAMMA = {GAMMA}, GAE = {GAE})");
    for &(p, m, nat) in &data { println!("p={p}: V2 sparse mean total_muls {m:.4e}, route native (sieve adds+lookups+enum adds) {nat:.4e}"); }
    println!("k(n) = E[max of n std normals]: {}", [1u64, 16, 256, 4096, 65536, 1_000_000].iter().map(|&n| format!("k({n})={:.3}", kmax(n as f64))).collect::<Vec<_>>().join(", "));
    for w in [0.0, 0.1, 1.0] {
        for (name, mode) in [("H (Horner)", Mode::Horner), ("T (table)", Mode::Table)] {
            let (n, g) = best(mode, w, &data);
            let per: Vec<String> = data.iter().map(|&(p, m, nat)| format!("p={p}: {:.4}", j(mode, n, w, p, m, nat))).collect();
            println!("w = {w}: {name}: n = {}  geo-mean J/C_P = {g:.4}  [{}]", if n.is_infinite() { "exhaustive".to_string() } else { format!("{n}") }, per.join(", "));
        }
    }
    println!("\nJ(n; w=0) / J(n; w=0.1) / J(n; w=1) pooled by geometric mean, per grid n:");
    for n in grid() {
        let row: Vec<String> = [0.0, 0.1, 1.0].iter().flat_map(|&w| [Mode::Horner, Mode::Table].iter().map(move |&md| (w, md))).map(|(w, md)| {
            if md == Mode::Horner && n.is_infinite() { return "   -   ".to_string(); }
            let g = (data.iter().map(|&(p, m, nat)| j(md, n, w, p, m, nat).ln()).sum::<f64>() / 3.0).exp();
            format!("{g:7.4}")
        }).collect();
        println!("n={:>10}  [H,T at w=0 | H,T at w=0.1 | H,T at w=1] {}", if n.is_infinite() { "exh".into() } else { format!("{n}") }, row.join(" "));
    }
}
