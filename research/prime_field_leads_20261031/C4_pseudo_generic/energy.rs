// C4: additive energy of F_B = {P : x(P) < B} in E(F_p) versus random subsets.
// Standalone (rustc energy.rs); no external crates. Counted work: curve additions.
// E(F) = #{(P1,P2,P3,P4) in F^4 : P1+P2 = P3+P4}, ordered, trivial solutions included.
use std::fmt::Write as _;
use std::collections::HashSet;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2;
    while d * d <= n { if n % d == 0 { return false; } d += 1; }
    true
}
fn pw(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1; b %= p;
    while e > 0 { if e & 1 == 1 { r = r * b % p; } b = b * b % p; e >>= 1; }
    r
}

type Pt = (u32, u32); // affine; infinity encoded as (u32::MAX, u32::MAX)
const INF: Pt = (u32::MAX, u32::MAX);

struct Curve { p: u64, a: u64, b: u64, adds: u64 }
impl Curve {
    fn add(&mut self, p1: Pt, p2: Pt) -> Pt {
        self.adds += 1;
        let p = self.p;
        if p1 == INF { return p2; }
        if p2 == INF { return p1; }
        let (x1, y1) = (p1.0 as u64, p1.1 as u64);
        let (x2, y2) = (p2.0 as u64, p2.1 as u64);
        let lam;
        if x1 == x2 {
            if (y1 + y2) % p == 0 { return INF; }
            lam = (3 * x1 % p * x1 + self.a) % p * pw(2 * y1 % p, p - 2, p) % p;
        } else {
            lam = (y2 + p - y1) % p * pw((x2 + p - x1) % p, p - 2, p) % p;
        }
        let x3 = (lam * lam % p + 2 * p - x1 - x2) % p;
        let y3 = (lam * ((x1 + p - x3) % p) % p + p - y1) % p;
        (x3 as u32, y3 as u32)
    }
}

fn energy(c: &mut Curve, f: &[Pt]) -> u64 {
    let mut sums: Vec<u64> = Vec::with_capacity(f.len() * f.len());
    for &u in f { for &v in f {
        let s = c.add(u, v);
        sums.push(((s.0 as u64) << 32) | s.1 as u64);
    } }
    sums.sort_unstable();
    let mut e = 0u64; let mut i = 0;
    while i < sums.len() {
        let mut j = i; while j < sums.len() && sums[j] == sums[i] { j += 1; }
        let r = (j - i) as u64; e += r * r; i = j;
    }
    e
}

fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64; let m = v.iter().sum::<f64>() / n;
    let var = if v.len() > 1 { v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0) } else { 0.0 };
    (m, var.sqrt())
}

fn main() {
    let ks: Vec<u32> = (12..=18).collect();
    let ncurves = 8; let nrand = 8;
    let mut rng = Rng(0xC4C4_2026_1031);
    let mut csv = String::new();
    writeln!(csv, "k,p,curve,a,b,N,size_kind,m,B,E_F,E_rand_mean,E_rand_sd,E_sym_mean,E_sym_sd,ratio_rand,ratio_sym,E_exp_rand,E_exp_sym,nontriv_F,nontriv_rand_mean,nontriv_sym_mean,nontriv_ratio_sym,adds").unwrap();
    let mut total_adds = 0u64;
    // summary rows: (k, kind, mean_ratio_sym, sd, mean_ratio_rand, sd, mean_nt_ratio_sym, sd, null_sd)
    let mut summ: Vec<(u32, u8, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
    for &k in &ks {
        let mut p = (1u64 << k) - 1; while !is_prime(p) { p -= 1; }
        // square-root table
        let mut sqrt_tab = vec![u32::MAX; p as usize];
        for y in 0..p { sqrt_tab[(y * y % p) as usize] = y as u32; }
        let mut per: Vec<Vec<(f64, f64, f64, f64)>> = vec![vec![], vec![]];
        let mut cfound = 0; let mut tries = 0;
        while cfound < ncurves {
            tries += 1;
            let a = rng.below(p); let b = rng.below(p);
            if (4 * pw(a, 3, p) % p + 27 * b % p * b) % p == 0 { continue; }
            // enumerate points
            let mut pts: Vec<Pt> = Vec::new();
            for x in 0..p {
                let rhs = (x * x % p * x + a * x + b) % p;
                let y = sqrt_tab[rhs as usize];
                if y == u32::MAX { continue; }
                if y == 0 { pts.push((x as u32, 0)); } else {
                    pts.push((x as u32, y)); pts.push((x as u32, (p - y as u64) as u32));
                }
            }
            let n = pts.len() as u64 + 1;
            if !is_prime(n) || n == p { continue; } // prime-order, non-anomalous only
            cfound += 1;
            // pts already sorted by x (then y asc/desc consistently)
            let mut c = Curve { p, a, b, adds: 0 };
            for (kind, m_raw) in [(0u8, (n as f64).sqrt().round() as usize), (1u8, (n as f64).cbrt().round() as usize)] {
                let m = 2 * ((m_raw + 1) / 2); // even, so F_B and symmetric null are both negation-closed
                let bnd = pts[m - 1].0 as u64 + 1; // B: all of first m points have x < B
                let ff: Vec<Pt> = pts[..m].to_vec();
                let ef = energy(&mut c, &ff);
                // pairs (x-groups) for symmetric null: points come as consecutive (x,y),(x,-y)
                let npairs = pts.len() / 2;
                let mut er = Vec::new(); let mut es = Vec::new();
                for _ in 0..nrand {
                    // uniform random m-subset of affine points (partial Fisher-Yates)
                    let mut idx: Vec<usize> = (0..pts.len()).collect();
                    for i in 0..m { let j = i + rng.below((idx.len() - i) as u64) as usize; idx.swap(i, j); }
                    let s: Vec<Pt> = idx[..m].iter().map(|&i| pts[i]).collect();
                    er.push(energy(&mut c, &s) as f64);
                    // negation-symmetric random subset of the same size
                    let mut seen: HashSet<usize> = HashSet::new();
                    let mut s2: Vec<Pt> = Vec::new();
                    while seen.len() < m / 2 { let q = rng.below(npairs as u64) as usize; if seen.insert(q) { s2.push(pts[2 * q]); s2.push(pts[2 * q + 1]); } }
                    es.push(energy(&mut c, &s2) as f64);
                }
                let (mr, sr) = mean_sd(&er); let (ms, ss) = mean_sd(&es);
                let mf = m as f64; let nf = n as f64;
                let e_rand = 2.0 * mf * mf - mf + mf * (mf - 1.0) * (mf - 2.0) * (mf - 3.0) / nf;
                let e_sym = e_rand + mf * mf - 2.0 * mf; // + P1=-P2,P3=-P4 trivial class (approx, overlaps removed)
                // "non-trivial" part: subtract the exact trivial count 3m^2-2m for symmetric sets (approx; see REPORT)
                let triv = 3.0 * mf * mf - 2.0 * mf;
                let nt_f = ef as f64 - triv; let nt_r = mr - (2.0 * mf * mf - mf); let nt_s = ms - triv;
                let rs = ef as f64 / ms; let rr = ef as f64 / mr;
                let nts = if nt_s > 0.0 { nt_f / nt_s } else { f64::NAN };
                writeln!(csv, "{k},{p},{},{a},{b},{n},{},{m},{bnd},{ef},{mr:.2},{sr:.2},{ms:.2},{ss:.2},{rr:.5},{rs:.5},{e_rand:.2},{e_sym:.2},{nt_f:.1},{nt_r:.1},{nt_s:.1},{nts:.5},{}",
                    cfound, if kind == 0 { "sqrtN" } else { "cbrtN" }, c.adds).unwrap();
                per[kind as usize].push((rs, rr, nts, ss / ms));
            }
            total_adds += c.adds;
        }
        for kind in 0..2 {
            let rs: Vec<f64> = per[kind].iter().map(|t| t.0).collect();
            let rr: Vec<f64> = per[kind].iter().map(|t| t.1).collect();
            let nt: Vec<f64> = per[kind].iter().map(|t| t.2).collect();
            let ns: Vec<f64> = per[kind].iter().map(|t| t.3).collect();
            let (a1, s1) = mean_sd(&rs); let (a2, s2) = mean_sd(&rr); let (a3, s3) = mean_sd(&nt); let (a4, _) = mean_sd(&ns);
            summ.push((k, kind as u8, a1, s1, a2, s2, a3, s3, a4));
        }
        eprintln!("k={k} p={p} done, curve candidates tried={tries}, adds so far={total_adds}");
    }
    std::fs::write("data_per_curve.csv", &csv).unwrap();
    let mut sc = String::from("k,size_kind,ratio_sym_mean,ratio_sym_sd_over_curves,ratio_rand_mean,ratio_rand_sd,nontriv_ratio_sym_mean,nontriv_ratio_sym_sd,null_rel_sd_within_curve\n");
    for t in &summ { writeln!(sc, "{},{},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5}", t.0, if t.1 == 0 { "sqrtN" } else { "cbrtN" }, t.2, t.3, t.4, t.5, t.6, t.7, t.8).unwrap(); }
    std::fs::write("data_summary.csv", &sc).unwrap();
    eprintln!("total curve additions = {total_adds}");
    // SVG: two panels' worth in one plot, ratio vs log2 p, error bars = sd over 8 curves
    let (w, h, ml, mb, mt) = (720.0, 460.0, 70.0, 60.0, 40.0);
    let mut svg = String::new();
    let (ymin, ymax) = (0.6f64, 1.8f64);
    let xs = |k: f64| ml + (k - 11.5) / (18.5 - 11.5) * (w - ml - 20.0);
    let ys = |v: f64| mt + (ymax - v) / (ymax - ymin) * (h - mt - mb);
    writeln!(svg, "<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='12'><rect width='{w}' height='{h}' fill='white'/>").unwrap();
    writeln!(svg, "<text x='{ml}' y='22' font-size='14'>C4: E(F_B) / E(negation-symmetric random subset), mean +/- sd over 8 curves</text>").unwrap();
    for i in 0..=6 { let v = ymin + i as f64 * 0.2; writeln!(svg, "<line x1='{ml}' x2='{}' y1='{y}' y2='{y}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{v:.1}</text>", w - 20.0, ml - 6.0, y = ys(v) + 4.0).unwrap(); }
    writeln!(svg, "<line x1='{ml}' x2='{}' y1='{y}' y2='{y}' stroke='black' stroke-dasharray='4'/>", w - 20.0, y = ys(1.0)).unwrap();
    for k in 12..=18 { writeln!(svg, "<text x='{}' y='{}' text-anchor='middle'>{k}</text>", xs(k as f64), h - mb + 18.0).unwrap(); }
    writeln!(svg, "<text x='{}' y='{}' text-anchor='middle'>log2 p</text><text x='16' y='{}' transform='rotate(-90 16 {})' text-anchor='middle'>energy ratio</text>", (ml + w) / 2.0, h - 12.0, h / 2.0, h / 2.0).unwrap();
    for (kind, col, dx) in [(0u8, "#1f77b4", -0.06), (1u8, "#d95f02", 0.06)] {
        let mut path = String::new();
        for t in summ.iter().filter(|t| t.1 == kind) {
            let x = xs(t.0 as f64 + dx); let y = ys(t.2);
            write!(path, "{}{x:.1},{y:.1} ", if path.is_empty() { "M" } else { "L" }).unwrap();
            writeln!(svg, "<line x1='{x}' x2='{x}' y1='{}' y2='{}' stroke='{col}'/><circle cx='{x}' cy='{y}' r='3.5' fill='{col}'/>", ys(t.2 - t.3), ys(t.2 + t.3)).unwrap();
        }
        writeln!(svg, "<path d='{path}' fill='none' stroke='{col}' stroke-width='1' opacity='0.5'/>").unwrap();
    }
    writeln!(svg, "<rect x='{}' y='{}' width='10' height='10' fill='#1f77b4'/><text x='{}' y='{}'>|F_B| = round(sqrt N)</text><rect x='{}' y='{}' width='10' height='10' fill='#d95f02'/><text x='{}' y='{}'>|F_B| = round(N^(1/3))</text>", ml + 10.0, h - mb - 40.0, ml + 26.0, h - mb - 31.0, ml + 10.0, h - mb - 24.0, ml + 26.0, h - mb - 15.0).unwrap();
    svg.push_str("</svg>\n");
    std::fs::write("graph_energy_ratio.svg", svg).unwrap();
}
