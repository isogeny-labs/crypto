//! Paired analysis of the sieve route's addition-saving changes (PROTOCOL.md).
//! Rust only.  usage: sieveadds_fit analyze <v2_raw_dir> <results_dir> <out_dir>
use serde_json::Value;
use std::fmt::Write as _;

const NB: usize = 20000;
const PRIMES: [u64; 3] = [1009, 1511, 1823];
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
/// Fields of a record that the changes may legitimately move (cost of the same relations).
const MOVED: &[&str] = &[
    "table_muls", "enum_muls", "enum_adds", "sieve_muls", "sieve_adds", "relation_muls", "c_rel",
    "total_muls", "total_plus", "s", "s_plus", "s_over_rho", "s_plus_over_rho", "relation_over_rho",
    "descent_over_rho", "la_over_rho", "wall_ms", "enum_lookups", "sieve_shared_reads",
    "inc_hinted_bs", "shared_bs",
];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn idx(&mut self, n: usize) -> usize {
        (((self.next() as u128) * (n as u128)) >> 64) as usize
    }
}
fn load(path: &str) -> Vec<Value> {
    let s = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("missing {path}"));
    serde_json::from_str::<Value>(&s).unwrap().as_array().unwrap().clone()
}
fn exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}
fn u(v: &Value, k: &str) -> u64 {
    v.get(k).and_then(|x| x.as_u64()).unwrap_or(0)
}
fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("missing {k}"))
}
fn gmean(x: &[f64]) -> f64 {
    (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp()
}
fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / x.len() as f64
}
fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn sq(v: &Value) -> f64 {
    f(v, "c_add_e") * f(v, "l").sqrt()
}
fn s_of(v: &Value) -> f64 {
    u(v, "total_muls") as f64 / sq(v)
}
fn sp_of(v: &Value) -> f64 {
    u(v, "total_plus") as f64 / sq(v)
}
/// Emitted native operations: route, search and the changes' own.
fn natives(v: &Value) -> f64 {
    (u(v, "total_plus") - u(v, "total_muls")) as f64
}
fn cost_w(v: &Value, w: f64) -> f64 {
    u(v, "total_muls") as f64 + w * f(v, "c_add_e") * natives(v)
}
fn strip(v: &Value, drop: &[&str]) -> Value {
    let mut c = v.clone();
    for k in drop {
        c.as_object_mut().unwrap().remove(*k);
    }
    c
}
fn ok(v: &Value) -> bool {
    v["solved"].as_bool().unwrap()
        && v["correct"].as_bool().unwrap()
        && u(v, "rels_failed_verify") == 0
        && !v["exhausted"].as_bool().unwrap()
}
fn boot(strata: &[Vec<f64>], rng: &mut Rng) -> (f64, f64, f64) {
    let all: Vec<f64> = strata.iter().flatten().copied().collect();
    let g = gmean(&all);
    let mut bs = Vec::with_capacity(NB);
    for _ in 0..NB {
        let mut v = Vec::with_capacity(all.len());
        for s in strata {
            for _ in 0..s.len() {
                v.push(s[rng.idx(s.len())]);
            }
        }
        bs.push(gmean(&v));
    }
    (g, pct(&mut bs, 0.025), pct(&mut bs, 0.975))
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.get(1).map(String::as_str) != Some("analyze") || a.len() < 5 {
        eprintln!("usage: sieveadds_fit analyze <v2_raw> <results> <out>");
        return;
    }
    let (v2, res, out) = (&a[2], &a[3], &a[4]);
    let mut t = String::new();
    macro_rules! say {
        ($($x:tt)*) => {{ let s = format!($($x)*); println!("{s}"); t += &s; t += "\n"; }}
    }
    let mut rng = Rng(0xADD5_0000_2026_1103);
    let arm = |name: &str| -> Vec<Vec<Value>> {
        PRIMES.iter().map(|p| load(&format!("{res}/{name}_{p}.json"))).collect()
    };
    let pd = arm("P");
    let n_runs: usize = pd.iter().map(|v| v.len()).sum();

    // ---- C1: P = V2 ----
    let (mut c1, mut c1n) = (0, 0);
    for (pi, &p) in PRIMES.iter().enumerate() {
        let v = load(&format!("{v2}/ic_{p}_sparse.json"));
        assert_eq!(v.len(), pd[pi].len());
        for (x, y) in v.iter().zip(&pd[pi]) {
            c1n += 1;
            if strip(x, &["wall_ms"]) == strip(y, &["wall_ms"]) {
                c1 += 1;
            }
        }
    }
    say!("C1 arm P (old binary) equals V2's sparse records on every field but wall_ms: {c1}/{c1n}: {}", if c1 == c1n { "PASS" } else { "FAIL" });

    // ---- C2: N0 (new binary, both switches off) = P ----
    let new_keys = ["enum_lookups", "sieve_shared_reads", "inc_hinted_bs", "shared_bs"];
    let n0 = arm("N0");
    let (mut c2, mut c2z) = (0, 0);
    for pi in 0..3 {
        for (x, y) in pd[pi].iter().zip(&n0[pi]) {
            if strip(x, &["wall_ms"]) == strip(y, &["wall_ms", new_keys[0], new_keys[1], new_keys[2], new_keys[3]]) {
                c2 += 1;
            }
            if new_keys.iter().all(|k| u(y, k) == 0) {
                c2z += 1;
            }
        }
    }
    say!("C2 arm N0 (new binary, JV_ADDS_INC=0 JV_ADDS_SHARE=0) equals P on every field but wall_ms: {c2}/{n_runs}; new fields all zero {c2z}/{n_runs}: {}", if c2 == n_runs && c2z == n_runs { "PASS" } else { "FAIL" });

    // outcome fields that must not move
    let skip: Vec<&str> = MOVED.to_vec();
    let mut notes: Vec<String> = vec![];

    struct Cmp {
        label: String,
        s: Vec<Vec<f64>>,
        sp: Vec<Vec<f64>>,
        band: Vec<Vec<Vec<f64>>>,
    }
    let mut cmps: Vec<Cmp> = vec![];
    let mut all_ok = true;
    let pairs = [("N", "P"), ("NU", "U"), ("NI", "P"), ("NS", "P"), ("U", "P")];
    for (new, old) in pairs {
        if !exists(&format!("{res}/{new}_1009.json")) || !exists(&format!("{res}/{old}_1009.json")) {
            continue;
        }
        let (nd, od) = (arm(new), arm(old));
        let mut cm = Cmp { label: format!("{new}/{old}"), s: vec![], sp: vec![], band: vec![vec![vec![]; 3]; 3] };
        // control: outcome fields identical, accounting identities
        let (mut same, mut ident, mut nonneg) = (0, 0, 0);
        for pi in 0..3 {
            let (mut sr, mut spr) = (vec![], vec![]);
            for (y, x) in nd[pi].iter().zip(&od[pi]) {
                assert_eq!(u(x, "seed"), u(y, "seed"));
                all_ok &= ok(y);
                if new != "U" {
                    if strip(x, &skip) == strip(y, &skip) {
                        same += 1;
                    } else {
                        let (xo, yo) = (x.as_object().unwrap(), y.as_object().unwrap());
                        for (k, v) in xo {
                            if !skip.contains(&k.as_str()) && yo.get(k) != Some(v) {
                                notes.push(format!("{new}/{old} p={} seed={}: field {k} differs", u(x, "p"), u(x, "seed")));
                            }
                        }
                    }
                }
                let dm = u(y, "total_muls") as i64 - u(x, "total_muls") as i64;
                let parts = (u(y, "table_muls") as i64 - u(x, "table_muls") as i64)
                    + (u(y, "enum_muls") as i64 - u(x, "enum_muls") as i64)
                    + (u(y, "sieve_muls") as i64 - u(x, "sieve_muls") as i64);
                if dm == parts {
                    ident += 1;
                }
                let nat_new = u(y, "sieve_adds") + u(y, "sieve_lookups") + u(y, "enum_adds") + u(y, "enum_lookups") + u(y, "sieve_shared_reads");
                let nat_old = u(x, "sieve_adds") + u(x, "sieve_lookups") + u(x, "enum_adds") + u(x, "enum_lookups") + u(x, "sieve_shared_reads");
                let tot_new = u(y, "total_plus") - u(y, "total_muls") - nat_new;
                let tot_old = u(x, "total_plus") - u(x, "total_muls") - nat_old;
                if tot_new == tot_old {
                    nonneg += 1; // search natives equal (identity of total_plus)
                }
                sr.push(s_of(y) / s_of(x));
                spr.push(sp_of(y) / sp_of(x));
                for (wi, &w) in BAND.iter().enumerate() {
                    cm.band[pi][wi].push(cost_w(y, w) / cost_w(x, w));
                }
            }
            cm.s.push(sr);
            cm.sp.push(spr);
        }
        let tag = if new == "U" { "(arm U vs P: the earlier study's line search; reference only)".to_string() } else {
            format!("outcome fields equal {same}/{n_runs}; d(total_muls) = d(table+enum+sieve muls) {ident}/{n_runs}; total_plus identity {nonneg}/{n_runs}") };
        say!("Control {new} vs {old}: {tag}");
        cmps.push(cm);
    }
    say!("Correctness of every run of every arm read above (solved, correct, 0 failed verifications, not exhausted): {}", if all_ok { "PASS" } else { "FAIL" });
    if !notes.is_empty() {
        say!("DIFFERENCES: {:?}", &notes[..notes.len().min(10)]);
    }

    // ---- ratios ----
    say!("\n### Paired geometric-mean ratio new/old (95% bootstrap over seeds, {NB} resamples, strata = primes)");
    for cm in &cmps {
        say!("\n{}", cm.label);
        for (name, series) in [("S ", &cm.s), ("S+", &cm.sp)] {
            let mut line = format!("  {name}:");
            for (pi, &p) in PRIMES.iter().enumerate() {
                let (g, lo, hi) = boot(&[series[pi].clone()], &mut rng);
                write!(line, " p={p} {g:.4} [{lo:.4}, {hi:.4}];").unwrap();
            }
            let (g, lo, hi) = boot(series, &mut rng);
            write!(line, " pooled {g:.4} [{lo:.4}, {hi:.4}]").unwrap();
            say!("{line}");
        }
        say!("  band (w GAE per emitted native op; 1 GAE = c_add_e muls):");
        for (wi, &w) in BAND.iter().enumerate() {
            let strata: Vec<Vec<f64>> = (0..3).map(|pi| cm.band[pi][wi].clone()).collect();
            let (g, lo, hi) = boot(&strata, &mut rng);
            let per: Vec<String> = (0..3).map(|pi| { let (g, lo, hi) = boot(&[strata[pi].clone()], &mut rng); format!("{g:.3} [{lo:.3},{hi:.3}]") }).collect();
            say!("    w = {w}: pooled {g:.4} [{lo:.4}, {hi:.4}]  per prime {}", per.join("  "));
        }
    }
    if let Some(c) = cmps.iter().find(|c| c.label == "N/P") {
        let (_, _, hs) = boot(&c.s, &mut rng);
        let (g, _, hsp) = boot(&c.sp, &mut rng);
        say!("\nH1 (N vs P): S ratio upper end {hs:.4} (<= 1: S does not rise) {}; S+ ratio {g:.4}, upper end {hsp:.4} (< 1: S+ falls) {}",
            if hs <= 1.0 { "HOLDS" } else { "FAILS" }, if hsp < 1.0 { "SUPPORTED" } else { "NOT SUPPORTED" });
    }
    if let Some(c) = cmps.iter().find(|c| c.label == "NU/U") {
        let (_, _, hs) = boot(&c.s, &mut rng);
        let (g, _, hsp) = boot(&c.sp, &mut rng);
        say!("H2 (NU vs U): S ratio upper end {hs:.4} {}; S+ ratio {g:.4}, upper end {hsp:.4} {}",
            if hs <= 1.0 { "HOLDS" } else { "FAILS" }, if hsp < 1.0 { "SUPPORTED" } else { "NOT SUPPORTED" });
    }

    // ---- components ----
    say!("\n### Mean per run of each cost component, by arm and prime");
    let arms: Vec<&str> = ["P", "N", "NI", "NS", "U", "NU"].into_iter().filter(|a| exists(&format!("{res}/{a}_1009.json"))).collect();
    writeln!(t, "| arm | p | total_muls | enum_muls | sieve_muls | table_muls | enum_adds | enum_lookups | sieve_adds | sieve_lookups | shared_reads | total_plus | S+ / S |").unwrap();
    writeln!(t, "|--|--|--|--|--|--|--|--|--|--|--|--|--|").unwrap();
    for a in &arms {
        let d = arm(a);
        for (pi, &p) in PRIMES.iter().enumerate() {
            let m = |k: &str| mean(&d[pi].iter().map(|r| u(r, k) as f64).collect::<Vec<_>>());
            let r = mean(&d[pi].iter().map(|r| sp_of(r) / s_of(r)).collect::<Vec<_>>());
            writeln!(t, "| {a} | {p} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.4e} | {:.2} |",
                m("total_muls"), m("enum_muls"), m("sieve_muls"), m("table_muls"), m("enum_adds"), m("enum_lookups"), m("sieve_adds"), m("sieve_lookups"), m("sieve_shared_reads"), m("total_plus"), r).unwrap();
        }
    }
    writeln!(t, "\nPer unit (mean): adds per B enumerated, per line; B's with shared arrays, B's hinted\n").unwrap();
    writeln!(t, "| arm | p | B's | lines | lines per B | enum adds per B | enum adds+lookups per B | sieve adds per line | sieve adds+reads per line | hinted B's | shared B's |").unwrap();
    writeln!(t, "|--|--|--|--|--|--|--|--|--|--|--|").unwrap();
    for a in &arms {
        let d = arm(a);
        for (pi, &p) in PRIMES.iter().enumerate() {
            let m = |g: &dyn Fn(&Value) -> f64| mean(&d[pi].iter().map(|r| g(r)).collect::<Vec<_>>());
            writeln!(t, "| {a} | {p} | {:.0} | {:.0} | {:.3} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} |",
                m(&|r| u(r, "bs") as f64), m(&|r| u(r, "lines") as f64), m(&|r| u(r, "lines") as f64 / u(r, "bs") as f64),
                m(&|r| u(r, "enum_adds") as f64 / u(r, "bs") as f64), m(&|r| (u(r, "enum_adds") + u(r, "enum_lookups")) as f64 / u(r, "bs") as f64),
                m(&|r| u(r, "sieve_adds") as f64 / u(r, "lines") as f64), m(&|r| (u(r, "sieve_adds") + u(r, "sieve_shared_reads")) as f64 / u(r, "lines") as f64),
                m(&|r| u(r, "inc_hinted_bs") as f64), m(&|r| u(r, "shared_bs") as f64)).unwrap();
        }
    }
    std::fs::create_dir_all(out).unwrap();
    std::fs::write(format!("{out}/analysis.txt"), &t).unwrap();

    // ---- graph: S+ and S ratios, pooled and per prime, for N/P and NU/U ----
    let (w, h) = (900.0f64, 500.0f64);
    let mut svg = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
    write!(svg, "<text x='{}' y='22' text-anchor='middle' font-size='15'>Sieve route, additions cut: paired ratio new / old (below 1: cheaper)</text>", w / 2.0).unwrap();
    let (l0, r0, t0, b0) = (60.0f64, w - 20.0, 50.0f64, h - 80.0);
    let (ymin, ymax) = (0.4f64, 1.2f64);
    let ys = |v: f64| b0 - (v.clamp(ymin, ymax) - ymin) / (ymax - ymin) * (b0 - t0);
    let mut k = ymin;
    while k <= ymax + 1e-9 {
        write!(svg, "<line x1='{l0}' y1='{}' x2='{r0}' y2='{}' stroke='{}'/><text x='{}' y='{}' text-anchor='end'>{k:.1}</text>", ys(k), ys(k), if (k - 1.0).abs() < 1e-9 { "#444" } else { "#ddd" }, l0 - 6.0, ys(k) + 4.0).unwrap();
        k += 0.1;
    }
    let cols = ["#2563eb", "#d97706"];
    let groups = ["p = 1009", "p = 1511", "p = 1823", "pooled"];
    let gw = (r0 - l0) / 4.0;
    let use_c: Vec<&Cmp> = ["N/P", "NU/U"].iter().filter_map(|l| cmps.iter().find(|c| c.label == *l)).collect();
    for (gi, gname) in groups.iter().enumerate() {
        write!(svg, "<text x='{}' y='{}' text-anchor='middle'>{gname}</text>", l0 + gw * (gi as f64 + 0.5), b0 + 18.0).unwrap();
        for (ai, cm) in use_c.iter().enumerate() {
            for (si, series) in [&cm.s, &cm.sp].iter().enumerate() {
                let strata: Vec<Vec<f64>> = if gi < 3 { vec![series[gi].clone()] } else { series.to_vec() };
                let (g, lo, hi) = boot(&strata, &mut rng);
                let x = l0 + gw * gi as f64 + 25.0 + (ai as f64 * 2.0 + si as f64) * (gw - 50.0) / 4.0;
                write!(svg, "<line x1='{x}' y1='{}' x2='{x}' y2='{}' stroke='{}' stroke-width='2'/>", ys(lo), ys(hi), cols[ai]).unwrap();
                if si == 0 {
                    write!(svg, "<circle cx='{x}' cy='{}' r='5' fill='{}'/>", ys(g), cols[ai]).unwrap();
                } else {
                    write!(svg, "<rect x='{}' y='{}' width='9' height='9' fill='white' stroke='{}' stroke-width='2'/>", x - 4.5, ys(g) - 4.5, cols[ai]).unwrap();
                }
            }
        }
    }
    for (ai, cm) in use_c.iter().enumerate() {
        write!(svg, "<text x='{}' y='{}' fill='{}'>{} (filled: S, hollow: S+)</text>", l0 + 8.0 + ai as f64 * 260.0, h - 36.0, cols[ai], cm.label).unwrap();
    }
    write!(svg, "<text x='{l0}' y='{}' fill='#555' font-size='11'>Points: geometric mean over seeds 17-32; bars: 95% bootstrap over seeds. All additions, lookups and reductions executed are charged.</text></svg>", h - 12.0).unwrap();
    std::fs::write(format!("{out}/ratios.svg"), svg).unwrap();
}
