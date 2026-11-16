//! POST HOC (written after the registered score; not part of the frozen
//! protocol). Pools units instead of taking per-unit ratios, so that units
//! with a zero mean (rare, heavily clustered counts at toy sizes) do not
//! turn a pooled geometric mean into 0 or infinity.
//!
//! For a group of units G and a class c, the pooled density of an arm is
//! K(arm) = (1/|G|) sum_u kappa_u(arm, c), kappa = mean N * #E / m^n, and
//! pooled ratios are ratios of pooled densities (H = K(hz)/K(std), etc.).
//! Pooled relations per trial per column: Eta(arm) = sum_u eta_u / sum_u eta_pred_u
//! (obs/pred). Intervals: bootstrap over targets within each unit
//! (B = 2000, seed tag posthoc-*), 99 % percentile.
//!
//! usage: posthoc <run_dir>
#[path = "../../../c1_twisted_line_ladder_20261027/src/json.rs"]
mod json;
#[path = "../rng.rs"]
mod rng;

use rng::{Rng, SEED};

const ARMS: [&str; 6] = ["std", "tz", "tw", "twh", "hz", "mix"];
const B: usize = 2000;

struct Unit {
    n: u32,
    d: u32,
    id: String,
    ne: f64,
    census: bool,
    m: [f64; 6],
    m0: [f64; 6],
    m1: [f64; 6],
    c: [f64; 6],
    counts: Vec<Vec<Vec<f64>>>, // [class 2E, off, odd][arm][target]
}

fn load(dir: &str) -> Vec<Unit> {
    let mut files: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path())
        .filter(|p| p.file_name().unwrap().to_str().unwrap().starts_with("cell_")).collect();
    files.sort();
    let mut us: Vec<Unit> = vec![];
    for p in files {
        for line in std::fs::read_to_string(&p).unwrap().lines().filter(|l| !l.trim().is_empty()) {
            let r = json::parse(line);
            if r.get("type").str() == "curve" {
                if r.get("tr_a").num().unwrap() != 0.0 {
                    us.push(Unit { n: 0, d: 0, id: "NEG".into(), ne: 0.0, census: false, m: [0.0; 6], m0: [0.0; 6], m1: [0.0; 6], c: [0.0; 6], counts: vec![vec![]; 3] });
                    continue;
                }
                let mut u = Unit {
                    n: r.get("n").num().unwrap() as u32,
                    d: r.get("d").num().unwrap() as u32,
                    id: r.get("id").str().into(),
                    ne: r.get("nE").num().unwrap(),
                    census: r.get("mode").str() == "census",
                    m: [0.0; 6], m0: [0.0; 6], m1: [0.0; 6], c: [0.0; 6],
                    counts: vec![vec![]; 3],
                };
                for (i, a) in r.get("arms").arr().iter().enumerate() {
                    assert_eq!(a.get("arm").str(), ARMS[i]);
                    u.m[i] = a.get("m").num().unwrap();
                    u.m0[i] = a.get("m0").num().unwrap();
                    u.m1[i] = a.get("m1").num().unwrap();
                    u.c[i] = a.get("c").num().unwrap();
                }
                us.push(u);
            } else {
                let u = us.last_mut().unwrap();
                let c = ["2E", "off", "odd"].iter().position(|x| *x == r.get("class").str()).unwrap();
                u.counts[c] = ARMS.iter().map(|a| r.get("counts").get(a).nums()).collect();
            }
        }
    }
    us.into_iter().filter(|u| u.id != "NEG").collect()
}

impl Unit {
    fn pw(&self, x: f64) -> f64 { x.powi(self.n as i32) }
    fn pred_mean(&self, a: usize) -> f64 {
        (self.pw(self.m0[a] + self.m1[a]) + self.pw(self.m0[a] - self.m1[a])) / self.ne
    }
}

fn mean(v: &[f64]) -> f64 { v.iter().sum::<f64>() / v.len() as f64 }

/// pooled stats for one resample: returns K[arm], Eta_obs/pred[arm]
fn pooled(units: &[&Unit], means: &[Vec<f64>]) -> ([f64; 6], [f64; 6]) {
    let mut k = [0.0; 6];
    let (mut eo, mut ep) = ([0.0; 6], [0.0; 6]);
    for (u, mm) in units.iter().zip(means) {
        for a in 0..6 {
            k[a] += mm[a] * u.ne / u.pw(u.m[a]) / units.len() as f64;
            eo[a] += mm[a] / u.c[a];
            ep[a] += u.pred_mean(a) / u.c[a];
        }
    }
    let mut e = [0.0; 6];
    for a in 0..6 { e[a] = eo[a] / ep[a]; }
    (k, e)
}

fn pct(v: &mut [f64], p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Greater));
    v[((v.len() as f64 - 1.0) * p).round() as usize]
}

fn main() {
    let dir = std::env::args().nth(1).expect("run dir");
    let all = load(&dir);
    let (std, tz, tw, twh, hz, mix) = (0, 1, 2, 3, 4, 5);
    let groups: Vec<(&str, Box<dyn Fn(&Unit) -> bool>)> = vec![
        ("odd n, all units", Box::new(|u: &Unit| u.n % 2 == 1)),
        ("odd n, census units", Box::new(|u: &Unit| u.n % 2 == 1 && u.census)),
        ("odd n, sampled units", Box::new(|u: &Unit| u.n % 2 == 1 && !u.census)),
        ("odd n, std trace-mixed (m1 > 0), all", Box::new(|u: &Unit| u.n % 2 == 1 && u.m1[0] > 0.0)),
        ("n = 3", Box::new(|u: &Unit| u.n == 3)),
        ("n = 5", Box::new(|u: &Unit| u.n == 5)),
        ("n = 5, std trace-mixed", Box::new(|u: &Unit| u.n == 5 && u.m1[0] > 0.0)),
        ("even n = 4", Box::new(|u: &Unit| u.n == 4)),
    ];
    let mut out = String::new();
    use std::fmt::Write as _;
    writeln!(out, "POST HOC pooled analysis (not registered). K = mean over units of kappa; ratios of pooled K; Eta = sum eta_obs / sum eta_pred. 99% bootstrap over targets within units, B = {B}.\n").unwrap();
    let mut json_rows = vec![];
    for (gname, f) in &groups {
        let units: Vec<&Unit> = all.iter().filter(|u| f(u)).collect();
        if units.is_empty() { continue; }
        writeln!(out, "== {gname}: {} units ({})", units.len(), units.iter().map(|u| format!("{}/{}/{}", u.n, u.d, u.id)).collect::<Vec<_>>().join(" ")).unwrap();
        for (ci, cname) in [(0usize, "2E"), (2, "odd")] {
            let obs_means: Vec<Vec<f64>> = units.iter().map(|u| u.counts[ci].iter().map(|v| mean(v)).collect()).collect();
            let (k, e) = pooled(&units, &obs_means);
            let stat = |k: &[f64; 6], e: &[f64; 6]| -> Vec<f64> {
                vec![k[hz] / k[std], k[tw] / k[hz], k[tw] / k[std], e[tz] / e[std], k[mix] / k[std],
                     k[std], k[tz], k[tw], k[twh], k[hz], k[mix], e[std], e[tz], e[tw], e[hz]]
            };
            let names = ["H=K_hz/K_std", "L=K_tw/K_hz", "D=K_tw/K_std", "Teta(obs/pred)_tz/std", "K_mix/K_std",
                         "K_std", "K_tz", "K_tw", "K_twh", "K_hz", "K_mix", "Eta_std", "Eta_tz", "Eta_tw", "Eta_hz"];
            let o = stat(&k, &e);
            let mut rng = Rng::new(SEED, &format!("posthoc-{gname}-{cname}"));
            let mut boots: Vec<Vec<f64>> = vec![vec![]; o.len()];
            for _ in 0..B {
                let bm: Vec<Vec<f64>> = units.iter().map(|u| {
                    let cl = &u.counts[ci];
                    let t = cl[0].len();
                    let mut s = vec![0.0; 6];
                    for _ in 0..t {
                        let i = rng.below(t as u64) as usize;
                        for a in 0..6 { s[a] += cl[a][i]; }
                    }
                    s.iter().map(|x| x / t as f64).collect()
                }).collect();
                let (bk, be) = pooled(&units, &bm);
                for (j, v) in stat(&bk, &be).into_iter().enumerate() { boots[j].push(if v.is_nan() { f64::INFINITY } else { v }); }
            }
            writeln!(out, "  class {cname}:").unwrap();
            for (j, nm) in names.iter().enumerate() {
                let lo = pct(&mut boots[j], 0.005);
                let hi = pct(&mut boots[j], 0.995);
                writeln!(out, "    {nm:<24} {:>9.4}  99% [{lo:>9.4}, {hi:>9.4}]", o[j]).unwrap();
                json_rows.push(format!("{{\"group\":\"{gname}\",\"class\":\"{cname}\",\"stat\":\"{nm}\",\"obs\":{:.6},\"lo\":{lo:.6},\"hi\":{hi:.6}}}", o[j]));
            }
        }
        writeln!(out).unwrap();
    }
    // exact composition consequence: hz versus std at matched size, per unit (census and sampled alike)
    writeln!(out, "Composition (exact, from measured m0, m1): predicted 2E-mean ratio hz/std and eta ratio hz/std, odd n:").unwrap();
    for u in all.iter().filter(|u| u.n % 2 == 1) {
        let r = u.pred_mean(hz) / u.pred_mean(std);
        let e = (u.pred_mean(hz) / u.c[hz]) / (u.pred_mean(std) / u.c[std]);
        let kr = (u.pred_mean(hz) * u.ne / u.pw(u.m[hz])) / (u.pred_mean(std) * u.ne / u.pw(u.m[std]));
        writeln!(out, "  n={} d={} {:<3} m_std={:<3} (m0 {:<3} m1 {:<3}) m_hz={:<3} m_tw={:<3}  pred yield hz/std {r:>7.4}  pred eta hz/std {e:>7.4}  pred kappa hz/std {kr:.4}", u.n, u.d, u.id, u.m[std], u.m0[std], u.m1[std], u.m[hz], u.m[tw]).unwrap();
    }
    print!("{out}");
    std::fs::write(format!("{dir}/posthoc.txt"), &out).unwrap();
    std::fs::write(format!("{dir}/posthoc.jsonl"), json_rows.join("\n") + "\n").unwrap();
}
