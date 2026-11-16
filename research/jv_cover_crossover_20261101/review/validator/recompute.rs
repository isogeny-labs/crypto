//! Independent recomputation for the validation of
//! `research/jv_cover_crossover_20261101` (validator-breakthrough, 2026-11-01).
//!
//! Shares no code with `fit/` (own JSON reader, own RNG, own t quantile, own
//! bootstrap). Build: `rustc -O --edition 2021 recompute.rs -o <out>`.
//! Usage: `recompute <study results dir> <predecessor results dir>`.
//!
//! Every printed number is **derived** from the raw records unless marked
//! `model` (sensitivity under an assumption that is not the frozen rule).
use std::collections::BTreeMap;
use std::fs;

// ───────────────────────── minimal JSON ─────────────────────────
#[derive(Clone, Debug)]
enum J {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}
impl J {
    fn get(&self, k: &str) -> Option<&J> {
        if let J::Obj(v) = self {
            v.iter().find(|(kk, _)| kk == k).map(|(_, v)| v)
        } else {
            None
        }
    }
    fn f(&self, k: &str) -> f64 {
        match self.get(k) {
            Some(J::Num(s)) => s.parse().unwrap(),
            _ => panic!("missing number {k}"),
        }
    }
    fn u(&self, k: &str) -> u64 {
        match self.get(k) {
            Some(J::Num(s)) => s.parse().unwrap_or_else(|_| panic!("not u64: {k}={s}")),
            _ => panic!("missing integer {k}"),
        }
    }
    fn b(&self, k: &str) -> bool {
        match self.get(k) {
            Some(J::Bool(b)) => *b,
            _ => panic!("missing bool {k}"),
        }
    }
    fn arr(&self) -> &Vec<J> {
        if let J::Arr(v) = self {
            v
        } else {
            panic!("not an array")
        }
    }
}
struct P<'a> {
    s: &'a [u8],
    i: usize,
}
impl<'a> P<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] as char).is_whitespace() {
            self.i += 1;
        }
    }
    fn string(&mut self) -> String {
        assert_eq!(self.s[self.i], b'"');
        self.i += 1;
        let st = self.i;
        while self.s[self.i] != b'"' {
            if self.s[self.i] == b'\\' {
                self.i += 1;
            }
            self.i += 1;
        }
        let r = String::from_utf8(self.s[st..self.i].to_vec()).unwrap();
        self.i += 1;
        r
    }
    fn parse(&mut self) -> J {
        self.ws();
        match self.s[self.i] {
            b'{' => {
                self.i += 1;
                let mut v = vec![];
                loop {
                    self.ws();
                    if self.s[self.i] == b'}' {
                        self.i += 1;
                        break;
                    }
                    let k = self.string();
                    self.ws();
                    assert_eq!(self.s[self.i], b':');
                    self.i += 1;
                    let val = self.parse();
                    v.push((k, val));
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    }
                }
                J::Obj(v)
            }
            b'[' => {
                self.i += 1;
                let mut v = vec![];
                loop {
                    self.ws();
                    if self.s[self.i] == b']' {
                        self.i += 1;
                        break;
                    }
                    v.push(self.parse());
                    self.ws();
                    if self.s[self.i] == b',' {
                        self.i += 1;
                    }
                }
                J::Arr(v)
            }
            b'"' => J::Str(self.string()),
            b't' => {
                self.i += 4;
                J::Bool(true)
            }
            b'f' => {
                self.i += 5;
                J::Bool(false)
            }
            b'n' => {
                self.i += 4;
                J::Null
            }
            _ => {
                let st = self.i;
                while self.i < self.s.len()
                    && matches!(self.s[self.i], b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E')
                {
                    self.i += 1;
                }
                J::Num(String::from_utf8(self.s[st..self.i].to_vec()).unwrap())
            }
        }
    }
}
fn load(path: &str) -> Vec<J> {
    let s = fs::read(path).unwrap_or_else(|_| panic!("read {path}"));
    let mut p = P { s: &s, i: 0 };
    p.parse().arr().clone()
}
fn files(dir: &str, prefix: &str) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().to_string())
        .filter(|p| {
            let n = p.rsplit('/').next().unwrap();
            n.starts_with(prefix) && n.ends_with(".json")
        })
        .collect();
    v.sort();
    v
}

// ───────────────────────── numerics ─────────────────────────
fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}
fn var(v: &[f64]) -> f64 {
    let m = mean(v);
    v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() as f64 - 1.0)
}
fn sd(v: &[f64]) -> f64 {
    var(v).sqrt()
}
fn gmean(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn ln_gamma(x: f64) -> f64 {
    // Lanczos, g = 7
    let c = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        return (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = c[0];
    let t = x + 7.5;
    for (i, ci) in c.iter().enumerate().skip(1) {
        a += ci / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}
/// Regularized incomplete beta I_x(a, b) by Lentz's continued fraction.
fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let bt = (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    let cf = |a: f64, b: f64, x: f64| -> f64 {
        let tiny = 1e-300;
        let qab = a + b;
        let qap = a + 1.0;
        let qam = a - 1.0;
        let mut c = 1.0;
        let mut d = 1.0 - qab * x / qap;
        if d.abs() < tiny {
            d = tiny;
        }
        d = 1.0 / d;
        let mut h = d;
        for m in 1..400 {
            let m = m as f64;
            let m2 = 2.0 * m;
            let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
            d = 1.0 + aa * d;
            if d.abs() < tiny {
                d = tiny;
            }
            c = 1.0 + aa / c;
            if c.abs() < tiny {
                c = tiny;
            }
            d = 1.0 / d;
            h *= d * c;
            let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
            d = 1.0 + aa * d;
            if d.abs() < tiny {
                d = tiny;
            }
            c = 1.0 + aa / c;
            if c.abs() < tiny {
                c = tiny;
            }
            d = 1.0 / d;
            let del = d * c;
            h *= del;
            if (del - 1.0).abs() < 1e-15 {
                break;
            }
        }
        h
    };
    if x < (a + 1.0) / (a + b + 2.0) {
        bt * cf(a, b, x) / a
    } else {
        1.0 - bt * cf(b, a, 1.0 - x) / b
    }
}
/// Student t CDF.
fn t_cdf(t: f64, nu: f64) -> f64 {
    let x = nu / (nu + t * t);
    let p = 0.5 * betai(nu / 2.0, 0.5, x);
    if t >= 0.0 {
        1.0 - p
    } else {
        p
    }
}
/// t quantile by bisection.
fn t_q(p: f64, nu: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, 1000.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if t_cdf(mid, nu) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}
struct Rng(u64);
impl Rng {
    // splitmix64
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn idx(&mut self, n: usize) -> usize {
        // unbiased by rejection
        let zone = u64::MAX - (u64::MAX % n as u64);
        loop {
            let r = self.next();
            if r < zone {
                return (r % n as u64) as usize;
            }
        }
    }
}
/// Percentile with linear interpolation (type 7).
fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let h = (v.len() - 1) as f64 * q;
    let lo = h.floor() as usize;
    let hi = h.ceil() as usize;
    v[lo] + (h - lo as f64) * (v[hi] - v[lo])
}
fn ols(x: &[f64], y: &[f64]) -> (f64, f64, f64) {
    let n = x.len() as f64;
    let mx = mean(x);
    let my = mean(y);
    let sxx: f64 = x.iter().map(|v| (v - mx).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let b = sxy / sxx;
    let a = my - b * mx;
    let rss: f64 = x.iter().zip(y).map(|(xi, yi)| (yi - a - b * xi).powi(2)).sum();
    let se = (rss / (n - 2.0) / sxx).sqrt();
    (a, b, se)
}
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}
/// Deterministic Miller–Rabin for u64.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

// ───────────────────────── records ─────────────────────────
#[derive(Clone)]
struct Cover {
    p: u64,
    seed: u64,
    l: u64,
    c_e: f64,
    total: u64,
    s_rec: f64,
    s: f64,
    ok: bool,
    comp_sum: u64,
    lin: u64,
    f4: u64,
    incomplete: u64,
    timed_out: u64,
    digest: Option<u64>,
}
fn cover(r: &J) -> Cover {
    let l = r.u("l");
    let c_e = r.f("c_add_e");
    let total = r.u("total_muls");
    let comp_sum = r.u("setup_muls")
        + r.u("base_muls")
        + r.u("stream_muls")
        + r.u("weil_muls")
        + r.u("f4_muls")
        + r.u("lin_muls")
        + r.u("post_muls")
        + r.u("verify_muls")
        + r.u("la_muls");
    Cover {
        p: r.u("p"),
        seed: r.u("seed"),
        l,
        c_e,
        total,
        s_rec: r.f("s"),
        s: total as f64 / (c_e * (l as f64).sqrt()),
        ok: r.b("solved") && r.b("correct") && r.u("mismatches") == 0,
        comp_sum,
        lin: r.u("lin_muls"),
        f4: r.u("f4_muls"),
        incomplete: r.u("incomplete"),
        timed_out: r.u("timed_out"),
        digest: r.get("relation_digest").map(|_| r.u("relation_digest")),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (rdir, pdir) = (&a[1], &a[2]);
    const HARNESS_ADD: f64 = 331.0; // 2·55 + 55 + 166, `jv_rho_strong::harness_prices`
    const TUNED_PER_HARNESS: f64 = 24.0 / 55.0; // not used for a verdict; see §tuned

    println!("== 1. cover records (study) ==");
    let mut cov: BTreeMap<u64, Vec<Cover>> = BTreeMap::new();
    for f in files(rdir, "dlp_p") {
        for r in load(&f) {
            let c = cover(&r);
            let p6 = (c.p as f64).powi(6);
            println!(
                "p={} seed={} l={} prime={} p^6/(4l)={:.6} c_add_e={} total_muls={} comp_sum={} sum_matches={} S={:.6} S_rec={:.6} |dS|={:.1e} ok={} incomplete={} timed_out={} lin/total={:.4} f4/total={:.4} digest={:?}",
                c.p, c.seed, c.l, is_prime(c.l), p6 / (4.0 * c.l as f64), c.c_e, c.total, c.comp_sum,
                c.total == c.comp_sum, c.s, c.s_rec, (c.s - c.s_rec).abs(), c.ok, c.incomplete, c.timed_out,
                c.lin as f64 / c.total as f64, c.f4 as f64 / c.total as f64, c.digest
            );
            // §5a, cover side: the only native count the cover arm emits (a lower bound)
            let nat = r.f("f4_native_ops");
            let sl = (c.l as f64).sqrt();
            println!(
                "    §5a cover-side (lower bound, f4_native_ops only): native/sqrt(l) = {:.2} GAE at w=1; S at w=0.1 = {:.3}; at w=1 = {:.3}  (model: the cover's own unpriced work priced as the protocol prices rho's)",
                nat / sl, c.s + 0.1 * nat / sl, c.s + nat / sl
            );
            cov.entry(c.p).or_default().push(c);
        }
    }
    // probe copy identical?
    if let (Ok(a), Ok(b)) = (
        fs::read(format!("{rdir}/probe/dlp_p1301_s1.json")),
        fs::read(format!("{rdir}/dlp_p1301_s1.json")),
    ) {
        println!("probe/dlp_p1301_s1.json byte-identical to dlp_p1301_s1.json: {}", a == b);
    }

    println!("\n== 2. reproduction control p=1009 seed 1 ==");
    let repro: Vec<Cover> = files(rdir, "repro_dlp_p1009").iter().flat_map(|f| load(f)).map(|r| cover(&r)).collect();
    let pred1009: Vec<J> = load(&format!("{pdir}/dlp_p1009_s1.json"));
    for c in &repro {
        for r in &pred1009 {
            let pc = cover(r);
            if pc.seed == c.seed {
                println!(
                    "new total={} pred total={} identical={} ratio={:.6}; f4 new={} pred={} same={}; lin new={} pred={} ratio={:.6}; post new={} pred={}; residuals new={} pred={}; S new={:.6} pred={:.6}",
                    c.total, pc.total, c.total == pc.total, c.total as f64 / pc.total as f64,
                    c.f4, pc.f4, c.f4 == pc.f4, c.lin, pc.lin, c.lin as f64 / pc.lin as f64,
                    0, 0, 0, 0, c.s, pc.s
                );
                let nl = c.total - c.lin;
                let pl = pc.total - pc.lin;
                println!("total minus lin: new={} pred={} diff={} (post_muls +902 is the only other change)", nl, pl, nl as i128 - pl as i128);
            }
        }
    }

    println!("\n== 3. rhodp records ==");
    let mut rhodp: BTreeMap<u64, BTreeMap<u64, Vec<f64>>> = BTreeMap::new();
    let mut rhodp_native: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for f in files(rdir, "rhodp_p") {
        for r in load(&f) {
            let (p, seed, l) = (r.u("p"), r.u("seed"), r.u("l"));
            let (steps, ops) = (r.u("steps"), r.u("group_ops"));
            let s = ops as f64 / (l as f64).sqrt();
            let dpb = r.u("dp_bits");
            println!(
                "p={} seed={} run={} threads={} dp_bits={} steps={} group_ops={} ops-steps={} S={:.6} S_rec={:.6} |dS|={:.1e} correct={} wall_s={:.0} steps/s={:.3e}",
                p, seed, r.u("run"), r.u("threads"), dpb, steps, ops, ops - steps, s, r.f("s"), (s - r.f("s")).abs(), r.b("correct"), r.f("wall_ms") / 1e3, steps as f64 / (r.f("wall_ms") / 1e3)
            );
            assert!(r.b("correct"));
            rhodp.entry(p).or_default().entry(seed).or_default().push(s);
            rhodp_native.entry(p).or_default().push((steps as f64 + steps as f64 / 2f64.powi(dpb as i32)) / (l as f64).sqrt());
        }
    }

    println!("\n== 4. rhostrong records ==");
    let mut strong: BTreeMap<u64, BTreeMap<u64, Vec<f64>>> = BTreeMap::new();
    let mut strong_tuned: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    let mut strong_native: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for f in files(rdir, "rhostrong_p") {
        for r in load(&f) {
            let (p, seed, l) = (r.u("p"), r.u("seed"), r.u("l"));
            let ops = r.get("ops").unwrap();
            let (mul, sq, inv) = (ops.f("mul"), ops.f("sq"), ops.f("inv"));
            let harness = mul * 55.0 + sq * 55.0 + inv * 166.0;
            let tuned = mul * 24.0 + sq * 18.0 + inv * 62.0;
            let native = ops.f("canon_cmp") + ops.f("hash") + ops.f("dp_check") + ops.f("scalar_ops") + ops.f("table_ops") + ops.f("cycle_cmp");
            let sl = (l as f64).sqrt();
            let sh = r.f("muls_harness") / (HARNESS_ADD * sl);
            let steps = r.f("steps");
            println!(
                "p={} seed={} run={} steps={} S_harness={:.6} rec={:.6} |d|={:.1e}; muls_harness/steps={:.2} (55,55,166)·ops/steps={:.2} match={}; muls_tuned/steps={:.2} (24,18,62)·ops/steps={:.2} match={}; s_tuned={:.4}; native/sqrt(l)={:.3}; correct={} capped={}",
                p, seed, r.u("run"), steps, sh, r.f("s_harness"), (sh - r.f("s_harness")).abs(),
                r.f("muls_harness") / steps, harness / steps, (harness - r.f("muls_harness")).abs() < 1.0,
                r.f("muls_tuned") / steps, tuned / steps, (tuned - r.f("muls_tuned")).abs() < 1.0,
                r.f("s_tuned"), native / sl, r.b("correct"), r.b("capped")
            );
            assert!(r.b("correct") && !r.b("capped"));
            strong.entry(p).or_default().entry(seed).or_default().push(sh);
            strong_tuned.entry(p).or_default().push(r.f("s_tuned"));
            strong_native.entry(p).or_default().push(native / sl);
        }
    }

    println!("\n== 5. per-prime ratios and bootstrap (frozen rule: percentile bootstrap, seeds × reference runs within instance) ==");
    let mut rng = Rng(0x5EED_2026_1101_0042);
    let mut point_rhodp: BTreeMap<u64, f64> = BTreeMap::new();
    for (p, cs) in &cov {
        let ss: Vec<f64> = cs.iter().filter(|c| c.ok).map(|c| c.s).collect();
        let gm = gmean(&ss);
        println!("p={p}: cover S per seed {:?} geo-mean {gm:.4}", ss.iter().map(|s| format!("{s:.4}")).collect::<Vec<_>>());
        for (name, arm) in [("rhodp", rhodp.get(p)), ("rhostrong", strong.get(p))] {
            let Some(arm) = arm else { continue };
            let all: Vec<f64> = arm.values().flatten().cloned().collect();
            let m = mean(&all);
            let point = gm / m;
            if name == "rhodp" {
                point_rhodp.insert(*p, m);
            }
            let per_seed: Vec<String> = ss.iter().map(|s| format!("{:.3}", s / m)).collect();
            // bootstrap, two sizes
            let mut res = vec![];
            for &nb in &[20_000usize, 1_000_000] {
                let mut bs = Vec::with_capacity(nb);
                for _ in 0..nb {
                    let g: Vec<f64> = (0..ss.len()).map(|_| ss[rng.idx(ss.len())]).collect();
                    let mut tot = 0.0;
                    let mut n = 0.0;
                    for v in arm.values() {
                        for _ in 0..v.len() {
                            tot += v[rng.idx(v.len())];
                            n += 1.0;
                        }
                    }
                    bs.push(gmean(&g) / (tot / n));
                }
                let lo = pct(&mut bs, 0.025);
                let hi = pct(&mut bs, 0.975);
                res.push((nb, lo, hi));
            }
            // normal-theory sensitivity on the log ratio (NOT the frozen rule)
            let ly: Vec<f64> = ss.iter().map(|s| s.ln()).collect();
            let v1 = var(&ly) / ly.len() as f64;
            let v2 = var(&all) / all.len() as f64 / (m * m);
            let df = (v1 + v2).powi(2) / (v1 * v1 / (ly.len() as f64 - 1.0) + v2 * v2 / (all.len() as f64 - 1.0));
            let tq = t_q(0.975, df);
            let (nlo, nhi) = ((point.ln() - tq * (v1 + v2).sqrt()).exp(), (point.ln() + tq * (v1 + v2).sqrt()).exp());
            let verdict = |lo: f64, hi: f64| if hi < 1.0 { "below" } else if lo > 1.0 { "above" } else { "not resolved" };
            println!(
                "  {name}: mean {m:.4} sd {:.4} n {} ({} instances); S/rho point {point:.4}; per-seed {:?}",
                sd(&all), all.len(), arm.len(), per_seed
            );
            for (nb, lo, hi) in &res {
                println!("    bootstrap {nb} resamples (own RNG): [{lo:.4}, {hi:.4}] -> {} (frozen rule)", verdict(*lo, *hi));
            }
            println!("    sensitivity (model, not the frozen rule): Welch-t interval on ln(S/rho), df {df:.1}, t {tq:.3}: [{nlo:.4}, {nhi:.4}] -> {}", verdict(nlo, nhi));
            // the seed direction alone: the 10 distinct seed multisets' geometric means over the fixed rho mean
            let mut gms = vec![];
            for i in 0..ss.len() {
                for j in i..ss.len() {
                    for k in j..ss.len() {
                        gms.push(gmean(&[ss[i], ss[j], ss[k]]) / m);
                    }
                }
            }
            gms.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!("    seed-direction support (10 multisets, rho fixed at its mean): min {:.4} max {:.4}", gms[0], gms[gms.len() - 1]);
        }
        if let Some(t) = strong_tuned.get(p) {
            let mt = mean(t);
            println!("  model (unit sensitivity, NOT the frozen rule): rhostrong s_tuned mean {mt:.4} -> S/rho_tuned = {:.3}; rhodp repriced at the measured tuned/harness step ratio {:.4}: {:.4} -> S/rho = {:.3}",
                gm / mt, TUNED_PER_HARNESS * 0.0 + 137.7 / 331.0, point_rhodp[p] * 137.7 / 331.0, gm / (point_rhodp[p] * 137.7 / 331.0));
        }
    }

    println!("\n== 6. calibration p=251: rhodp (study) vs rho_e (predecessor) ==");
    let mut cal: Vec<f64> = vec![];
    let mut cal_by_seed: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    let mut lag_s = vec![];
    for f in files(rdir, "calib_p251") {
        for r in load(&f) {
            assert!(r.b("correct"));
            let l = r.u("l");
            let s = r.u("group_ops") as f64 / (l as f64).sqrt();
            assert!((s - r.f("s")).abs() < 1e-9);
            cal.push(s);
            cal_by_seed.entry(r.u("seed")).or_default().push(s);
            lag_s.push(3.0 * 524288.0 / (l as f64).sqrt());
        }
    }
    let mut pred_rho_all: Vec<(f64, f64)> = vec![];
    let mut pred251: Vec<f64> = vec![];
    let mut pred_pts: Vec<(Cover, Vec<f64>)> = vec![];
    for f in files(pdir, "dlp_p") {
        for r in load(&f) {
            let c = cover(&r);
            let rho: Vec<f64> = r.get("rho").unwrap().arr().iter().map(|x| x.f("s")).collect();
            for &s in &rho {
                pred_rho_all.push((c.p as f64, s));
                if c.p == 251 {
                    pred251.push(s);
                }
            }
            pred_pts.push((c, rho));
        }
    }
    let pooled = mean(&pred_rho_all.iter().map(|x| x.1).collect::<Vec<_>>());
    let (m1, m2) = (mean(&cal), mean(&pred251));
    let (v1, v2) = (var(&cal) / cal.len() as f64, var(&pred251) / pred251.len() as f64);
    let df = (v1 + v2).powi(2) / (v1 * v1 / (cal.len() as f64 - 1.0) + v2 * v2 / (pred251.len() as f64 - 1.0));
    let tq = t_q(0.975, df);
    let d = m1 - m2;
    println!(
        "rhodp mean {m1:.4} sd {:.4} n {} (per seed: {:?}); rho_e mean {m2:.4} sd {:.4} n {}; diff {d:+.4}; Welch df {df:.2}; t.975 {tq:.4} (own incomplete-beta inversion); CI [{:+.4}, {:+.4}]; contains 0: {}; |diff|<0.3: {}",
        sd(&cal), cal.len(), cal_by_seed.iter().map(|(k, v)| format!("seed {k}: {:.3} (n {})", mean(v), v.len())).collect::<Vec<_>>(),
        sd(&pred251), pred251.len(), d - tq * (v1 + v2).sqrt(), d + tq * (v1 + v2).sqrt(), d - tq * (v1 + v2).sqrt() <= 0.0 && 0.0 <= d + tq * (v1 + v2).sqrt(), d.abs() < 0.3
    );
    println!("predecessor pooled rho_e S = {pooled:.4} (n={})", pred_rho_all.len());
    println!("derived: expected stop-poll lag of the 3 non-finding walkers (uniform in [0, 2^20) steps each, from rho_e_dp's 2^20 stop granularity) at p=251 = 3*2^19/sqrt(l) = {:.4} S units; at p=1301/1451/1601 = {:.4}/{:.4}/{:.4}",
        mean(&lag_s), 3.0 * 524288.0 / (1212282365425721801f64).sqrt(), 3.0 * 524288.0 / (2333159791230987941f64).sqrt(), 3.0 * 524288.0 / (4210057237981759961f64).sqrt());
    println!("derived: calibration difference net of that lag = {:+.4}", d - mean(&lag_s));
    println!("derived: DP detection latency, 4 walkers x 2^dp_bits steps: p=251 {:.4}; p=1301 {:.4}; p=1601 {:.4} S units",
        4.0 * 4096.0 / (62514729418501f64).sqrt(), 4.0 * 262144.0 / (1212282365425721801f64).sqrt(), 4.0 * 262144.0 / (4210057237981759961f64).sqrt());

    println!("\n== 7. refit variant A (ln(S/rho) on ln p; predecessor p<=251 own rho_e mean, 503/1009 pooled, new primes rhodp mean) ==");
    let mut pts: Vec<(f64, f64, u64)> = vec![]; // (p, S/rho, cluster)
    for (c, rho) in &pred_pts {
        if !c.ok {
            continue;
        }
        let r = if rho.is_empty() { pooled } else { mean(rho) };
        pts.push((c.p as f64, c.s / r, c.p));
    }
    let n_pred = pts.len();
    for (p, cs) in &cov {
        for c in cs.iter().filter(|c| c.ok) {
            pts.push((c.p as f64, c.s / point_rhodp[p], *p));
        }
    }
    let x: Vec<f64> = pts.iter().map(|q| q.0.ln()).collect();
    let y: Vec<f64> = pts.iter().map(|q| q.1.ln()).collect();
    let (ia, ib, se) = ols(&x, &y);
    let tq = t_q(0.975, (pts.len() - 2) as f64);
    println!(
        "n={} ({} predecessor + {} new) slope {ib:.4} se {se:.4} t-CI95 [{:.4}, {:.4}] intercept {ia:.4} p*=exp(-a/b)={:.1}; line at 1301/1451/1601: {:.4}/{:.4}/{:.4}",
        pts.len(), n_pred, pts.len() - n_pred, ib - tq * se, ib + tq * se, (-ia / ib).exp(),
        (ia + ib * 1301f64.ln()).exp(), (ia + ib * 1451f64.ln()).exp(), (ia + ib * 1601f64.ln()).exp()
    );
    // cluster bootstrap (seeds within prime; rhodp runs within instance at new primes), own RNG
    let mut by_p: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for (i, q) in pts.iter().enumerate() {
        by_p.entry(q.2).or_default().push(i);
    }
    let mut bs_b = vec![];
    let mut bs_ps = vec![];
    for _ in 0..20_000 {
        let mut xs = vec![];
        let mut ys = vec![];
        let mut rn: BTreeMap<u64, f64> = BTreeMap::new();
        for (p, arm) in &rhodp {
            let mut tot = 0.0;
            let mut n = 0.0;
            for v in arm.values() {
                for _ in 0..v.len() {
                    tot += v[rng.idx(v.len())];
                    n += 1.0;
                }
            }
            rn.insert(*p, tot / n);
        }
        for (p, v) in &by_p {
            for _ in 0..v.len() {
                let q = &pts[v[rng.idx(v.len())]];
                let ratio = if cov.contains_key(p) { q.1 * point_rhodp[p] / rn[p] } else { q.1 };
                xs.push(q.0.ln());
                ys.push(ratio.ln());
            }
        }
        let (a2, b2, _) = ols(&xs, &ys);
        if b2 < -0.05 {
            bs_b.push(b2);
            bs_ps.push((-a2 / b2).exp());
        }
    }
    println!(
        "cluster bootstrap (own RNG, 20000): slope [{:.4}, {:.4}], p* [{:.0}, {:.0}] ({} valid)",
        pct(&mut bs_b.clone(), 0.025), pct(&mut bs_b.clone(), 0.975), pct(&mut bs_ps.clone(), 0.025), pct(&mut bs_ps.clone(), 0.975), bs_b.len()
    );

    println!("\n== 8. section 5a native counts per sqrt(l) (reference arms only) ==");
    for p in cov.keys() {
        if let Some(v) = rhodp_native.get(p) {
            println!("p={p} rhodp native/sqrt(l) (derived: steps + steps/2^dp_bits) mean {:.4}", mean(v));
        }
        if let Some(v) = strong_native.get(p) {
            println!("p={p} rhostrong native/sqrt(l) (measured counters) mean {:.4}", mean(v));
        }
    }
}
