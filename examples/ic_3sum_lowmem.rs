//! Low-memory 3-sum oracle study for prime-field index calculus (m = 3).
//!
//! The existing m = 3 path (`collect_relations` with a `PairTable`) decides
//! `R = ±F_i ± F_j ± F_k` by sweeping `T = R ∓ F_i` (O(B) per target) and
//! looking `x(T)` up in a B^2-entry table of `x(F_j ± F_k)`.  This harness
//! compares it with oracles that decide the *same* set using O(B) or
//! O(chunk·B) memory, on the *same* fixed target list, and checks that they
//! return the identical relation set per target before anything is timed.
//!
//! Oracles (all return, per target, the set of merged relation rows):
//!   table  - copy of the crate's pair-table m=3 per-target logic.
//!   nested - O(B) memory: for every (i, eps) solve the 2-sum T = ±F_j ± F_k
//!            by an O(B) sweep.  O(B^2) work per target.
//!   stream - multi-target meet-in-the-middle: hold the sweep candidates
//!            x(R_t ∓ F_i) of a chunk of targets (chunk·2B entries), stream
//!            every x(F_j ± F_k) once per chunk without storing it, probe.
//!
//! Public synthetic known-answer instances only.  Producer-side numbers.

use std::collections::HashSet;
use std::time::Instant;

use crypto_lib::cryptanalysis::prime_fast::{
    collect_relations, find_a3_curve, solve_relations, FastCurve, FastFactorBase, FastRelation,
    PairTable, Pt,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

type Row = Vec<(u32, i8)>;

fn vmhwm_kb() -> u64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    for l in s.lines() {
        if let Some(r) = l.strip_prefix("VmHWM:") {
            return r.trim().trim_end_matches("kB").trim().parse().unwrap_or(0);
        }
    }
    0
}
fn vmrss_kb() -> u64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    for l in s.lines() {
        if let Some(r) = l.strip_prefix("VmRSS:") {
            return r.trim().trim_end_matches("kB").trim().parse().unwrap_or(0);
        }
    }
    0
}

fn merge_entries(mut entries: Row) -> Row {
    entries.sort_unstable_by_key(|e| e.0);
    let mut merged: Row = Vec::with_capacity(entries.len());
    for (idx, s) in entries {
        if let Some(last) = merged.last_mut() {
            if last.0 == idx {
                last.1 += s;
                continue;
            }
        }
        merged.push((idx, s));
    }
    merged.retain(|e| e.1 != 0);
    merged
}

fn relation_holds(curve: &FastCurve, fb: &FastFactorBase, r: Pt, entries: &[(u32, i8)]) -> bool {
    let mut acc: Option<Pt> = None;
    for &(idx, s) in entries {
        let pt = fb.pts[idx as usize];
        let pt = if s < 0 { curve.neg(pt) } else { pt };
        for _ in 0..s.unsigned_abs() {
            acc = curve.add(acc, Some(pt));
        }
    }
    acc == Some(r)
}

/// merge + dedup + group-verify the raw decompositions of one target.
fn finalize(curve: &FastCurve, fb: &FastFactorBase, r: Pt, found: Vec<Row>) -> Vec<Row> {
    let mut seen: HashSet<Row> = HashSet::new();
    let mut out = Vec::new();
    for e in found {
        let m = merge_entries(e);
        if m.is_empty() || !seen.insert(m.clone()) {
            continue;
        }
        if !relation_holds(curve, fb, r, &m) {
            panic!("oracle produced an invalid relation");
        }
        out.push(m);
    }
    out.sort();
    out
}

const DIFF: u32 = 1 << 31;

/// The O(B) sweep shared by `table` and `stream`: emits 1- and 2-term rows
/// directly and calls `on(i, eps, x_canonical)` for every sweep candidate.
struct Sweeper {
    d: Vec<u64>,
    degenerate: Vec<bool>,
    scratch: Vec<u64>,
}
impl Sweeper {
    fn new(b: usize) -> Self {
        Sweeper {
            d: vec![0; b],
            degenerate: vec![false; b],
            scratch: Vec::with_capacity(b),
        }
    }
    fn run<F: FnMut(usize, i8, u64)>(
        &mut self,
        curve: &FastCurve,
        fb: &FastFactorBase,
        r: Pt,
        found: &mut Vec<Row>,
        mut on: F,
    ) {
        let f = curve.f;
        let b_len = fb.len();
        for i in 0..b_len {
            let dd = f.sub(r.x, fb.pts[i].x);
            self.degenerate[i] = dd == 0;
            self.d[i] = if dd == 0 { f.one } else { dd };
        }
        f.batch_inv(&mut self.d, &mut self.scratch);
        for i in 0..b_len {
            let fi = fb.pts[i];
            if self.degenerate[i] {
                let s = if r.y == fi.y { 1i8 } else { -1 };
                found.push(vec![(i as u32, s)]);
                continue;
            }
            let inv = self.d[i];
            let lam_minus = f.mul(f.add(r.y, fi.y), inv);
            let lam_plus = f.mul(f.sub(r.y, fi.y), inv);
            for (eps, lam) in [(1i8, lam_minus), (-1i8, lam_plus)] {
                let x3 = f.sub(f.sub(f.sqr(lam), r.x), fi.x);
                let xc = f.from_mont(x3);
                if let Some(j) = fb.lookup(xc) {
                    if j >= i {
                        let y = f.sub(f.mul(lam, f.sub(r.x, x3)), r.y);
                        let tau = if y == fb.pts[j].y { 1i8 } else { -1 };
                        found.push(vec![(i as u32, eps), (j as u32, tau)]);
                    }
                }
                on(i, eps, xc);
            }
        }
    }
}

/// y-coordinate (Montgomery) of T = R - eps*F_i, recomputed on demand.
fn y_of_t(curve: &FastCurve, fb: &FastFactorBase, r: Pt, i: usize, eps: i8) -> u64 {
    let f = curve.f;
    let fi = fb.pts[i];
    let inv = f.inv(f.sub(r.x, fi.x));
    let lam = if eps == 1 {
        f.mul(f.add(r.y, fi.y), inv)
    } else {
        f.mul(f.sub(r.y, fi.y), inv)
    };
    let x3 = f.sub(f.sub(f.sqr(lam), r.x), fi.x);
    f.sub(f.mul(lam, f.sub(r.x, x3)), r.y)
}

fn push_triple(
    curve: &FastCurve,
    fb: &FastFactorBase,
    y3: u64,
    i: usize,
    eps: i8,
    j: u32,
    k_raw: u32,
    found: &mut Vec<Row>,
) {
    let sigma = if k_raw & DIFF != 0 { -1i8 } else { 1 };
    let k = (k_raw & !DIFF) as usize;
    let fk = fb.pts[k];
    let fk = if sigma < 0 { curve.neg(fk) } else { fk };
    let Some(pjk) = curve.add(Some(fb.pts[j as usize]), Some(fk)) else {
        return;
    };
    let tau = if y3 == pjk.y { 1i8 } else { -1 };
    found.push(vec![(i as u32, eps), (j, tau), (k as u32, tau * sigma)]);
}

// ---- oracle: table (copy of the crate logic) -----------------------------
struct TableOracle<'a> {
    pairs: &'a PairTable,
    sw: Sweeper,
}
impl<'a> TableOracle<'a> {
    fn target(&mut self, curve: &FastCurve, fb: &FastFactorBase, r: Pt) -> Vec<Row> {
        let f = curve.f;
        let mut found: Vec<Row> = Vec::new();
        let mut hits: Vec<(usize, i8, u32, u32)> = Vec::new();
        let max_x = self.pairs.max_x();
        let pairs = self.pairs;
        self.sw.run(curve, fb, r, &mut found, |i, eps, xc| {
            if xc <= max_x {
                for &(_, j, k_raw) in pairs.lookup(xc) {
                    if (j as usize) >= i {
                        hits.push((i, eps, j, k_raw));
                    }
                }
            }
        });
        let _ = f;
        for (i, eps, j, k_raw) in hits {
            let y3 = y_of_t(curve, fb, r, i, eps);
            push_triple(curve, fb, y3, i, eps, j, k_raw, &mut found);
        }
        finalize(curve, fb, r, found)
    }
}

// ---- oracle: nested (O(B) memory, O(B^2) work per target) ---------------
struct NestedOracle {
    sw: Sweeper,
    d: Vec<u64>,
    scratch: Vec<u64>,
}
impl NestedOracle {
    fn target(&mut self, curve: &FastCurve, fb: &FastFactorBase, r: Pt) -> Vec<Row> {
        let f = curve.f;
        let b = fb.len();
        let mut found: Vec<Row> = Vec::new();
        let mut cands: Vec<(usize, i8)> = Vec::new();
        self.sw.run(curve, fb, r, &mut found, |i, eps, _| cands.push((i, eps)));
        for (i, eps) in cands {
            let fi = fb.pts[i];
            let fie = if eps == 1 { curve.neg(fi) } else { fi };
            // T = R - eps*F_i = R + (-eps F_i)
            let Some(t) = curve.add(Some(r), Some(fie)) else { continue };
            // 2-sum on T: T = s*F_j + t'*F_k, j >= i, k >= j.
            self.d.clear();
            let mut idxs = Vec::new();
            for j in i..b {
                let dd = f.sub(t.x, fb.pts[j].x);
                if dd == 0 {
                    continue;
                }
                self.d.push(dd);
                idxs.push(j);
            }
            f.batch_inv(&mut self.d, &mut self.scratch);
            for (n, &j) in idxs.iter().enumerate() {
                let fj = fb.pts[j];
                let inv = self.d[n];
                for s in [1i8, -1] {
                    let fjs = if s == 1 { fj } else { curve.neg(fj) };
                    // U = T - s F_j = T + (-s F_j) ; need inverse of (T.x - (-sFj).x) = T.x - Fj.x
                    let negfjs = curve.neg(fjs);
                    let u = curve.add_with_inv(t, negfjs, f.neg(inv));
                    // add_with_inv expects inverse of (p2.x - p1.x) = (Fj.x - t.x) = -(t.x - Fj.x)
                    let xc = f.from_mont(u.x);
                    if let Some(k) = fb.lookup(xc) {
                        if k >= j {
                            let tp = if u.y == fb.pts[k].y { 1i8 } else { -1 };
                            found.push(vec![(i as u32, eps), (j as u32, s), (k as u32, tp)]);
                        }
                    }
                }
            }
        }
        finalize(curve, fb, r, found)
    }
}

// ---- oracle: stream (chunked meet-in-the-middle) -------------------------
struct StreamOracle {
    sw: Sweeper,
    cands: Vec<(u64, u32, u32)>, // (xc, target idx in chunk, i*2 + (eps==-1))
    sorted: Vec<(u64, u32, u32)>,
    starts: Vec<u32>,
    shift: u32,
    pub pair_sums: u64,
    pub probes_hit: u64,
}
impl StreamOracle {
    fn chunk(&mut self, curve: &FastCurve, fb: &FastFactorBase, targets: &[Pt]) -> Vec<Vec<Row>> {
        let f = curve.f;
        let b = fb.len();
        let mut found: Vec<Vec<Row>> = vec![Vec::new(); targets.len()];
        self.cands.clear();
        for (t, &r) in targets.iter().enumerate() {
            let cands = &mut self.cands;
            self.sw.run(curve, fb, r, &mut found[t], |i, eps, xc| {
                cands.push((xc, t as u32, (i as u32) * 2 + (eps == -1) as u32));
            });
        }
        // bucket the candidates by high bits of x (counting sort)
        let bits_p = 64 - (f.p.max(2) - 1).leading_zeros();
        let target_bits = (usize::BITS - 1 - (self.cands.len().max(32) / 2).leading_zeros()).max(4);
        self.shift = bits_p.saturating_sub(target_bits);
        let nb = (((f.p.max(2) - 1) >> self.shift) + 1) as usize;
        self.starts.clear();
        self.starts.resize(nb + 1, 0);
        for e in &self.cands {
            self.starts[((e.0 >> self.shift) as usize) + 1] += 1;
        }
        for i in 0..nb {
            self.starts[i + 1] += self.starts[i];
        }
        let mut cursor: Vec<u32> = self.starts[..nb].to_vec();
        self.sorted.clear();
        self.sorted.resize(self.cands.len(), (0, 0, 0));
        for e in &self.cands {
            let bk = (e.0 >> self.shift) as usize;
            self.sorted[cursor[bk] as usize] = *e;
            cursor[bk] += 1;
        }
        drop(cursor);
        // stream pair sums x(F_j +- F_k), never stored
        let mut d = Vec::with_capacity(b);
        let mut scratch = Vec::with_capacity(b);
        let mut hits: Vec<(u32, u32, u32, u32, u32)> = Vec::new(); // (t, i2, j, k_raw)
        let mut probe = |xc: u64, j: u32, k_raw: u32, this: &Self, hits: &mut Vec<(u32, u32, u32, u32, u32)>| {
            let bk = (xc >> this.shift) as usize;
            if bk + 1 >= this.starts.len() {
                return;
            }
            let s = &this.sorted[this.starts[bk] as usize..this.starts[bk + 1] as usize];
            for e in s {
                if e.0 == xc && (e.2 >> 1) <= j {
                    hits.push((e.1, e.2, j, k_raw, 0));
                }
            }
        };
        let mut sums = 0u64;
        for j in 0..b {
            let fj = fb.pts[j];
            if let Some(dbl) = curve.double(fj) {
                sums += 1;
                probe(f.from_mont(dbl.x), j as u32, j as u32, self, &mut hits);
            }
            d.clear();
            for k in (j + 1)..b {
                d.push(f.sub(fb.pts[k].x, fj.x));
            }
            f.batch_inv(&mut d, &mut scratch);
            for (idx, k) in ((j + 1)..b).enumerate() {
                let fk = fb.pts[k];
                let inv = d[idx];
                let sum_x = curve.add_x_with_inv(fj, fk, inv);
                let diff_x = curve.add_x_with_inv(fj, curve.neg(fk), inv);
                sums += 2;
                probe(f.from_mont(sum_x), j as u32, k as u32, self, &mut hits);
                probe(f.from_mont(diff_x), j as u32, k as u32 | DIFF, self, &mut hits);
            }
        }
        self.pair_sums += sums;
        self.probes_hit += hits.len() as u64;
        for (t, i2, j, k_raw, _) in hits {
            let i = (i2 >> 1) as usize;
            let eps = if i2 & 1 == 1 { -1i8 } else { 1 };
            let r = targets[t as usize];
            let y3 = y_of_t(curve, fb, r, i, eps);
            push_triple(curve, fb, y3, i, eps, j, k_raw, &mut found[t as usize]);
        }
        found
            .into_iter()
            .zip(targets)
            .map(|(f_, &r)| finalize(curve, fb, r, f_))
            .collect()
    }
    fn mem_bytes(&self) -> usize {
        (self.cands.capacity() + self.sorted.capacity()) * 16 + self.starts.capacity() * 4
    }
}

// ---- fixed target stream --------------------------------------------------
fn gen_targets(curve: &FastCurve, g: Pt, q: Pt, n: usize, seed: u64) -> Vec<(u64, u64, Pt)> {
    let mut rng = StdRng::seed_from_u64(seed ^ 0x5eed);
    let mut out = Vec::with_capacity(n);
    let mut cur: Option<(u64, u64, Pt)> = None;
    while out.len() < n {
        let nxt = match cur {
            Some((a, b, r)) if out.len() % 64 != 0 => curve.add(Some(r), Some(g)).map(|nr| (a + 1, b, nr)),
            _ => None,
        };
        let t = match nxt {
            Some(t) => t,
            None => loop {
                let a = rng.gen_range(1..curve.n - 1);
                let b = rng.gen_range(1..curve.n);
                if let Some(r) = curve.combine(g, q, a, b) {
                    break (a, b, r);
                }
            },
        };
        cur = Some(t);
        out.push(t);
    }
    out
}

struct Args {
    mode: String,
    oracle: String,
    bits: u32,
    b: usize,
    n: usize,
    chunk: usize,
    seed: u64,
    extra: usize,
    nested_max: usize,
}

fn parse() -> Args {
    let argv: Vec<String> = std::env::args().collect();
    let mut a = Args {
        mode: "validate".into(),
        oracle: "table".into(),
        bits: 24,
        b: 120,
        n: 2000,
        chunk: 256,
        seed: 20_261_117,
        extra: 16,
        nested_max: 160,
    };
    let mut i = 1;
    while i < argv.len() {
        let v = argv.get(i + 1).cloned().unwrap_or_default();
        match argv[i].as_str() {
            "--mode" => a.mode = v,
            "--oracle" => a.oracle = v,
            "--bits" => a.bits = v.parse().unwrap(),
            "--b" => a.b = v.parse().unwrap(),
            "--n" => a.n = v.parse().unwrap(),
            "--chunk" => a.chunk = v.parse().unwrap(),
            "--seed" => a.seed = v.parse().unwrap(),
            "--extra" => a.extra = v.parse().unwrap(),
            "--nested-max" => a.nested_max = v.parse().unwrap(),
            o => panic!("unknown arg {o}"),
        }
        i += 2;
    }
    a
}

fn main() {
    let a = parse();
    let (fc, g) = find_a3_curve(a.bits, 3).expect("curve");
    let mut rng = StdRng::seed_from_u64(a.seed ^ a.bits as u64);
    let k = rng.gen_range(1..fc.n);
    let q = fc.mul(Some(g), k).unwrap();
    let fb = FastFactorBase::build(&fc, a.b);
    let b = fb.len();
    let needed = b + a.extra;
    let base_rss = vmrss_kb();

    match a.mode.as_str() {
        // existing crate m=3 path, adaptive walk, end to end
        "baseline" => {
            let t = Instant::now();
            let pairs = PairTable::build(&fc, &fb);
            let build_s = t.elapsed().as_secs_f64();
            let rss_after_build = vmrss_kb();
            let t = Instant::now();
            let (rels, st) = collect_relations(&fc, g, q, &fb, Some(&pairs), needed, 4_000_000_000, a.seed);
            let collect_s = t.elapsed().as_secs_f64();
            let t = Instant::now();
            let log = solve_relations(&fc, b, &rels);
            let la_s = t.elapsed().as_secs_f64();
            assert_eq!(log, Some(k), "baseline wrong log");
            println!("{}", json!({"mode":"baseline","bits":a.bits,"p":fc.f.p,"B":b,"needed":needed,
                "pair_entries":pairs.len(),"pair_bytes":pairs.len()*16,
                "build_s":build_s,"collect_s":collect_s,"la_s":la_s,"total_s":build_s+collect_s+la_s,
                "targets":st.targets,"rows":[st.rows_one,st.rows_two,st.rows_three],
                "rss_base_kb":base_rss,"rss_after_build_kb":rss_after_build,"vmhwm_kb":vmhwm_kb(),"log_ok":true}));
        }
        // equality of oracles on a fixed target list + copy-vs-crate adaptive check
        "validate" => {
            let pairs = PairTable::build(&fc, &fb);
            let tg = gen_targets(&fc, g, q, a.n, a.seed);
            let mut to = TableOracle { pairs: &pairs, sw: Sweeper::new(b) };
            let ref_rows: Vec<Vec<Row>> = tg.iter().map(|t| to.target(&fc, &fb, t.2)).collect();
            let total_rows: usize = ref_rows.iter().map(|r| r.len()).sum();
            let rows3: usize = ref_rows.iter().flatten().filter(|r| r.len() == 3).count();
            let mut res = json!({"mode":"validate","bits":a.bits,"B":b,"targets":a.n,"rows_total":total_rows,"rows3":rows3});
            // copy-vs-crate: reproduce crate's adaptive collect with the copied table oracle.
            {
                let n = fc.n;
                let mut rng2 = StdRng::seed_from_u64(a.seed);
                let fresh = |rng: &mut StdRng| loop {
                    let aa = rng.gen_range(1..n);
                    let bb = rng.gen_range(1..n);
                    if let Some(r) = fc.combine(g, q, aa, bb) {
                        break (aa, bb, r);
                    }
                };
                let want = b.min(40);
                let (crate_rels, _) = collect_relations(&fc, g, q, &fb, Some(&pairs), want, 4_000_000_000, a.seed);
                let (mut aa, mut bb, mut r) = fresh(&mut rng2);
                let mut mine: Vec<FastRelation> = Vec::new();
                while mine.len() < want {
                    let rows = to.target(&fc, &fb, r);
                    let mut prod = false;
                    for m in rows {
                        // crate pushes in discovery order; compare as set below, order-insensitively per target
                        mine.push(FastRelation { a: aa, b: bb, entries: m });
                        prod = true;
                    }
                    if prod {
                        let nn = fresh(&mut rng2);
                        aa = nn.0; bb = nn.1; r = nn.2;
                    } else if let Some(nr) = fc.add(Some(r), Some(g)) {
                        r = nr;
                        aa = (aa + 1) % n;
                    } else {
                        let nn = fresh(&mut rng2);
                        aa = nn.0; bb = nn.1; r = nn.2;
                    }
                }
                // compare as multiset of (a,b,sorted entries) over the first `want`
                let norm = |v: &[FastRelation]| {
                    let mut x: Vec<(u64, u64, Row)> = v.iter().take(want).map(|r| (r.a, r.b, r.entries.clone())).collect();
                    x.sort();
                    x
                };
                // the crate stops mid-target at `want`; compare only fully-consumed targets
                let cn = norm(&crate_rels);
                let mn = norm(&mine);
                let common = cn.iter().filter(|x| mn.binary_search(x).is_ok()).count();
                res["copy_vs_crate_rows_crate"] = json!(cn.len());
                res["copy_vs_crate_rows_in_copy"] = json!(common);
                res["copy_vs_crate_ok"] = json!(common == cn.len());
            }
            if b <= a.nested_max {
                let mut no = NestedOracle { sw: Sweeper::new(b), d: vec![], scratch: vec![] };
                let t = Instant::now();
                let mut ok = true;
                for (ti, t_) in tg.iter().enumerate() {
                    if no.target(&fc, &fb, t_.2) != ref_rows[ti] {
                        ok = false;
                        eprintln!("nested mismatch at target {ti}");
                    }
                }
                res["nested_equal"] = json!(ok);
                res["nested_s"] = json!(t.elapsed().as_secs_f64());
            }
            for ch in [1usize, 7, 64, 1000, a.n] {
                let ch = ch.min(a.n).max(1);
                let mut so = StreamOracle { sw: Sweeper::new(b), cands: vec![], sorted: vec![], starts: vec![], shift: 0, pair_sums: 0, probes_hit: 0 };
                let mut ok = true;
                let mut ti = 0;
                for c in tg.chunks(ch) {
                    let pts: Vec<Pt> = c.iter().map(|t| t.2).collect();
                    let out = so.chunk(&fc, &fb, &pts);
                    for o in out {
                        if o != ref_rows[ti] {
                            ok = false;
                            eprintln!("stream(chunk={ch}) mismatch at target {ti}");
                        }
                        ti += 1;
                    }
                }
                res[format!("stream_chunk{ch}_equal")] = json!(ok);
            }
            println!("{res}");
        }
        // timing of one oracle on a fixed target list (one process per config for VmHWM)
        "bench" => {
            let tg = gen_targets(&fc, g, q, a.n, a.seed);
            let t_all = Instant::now();
            let (rows_total, rows3, mem_struct, build_s, extra_j);
            match a.oracle.as_str() {
                "table" => {
                    let t = Instant::now();
                    let pairs = PairTable::build(&fc, &fb);
                    build_s = t.elapsed().as_secs_f64();
                    let mut to = TableOracle { pairs: &pairs, sw: Sweeper::new(b) };
                    let mut rt = 0; let mut r3 = 0;
                    for t_ in &tg {
                        let rows = to.target(&fc, &fb, t_.2);
                        rt += rows.len();
                        r3 += rows.iter().filter(|r| r.len() == 3).count();
                    }
                    rows_total = rt; rows3 = r3; mem_struct = pairs.len() * 16; extra_j = json!({});
                }
                "stream" => {
                    build_s = 0.0;
                    let mut so = StreamOracle { sw: Sweeper::new(b), cands: vec![], sorted: vec![], starts: vec![], shift: 0, pair_sums: 0, probes_hit: 0 };
                    let mut rt = 0; let mut r3 = 0;
                    for c in tg.chunks(a.chunk) {
                        let pts: Vec<Pt> = c.iter().map(|t| t.2).collect();
                        for rows in so.chunk(&fc, &fb, &pts) {
                            rt += rows.len();
                            r3 += rows.iter().filter(|r| r.len() == 3).count();
                        }
                    }
                    rows_total = rt; rows3 = r3; mem_struct = so.mem_bytes();
                    extra_j = json!({"chunk":a.chunk,"pair_sums":so.pair_sums,"probe_hits":so.probes_hit,"chunks":(a.n + a.chunk - 1)/a.chunk});
                }
                "nested" => {
                    build_s = 0.0;
                    let mut no = NestedOracle { sw: Sweeper::new(b), d: vec![], scratch: vec![] };
                    let mut rt = 0; let mut r3 = 0;
                    for t_ in &tg {
                        let rows = no.target(&fc, &fb, t_.2);
                        rt += rows.len();
                        r3 += rows.iter().filter(|r| r.len() == 3).count();
                    }
                    rows_total = rt; rows3 = r3; mem_struct = 0; extra_j = json!({});
                }
                o => panic!("oracle {o}"),
            }
            let total = t_all.elapsed().as_secs_f64();
            println!("{}", json!({"mode":"bench","oracle":a.oracle,"bits":a.bits,"p":fc.f.p,"B":b,"targets":a.n,
                "rows_total":rows_total,"rows3":rows3,"build_s":build_s,"total_s":total,
                "s_per_target":total/a.n as f64,"struct_bytes":mem_struct,
                "rss_base_kb":base_rss,"vmhwm_kb":vmhwm_kb(),"extra":extra_j}));
        }
        // complete solve using the stream oracle on a non-adaptive stream; log checked against planted k
        "solve" => {
            let t0 = Instant::now();
            let mut so = StreamOracle { sw: Sweeper::new(b), cands: vec![], sorted: vec![], starts: vec![], shift: 0, pair_sums: 0, probes_hit: 0 };
            let mut rels: Vec<FastRelation> = Vec::new();
            let mut targets = 0u64;
            let mut gen_seed = a.seed;
            while rels.len() < needed {
                let tg = gen_targets(&fc, g, q, a.chunk, gen_seed);
                gen_seed = gen_seed.wrapping_add(0x9e37_79b9);
                let pts: Vec<Pt> = tg.iter().map(|t| t.2).collect();
                let out = so.chunk(&fc, &fb, &pts);
                targets += tg.len() as u64;
                for (t_, rows) in tg.iter().zip(out) {
                    for m in rows {
                        if rels.len() < needed {
                            rels.push(FastRelation { a: t_.0, b: t_.1, entries: m });
                        }
                    }
                }
            }
            let collect_s = t0.elapsed().as_secs_f64();
            let t = Instant::now();
            let log = solve_relations(&fc, b, &rels);
            let la_s = t.elapsed().as_secs_f64();
            println!("{}", json!({"mode":"solve","bits":a.bits,"p":fc.f.p,"B":b,"chunk":a.chunk,"needed":needed,
                "targets":targets,"chunks":(targets as usize + a.chunk -1)/a.chunk,"collect_s":collect_s,"la_s":la_s,
                "total_s":collect_s+la_s,"log_ok":log==Some(k),"struct_bytes":so.mem_bytes(),
                "rss_base_kb":base_rss,"vmhwm_kb":vmhwm_kb()}));
            assert_eq!(log, Some(k));
        }
        // is sorting by x a usable order for a priority-queue merge? fraction of order-preserving pairs
        "monotone" => {
            let mut cnt = 0u64;
            let mut tot = 0u64;
            let f = fc.f;
            let mut srt: Vec<(u64, usize)> = fb.pts.iter().enumerate().map(|(i, p)| (f.from_mont(p.x), i)).collect();
            srt.sort();
            for bi in 0..b.min(64) {
                let fbp = fb.pts[bi];
                let mut prev: Option<(u64, u64)> = None;
                for &(x, i) in &srt {
                    if i == bi { continue; }
                    let s = fc.add(Some(fb.pts[i]), Some(fbp));
                    let Some(s) = s else { continue };
                    let sx = f.from_mont(s.x);
                    if let Some((_, px)) = prev {
                        tot += 1;
                        if sx > px { cnt += 1; }
                    }
                    prev = Some((x, sx));
                }
            }
            println!("{}", json!({"mode":"monotone","B":b,"adjacent_pairs":tot,"order_preserving_fraction":cnt as f64/tot as f64}));
        }
        m => panic!("mode {m}"),
    }
}
