// Utilities: RNG, FNV seeds, counters, TSV records, curve file reader.
use std::fmt::Write as FW;

pub const MSEED: u64 = 20261107;

pub fn fnv(s: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

#[derive(Clone)]
pub struct Rng(pub u64);
impl Rng {
    #[inline]
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}
// optional seed offset for independent replays (default 0 = the registered master seed)
pub fn seed_xor() -> u64 {
    std::env::var("L2SEED_XOR").ok().and_then(|v| v.parse().ok()).unwrap_or(0)
}
pub fn rng_for(role: &str, k: u32, m: u32, c: usize, t: usize) -> Rng {
    Rng(MSEED ^ seed_xor() ^ fnv(&format!("{}/{}/{}/{}/{}", role, k, m, c, t)))
}

macro_rules! cnt_struct {
    ($($f:ident),* $(,)?) => {
        #[derive(Default, Clone, Debug)]
        pub struct Cnt { $(pub $f: u64),* }
        impl Cnt {
            pub fn add(&mut self, o: &Cnt) { $(self.$f += o.$f;)* }
            pub fn fields(&self) -> Vec<(&'static str, u64)> { vec![$((stringify!($f), self.$f)),*] }
            pub fn set(&mut self, name: &str, v: u64) { $(if name == stringify!($f) { self.$f = v; })* }
        }
    };
}
cnt_struct!(
    g, s, m, t, x, l, la,
    targets, st1, st2, tfail, hroot, deg,
    raw, uniq, n_fail, ops_fail, n_pass, ops_pass,
    flagged, f4_mis, pre_mis,
    cls_alle, cls_pure_d, cls_pure_nd, cls_mix_d, cls_mix_nd, cls_neutral,
    lift_att, lift_fail, signchk_g, rel_ok, rel_bad, rel_unlift,
    s_flag_raw, s_avoid_lifts,
    la_rows, la_madds, hash_lookups
);
impl Cnt {
    #[inline]
    pub fn native(&self) -> u64 {
        self.s + self.m + self.t + self.x + self.l + self.la
    }
}

// Record: ordered (key, value) list, written as one TSV row
#[derive(Default, Clone)]
pub struct Rec(pub Vec<(String, f64)>);
impl Rec {
    pub fn put(&mut self, k: &str, v: f64) {
        self.0.push((k.to_string(), v));
    }
    pub fn put_cnt(&mut self, prefix: &str, c: &Cnt) {
        for (n, v) in c.fields() {
            self.0.push((format!("{}_{}", prefix, n), v as f64));
        }
    }
    pub fn get(&self, k: &str) -> f64 {
        self.0.iter().find(|(n, _)| n == k).map(|(_, v)| *v).unwrap_or(f64::NAN)
    }
}
pub fn fmtv(v: f64) -> String {
    if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{:.9e}", v)
    }
}
pub fn write_tsv(path: &str, rows: &[Rec]) {
    let mut s = String::new();
    if let Some(r0) = rows.first() {
        let h: Vec<&str> = r0.0.iter().map(|(n, _)| n.as_str()).collect();
        writeln!(s, "{}", h.join("\t")).unwrap();
        for r in rows {
            let v: Vec<String> = r.0.iter().map(|(_, v)| fmtv(*v)).collect();
            writeln!(s, "{}", v.join("\t")).unwrap();
        }
    }
    std::fs::write(path, s).unwrap();
}
pub fn read_tsv(path: &str) -> Vec<Rec> {
    let txt = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return vec![],
    };
    let mut lines = txt.lines();
    let hdr: Vec<String> = match lines.next() {
        Some(h) => h.split('\t').map(|s| s.to_string()).collect(),
        None => return vec![],
    };
    let mut out = vec![];
    for l in lines {
        let vals: Vec<f64> = l.split('\t').map(|s| s.parse::<f64>().unwrap_or(f64::NAN)).collect();
        out.push(Rec(hdr.iter().cloned().zip(vals.into_iter()).collect()));
    }
    out
}

#[derive(Clone, Debug)]
pub struct BinCurve {
    pub k: u32,
    pub idx: usize,
    pub poly: u64,
    pub b: u64,
    pub ne: u64,
    pub n: u64,
}
#[derive(Clone, Debug)]
pub struct PrimeCurve {
    pub k: u32,
    pub idx: usize,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub n: u64,
}
pub fn read_curves(dir: &str) -> (Vec<BinCurve>, Vec<PrimeCurve>) {
    let txt = std::fs::read_to_string(format!("{}/results/curves.txt", dir)).expect("curves.txt (run gp/gen_curves.gp)");
    let (mut bs, mut ps) = (vec![], vec![]);
    for l in txt.lines() {
        let f: Vec<&str> = l.split_whitespace().collect();
        if f.is_empty() {
            continue;
        }
        let u = |i: usize| f[i].parse::<u64>().unwrap();
        if f[0] == "B" {
            bs.push(BinCurve { k: u(1) as u32, idx: u(2) as usize, poly: u(3), b: u(4), ne: u(5), n: u(6) });
        } else {
            ps.push(PrimeCurve { k: u(1) as u32, idx: u(2) as usize, p: u(3), a: u(4), b: u(5), n: u(6) });
        }
    }
    (bs, ps)
}
pub fn mean_sd(v: &[f64]) -> (f64, f64) {
    let n = v.len() as f64;
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let m = v.iter().sum::<f64>() / n;
    let var = if v.len() > 1 { v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (n - 1.0) } else { 0.0 };
    (m, var.sqrt())
}
