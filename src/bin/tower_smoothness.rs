//! Measure the probability that a near-uniform reduced divisor on a genus-g
//! Jacobian is smooth over the degree-1 places (u splits into linear factors
//! over F_p, repeated roots allowed, deg u < g allowed), the criterion of
//! `ecbench/hyper_ic.rs` (`Fp::split` returns `Some`).
//!
//! Usage: tower_smoothness [--json] [--svg PATH] [--n N] [--curves C] [--seed S]
use crypto_lib::cryptanalysis::hyperelliptic_word::{HCurve, HDiv};
use std::fmt::Write as _;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const PRIMES: [u64; 5] = [53, 101, 151, 211, 251];
const GENERA: [u32; 4] = [1, 2, 3, 4];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// u (monic, implicit leading 1) splits into linear factors over F_p,
/// counting multiplicity; deg 0 is smooth.
fn splits(d: &HDiv, p: u64) -> bool {
    let mut poly: Vec<u64> = d.u_coeffs(); // lowest first, leading 1
    for x in 0..p {
        loop {
            if poly.len() <= 1 {
                return true;
            }
            // evaluate and synthetic-divide by (X - x)
            let n = poly.len();
            let mut q = vec![0u64; n - 1];
            let mut acc = 0u64;
            for i in (0..n).rev() {
                acc = (acc * x + poly[i]) % p;
                if i > 0 {
                    q[i - 1] = acc;
                }
            }
            if acc != 0 {
                break;
            }
            poly = q;
        }
    }
    poly.len() <= 1
}

#[derive(Clone, Default)]
struct Stat {
    n: u64,
    deg: [u64; 5],
    smooth: u64,
    smooth_full: u64,
}

struct CurveRes {
    g: u32,
    p: u64,
    idx: usize,
    f: Vec<u64>,
    stat: Stat,
}

fn random_curve(g: u32, p: u64, rng: &mut Rng) -> (HCurve, Vec<u64>) {
    loop {
        let mut f: Vec<u64> = (0..=2 * g as usize).map(|_| rng.next() % p).collect();
        f.push(1 + rng.next() % (p - 1));
        if let Ok(c) = HCurve::new(p, g, &f) {
            return (c, f);
        }
    }
}

fn run_curve(g: u32, p: u64, idx: usize, n: u64, seed: u64) -> CurveRes {
    let mut rng = Rng(seed ^ (u64::from(g) << 56) ^ (p << 40) ^ ((idx as u64) << 24));
    let (c, f) = random_curve(g, p, &mut rng);
    let mut st = Stat::default();
    let mut ds = rng.next();
    for _ in 0..n {
        // fresh random divisor, then a random 48-bit scalar (the Jacobian
        // has order < 2^33 here, so the product is near-uniform on <D>).
        let base = c.random_divisor(&mut ds);
        let k = rng.next() >> 16;
        let d = c.mul_uncounted(base, k);
        st.n += 1;
        st.deg[d.deg as usize] += 1;
        if splits(&d, p) {
            st.smooth += 1;
            if u32::from(d.deg) == g {
                st.smooth_full += 1;
            }
        }
    }
    CurveRes {
        g,
        p,
        idx,
        f,
        stat: st,
    }
}

fn wilson(k: u64, n: u64) -> (f64, f64, f64) {
    if n == 0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let z = 1.959_963_984_540_054_f64;
    let nf = n as f64;
    let ph = k as f64 / nf;
    let den = 1.0 + z * z / nf;
    let c = (ph + z * z / (2.0 * nf)) / den;
    let h = z * (ph * (1.0 - ph) / nf + z * z / (4.0 * nf * nf)).sqrt() / den;
    (ph, (c - h).max(0.0), (c + h).min(1.0))
}

fn fact(g: u32) -> f64 {
    (1..=g).map(f64::from).product()
}

/// Exact fraction of monic degree-g polynomials over F_p that split:
/// multisets of size g from p roots over p^g.
fn exact_split(g: u32, p: u64) -> f64 {
    let mut num = 1.0;
    for i in 0..g {
        num *= (p + u64::from(i)) as f64 / f64::from(i + 1);
    }
    num / (p as f64).powi(g as i32)
}

struct Cell {
    g: u32,
    p: u64,
    curves: Vec<usize>, // indices into results
    st: Stat,
}

fn svg(cells: &[Cell]) -> String {
    let (w, h, ml, mr, mt, mb) = (760.0, 460.0, 70.0, 130.0, 30.0, 60.0);
    let (ymin, ymax) = (-2.0f64, 0.1f64); // log10
    let px = |g: f64| ml + (g - 0.7) / 3.6 * (w - ml - mr);
    let py = |v: f64| mt + (ymax - v.log10()) / (ymax - ymin) * (h - mt - mb);
    let cols = ["#1b6ca8", "#d9822b", "#2e8b57", "#8e44ad", "#c0392b"];
    let mut s = String::new();
    let _ = write!(s, "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 {w} {h}' font-family='sans-serif' font-size='12'><rect width='{w}' height='{h}' fill='white'/>");
    let _ = write!(s, "<text x='{}' y='18' font-size='14'>Smooth fraction of a near-uniform reduced divisor vs genus (log scale)</text>", ml);
    for e in [-2.0f64, -1.0, 0.0] {
        let y = py(10f64.powf(e));
        let _ = write!(s, "<line x1='{ml}' x2='{}' y1='{y}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>1e{}</text>", w - mr, ml - 6.0, y + 4.0, e as i32);
    }
    for g in GENERA {
        let x = px(f64::from(g));
        let _ = write!(s, "<line x1='{x}' x2='{x}' y1='{mt}' y2='{}' stroke='#eee'/><text x='{x}' y='{}' text-anchor='middle'>g={g}</text>", h - mb, h - mb + 18.0);
    }
    let _ = write!(s, "<text x='{}' y='{}' text-anchor='middle'>genus g</text><text x='16' y='{}' transform='rotate(-90 16 {})' text-anchor='middle'>fraction smooth (95% Wilson, pooled)</text>", (ml + w - mr) / 2.0, h - 14.0, h / 2.0, h / 2.0);
    // 1/g! line
    let pts: Vec<String> = GENERA
        .iter()
        .map(|&g| format!("{},{}", px(f64::from(g)), py(1.0 / fact(g))))
        .collect();
    let _ = write!(s, "<polyline points='{}' fill='none' stroke='black' stroke-width='2' stroke-dasharray='6 4'/>", pts.join(" "));
    let _ = write!(
        s,
        "<text x='{}' y='{}'>1/g!</text>",
        w - mr + 10.0,
        mt + 14.0
    );
    for (i, &p) in PRIMES.iter().enumerate() {
        let col = cols[i];
        let mut pts = Vec::new();
        for &g in &GENERA {
            if let Some(c) = cells.iter().find(|c| c.g == g && c.p == p) {
                let (ph, lo, hi) = wilson(c.st.smooth, c.st.n);
                let x = px(f64::from(g)) + (i as f64 - 2.0) * 6.0;
                pts.push(format!("{x},{}", py(ph)));
                let _ = write!(s, "<line x1='{x}' x2='{x}' y1='{}' y2='{}' stroke='{col}'/><circle cx='{x}' cy='{}' r='3.5' fill='{col}'/>", py(lo.max(1e-9)), py(hi), py(ph));
            }
        }
        let _ = write!(
            s,
            "<polyline points='{}' fill='none' stroke='{col}' stroke-width='1'/>",
            pts.join(" ")
        );
        let _ = write!(s, "<rect x='{}' y='{}' width='10' height='10' fill='{col}'/><text x='{}' y='{}'>p={p}</text>", w - mr + 10.0, mt + 30.0 + 18.0 * i as f64, w - mr + 26.0, mt + 39.0 + 18.0 * i as f64);
    }
    s.push_str("</svg>\n");
    s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |k: &str| args.iter().any(|a| a == k);
    let val = |k: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let n: u64 = val("--n").and_then(|v| v.parse().ok()).unwrap_or(20_000);
    let ncur: usize = val("--curves").and_then(|v| v.parse().ok()).unwrap_or(8);
    let seed: u64 = val("--seed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(0x544F_5745_5231);
    let mut jobs = Vec::new();
    for &g in &GENERA {
        for &p in &PRIMES {
            for i in 0..ncur {
                jobs.push((g, p, i));
            }
        }
    }
    let next = AtomicUsize::new(0);
    let out = Mutex::new(Vec::new());
    let th = std::thread::available_parallelism().map_or(2, |x| x.get());
    std::thread::scope(|sc| {
        for _ in 0..th {
            sc.spawn(|| loop {
                let j = next.fetch_add(1, Ordering::SeqCst);
                if j >= jobs.len() {
                    break;
                }
                let (g, p, i) = jobs[j];
                let r = run_curve(g, p, i, n, seed);
                out.lock().unwrap().push(r);
            });
        }
    });
    let mut res = out.into_inner().unwrap();
    res.sort_by_key(|r| (r.g, r.p, r.idx));
    let mut cells = Vec::new();
    for &g in &GENERA {
        for &p in &PRIMES {
            let mut c = Cell {
                g,
                p,
                curves: vec![],
                st: Stat::default(),
            };
            for (i, r) in res.iter().enumerate() {
                if r.g == g && r.p == p {
                    c.curves.push(i);
                    c.st.n += r.stat.n;
                    c.st.smooth += r.stat.smooth;
                    c.st.smooth_full += r.stat.smooth_full;
                    for d in 0..5 {
                        c.st.deg[d] += r.stat.deg[d];
                    }
                }
            }
            cells.push(c);
        }
    }
    if let Some(path) = val("--svg") {
        std::fs::write(&path, svg(&cells)).expect("write svg");
    }
    // JSON
    let mut j = String::new();
    let _ = write!(j, "{{\"schema\":\"tower_smoothness/v1\",\"n_per_curve\":{n},\"curves_per_cell\":{ncur},\"seed\":{seed},\"cells\":[");
    for (ci, c) in cells.iter().enumerate() {
        let g = c.g as usize;
        let (a, al, ah) = wilson(c.st.smooth, c.st.n);
        let (b, bl, bh) = wilson(c.st.deg[g], c.st.n);
        let (d, dl, dh) = wilson(c.st.smooth_full, c.st.deg[g]);
        let fr: Vec<f64> = c
            .curves
            .iter()
            .map(|&i| res[i].stat.smooth as f64 / res[i].stat.n as f64)
            .collect();
        let frf: Vec<f64> = c
            .curves
            .iter()
            .map(|&i| {
                let s = &res[i].stat;
                s.smooth_full as f64 / s.deg[g].max(1) as f64
            })
            .collect();
        let mn = |v: &[f64]| v.iter().cloned().fold(f64::INFINITY, f64::min);
        let mx = |v: &[f64]| v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        // Curve-level (cluster) interval: t(0.975, m-1) on per-curve fractions.
        let m = fr.len() as f64;
        let mean = fr.iter().sum::<f64>() / m;
        let sd = (fr.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (m - 1.0)).sqrt();
        let tq = [
            0.0, 0.0, 12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262,
        ];
        let hw = tq.get(fr.len()).copied().unwrap_or(1.96) * sd / m.sqrt();
        if ci > 0 {
            j.push(',');
        }
        let cluster = format!("[{},{},{}]", mean, mean - hw, mean + hw);
        let fs: Vec<String> = c
            .curves
            .iter()
            .map(|&i| format!("{:?}", res[i].f))
            .collect();
        let _ = write!(j, "{{\"g\":{},\"p\":{},\"n\":{},\"deg_counts\":{:?},\"frac_deg_g\":[{b},{bl},{bh}],\"smooth_overall\":[{a},{al},{ah}],\"smooth_given_deg_g\":[{d},{dl},{dh}],\"curve_mean_t95\":{cluster},\"per_curve_overall_min_max\":[{},{}],\"per_curve_full_min_max\":[{},{}],\"exact_split_multiset\":{},\"inv_g_fact\":{},\"f\":[{}]}}",
            c.g, c.p, c.st.n, &c.st.deg[..=g], mn(&fr), mx(&fr), mn(&frf), mx(&frf), exact_split(c.g, c.p), 1.0 / fact(c.g), fs.join(","));
    }
    j.push_str("],\"ratios\":[");
    let mut first = true;
    for &p in &PRIMES {
        for g in 1..4u32 {
            let c0 = cells.iter().find(|c| c.g == g && c.p == p).unwrap();
            let c1 = cells.iter().find(|c| c.g == g + 1 && c.p == p).unwrap();
            let (a0, l0, h0) = wilson(c0.st.smooth, c0.st.n);
            let (a1, l1, h1) = wilson(c1.st.smooth, c1.st.n);
            if !first {
                j.push(',');
            }
            first = false;
            let _ = write!(
                j,
                "{{\"p\":{p},\"g\":{g},\"trials_ratio\":[{},{},{}]}}",
                a0 / a1,
                l0 / h1,
                h0 / l1
            );
        }
    }
    j.push_str("]}\n");
    if flag("--json") {
        print!("{j}");
    }
    // human table
    eprintln!("g    p    n   frac_deg_g  smooth_all [lo,hi]  smooth|deg_g [lo,hi]  1/g!  exact  curve-mean [t95 lo,hi]");
    for c in &cells {
        let g = c.g as usize;
        let (a, al, ah) = wilson(c.st.smooth, c.st.n);
        let (b, _, _) = wilson(c.st.deg[g], c.st.n);
        let (d, dl, dh) = wilson(c.st.smooth_full, c.st.deg[g]);
        eprintln!(
            "{} {:4} {:7} {:.4}  {:.4} [{:.4},{:.4}]  {:.4} [{:.4},{:.4}]  {:.4} {:.4}",
            c.g,
            c.p,
            c.st.n,
            b,
            a,
            al,
            ah,
            d,
            dl,
            dh,
            1.0 / fact(c.g),
            exact_split(c.g, c.p)
        );
    }
}
