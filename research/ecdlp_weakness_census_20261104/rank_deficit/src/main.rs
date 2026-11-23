use std::collections::HashMap;
use std::fmt::Write as _;
use std::time::Instant;

type Pt = Option<(u64, u64)>;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn mulm(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
fn powm(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, b, m);
        }
        b = mulm(b, b, m);
        e >>= 1;
    }
    r
}
fn invm(a: u64, m: u64) -> u64 {
    powm(a, m - 2, m)
}

#[derive(Clone, Copy)]
struct Curve {
    p: u64,
    a: u64,
    b: u64,
}
impl Curve {
    fn neg(&self, q: Pt) -> Pt {
        q.map(|(x, y)| (x, (self.p - y) % self.p))
    }
    fn add(&self, q1: Pt, q2: Pt) -> Pt {
        let p = self.p;
        match (q1, q2) {
            (None, q) | (q, None) => q,
            (Some((x1, y1)), Some((x2, y2))) => {
                let lam;
                if x1 == x2 {
                    if (y1 + y2) % p == 0 {
                        return None;
                    }
                    let num = (3 * mulm(x1, x1, p) + self.a) % p;
                    lam = mulm(num, invm(2 * y1 % p, p), p);
                } else {
                    let num = (y2 + p - y1) % p;
                    let den = (x2 + p - x1) % p;
                    lam = mulm(num, invm(den, p), p);
                }
                let x3 = (mulm(lam, lam, p) + 2 * p - x1 - x2) % p;
                let y3 = (mulm(lam, (x1 + p - x3) % p, p) + p - y1) % p;
                Some((x3, y3))
            }
        }
    }
    fn mul(&self, mut k: u64, q: Pt) -> Pt {
        let mut acc = None;
        let mut base = q;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
    fn on(&self, x: u64, y: u64) -> bool {
        let p = self.p;
        mulm(y, y, p) == (mulm(mulm(x, x, p), x, p) + mulm(self.a, x, p) + self.b) % p
    }
}

fn count_points(c: &Curve) -> u64 {
    let p = c.p;
    let mut cnt = vec![0u8; p as usize];
    for y in 0..p {
        cnt[mulm(y, y, p) as usize] += 1;
    }
    let mut n = 1u64;
    for x in 0..p {
        let rhs = (mulm(mulm(x, x, p), x, p) + mulm(c.a, x, p) + c.b) % p;
        n += cnt[rhs as usize] as u64;
    }
    n
}

/// rank over Z/r of a dense matrix (rows of equal length), r prime
fn rank_mod(mut m: Vec<Vec<u64>>, r: u64) -> usize {
    let ncols = if m.is_empty() { 0 } else { m[0].len() };
    let nrows = m.len();
    let mut rk = 0;
    for col in 0..ncols {
        if rk == nrows {
            break;
        }
        let mut piv = None;
        for i in rk..nrows {
            if m[i][col] != 0 {
                piv = Some(i);
                break;
            }
        }
        let Some(pi) = piv else { continue };
        m.swap(rk, pi);
        let inv = invm(m[rk][col], r);
        for j in col..ncols {
            m[rk][j] = mulm(m[rk][j], inv, r);
        }
        let prow = m[rk].clone();
        for i in (rk + 1)..nrows {
            let f = m[i][col];
            if f != 0 {
                for j in col..ncols {
                    let t = mulm(f, prow[j], r);
                    m[i][j] = (m[i][j] + r - t) % r;
                }
            }
        }
        rk += 1;
    }
    rk
}

struct Row {
    p: u64,
    r: u64,
    a: u64,
    b: u64,
    curve: usize,
    bsize: usize,
    shape: &'static str,
    delta: i64,
    sample: usize,
    seed: u64,
    rows: usize,
    rank: usize,
    deficit: usize,
    mrank: usize,
    mdeficit: usize,
    consistent: bool,
}

fn md(c: i64, r: u64) -> u64 {
    if c >= 0 {
        c as u64 % r
    } else {
        r - ((-c) as u64 % r)
    }
}

/// returns (rank, rank_aug, merged_rank, merged_rank_aug) for first n rows
fn ranks(rows: &[(Vec<i64>, u64)], b: usize, r: u64) -> (usize, usize, usize, usize) {
    let full: Vec<Vec<u64>> = rows
        .iter()
        .map(|(c, _)| c.iter().map(|&v| md(v, r)).collect())
        .collect();
    let fulla: Vec<Vec<u64>> = rows
        .iter()
        .map(|(c, h)| {
            let mut v: Vec<u64> = c.iter().map(|&v| md(v, r)).collect();
            v.push(*h % r);
            v
        })
        .collect();
    let mut mer = Vec::new();
    let mut mera = Vec::new();
    for (c, h) in rows {
        let mut v = vec![0i64; b / 2];
        for i in 0..b {
            v[i / 2] += if i % 2 == 0 { c[i] } else { -c[i] };
        }
        let mut u: Vec<u64> = v.iter().map(|&t| md(t, r)).collect();
        mer.push(u.clone());
        u.push(*h % r);
        mera.push(u);
    }
    (
        rank_mod(full, r),
        rank_mod(fulla, r),
        rank_mod(mer, r),
        rank_mod(mera, r),
    )
}

fn find_curves(p: u64) -> Vec<(Curve, u64)> {
    let mut rng = Rng(0xC0FFEE ^ p);
    let mut out: Vec<(Curve, u64)> = Vec::new();
    while out.len() < 2 {
        let a = rng.below(p);
        let b = rng.below(p);
        if (4 * mulm(mulm(a, a, p), a, p) + 27 * mulm(b, b, p)) % p == 0 {
            continue;
        }
        let c = Curve { p, a, b };
        let n = count_points(&c);
        if n > 2 && is_prime(n) && !out.iter().any(|(o, _)| o.a == a && o.b == b) {
            out.push((c, n));
        }
    }
    out
}

fn med(v: &mut Vec<usize>) -> (usize, usize, usize) {
    v.sort();
    (v[0], v[v.len() / 2], v[v.len() - 1])
}

fn main() {
    let t0 = Instant::now();
    let primes_in = [1009u64, 2003, 4001, 8009, 16001];
    let deltas: [usize; 8] = [0, 1, 2, 3, 5, 10, 20, 40];
    let dmax = 40usize;
    let nsamples = 5usize;
    let mut results: Vec<Row> = Vec::new();
    let mut table = String::new();
    let mut tableb = String::new();
    let mut firstzero = String::new();
    writeln!(table, "| p | curve | r | B | Delta | deficit min/med/max | merged deficit min/med/max |").unwrap();
    writeln!(table, "|---|---|---|---|---|---|---|").unwrap();
    writeln!(tableb, "| p | curve | r | B | free relations | rank | deficit | merged rank | merged deficit |").unwrap();
    writeln!(tableb, "|---|---|---|---|---|---|---|---|---|").unwrap();
    for &p0 in &primes_in {
        let mut p = p0;
        while !is_prime(p) {
            p += 1;
        }
        if p != p0 {
            println!("note: {} not prime, using {}", p0, p);
        }
        let curves = find_curves(p);
        for (ci, (c, r)) in curves.iter().enumerate() {
            let r = *r;
            // all points
            let mut sqrt_tab: HashMap<u64, Vec<u64>> = HashMap::new();
            for y in 0..p {
                sqrt_tab.entry(mulm(y, y, p)).or_default().push(y);
            }
            let mut pts: Vec<(u64, u64)> = Vec::new();
            for x in 0..p {
                let rhs = (mulm(mulm(x, x, p), x, p) + mulm(c.a, x, p) + c.b) % p;
                if let Some(ys) = sqrt_tab.get(&rhs) {
                    for &y in ys {
                        pts.push((x, y));
                    }
                }
            }
            assert_eq!(pts.len() as u64 + 1, r);
            let g: Pt = Some(pts[0]);
            assert!(c.mul(r, g).is_none());
            // factor base: closest to sqrt(r)
            let target = (r as f64).sqrt();
            let mut best = (f64::MAX, 0usize);
            for k in 1..pts.len() {
                if pts[k].0 != pts[k - 1].0 {
                    let d = (k as f64 - target).abs();
                    if d < best.0 {
                        best = (d, k);
                    }
                }
            }
            let bsize = best.1;
            let xmax = pts[bsize - 1].0;
            let fb: Vec<(u64, u64)> = pts[..bsize].to_vec();
            assert!(bsize % 2 == 0);
            for i in 0..bsize / 2 {
                assert!(fb[2 * i].0 == fb[2 * i + 1].0 && (fb[2 * i].1 + fb[2 * i + 1].1) % p == 0);
            }
            let idx: HashMap<(u64, u64), usize> =
                fb.iter().enumerate().map(|(i, &q)| (q, i)).collect();
            println!(
                "p={} curve{} a={} b={} r={} B={} X={}",
                p, ci, c.a, c.b, r, bsize, xmax
            );
            let mut mk = |shape: &'static str, delta: i64, sample: usize, seed: u64, rows: &[(Vec<i64>, u64)]| {
                let (rk, rka, mrk, mrka) = ranks(rows, bsize, r);
                let cons = rk == rka && mrk == mrka;
                assert!(cons, "inconsistent system");
                Row {
                    p, r, a: c.a, b: c.b, curve: ci, bsize, shape, delta, sample, seed,
                    rows: rows.len(), rank: rk, deficit: bsize - rk,
                    mrank: mrk, mdeficit: bsize / 2 - mrk, consistent: cons,
                }
            };
            // shape A
            let mut perdelta: Vec<Vec<Row>> = deltas.iter().map(|_| Vec::new()).collect();
            for s in 0..nsamples {
                let seed = 0xA11CE ^ (p << 20) ^ ((ci as u64) << 16) ^ (s as u64 * 7919 + 1);
                println!("  shape A sample {} seed {:#x}", s, seed);
                let mut rng = Rng(seed);
                let mut rows: Vec<(Vec<i64>, u64)> = Vec::new();
                let mut draws = 0u64;
                while rows.len() < bsize + dmax {
                    let a = 1 + rng.below(r - 1);
                    draws += 1;
                    let rr = c.mul(a, g);
                    for i in 0..bsize {
                        let q = c.add(rr, c.neg(Some(fb[i])));
                        if let Some(qq) = q {
                            if let Some(&j) = idx.get(&qq) {
                                let mut v = vec![0i64; bsize];
                                v[i] += 1;
                                v[j] += 1;
                                rows.push((v, a));
                                break;
                            }
                        }
                    }
                }
                println!("    draws {} relations {}", draws, rows.len());
                for (di, &d) in deltas.iter().enumerate() {
                    let row = mk("A", d as i64, s, seed, &rows[..bsize + d]);
                    perdelta[di].push(row);
                }
            }
            let mut fz = None;
            for (di, &d) in deltas.iter().enumerate() {
                let mut dv: Vec<usize> = perdelta[di].iter().map(|r| r.deficit).collect();
                let mut mv: Vec<usize> = perdelta[di].iter().map(|r| r.mdeficit).collect();
                let (a1, a2, a3) = med(&mut dv);
                let (m1, m2, m3) = med(&mut mv);
                writeln!(table, "| {} | {} | {} | {} | {} | {}/{}/{} | {}/{}/{} |", p, ci, r, bsize, d, a1, a2, a3, m1, m2, m3).unwrap();
                if fz.is_none() && a3 == 0 {
                    fz = Some(d);
                }
            }
            let mut fzm = None;
            for (di, &d) in deltas.iter().enumerate() {
                if perdelta[di].iter().all(|r| r.mdeficit == 0) {
                    fzm = Some(d);
                    break;
                }
            }
            writeln!(firstzero, "| {} | {} | {} | {} | {} |", p, ci, r,
                fz.map_or("none<=40".to_string(), |d| d.to_string()),
                fzm.map_or("none<=40".to_string(), |d| d.to_string())).unwrap();
            for v in perdelta {
                results.extend(v);
            }
            // shape B
            let mut rows: Vec<(Vec<i64>, u64)> = Vec::new();
            for i in 0..bsize {
                for j in i..bsize {
                    if let Some(s) = c.add(Some(fb[i]), Some(fb[j])) {
                        if let Some(&k) = idx.get(&s) {
                            let mut v = vec![0i64; bsize];
                            v[i] += 1;
                            v[j] += 1;
                            v[k] -= 1;
                            rows.push((v, 0));
                        }
                    }
                }
            }
            let row = mk("B", -1, 0, 0, &rows);
            println!("  shape B: {} free relations rank {} deficit {}", row.rows, row.rank, row.deficit);
            writeln!(tableb, "| {} | {} | {} | {} | {} | {} | {} | {} | {} |", p, ci, r, bsize, row.rows, row.rank, row.deficit, row.mrank, row.mdeficit).unwrap();
            results.push(row);
        }
    }
    // outputs
    let mut csv = String::from("p,r,a,b,curve,B,shape,delta,sample,seed,rows,rank,deficit,merged_rank,merged_deficit,consistent\n");
    let mut js = String::from("[\n");
    for (i, x) in results.iter().enumerate() {
        writeln!(csv, "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}", x.p, x.r, x.a, x.b, x.curve, x.bsize, x.shape, x.delta, x.sample, x.seed, x.rows, x.rank, x.deficit, x.mrank, x.mdeficit, x.consistent).unwrap();
        write!(js, "  {{\"p\":{},\"r\":{},\"a\":{},\"b\":{},\"curve\":{},\"B\":{},\"shape\":\"{}\",\"delta\":{},\"sample\":{},\"seed\":{},\"rows\":{},\"rank\":{},\"deficit\":{},\"merged_rank\":{},\"merged_deficit\":{},\"consistent\":{}}}", x.p, x.r, x.a, x.b, x.curve, x.bsize, x.shape, x.delta, x.sample, x.seed, x.rows, x.rank, x.deficit, x.mrank, x.mdeficit, x.consistent).unwrap();
        js.push_str(if i + 1 < results.len() { ",\n" } else { "\n" });
    }
    js.push_str("]\n");
    std::fs::create_dir_all("results").unwrap();
    std::fs::write("results/rank_deficit.csv", csv).unwrap();
    std::fs::write("results/rank_deficit.json", js).unwrap();
    let mut t = String::new();
    writeln!(t, "## Shape A\n\n{}\n## Shape B\n\n{}\n## Smallest Delta with deficit 0 in all 5 samples\n\n| p | curve | r | full | merged |\n|---|---|---|---|---|\n{}", table, tableb, firstzero).unwrap();
    std::fs::write("results/tables.md", &t).unwrap();
    println!("{}", t);
    println!("runtime {:.2}s", t0.elapsed().as_secs_f64());
}
