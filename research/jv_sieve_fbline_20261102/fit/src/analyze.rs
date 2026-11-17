//! Paired analysis of the sieve-route line search (PROTOCOL.md).  Rust only.
use crate::rule::{GAMMA, PRIMES};
use serde_json::Value;
use std::fmt::Write as _;

const NB: usize = 20000;
const GAE_BAND: [f64; 3] = [0.0, 0.1, 1.0];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn idx(&mut self, n: usize) -> usize { (((self.next() as u128) * (n as u128)) >> 64) as usize }
}
fn load(path: &str) -> Vec<Value> {
    let s = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("missing {path}"));
    serde_json::from_str::<Value>(&s).unwrap().as_array().unwrap().clone()
}
fn exists(path: &str) -> bool { std::path::Path::new(path).exists() }
fn u(v: &Value, k: &str) -> u64 { v[k].as_u64().unwrap_or_else(|| panic!("missing {k}")) }
fn f(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap_or_else(|| panic!("missing {k}")) }
fn gmean(x: &[f64]) -> f64 { (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp() }
fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn sqrt_l(v: &Value) -> f64 { f(v, "c_add_e") * f(v, "l").sqrt() }
fn s_of(v: &Value) -> f64 { u(v, "total_muls") as f64 / sqrt_l(v) }
fn sp_of(v: &Value) -> f64 { u(v, "total_plus") as f64 / sqrt_l(v) }
fn search(v: &Value, k: &str) -> u64 { v.get("fb_line").and_then(|x| x.get(k)).and_then(|x| x.as_u64()).unwrap_or(0) }
/// Native operations the record emits (route + search); F4/descent additions are not emitted.
fn natives(v: &Value) -> f64 {
    (u(v, "sieve_adds") + u(v, "sieve_lookups") + u(v, "enum_adds") + search(v, "adds") + search(v, "lookups")) as f64
}
fn cost_w(v: &Value, w: f64) -> f64 { u(v, "total_muls") as f64 + w * f(v, "c_add_e") * natives(v) }
fn strip(v: &Value) -> Value {
    let mut c = v.clone();
    c.as_object_mut().unwrap().remove("wall_ms");
    c
}
fn ok(v: &Value) -> bool { v["solved"].as_bool().unwrap() && v["correct"].as_bool().unwrap() && u(v, "rels_failed_verify") == 0 && !v["exhausted"].as_bool().unwrap() }

/// Geometric mean of paired ratios with a stratified bootstrap over seeds (strata = primes).
fn boot(strata: &[Vec<f64>], rng: &mut Rng) -> (f64, f64, f64) {
    let all: Vec<f64> = strata.iter().flatten().copied().collect();
    let g = gmean(&all);
    let mut bs = Vec::with_capacity(NB);
    for _ in 0..NB {
        let mut v = Vec::with_capacity(all.len());
        for s in strata { for _ in 0..s.len() { v.push(s[rng.idx(s.len())]); } }
        bs.push(gmean(&v));
    }
    (g, pct(&mut bs, 0.025), pct(&mut bs, 0.975))
}

pub fn run(v2: &str, res: &str, out: &str) {
    let mut t = String::new();
    macro_rules! say { ($($x:tt)*) => {{ let s = format!($($x)*); println!("{s}"); t += &s; t += "\n"; }} }
    let mut rng = Rng(0x51EF_B11E_2026_1102);
    let mut notes: Vec<String> = vec![];

    // ---- C1: P reproduces V2's sparse records bit-for-bit ----
    let mut c1_total = 0;
    let mut c1_eq = 0;
    let mut c1_key = 0;
    let mut pdata: Vec<Vec<Value>> = vec![];
    for &p in &PRIMES {
        let a = load(&format!("{v2}/ic_{p}_sparse.json"));
        let b = load(&format!("{res}/P_{p}.json"));
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            c1_total += 1;
            if strip(x) == strip(y) { c1_eq += 1; }
            if ["total_muls", "total_plus", "relations", "descent_digest", "descent_muls", "hits", "lines", "base"].iter().all(|k| x[k] == y[k]) { c1_key += 1; }
        }
        pdata.push(b);
    }
    say!("C1 arm P vs V2 sparse records: {c1_key}/{c1_total} equal on total_muls, total_plus, relations, descent_digest, descent_muls, hits, lines, base; {c1_eq}/{c1_total} equal on every field but wall_ms: {}", if c1_key == c1_total && c1_eq == c1_total { "PASS" } else { "FAIL" });
    if c1_eq != c1_total { notes.push("C1".into()); }

    // correctness of every run of every arm
    let arms: Vec<(&str, String)> = [("H", "H"), ("T", "T"), ("U", "U")].iter().filter(|(a, _)| exists(&format!("{res}/{a}_1009.json"))).map(|(a, b)| (*a, b.to_string())).collect();
    let mut all_ok = true;
    let mut nruns = 0;
    for &p in &PRIMES {
        for r in &pdata[PRIMES.iter().position(|&x| x == p).unwrap()] { nruns += 1; all_ok &= ok(r); }
    }

    // ---- per-arm paired tables ----
    struct ArmOut { name: String, s_r: Vec<Vec<f64>>, sp_r: Vec<Vec<f64>>, band: Vec<Vec<Vec<f64>>>, route_r: Vec<Vec<f64>>, bratio: Vec<Vec<f64>> }
    let mut outs: Vec<ArmOut> = vec![];
    for (arm, _) in &arms {
        let mut ao = ArmOut { name: arm.to_string(), s_r: vec![], sp_r: vec![], band: vec![vec![vec![]; 3]; 3], route_r: vec![], bratio: vec![] };
        say!("\n### Arm {arm} against P (paired by seed)");
        writeln!(t, "| p | n lines | B plain->best (mean) | S_P | S_arm | S ratio | S+_P | S+_arm | S+ ratio | route-only ratio | (B_p/B_b)^5 | search share of total_muls | search native / total_plus |").unwrap();
        writeln!(t, "|--|--|--|--|--|--|--|--|--|--|--|--|--|").unwrap();
        for (pi, &p) in PRIMES.iter().enumerate() {
            let b = load(&format!("{res}/{arm}_{p}.json"));
            assert_eq!(b.len(), pdata[pi].len());
            let (mut sr, mut spr, mut rr, mut br) = (vec![], vec![], vec![], vec![]);
            for (x, y) in pdata[pi].iter().zip(&b) {
                assert_eq!(u(x, "seed"), u(y, "seed"));
                all_ok &= ok(y);
                nruns += 1;
                assert_eq!(u(&y["fb_line"], "b_plain"), u(x, "base"), "b_plain = P's base");
                assert_eq!(u(&y["fb_line"], "b_best"), u(y, "base"), "b_best = arm's base");
                sr.push(s_of(y) / s_of(x));
                spr.push(sp_of(y) / sp_of(x));
                rr.push((u(y, "total_muls") - search(y, "muls")) as f64 / u(x, "total_muls") as f64);
                br.push((u(&y["fb_line"], "b_plain") as f64 / u(&y["fb_line"], "b_best") as f64).powf(GAMMA));
                for (wi, &w) in GAE_BAND.iter().enumerate() {
                    ao.band[pi][wi].push(cost_w(y, w) / cost_w(x, w));
                }
            }
            let mean = |k: &str, v: &[Value]| v.iter().map(|r| f(r, k)).sum::<f64>() / v.len() as f64;
            let share = b.iter().map(|y| search(y, "muls") as f64 / u(y, "total_muls") as f64).sum::<f64>() / b.len() as f64;
            let nat = b.iter().map(|y| (search(y, "adds") + search(y, "lookups")) as f64 / u(y, "total_plus") as f64).sum::<f64>() / b.len() as f64;
            let fbm = |k: &str| b.iter().map(|y| f(&y["fb_line"], k)).sum::<f64>() / b.len() as f64;
            writeln!(t, "| {p} | {} | {:.1} -> {:.1} | {:.5} | {:.5} | **{:.3}** | {:.5} | {:.5} | **{:.3}** | {:.3} | {:.3} | {:.2}% | {:.2}% |",
                b[0]["fb_line"]["candidates"], fbm("b_plain"), fbm("b_best"),
                pdata[pi].iter().map(s_of).sum::<f64>() / 16.0, b.iter().map(s_of).sum::<f64>() / 16.0, gmean(&sr),
                pdata[pi].iter().map(sp_of).sum::<f64>() / 16.0, b.iter().map(sp_of).sum::<f64>() / 16.0, gmean(&spr),
                gmean(&rr), gmean(&br), share * 100.0, nat * 100.0).unwrap();
            let _ = mean;
            ao.s_r.push(sr); ao.sp_r.push(spr); ao.route_r.push(rr); ao.bratio.push(br);
        }
        let _ = writeln!(t);
        say!("Paired geometric-mean ratio arm/P, 95% bootstrap over seeds ({NB} resamples, strata = primes):");
        for (label, series) in [("S  (muls, search charged)", &ao.s_r), ("S+ (muls + emitted adds/lookups, search incl.)", &ao.sp_r)] {
            let mut line = format!("  {label}:");
            for (pi, &p) in PRIMES.iter().enumerate() {
                let (g, lo, hi) = boot(&[series[pi].clone()], &mut rng);
                write!(line, " p={p} {g:.4} [{lo:.4}, {hi:.4}];").unwrap();
            }
            let (g, lo, hi) = boot(series, &mut rng);
            write!(line, " pooled {g:.4} [{lo:.4}, {hi:.4}]").unwrap();
            say!("{line}");
        }
        let (g, _lo, hi) = boot(&ao.s_r, &mut rng);
        say!("  H1 (arm {arm}: upper end of pooled S ratio < 1): {} (upper {hi:.4}, point {g:.4})", if hi < 1.0 { "SUPPORTED" } else { "NOT SUPPORTED" });
        say!("  §5a band, arm/P (native = emitted adds+lookups of route and search at w GAE each; descent/F4 adds are unemitted on both arms), pooled [per prime]:");
        for (wi, &w) in GAE_BAND.iter().enumerate() {
            let strata: Vec<Vec<f64>> = (0..3).map(|pi| ao.band[pi][wi].clone()).collect();
            let (g, lo, hi) = boot(&strata, &mut rng);
            let per: Vec<String> = (0..3).map(|pi| { let (g, lo, hi) = boot(&[strata[pi].clone()], &mut rng); format!("{g:.3} [{lo:.3},{hi:.3}]") }).collect();
            say!("    w = {w}: pooled {g:.4} [{lo:.4}, {hi:.4}]  per prime {}", per.join("  "));
        }
        // B model check
        let route_all: Vec<f64> = ao.route_r.iter().flatten().copied().collect();
        let b_all: Vec<f64> = ao.bratio.iter().flatten().copied().collect();
        let dev: Vec<f64> = route_all.iter().zip(&b_all).map(|(r, b)| r / b).collect();
        // regression through origin: ln(route) = gamma_hat * ln(B_p/B_b)
        let xs: Vec<f64> = b_all.iter().map(|b| b.ln() / GAMMA).collect(); // ln(Bp/Bb)
        let ys: Vec<f64> = route_all.iter().map(|r| r.ln()).collect();
        let gam = |idx: &[usize]| { let sxy: f64 = idx.iter().map(|&i| xs[i] * ys[i]).sum(); let sxx: f64 = idx.iter().map(|&i| xs[i] * xs[i]).sum(); sxy / sxx };
        let all_idx: Vec<usize> = (0..xs.len()).collect();
        let g_hat = gam(&all_idx);
        let mut gb = vec![];
        for _ in 0..NB { let idx: Vec<usize> = (0..xs.len()).map(|_| rng.idx(xs.len())).collect(); gb.push(gam(&idx)); }
        let (glo, ghi) = (pct(&mut gb, 0.025), pct(&mut gb, 0.975));
        let sd = { let m = dev.iter().map(|x| x.ln()).sum::<f64>() / dev.len() as f64; (dev.iter().map(|x| (x.ln() - m).powi(2)).sum::<f64>() / (dev.len() - 1) as f64).sqrt() };
        say!("  B model check (route-only ratio = (total_muls - search muls)/P total_muls vs (B_plain/B_best)^{GAMMA}): geo-mean measured/model {:.4}, sd of log {:.4}; fitted gamma (through origin) {g_hat:.3} [{glo:.3}, {ghi:.3}] vs {GAMMA}", gmean(&dev), sd);
        outs.push(ao);
    }

    // ---- phase shares ----
    say!("\n### Phase shares (share of the arm's own total_muls | S+ numerator in brackets), mean over seeds, pooled over primes by prime");
    writeln!(t, "| arm | p | enumeration | sieve | descent | linear algebra | search | other (setup, base, extract, verify, table) |").unwrap();
    writeln!(t, "|--|--|--|--|--|--|--|--|").unwrap();
    let mut all_arms: Vec<(String, Vec<Vec<Value>>)> = vec![("P".into(), pdata.clone())];
    for (arm, _) in &arms { all_arms.push((arm.to_string(), PRIMES.iter().map(|p| load(&format!("{res}/{arm}_{p}.json"))).collect())); }
    for (name, d) in &all_arms {
        for (pi, &p) in PRIMES.iter().enumerate() {
            let m = |g: &dyn Fn(&Value) -> f64| d[pi].iter().map(|r| g(r) / u(r, "total_muls") as f64).sum::<f64>() / d[pi].len() as f64;
            let en = m(&|r| u(r, "enum_muls") as f64);
            let si = m(&|r| u(r, "sieve_muls") as f64);
            let de = m(&|r| u(r, "descent_muls") as f64);
            let la = m(&|r| u(r, "la_muls") as f64);
            let se = m(&|r| search(r, "muls") as f64);
            writeln!(t, "| {name} | {p} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.2}% | {:.1}% |", en * 100.0, si * 100.0, de * 100.0, la * 100.0, se * 100.0, (1.0 - en - si - de - la - se) * 100.0).unwrap();
        }
    }
    writeln!(t, "\nS+ composition (share of total_plus): enumeration adds, sieve adds+lookups, search adds+lookups, muls").unwrap();
    writeln!(t, "| arm | p | muls | enum adds | sieve adds+lookups | search adds+lookups |\n|--|--|--|--|--|--|").unwrap();
    for (name, d) in &all_arms {
        for (pi, &p) in PRIMES.iter().enumerate() {
            let m = |g: &dyn Fn(&Value) -> f64| d[pi].iter().map(|r| g(r) / u(r, "total_plus") as f64).sum::<f64>() / d[pi].len() as f64;
            writeln!(t, "| {name} | {p} | {:.1}% | {:.1}% | {:.1}% | {:.2}% |", m(&|r| u(r, "total_muls") as f64) * 100.0, m(&|r| u(r, "enum_adds") as f64) * 100.0, m(&|r| (u(r, "sieve_adds") + u(r, "sieve_lookups")) as f64) * 100.0, m(&|r| (search(r, "adds") + search(r, "lookups")) as f64) * 100.0).unwrap();
        }
    }
    say!("\nCorrectness: every run solved, correct, 0 failed relation verifications, not exhausted, over {nruns} runs: {}", if all_ok { "PASS" } else { "FAIL" });

    // ---- C2 / C3 controls ----
    for (label, pat) in [("C2h (n = 1, Horner)", "c2h"), ("C2t (n = 1, table)", "c2t")] {
        if !exists(&format!("{res}/{pat}_1009.json")) { continue; }
        let mut pass = true;
        let mut line = String::new();
        for (pi, &p) in PRIMES.iter().enumerate() {
            let c = load(&format!("{res}/{pat}_{p}.json"));
            let x = &pdata[pi][0];
            let y = &c[0];
            let same = ["relations", "descent_digest", "descent_residuals", "base", "hits", "lines"].iter().all(|k| x[k] == y[k]);
            let dm = u(y, "total_muls") as i64 - u(x, "total_muls") as i64;
            let dp = u(y, "total_plus") as i64 - u(x, "total_plus") as i64;
            let sm = search(y, "muls") as i64;
            let sn = (search(y, "adds") + search(y, "lookups")) as i64;
            let good = same && dm == sm && dp == sm + sn && ok(y) && u(&y["fb_line"], "best_index") == 0;
            pass &= good;
            write!(line, " p={p}: route fields equal {same}, d(total_muls) {dm} vs search muls {sm}, d(total_plus) {dp} vs search muls+native {};", sm + sn).unwrap();
        }
        say!("{label} on seed {} per prime: {}  [{line}]", u(&pdata[0][0], "seed"), if pass { "PASS" } else { "FAIL" });
    }
    if exists(&format!("{res}/c3_1009.json")) && arms.iter().any(|a| a.0 == "H") {
        let mut pass = true;
        let mut line = String::new();
        for (_pi, &p) in PRIMES.iter().enumerate() {
            let c = load(&format!("{res}/c3_{p}.json"));
            let h = load(&format!("{res}/H_{p}.json"));
            let (x, y) = (&h[0], &c[0]);
            let same = ["b_plain", "b_best", "best_index"].iter().all(|k| x["fb_line"][k] == y["fb_line"][k]) && ["relations", "descent_digest", "total_muls"].iter().all(|k| k == &"total_muls" || x[k] == y[k]);
            let dm = u(y, "total_muls") as i64 - search(y, "muls") as i64 - (u(x, "total_muls") as i64 - search(x, "muls") as i64);
            pass &= same && dm == 0 && ok(y);
            write!(line, " p={p}: same line (B {} -> {}, index {}), route muls equal {}, tbl search muls {} vs horner {};", y["fb_line"]["b_plain"], y["fb_line"]["b_best"], y["fb_line"]["best_index"], dm == 0, search(y, "muls"), search(x, "muls")).unwrap();
        }
        say!("C3 table at n = n_H chooses the same line as Horner (equal B, index, route records): {}  [{line}]", if pass { "PASS" } else { "FAIL" });
    }

    say!("\nNotes/failures: {:?}", notes);
    std::fs::create_dir_all(out).unwrap();
    std::fs::write(format!("{out}/analysis.txt"), &t).unwrap();

    // ---- graph ----
    let (w, h) = (900.0f64, 520.0f64);
    let mut svg = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='100%' height='100%' fill='white'/>");
    write!(svg, "<text x='{}' y='22' text-anchor='middle' font-size='15'>Factor-base line search on the JV sieve route: paired ratio arm / plain (below 1: cheaper)</text>", w / 2.0).unwrap();
    // panel A: pooled and per-prime ratio with CI for S and S+, per arm
    let (l0, r0, t0, b0) = (60.0f64, w - 20.0, 50.0f64, h - 80.0);
    let (ymin, ymax) = (0.4f64, 1.6f64);
    let ys = |v: f64| b0 - (v.clamp(ymin, ymax) - ymin) / (ymax - ymin) * (b0 - t0);
    let mut k = ymin;
    while k <= ymax + 1e-9 { write!(svg, "<line x1='{l0}' y1='{}' x2='{r0}' y2='{}' stroke='{}'/><text x='{}' y='{}' text-anchor='end'>{k:.1}</text>", ys(k), ys(k), if (k - 1.0).abs() < 1e-9 { "#444" } else { "#ddd" }, l0 - 6.0, ys(k) + 4.0).unwrap(); k += 0.2; }
    let cols = ["#2563eb", "#d97706", "#059669", "#7c3aed"];
    let groups = ["p = 1009", "p = 1511", "p = 1823", "pooled"];
    let gw = (r0 - l0) / 4.0;
    for (gi, gname) in groups.iter().enumerate() {
        write!(svg, "<text x='{}' y='{}' text-anchor='middle'>{gname}</text>", l0 + gw * (gi as f64 + 0.5), b0 + 18.0).unwrap();
        for (ai, ao) in outs.iter().enumerate() {
            for (si, series) in [&ao.s_r, &ao.sp_r].iter().enumerate() {
                let strata: Vec<Vec<f64>> = if gi < 3 { vec![series[gi].clone()] } else { series.to_vec() };
                let (g, lo, hi) = boot(&strata, &mut rng);
                let x = l0 + gw * gi as f64 + 20.0 + (ai as f64 * 2.0 + si as f64) * (gw - 40.0) / (outs.len() as f64 * 2.0);
                write!(svg, "<line x1='{x}' y1='{}' x2='{x}' y2='{}' stroke='{}' stroke-width='2'/>", ys(lo), ys(hi), cols[ai]).unwrap();
                if si == 0 { write!(svg, "<circle cx='{x}' cy='{}' r='5' fill='{}'/>", ys(g), cols[ai]).unwrap(); }
                else { write!(svg, "<rect x='{}' y='{}' width='9' height='9' fill='white' stroke='{}' stroke-width='2'/>", x - 4.5, ys(g) - 4.5, cols[ai]).unwrap(); }
            }
        }
    }
    for (ai, ao) in outs.iter().enumerate() {
        write!(svg, "<text x='{}' y='{}' fill='{}'>arm {} (filled: S, hollow: S+)</text>", l0 + 8.0 + ai as f64 * 220.0, h - 36.0, cols[ai], ao.name).unwrap();
    }
    write!(svg, "<text x='{l0}' y='{}' fill='#555' font-size='11'>Points: geometric mean over seeds 17-32; bars: 95% bootstrap over seeds. Search multiplications, additions and lookups are charged; y clipped to [0.4, 1.6].</text></svg>", h - 12.0).unwrap();
    std::fs::write(format!("{out}/ratios.svg"), svg).unwrap();
}
