//! Red-team analysis of two merged studies (main aa1c89ab2a):
//!   A = research/jv_sieve_fbline_20261102 (factor-base line search on the JV sieve route)
//!   B = research/jv_sieve_negrho_20261102 (matched negation rho at p = 1511)
//! Reads committed records only; writes nothing but stdout.  Rust only (AGENTS.md "no Python").
//! Usage: rt_sieve_review <repo root>
use serde_json::Value;
use std::fmt::Write as _;

const PR: [u64; 3] = [1009, 1511, 1823];
const ARMS: [&str; 3] = ["H", "T", "U"];
const NB: usize = 5000;
const GAE: f64 = 331.0;

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
fn g(v: &Value, k: &str) -> f64 { v[k].as_f64().unwrap_or_else(|| panic!("missing {k}")) }
fn fb(v: &Value, k: &str) -> f64 { v.get("fb_line").and_then(|x| x.get(k)).and_then(|x| x.as_f64()).unwrap_or(0.0) }
fn mean(x: &[f64]) -> f64 { x.iter().sum::<f64>() / x.len() as f64 }
fn sd(x: &[f64]) -> f64 { let m = mean(x); (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt() }
fn gmean(x: &[f64]) -> f64 { (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp() }
fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[((v.len() - 1) as f64 * q).round() as usize]
}
fn ci(v: &mut Vec<f64>) -> (f64, f64) { (pct(v, 0.025), pct(v, 0.975)) }
fn sqrt_l(v: &Value) -> f64 { g(v, "c_add_e") * g(v, "l").sqrt() }

// ---------- per-record quantities ----------
type Q = (&'static str, fn(&Value) -> f64);
fn route_muls(v: &Value) -> f64 { g(v, "total_muls") - fb(v, "muls") }
fn route_plus(v: &Value) -> f64 { g(v, "total_plus") - fb(v, "muls") - fb(v, "adds") - fb(v, "lookups") }
fn quantities() -> Vec<Q> {
    vec![
        ("total_muls (S, search charged)", |v| g(v, "total_muls")),
        ("route_muls (S, search removed)", route_muls),
        ("route_plus (S+, search removed)", route_plus),
        ("relations", |v| g(v, "relations")),
        ("lines (sieve families)", |v| g(v, "lines")),
        ("rels_per_line", |v| g(v, "rels_per_line")),
        ("sieve_steps per family", |v| g(v, "sieve_steps") / g(v, "lines")),
        ("enum_muls", |v| g(v, "enum_muls")),
        ("enum_adds", |v| g(v, "enum_adds")),
        ("sieve_muls", |v| g(v, "sieve_muls")),
        ("sieve_adds+lookups", |v| g(v, "sieve_adds") + g(v, "sieve_lookups")),
        ("relation phase muls (enum+sieve)", |v| g(v, "enum_muls") + g(v, "sieve_muls")),
        ("descent tests (residuals)", |v| g(v, "descent_residuals")),
        ("descent cost per test (c_cov)", |v| g(v, "descent_c_cov")),
        ("descent_muls", |v| g(v, "descent_muls")),
        ("la_muls", |v| g(v, "la_muls")),
        ("route muls without descent", |v| route_muls(v) - g(v, "descent_muls")),
    ]
}

struct Data { p: Vec<Vec<Value>>, arms: Vec<Vec<Vec<Value>>> } // p[pi][i], arms[ai][pi][i]

fn load_a(root: &str) -> Data {
    let res = format!("{root}/research/jv_sieve_fbline_20261102/results");
    let p = PR.iter().map(|p| load(&format!("{res}/P_{p}.json"))).collect();
    let arms = ARMS.iter().map(|a| PR.iter().map(|p| load(&format!("{res}/{a}_{p}.json"))).collect()).collect();
    Data { p, arms }
}

/// slope through origin and OLS (slope, intercept)
fn slope0(x: &[f64], y: &[f64]) -> f64 { x.iter().zip(y).map(|(a, b)| a * b).sum::<f64>() / x.iter().map(|a| a * a).sum::<f64>() }
fn ols(x: &[f64], y: &[f64]) -> (f64, f64) {
    let (mx, my) = (mean(x), mean(y));
    let b = x.iter().zip(y).map(|(a, c)| (a - mx) * (c - my)).sum::<f64>() / x.iter().map(|a| (a - mx).powi(2)).sum::<f64>();
    (b, my - b * mx)
}

fn study_a(root: &str, o: &mut String) {
    let d = load_a(root);
    let mut rng = Rng(0x0BAD_5EED_2026_1103);
    let qs = quantities();
    let n = 16usize;
    let b = |v: &Value| g(v, "base");

    // ---------------- A1: elasticities, move (within instance) vs natural (cross-section of P) ----------------
    writeln!(o, "## A1. Elasticity d ln Q / d ln B: line move (within instance, arms H,T,U vs P; 144 pairs) vs natural variation (cross-section of P, prime-demeaned; 48 instances)").unwrap();
    writeln!(o, "Bootstrap {NB} resamples of instances within primes (an instance carries its 3 move pairs). 'move, with intercept' tests for an effect of moving beyond B (intercept delta = ln ratio at equal B).").unwrap();
    writeln!(o, "| quantity | move e (through 0) [95%] | move e, OLS slope | move intercept delta [95%] | natural e (P cross-section) [95%] | move - natural [95%] |").unwrap();
    writeln!(o, "|--|--|--|--|--|--|").unwrap();
    let mut move_e: Vec<f64> = vec![];
    for (qn, qf) in &qs {
        let calc = |idx: &[Vec<usize>]| -> (f64, f64, f64, f64) {
            let (mut x, mut y) = (vec![], vec![]);
            let (mut cx, mut cy) = (vec![], vec![]);
            for pi in 0..3 {
                let (mut lx, mut ly) = (vec![], vec![]);
                for &i in &idx[pi] {
                    let pv = &d.p[pi][i];
                    for ai in 0..3 {
                        let av = &d.arms[ai][pi][i];
                        x.push((b(av) / b(pv)).ln());
                        y.push((qf(av) / qf(pv)).ln());
                    }
                    lx.push(b(pv).ln());
                    ly.push(qf(pv).ln());
                }
                let (mx, my) = (mean(&lx), mean(&ly));
                cx.extend(lx.iter().map(|v| v - mx));
                cy.extend(ly.iter().map(|v| v - my));
            }
            let e0 = slope0(&x, &y);
            let (e1, d1) = ols(&x, &y);
            let en = slope0(&cx, &cy);
            (e0, e1, d1, en)
        };
        let full: Vec<Vec<usize>> = (0..3).map(|_| (0..n).collect()).collect();
        let (e0, e1, d1, en) = calc(&full);
        let (mut b0, mut bd, mut bn, mut bdiff) = (vec![], vec![], vec![], vec![]);
        for _ in 0..NB {
            let idx: Vec<Vec<usize>> = (0..3).map(|_| (0..n).map(|_| rng.idx(n)).collect()).collect();
            let (a0, _a1, ad, an) = calc(&idx);
            b0.push(a0); bd.push(ad); bn.push(an); bdiff.push(a0 - an);
        }
        let (l0, h0) = ci(&mut b0); let (ld, hd) = ci(&mut bd); let (ln_, hn) = ci(&mut bn); let (lf, hf) = ci(&mut bdiff);
        writeln!(o, "| {qn} | {e0:.2} [{l0:.2}, {h0:.2}] | {e1:.2} | {d1:+.3} [{ld:+.3}, {hd:+.3}] | {en:.2} [{ln_:.2}, {hn:.2}] | {:+.2} [{lf:+.2}, {hf:+.2}] |", e0 - en).unwrap();
        move_e.push(e0);
    }
    let qi = |name: &str| qs.iter().position(|q| q.0.starts_with(name)).unwrap();

    // ---------------- A2: per-prime decay; gamma_eff per prime; phase-mixture model ----------------
    writeln!(o, "\n## A2. Decay with p: gamma_eff = mean ln(route ratio) / mean ln(B_P/B_arm) per prime; phase-mixture prediction").unwrap();
    writeln!(o, "Phase model (p-independent): route_arm/route_P = sum_ph share_ph,P * (B_arm/B_P)^e_ph, e_ph = the pooled move elasticities of A1 (enum, sieve, descent, LA; 'other' fixed). In-sample consistency check, not a prediction.").unwrap();
    writeln!(o, "| arm | p | mean B_arm/B_P | S ratio (search charged) | route-only ratio | (B_P/B_arm)^5 (rule) | phase-mixture model | gamma_eff [95%] | model gamma_eff |").unwrap();
    writeln!(o, "|--|--|--|--|--|--|--|--|--|").unwrap();
    let ph = [("enum_muls", qi("enum_muls")), ("sieve_muls", qi("sieve_muls")), ("descent_muls", qi("descent_muls")), ("la_muls", qi("la_muls"))];
    let mut gam_by: Vec<Vec<Vec<f64>>> = vec![vec![vec![]; 3]; 3];
    for ai in 0..3 {
        for pi in 0..3 {
            let calc = |idx: &[usize]| -> (f64, f64, f64, f64, f64, f64) {
                let (mut lr, mut lb, mut ls, mut lm, mut l5) = (vec![], vec![], vec![], vec![], vec![]);
                for &i in idx {
                    let (pv, av) = (&d.p[pi][i], &d.arms[ai][pi][i]);
                    let br = b(av) / b(pv);
                    lr.push((route_muls(av) / route_muls(pv)).ln());
                    ls.push((g(av, "total_muls") / g(pv, "total_muls")).ln());
                    lb.push(br.ln());
                    l5.push(-5.0 * br.ln());
                    let rp = route_muls(pv);
                    let mut pred = 0.0;
                    let mut used = 0.0;
                    for (k, (nm, q)) in ph.iter().enumerate() {
                        let share = g(pv, nm) / rp;
                        used += share;
                        pred += share * br.powf(move_e[*q]);
                        let _ = k;
                    }
                    pred += 1.0 - used;
                    lm.push(pred.ln());
                }
                let mb = mean(&lb);
                (mean(&lb).exp(), mean(&ls).exp(), mean(&lr).exp(), mean(&l5).exp(), mean(&lm).exp(), -mean(&lr) / mb.abs())
            };
            let full: Vec<usize> = (0..n).collect();
            let (mb, sr, rr, r5, rm, ge) = calc(&full);
            let mut bg = vec![];
            for _ in 0..NB { let idx: Vec<usize> = (0..n).map(|_| rng.idx(n)).collect(); bg.push(calc(&idx).5); }
            gam_by[ai][pi] = bg.clone();
            let (lo, hi) = ci(&mut bg);
            let gm_model = -rm.ln() / mb.ln().abs();
            writeln!(o, "| {} | {} | {mb:.4} | {sr:.3} | {rr:.3} | {r5:.3} | {rm:.3} | {ge:.2} [{lo:.2}, {hi:.2}] | {gm_model:.2} |", ARMS[ai], PR[pi]).unwrap();
        }
    }
    for ai in 0..3 {
        let mut dif: Vec<f64> = gam_by[ai][0].iter().zip(&gam_by[ai][2]).map(|(a, c)| a - c).collect();
        let pt = mean(&dif);
        let (lo, hi) = ci(&mut dif);
        writeln!(o, "gamma_eff(1009) - gamma_eff(1823), arm {}: mean of bootstrap {pt:.2} [{lo:.2}, {hi:.2}] (independent resamples per prime)", ARMS[ai]).unwrap();
    }
    // slope of ln(S ratio) on ln p, arm H, and the rule model's slope
    {
        let ai = 0;
        let lp: Vec<f64> = PR.iter().map(|&p| (p as f64).ln()).collect();
        let calc = |idx: &[Vec<usize>]| -> f64 {
            let ys: Vec<f64> = (0..3).map(|pi| mean(&idx[pi].iter().map(|&i| (g(&d.arms[ai][pi][i], "total_muls") / g(&d.p[pi][i], "total_muls")).ln()).collect::<Vec<_>>())).collect();
            ols(&lp, &ys).0
        };
        let full: Vec<Vec<usize>> = (0..3).map(|_| (0..n).collect()).collect();
        let s0 = calc(&full);
        let mut bs = vec![];
        for _ in 0..NB { let idx: Vec<Vec<usize>> = (0..3).map(|_| (0..n).map(|_| rng.idx(n)).collect()).collect(); bs.push(calc(&idx)); }
        let (lo, hi) = ci(&mut bs);
        let rule = [0.6027f64, 0.6702, 0.6968];
        let sm = ols(&lp, &rule.iter().map(|v| v.ln()).collect::<Vec<_>>()).0;
        writeln!(o, "Arm H: slope of ln(S ratio) on ln p = {s0:.3} [{lo:.3}, {hi:.3}]; the frozen rule's model (rule.txt J at n=4096, w=0) has slope {sm:.3}.").unwrap();
    }

    // ---------------- A3: noise and seeds ----------------
    writeln!(o, "\n## A3. Run-to-run noise of the paired log ratio (sd over 16 seeds) and seeds needed for a +-5% per-prime interval (n = (1.96 sd / ln 1.05)^2)").unwrap();
    writeln!(o, "| arm | p | sd ln S ratio | sd ln descent_muls ratio | sd ln (route without descent) ratio | seeds for +-5% on S | seeds for +-5% on route w/o descent |").unwrap();
    writeln!(o, "|--|--|--|--|--|--|--|").unwrap();
    for ai in 0..3 {
        for pi in 0..3 {
            let r = |f: &dyn Fn(&Value) -> f64| -> Vec<f64> { (0..n).map(|i| (f(&d.arms[ai][pi][i]) / f(&d.p[pi][i])).ln()).collect() };
            let s1 = sd(&r(&|v| g(v, "total_muls")));
            let s2 = sd(&r(&|v| g(v, "descent_muls")));
            let s3 = sd(&r(&|v| route_muls(v) - g(v, "descent_muls")));
            let k = |s: f64| (1.96 * s / 1.05f64.ln()).powi(2).ceil();
            writeln!(o, "| {} | {} | {s1:.3} | {s2:.3} | {s3:.3} | {} | {} |", ARMS[ai], PR[pi], k(s1), k(s3)).unwrap();
        }
    }

    // ---------------- A4: charging completeness: index arithmetic of table lookups; LA price ----------------
    writeln!(o, "\n## A4. Charging completeness (derived re-pricings of the committed records; geometric mean of arm/P over 16 seeds per prime, pooled 48, bootstrap 95%)").unwrap();
    writeln!(o, "walk lookups = fb_line.lookups - q in table mode (the chi-table build's q lookups index directly); each walk lookup executes `chi[(x0 * p + x1)]`, one integer multiply and one add, uncounted.").unwrap();
    let variants: Vec<(&str, Box<dyn Fn(&Value, u64) -> f64>)> = vec![
        ("S as reported", Box::new(|v: &Value, _p| g(v, "total_muls"))),
        ("S, index multiply priced as one F_p mult", Box::new(|v: &Value, p| g(v, "total_muls") + walk_lookups(v, p))),
        ("S+ as reported", Box::new(|v: &Value, _p| g(v, "total_plus"))),
        ("S+, index arithmetic as 2 native ops per walk lookup", Box::new(|v: &Value, p| g(v, "total_plus") + 2.0 * walk_lookups(v, p))),
        ("band w=0.1 as reported", Box::new(|v: &Value, _p| band(v, 0.1, 0.0))),
        ("band w=0.1, +2 index ops per walk lookup", Box::new(|v: &Value, p| band(v, 0.1, 2.0 * walk_lookups(v, p)))),
        ("band w=1, +2 index ops per walk lookup", Box::new(|v: &Value, p| band(v, 1.0, 2.0 * walk_lookups(v, p)))),
        ("S, mod-l LA op at 31 F_p mults (ic_acct1 bitop/v1) instead of 16", Box::new(|v: &Value, _p| g(v, "total_muls") + 15.0 * g(v, "la_ops"))),
        ("S+, LA op at 31", Box::new(|v: &Value, _p| g(v, "total_plus") + 15.0 * g(v, "la_ops"))),
        ("S+, unemitted descent+LA additions imputed (1 per descent mult, 1 per LA op)", Box::new(|v: &Value, _p| g(v, "total_plus") + g(v, "descent_muls") + g(v, "la_ops"))),
        ("bitop/v1 proxy (model): muls + 15.2 la_ops + 0.096 (all emitted adds + imputed descent adds) + 0.043 lookups", Box::new(|v: &Value, p| bitproxy(v, p))),
    ];
    writeln!(o, "| variant | arm | p=1009 | p=1511 | p=1823 | pooled [95%] |").unwrap();
    writeln!(o, "|--|--|--|--|--|--|").unwrap();
    for (vn, vf) in &variants {
        for ai in 0..3 {
            let strata: Vec<Vec<f64>> = (0..3).map(|pi| (0..n).map(|i| vf(&d.arms[ai][pi][i], PR[pi]) / vf(&d.p[pi][i], PR[pi])).collect()).collect();
            let per: Vec<String> = strata.iter().map(|s| format!("{:.3}", gmean(s))).collect();
            let all: Vec<f64> = strata.iter().flatten().copied().collect();
            let mut bs = vec![];
            for _ in 0..NB { let mut v = vec![]; for s in &strata { for _ in 0..n { v.push(s[rng.idx(n)]); } } bs.push(gmean(&v)); }
            let (lo, hi) = ci(&mut bs);
            writeln!(o, "| {vn} | {} | {} | {} | {} | {:.3} [{lo:.3}, {hi:.3}] |", ARMS[ai], per[0], per[1], per[2], gmean(&all)).unwrap();
        }
    }
    // paired U vs H (same instances): is "U is the arm to use" resolved?
    writeln!(o, "\nPaired U/H (same 48 instances), geometric mean and 95% bootstrap, per unit:").unwrap();
    for (vi, vn) in [(0usize, "S"), (2, "S+"), (4, "band w=0.1"), (10, "bitop proxy")] {
        let vf = &variants[vi].1;
        let strata: Vec<Vec<f64>> = (0..3).map(|pi| (0..n).map(|i| vf(&d.arms[2][pi][i], PR[pi]) / vf(&d.arms[0][pi][i], PR[pi])).collect()).collect();
        let all: Vec<f64> = strata.iter().flatten().copied().collect();
        let mut bs = vec![];
        for _ in 0..NB { let mut v = vec![]; for s in &strata { for _ in 0..n { v.push(s[rng.idx(n)]); } } bs.push(gmean(&v)); }
        let (lo, hi) = ci(&mut bs);
        writeln!(o, "  {vn} ({}): U/H pooled {:.3} [{lo:.3}, {hi:.3}]; per prime {:.3} / {:.3} / {:.3}", variants[vi].0, gmean(&all), gmean(&strata[0]), gmean(&strata[1]), gmean(&strata[2])).unwrap();
    }
    // bitop-proxy composition: where would a lever act in the like-for-like unit?
    writeln!(o, "\nbitop/v1 proxy composition (share of the proxy total, mean over seeds) and the ceiling of each lever (total / (total - component)):").unwrap();
    for (an, arm) in [("P", None), ("U", Some(2usize))] {
        for pi in 0..3 {
            let rows: &Vec<Value> = match arm { None => &d.p[pi], Some(a) => &d.arms[a][pi] };
            let p = PR[pi];
            let b = 64.0 - p.leading_zeros() as f64;
            let fm = 2.0 * (b * b + 5.0 * b * (b - 1.0)) + 13.0 * b;
            let (fa, wo) = (13.0 * b / fm, 64.0 / fm);
            let mr = (2.0 * (62.0f64 * 62.0 + 5.0 * 62.0 * 61.0) + 13.0 * 62.0) / fm;
            let comp = |v: &Value| -> [f64; 6] {
                let tot = bitproxy(v, p);
                let la = mr * g(v, "la_ops");
                let desc = g(v, "descent_muls") * (1.0 + fa);
                let adds = fa * (g(v, "sieve_adds") + g(v, "enum_adds") + fb(v, "adds"));
                let looks = wo * (g(v, "sieve_lookups") + fb(v, "lookups"));
                let rel = g(v, "enum_muls") + g(v, "sieve_muls");
                [la / tot, desc / tot, adds / tot, looks / tot, rel / tot, 1.0 - (la + desc + adds + looks + rel) / tot]
            };
            let mut m = [0.0; 6];
            for v in rows.iter() { let c = comp(v); for k in 0..6 { m[k] += c[k] / rows.len() as f64; } }
            writeln!(o, "  {an} p={p}: LA {:.1}% (ceiling x{:.2}), descent {:.1}%, emitted adds {:.1}% (L8 ceiling x{:.2}), lookups {:.1}%, enum+sieve muls {:.1}%, other {:.1}%",
                m[0] * 100.0, 1.0 / (1.0 - m[0]), m[1] * 100.0, m[2] * 100.0, 1.0 / (1.0 - m[2]), m[3] * 100.0, m[4] * 100.0, m[5] * 100.0).unwrap();
        }
    }
    // LA share and growth
    writeln!(o, "\nLA (mod l) share of total_muls at 16 vs at 31 F_p mults per op (mean over seeds), and LA growth arm/P:").unwrap();
    for pi in 0..3 {
        let sh = |v: &Value, c: f64| g(v, "la_ops") * c / (g(v, "total_muls") + (c - 16.0) * g(v, "la_ops"));
        let ps16 = mean(&d.p[pi].iter().map(|v| sh(v, 16.0)).collect::<Vec<_>>());
        let ps31 = mean(&d.p[pi].iter().map(|v| sh(v, 31.0)).collect::<Vec<_>>());
        let us31 = mean(&d.arms[2][pi].iter().map(|v| sh(v, 31.0)).collect::<Vec<_>>());
        let gr = gmean(&(0..n).map(|i| g(&d.arms[2][pi][i], "la_ops") / g(&d.p[pi][i], "la_ops")).collect::<Vec<_>>());
        writeln!(o, "  p={}: P share {:.1}% (at 16) / {:.1}% (at 31); U share at 31 {:.1}%; U/P la_ops {:.3}", PR[pi], ps16 * 100.0, ps31 * 100.0, us31 * 100.0, gr).unwrap();
    }

    // ---------------- A5: the rule under the measured gamma (regret of borrowing gamma = 5) ----------------
    writeln!(o, "\n## A5. The n-rule re-run under other gamma (Horner, w = 0; C_P from arm P = V2): optimum n and J; J at the frozen n_H = 4096").unwrap();
    let mp: Vec<f64> = (0..3).map(|pi| mean(&d.p[pi].iter().map(|v| g(v, "total_muls")).collect::<Vec<_>>())).collect();
    for gam in [4.0, 5.0, 5.4, 5.9, 6.5, 7.5] {
        let jf = |nn: f64| -> f64 {
            let v: Vec<f64> = (0..3).map(|pi| {
                let pf = PR[pi] as f64;
                let route = mp[pi] * (1.0 + kmax(nn) / pf.sqrt()).powf(-gam);
                ((route + nn * pf * 17.0) / mp[pi]).ln()
            }).collect();
            mean(&v).exp()
        };
        let mut best = (0.0, f64::INFINITY);
        for e in 0..=20 { let nn = (1u64 << e) as f64; let j = jf(nn); if j < best.1 { best = (nn, j); } }
        writeln!(o, "  gamma {gam}: optimum n = {} J = {:.4}; J(4096) = {:.4} (regret {:.1}%)", best.0, best.1, jf(4096.0), (jf(4096.0) / best.1 - 1.0) * 100.0).unwrap();
    }

    // ---------------- A6: pilot cross-check (seeds 17-24, p = 1511, n = 4096) ----------------
    writeln!(o, "\n## A6. The exploratory pilot (research/jv_cover_fbline_20261102/review/red_team/sieve_transfer_pilot_p1511.jsonl, n = 4096) against frozen arm H, p = 1511, seeds 17-24").unwrap();
    let pilot = std::fs::read_to_string(format!("{root}/research/jv_cover_fbline_20261102/review/red_team/sieve_transfer_pilot_p1511.jsonl")).unwrap();
    let mut pr_h = vec![];
    let mut pr_p = vec![];
    for line in pilot.lines() {
        let js: Value = serde_json::from_str(line.trim_start_matches("JSON ").trim()).unwrap();
        if g(&js, "n_lines") as u64 != 4096 { continue; }
        let seed = g(&js, "seed") as u64;
        let i = (seed - 17) as usize;
        let h = &d.arms[0][1][i];
        let same_b = g(&js, "b_best") == b(h);
        let same_route = g(&js["line"], "total_muls_route") == route_muls(h);
        let pilot_ratio = g(&js["line"], "total_muls_charged") / g(&js["plain"], "total_muls");
        let frozen_ratio = g(h, "total_muls") / g(&d.p[1][i], "total_muls");
        pr_h.push(frozen_ratio);
        pr_p.push(pilot_ratio);
        writeln!(o, "  seed {seed}: same b_best {same_b}, same route muls {same_route}; pilot search muls {:.4e} vs frozen {:.4e}; ratio pilot {pilot_ratio:.3} frozen {frozen_ratio:.3}", g(&js["line"], "search_muls"), fb(h, "muls")).unwrap();
    }
    let rest: Vec<f64> = (8..16).map(|i| g(&d.arms[0][1][i], "total_muls") / g(&d.p[1][i], "total_muls")).collect();
    writeln!(o, "  geo-mean ratio seeds 17-24: pilot {:.3}, frozen {:.3}; frozen seeds 25-32 (fresh): {:.3}", gmean(&pr_p), gmean(&pr_h), gmean(&rest)).unwrap();
}

/// bitop/v1 proxy in F_p-mult units at b = bits(p): field_add/field_mul = 13b / (2(b^2+5b(b-1))+13b),
/// word_op (64) / field_mul, modr_mul at 62 bits / field_mul; descent adds imputed 1 per descent mult;
/// index arithmetic of table walks as 1 integer product (w = b) + 1 add.
fn bitproxy(v: &Value, p: u64) -> f64 {
    let b = 64.0 - (p as u64).leading_zeros() as f64;
    let fm = 2.0 * (b * b + 5.0 * b * (b - 1.0)) + 13.0 * b;
    let fa = 13.0 * b / fm;
    let wo = 64.0 / fm;
    let wl = 62.0;
    let mr = (2.0 * (wl * wl + 5.0 * wl * (wl - 1.0)) + 13.0 * wl) / fm;
    let imul = (b * b + 5.0 * b * (b - 1.0)) / fm;
    let adds = g(v, "sieve_adds") + g(v, "enum_adds") + fb(v, "adds") + g(v, "descent_muls");
    let looks = g(v, "sieve_lookups") + fb(v, "lookups");
    let wl_ = walk_lookups(v, p);
    g(v, "total_muls") + (mr - 16.0) * g(v, "la_ops") + fa * adds + wo * looks + wl_ * (imul + 5.0 * b / fm)
}
fn walk_lookups(v: &Value, p: u64) -> f64 {
    if v.get("fb_line").is_none() { return 0.0; }
    let table = v["fb_line"]["mode"].as_str() == Some("table");
    if table { fb(v, "lookups") - (p * p) as f64 } else { 0.0 }
}
fn band(v: &Value, w: f64, extra_native: f64) -> f64 {
    let nat = g(v, "sieve_adds") + g(v, "sieve_lookups") + g(v, "enum_adds") + fb(v, "adds") + fb(v, "lookups") + extra_native;
    g(v, "total_muls") + w * g(v, "c_add_e") * nat
}

// E[max of n standard normals], as the frozen rule (numerical quadrature)
fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t * (-z * z - 1.26551223 + t * (1.00002368 + t * (0.37409196 + t * (0.09678418 + t * (-0.18628806 + t * (0.27886807 + t * (-1.13520398 + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277))))))))).exp();
    if x >= 0.0 { r } else { 2.0 - r }
}
fn kmax(n: f64) -> f64 {
    if n <= 1.0 { return 0.0; }
    let (lo, hi, h) = (-9.0f64, 12.0f64, 0.001f64);
    let mut s = 0.0;
    let mut x = lo;
    while x < hi {
        let xm = x + h / 2.0;
        let phi = (-xm * xm / 2.0).exp() / (2.0 * std::f64::consts::PI).sqrt();
        let q = 0.5 * erfc(xm / std::f64::consts::SQRT_2);
        s += xm * n * phi * ((n - 1.0) * (-q).ln_1p()).exp() * h;
        x += h;
    }
    s
}

// ======================= study B =======================
struct RhoRun { steps_n: f64, u1: f64, u2: f64, nat: f64, fpa: f64, steps: f64, l: f64, batch: f64, dp: f64, inv: f64, mul: f64, sq: f64 }
fn rho_runs(dir: &str) -> Vec<RhoRun> {
    let mut files: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| { let n = p.file_name().unwrap().to_string_lossy().to_string(); n.starts_with("rhostrong") && n.ends_with(".json") }).collect();
    files.sort();
    let mut out = vec![];
    for f in files {
        for r in load(f.to_str().unwrap()) {
            if !r["correct"].as_bool().unwrap() || r["capped"].as_bool().unwrap() || !r["negation"].as_bool().unwrap() || g(&r, "batch") != 16.0 || g(&r, "r") != 256.0 || g(&r, "dp_bits") != 16.0 { continue; }
            let sl = g(&r, "l").sqrt();
            let ops = &r["ops"];
            let nat: f64 = ["canon_cmp", "hash", "dp_check", "scalar_ops", "table_ops", "cycle_cmp"].iter().map(|k| g(ops, k)).sum();
            out.push(RhoRun { steps_n: g(&r, "steps") / sl, u1: g(&r, "s_harness"), u2: g(&r, "s_tuned"), nat: nat / sl, fpa: g(ops, "fp_adds") / sl,
                steps: g(&r, "steps"), l: g(&r, "l"), batch: g(&r, "batch"), dp: g(&r, "dp_bits"), inv: g(ops, "inv"), mul: g(ops, "mul"), sq: g(ops, "sq") });
        }
    }
    out
}

fn perm_tests(a: &[f64], b: &[f64], nperm: usize, rng: &mut Rng) -> (f64, f64) {
    // returns (p of |mean diff|, p of KS D), permutation, group sizes kept
    let mut all: Vec<f64> = a.iter().chain(b).copied().collect();
    let na = a.len();
    let stat = |v: &[f64]| -> (f64, f64) {
        let (x, y) = v.split_at(na);
        let md = (mean(x) - mean(y)).abs();
        let mut xs = x.to_vec(); xs.sort_by(|p, q| p.partial_cmp(q).unwrap());
        let mut ys = y.to_vec(); ys.sort_by(|p, q| p.partial_cmp(q).unwrap());
        let (mut i, mut j, mut dmax) = (0usize, 0usize, 0.0f64);
        while i < xs.len() && j < ys.len() {
            if xs[i] <= ys[j] { i += 1; } else { j += 1; }
            dmax = dmax.max((i as f64 / xs.len() as f64 - j as f64 / ys.len() as f64).abs());
        }
        (md, dmax)
    };
    let (m0, k0) = stat(&all);
    let (mut cm, mut ck) = (0usize, 0usize);
    for _ in 0..nperm {
        for i in (1..all.len()).rev() { let j = rng.idx(i + 1); all.swap(i, j); }
        let (m, k) = stat(&all);
        if m >= m0 - 1e-12 { cm += 1; }
        if k >= k0 - 1e-12 { ck += 1; }
    }
    ((cm as f64 + 1.0) / (nperm as f64 + 1.0), (ck as f64 + 1.0) / (nperm as f64 + 1.0))
}

fn study_b(root: &str, o: &mut String) {
    let mut rng = Rng(0x0B0B_2026_1103);
    let newr = rho_runs(&format!("{root}/research/jv_sieve_negrho_20261102/results/runs"));
    let mut old = rho_runs(&format!("{root}/research/jv_cover_negrho_rescore_20261101/results/inputs/old_rhostrong"));
    old.extend(rho_runs(&format!("{root}/research/jv_cover_negrho_rescore_20261101/results/new")));
    let a: Vec<f64> = newr.iter().map(|r| r.steps_n).collect();
    let bb: Vec<f64> = old.iter().map(|r| r.steps_n).collect();
    let theory = (std::f64::consts::PI / 4.0).sqrt() / (1.0 - 1.0 / 256.0f64).sqrt();
    writeln!(o, "## B1. Reference runs: new matched n = {} mean {:.4} sd {:.4}; #3278 n = {} mean {:.4} sd {:.4}; theory {theory:.5}", a.len(), mean(&a), sd(&a), bb.len(), mean(&bb), sd(&bb)).unwrap();
    let mut all = a.clone(); all.extend(&bb);
    writeln!(o, "pooled 142: mean {:.4} (theory +{:.1}%); matched/pooled {:.3}", mean(&all), (mean(&all) / theory - 1.0) * 100.0, mean(&a) / mean(&all)).unwrap();
    let se_diff = sd(&all) * (1.0 / a.len() as f64 + 1.0 / bb.len() as f64).sqrt();
    writeln!(o, "normal approximation: se of the mean difference {se_diff:.3}; 80%-power detectable shift (2.8 se) = {:.3} = {:.0}% of the pooled mean", 2.8 * se_diff, 2.8 * se_diff / mean(&all) * 100.0).unwrap();
    // power of the frozen homogeneity rule (both permutation p >= 0.05 => 'homogeneous')
    writeln!(o, "\nPower of the frozen rule to reject homogeneity when the matched instances' true mean is c x the pooled one (simulation: 16 draws from the #3278 empirical distribution x c vs 126 bootstrap draws; 1000 permutations per test; 400 simulations per c):").unwrap();
    for c in [0.6, 0.7, 0.8, 0.9, 1.0, 1.15, 1.3, 1.5] {
        let mut rej = 0;
        let sims = 400;
        for _ in 0..sims {
            let x: Vec<f64> = (0..16).map(|_| bb[rng.idx(bb.len())] * c).collect();
            let y: Vec<f64> = (0..bb.len()).map(|_| bb[rng.idx(bb.len())]).collect();
            let (pm, pk) = perm_tests(&x, &y, 1000, &mut rng);
            if pm < 0.05 || pk < 0.05 { rej += 1; }
        }
        writeln!(o, "  c = {c}: P(reject homogeneity) = {:.2}", rej as f64 / sims as f64).unwrap();
    }
    // per-step costs and dp overhead
    let per = |f: &dyn Fn(&RhoRun) -> f64, v: &[RhoRun]| mean(&v.iter().map(f).collect::<Vec<_>>());
    writeln!(o, "\n## B2. Rho per-step costs (new runs, means): harness F_p mults/step {:.1}; tuned {:.1}; F_q3 mul {:.3}/step, sq {:.3}/step, inv {:.4}/step; fp_adds/step {:.1}; native/step {:.1}",
        per(&|r| r.u1 * GAE * r.l.sqrt() / r.steps, &newr), per(&|r| r.u2 * GAE * r.l.sqrt() / r.steps, &newr), per(&|r| r.mul / r.steps, &newr), per(&|r| r.sq / r.steps, &newr), per(&|r| r.inv / r.steps, &newr),
        per(&|r| r.fpa * r.l.sqrt() / r.steps, &newr), per(&|r| r.nat * r.l.sqrt() / r.steps, &newr)).unwrap();
    let inv_share = per(&|r| r.inv * 62.0 / (r.u2 * GAE * r.l.sqrt()), &newr);
    writeln!(o, "batched-inversion share of the tuned price (inv x 62 / tuned): {:.1}%; a minimal F_q3 product (5-6 F_q products, s in F_p) at 18 F_p mults instead of TUNED's 24 would give {:.1} F_p mults/step (x{:.2} cheaper than tuned)", inv_share * 100.0,
        per(&|r| (r.mul * 18.0 + r.sq * 12.0 + r.inv * 62.0) / r.steps, &newr), per(&|r| r.u2 * GAE * r.l.sqrt() / (r.mul * 18.0 + r.sq * 12.0 + r.inv * 62.0), &newr)).unwrap();
    let dp16 = per(&|r| r.batch * 2f64.powf(r.dp) / r.l.sqrt(), &newr);
    let dp15 = per(&|r| r.batch * 2f64.powf(r.dp - 1.0) / r.l.sqrt(), &newr);
    writeln!(o, "post-collision distinguished-point overhead (batch x 2^dp / sqrt l): dp 16 {:.5} vs dp 15 {:.5} of sqrt(l) (difference {:.5}, i.e. {:.2}% of the 0.918 mean)", dp16, dp15, dp16 - dp15, (dp16 - dp15) / 0.918 * 100.0).unwrap();
}

// ======================= combined A x B, computed directly =======================
fn combined(root: &str, o: &mut String) {
    let mut rng = Rng(0xC0B1_2026_1103);
    let fj: Value = serde_json::from_str(&std::fs::read_to_string(format!("{root}/research/jv_sieve_negrho_20261102/fit.json")).unwrap()).unwrap();
    let r: Vec<f64> = fj["reference_means"]["pooled"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let (u1, u2, nat_r, fpa_r) = (r[0], r[1], r[3], r[4]);
    writeln!(o, "## C. Line-search arms against study B's pooled rho, computed directly from the arm records at p = 1511 (no product of ratios)").unwrap();
    writeln!(o, "reference (study B fit.json, pooled 142): s_harness {u1:.4}, s_tuned {u2:.4}, native/sqrt l {nat_r:.3}, fp_adds/sqrt l {fpa_r:.3}. Interval: bootstrap over cover seeds only (reference fixed).").unwrap();
    // rho's adds and native ops in the same bitop proxy (fp_adds at field_add, native at word_op), per sqrt(l), in GAE
    let bq = 11.0f64; let fm = 2.0 * (bq * bq + 5.0 * bq * (bq - 1.0)) + 13.0 * bq;
    let rho_bit_extra = (fpa_r * 13.0 * bq / fm + nat_r * 64.0 / fm) / GAE;
    writeln!(o, "bitop proxy: field_add = {:.3} field_mul, word_op = {:.3}, modr_mul(62 bit) = {:.1} field_mul at b = 11; rho extra (adds + native) = {:.4} GAE per sqrt(l)", 13.0 * bq / fm, 64.0 / fm, (2.0 * (62.0f64 * 62.0 + 5.0 * 62.0 * 61.0) + 13.0 * 62.0) / fm, rho_bit_extra).unwrap();
    let res = format!("{root}/research/jv_sieve_fbline_20261102/results");
    writeln!(o, "| arm | seeds | S/rho U1 | S/rho U2 | S+/rho U1 (rho unpriced) | S+/rho U2 (rho unpriced) | S+/rhoS+ U1 | S+/rhoS+ U2 | band A U2 w=1 | band B U2 w=1 | bitop proxy / rho harness (U1-like) | bitop proxy / rho tuned (U2-like) |").unwrap();
    writeln!(o, "|--|--|--|--|--|--|--|--|--|--|--|--|").unwrap();
    for arm in ["P", "H", "T", "U"] {
        let v = load(&format!("{res}/{arm}_1511.json"));
        for (lab, rng_s) in [("17-24", 0..8usize), ("17-32", 0..16usize)] {
            let rows: Vec<&Value> = rng_s.clone().map(|i| &v[i]).collect();
            let cell = |rows: &[&Value]| -> Vec<f64> {
                let s: Vec<f64> = rows.iter().map(|x| g(x, "total_muls") / sqrt_l(x)).collect();
                let sp: Vec<f64> = rows.iter().map(|x| g(x, "total_plus") / sqrt_l(x)).collect();
                let sl = |x: &Value| g(x, "l").sqrt();
                let nat_a: Vec<f64> = rows.iter().map(|x| (g(x, "sieve_adds") + g(x, "sieve_lookups") + g(x, "enum_adds") + (g(x, "total_muls") - g(x, "sieve_muls") - g(x, "enum_muls")) + fb(x, "adds") + fb(x, "lookups")) / sl(x)).collect();
                let nat_b: Vec<f64> = rows.iter().map(|x| (g(x, "sieve_lookups") + fb(x, "lookups")) / sl(x)).collect();
                let rsp = (nat_r + fpa_r) / GAE;
                let ba: Vec<f64> = s.iter().zip(&nat_a).map(|(a, n)| a + n).collect();
                let bbv: Vec<f64> = s.iter().zip(&nat_b).map(|(a, n)| a + n).collect();
                vec![gmean(&s) / u1, gmean(&s) / u2, gmean(&sp) / u1, gmean(&sp) / u2, gmean(&sp) / (u1 + rsp), gmean(&sp) / (u2 + rsp),
                     gmean(&ba) / (u2 + nat_r + fpa_r), gmean(&bbv) / (u2 + nat_r),
                     gmean(&rows.iter().map(|x| bitproxy(x, 1511) / sqrt_l(x)).collect::<Vec<_>>()) / (u1 + rho_bit_extra),
                     gmean(&rows.iter().map(|x| bitproxy(x, 1511) / sqrt_l(x)).collect::<Vec<_>>()) / (u2 + rho_bit_extra)]
            };
            let pt = cell(&rows);
            let k = rows.len();
            let mut bs: Vec<Vec<f64>> = vec![vec![]; pt.len()];
            for _ in 0..NB { let rr: Vec<&Value> = (0..k).map(|_| rows[rng.idx(k)]).collect(); for (j, x) in cell(&rr).into_iter().enumerate() { bs[j].push(x); } }
            let cells: Vec<String> = pt.iter().enumerate().map(|(j, x)| { let (lo, hi) = ci(&mut bs[j]); format!("{x:.4} [{lo:.4}, {hi:.4}]") }).collect();
            writeln!(o, "| {arm} | {lab} | {} |", cells.join(" | ")).unwrap();
        }
    }
    writeln!(o, "LEADS.md products for comparison: H S/rho U1 0.0065 x 0.644 = 0.0042, U2 ~0.010; U S+/rho U1 0.027 x 0.525 = 0.014, U2 0.065 x 0.525 = 0.034.").unwrap();
}

fn main() {
    let root = std::env::args().nth(1).unwrap_or_else(|| "/home/user/crypto".into());
    let mut o = String::new();
    writeln!(o, "# Red-team derived numbers (Rust, rt_sieve_review; inputs: committed records at main aa1c89ab2a)\n").unwrap();
    study_a(&root, &mut o);
    writeln!(o).unwrap();
    study_b(&root, &mut o);
    writeln!(o).unwrap();
    combined(&root, &mut o);
    print!("{o}");
}
