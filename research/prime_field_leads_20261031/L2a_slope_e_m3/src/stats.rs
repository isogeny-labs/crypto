//! Fit + bootstrap + SVG for L2a. Reads attempts.csv (recorded counted values only).
//! Statistic (frozen before data): per s, log2 of the mean work (mults+decisions) over the
//! attempts of the class; OLS slope e of that against s; hierarchical bootstrap
//! (resample the 3 curves with replacement, then attempts within each drawn curve),
//! 4000 replicates, percentile 95% interval.
use std::collections::BTreeMap;
use std::io::Write;
struct R { s: u32, curve: usize, arm: char, sat: i32, pairs: u64, mults: u64, work: u64, ok: bool }
struct Rng(u64);
impl Rng { fn next(&mut self) -> u64 { self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15); let mut z = self.0; z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9); z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB); z ^ (z >> 31) }
  fn below(&mut self, n: usize) -> usize { (self.next() % n as u64) as usize } }
fn ols(xs: &[f64], ys: &[f64]) -> f64 {
    let n = xs.len() as f64; let mx = xs.iter().sum::<f64>() / n; let my = ys.iter().sum::<f64>() / n;
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum(); sxy / sxx
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let txt = std::fs::read_to_string(&a[1]).unwrap();
    let mut rows = vec![];
    for l in txt.lines().skip(1) {
        let c: Vec<&str> = l.split(',').collect();
        rows.push(R { s: c[0].parse().unwrap(), curve: c[2].parse().unwrap(), arm: c[4].chars().next().unwrap(), sat: c[5].parse().unwrap(),
            pairs: c[6].parse().unwrap(), mults: c[7].parse().unwrap(), work: c[9].parse().unwrap(), ok: c[10] == "OK" });
    }
    let outdir = &a[2];
    let mut rng = Rng(20261031);
    let mut fit = std::fs::File::create(format!("{}/fit.csv", outdir)).unwrap();
    writeln!(fit, "arm,class,n_s_points,e,ci_lo,ci_hi,reps_used").unwrap();
    let mut pts = std::fs::File::create(format!("{}/per_s_means.csv", outdir)).unwrap();
    writeln!(pts, "arm,class,s,n_attempts,mean_work,log2_mean_work,mean_pairs,mean_mults_per_pair").unwrap();
    for arm in ['G', 'S', 'T'] {
        for class in ["ALL", "SAT", "UNSAT"] {
            let sel = |r: &&R| r.arm == arm && r.ok && match class { "SAT" => r.sat == 1, "UNSAT" => r.sat == 0, _ => true };
            let sset: Vec<u32> = { let mut v: Vec<u32> = rows.iter().filter(sel).map(|r| r.s).collect(); v.sort(); v.dedup(); v };
            if sset.len() < 2 { continue; }
            // by s -> by curve -> work list
            let mut data: BTreeMap<u32, Vec<Vec<f64>>> = BTreeMap::new();
            for &s in &sset { data.insert(s, vec![vec![]; 3]); }
            for r in rows.iter().filter(sel) { data.get_mut(&r.s).unwrap()[r.curve].push(r.work as f64); }
            let mut xs = vec![]; let mut ys = vec![];
            for &s in &sset {
                let all: Vec<&R> = rows.iter().filter(sel).filter(|r| r.s == s).collect();
                let n = all.len() as f64; let mw = all.iter().map(|r| r.work as f64).sum::<f64>() / n;
                let mp = all.iter().map(|r| r.pairs as f64).sum::<f64>() / n;
                let mm = all.iter().map(|r| r.mults as f64).sum::<f64>() / n;
                xs.push(s as f64); ys.push(mw.log2());
                writeln!(pts, "{},{},{},{},{:.3},{:.6},{:.3},{:.3}", arm, class, s, all.len(), mw, mw.log2(), mp, mm / mp).unwrap();
            }
            let e = ols(&xs, &ys);
            let mut es = vec![];
            for _ in 0..4000 {
                let mut yb = vec![]; let mut ok = true;
                for &s in &sset {
                    let cs = &data[&s]; let mut tot = 0.0; let mut cnt = 0usize;
                    let nonempty: Vec<usize> = (0..3).filter(|&c| !cs[c].is_empty()).collect();
                    if nonempty.is_empty() { ok = false; break; }
                    for _ in 0..3 {
                        // draw a curve among the 3; empty ones contribute nothing (class absent there)
                        let c = rng.below(3); let v = &cs[c];
                        for _ in 0..v.len() { tot += v[rng.below(v.len())]; cnt += 1; }
                    }
                    if cnt == 0 { ok = false; break; }
                    yb.push((tot / cnt as f64).log2());
                }
                if ok { es.push(ols(&xs, &yb)); }
            }
            es.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let (lo, hi) = (es[(es.len() as f64 * 0.025) as usize], es[(es.len() as f64 * 0.975) as usize]);
            writeln!(fit, "{},{},{},{:.4},{:.4},{:.4},{}", arm, class, sset.len(), e, lo, hi, es.len()).unwrap();
        }
    }
    // SVG: log2(work) vs s for arms G,S,T ; ALL class, with OLS line
    let mut series: Vec<(char, Vec<(f64, f64)>)> = vec![];
    for line in std::fs::read_to_string(format!("{}/per_s_means.csv", outdir)).unwrap().lines().skip(1) {
        let c: Vec<&str> = line.split(',').collect();
        if c[1] != "ALL" { continue; }
        let arm = c[0].chars().next().unwrap();
        let pt = (c[2].parse::<f64>().unwrap(), c[5].parse::<f64>().unwrap());
        if let Some(e) = series.iter_mut().find(|e| e.0 == arm) { e.1.push(pt); } else { series.push((arm, vec![pt])); }
    }
    let (w, h, ml, mb, mt) = (760.0, 520.0, 70.0, 60.0, 50.0);
    let xmin = 5.5; let xmax = 12.5;
    let ymax = series.iter().flat_map(|s| s.1.iter().map(|p| p.1)).fold(0.0, f64::max).ceil() + 1.0;
    let ymin = series.iter().flat_map(|s| s.1.iter().map(|p| p.1)).fold(1e9, f64::min).floor() - 1.0;
    let px = |x: f64| ml + (x - xmin) / (xmax - xmin) * (w - ml - 150.0);
    let py = |y: f64| h - mb - (y - ymin) / (ymax - ymin) * (h - mb - mt);
    let mut svg = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' viewBox='0 0 {w} {h}' font-family='Helvetica,Arial,sans-serif' font-size='12'>\n<rect width='100%' height='100%' fill='white'/>\n");
    svg += &format!("<text x='{ml}' y='24' font-size='15' font-weight='bold'>L2a: counted work (field mults + decisions) per attempt vs s, m=3, p ~ 2^(3s)</text>\n<text x='{ml}' y='40' fill='#444'>log2 of mean over 96 attempts per s (3 curves x 32 targets); line = OLS fit; slope e in legend</text>\n");
    let mut yt = ymin.ceil(); while yt <= ymax { svg += &format!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='#ddd'/><text x='{}' y='{}' text-anchor='end'>{}</text>\n", ml, w - 150.0, py(yt), py(yt), ml - 6.0, py(yt) + 4.0, yt); yt += 2.0; }
    for s in 6..=12 { svg += &format!("<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='#eee'/><text x='{}' y='{}' text-anchor='middle'>{}</text>\n", px(s as f64), px(s as f64), mt, h - mb, px(s as f64), h - mb + 18.0, s); }
    svg += &format!("<text x='{}' y='{}' text-anchor='middle'>s = bits per window (B = 2^s, p ~ 2^(3s))</text>\n<text transform='translate(18,{}) rotate(-90)' text-anchor='middle'>log2( mean counted work per attempt )</text>\n", (ml + w - 150.0) / 2.0, h - 14.0, (mt + h - mb) / 2.0);
    let fits: Vec<String> = std::fs::read_to_string(format!("{}/fit.csv", outdir)).unwrap().lines().skip(1).map(|s| s.to_string()).collect();
    let cols = [('G', "#1b6ca8", "G: group MITM"), ('S', "#c4501b", "S: S4 pair + root find"), ('T', "#2a7f3f", "T: triple scan")];
    for (i, (arm, col, name)) in cols.iter().enumerate() {
        let Some(sr) = series.iter().find(|s| s.0 == *arm) else { continue };
        let fl = fits.iter().find(|l| l.starts_with(&format!("{},ALL,", arm))).unwrap(); let fc: Vec<&str> = fl.split(',').collect();
        let e: f64 = fc[3].parse().unwrap();
        let n = sr.1.len() as f64; let mx = sr.1.iter().map(|p| p.0).sum::<f64>() / n; let my = sr.1.iter().map(|p| p.1).sum::<f64>() / n;
        let (x0, x1) = (sr.1.first().unwrap().0, sr.1.last().unwrap().0);
        svg += &format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{}' stroke-dasharray='5 3'/>\n", px(x0), py(my + e * (x0 - mx)), px(x1), py(my + e * (x1 - mx)), col);
        for p in &sr.1 { svg += &format!("<circle cx='{}' cy='{}' r='4' fill='{}'/>\n", px(p.0), py(p.1), col); }
        let ly = mt + 20.0 + i as f64 * 44.0;
        svg += &format!("<circle cx='{}' cy='{}' r='4' fill='{}'/><text x='{}' y='{}'>{}</text><text x='{}' y='{}' fill='#444'>e={:.3} [{:.3},{:.3}]</text>\n", w - 140.0, ly, col, w - 130.0, ly + 4.0, name, w - 130.0, ly + 19.0, e, fc[4].parse::<f64>().unwrap(), fc[5].parse::<f64>().unwrap());
    }
    svg += "</svg>\n";
    std::fs::write(format!("{}/L2a_work_vs_s.svg", outdir), svg).unwrap();
}
