//! Arm runners, stopping, JSONL output (SPEC sections 6, 11).
use crate::curve::*;
use crate::exact::Exact;
use crate::field::{cost, diff, Fp};
use crate::graph::Graph;
use crate::poly::*;
use crate::rng::{derive, SplitMix64};
use std::collections::{HashMap, HashSet};

pub const BETAS: [(u64, u64); 6] = [(1, 8), (3, 16), (1, 4), (3, 8), (1, 2), (3, 4)];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Arm {
    A,
    A2,
    B,
    PC1,
    PC4,
    B3,
}
impl Arm {
    pub fn name(&self) -> &'static str {
        match self {
            Arm::A => "A",
            Arm::A2 => "A2",
            Arm::B => "B",
            Arm::PC1 => "PC1",
            Arm::PC4 => "PC4",
            Arm::B3 => "B3",
        }
    }
    pub fn parse(s: &str) -> Arm {
        match s {
            "A" => Arm::A,
            "A2" => Arm::A2,
            "B" => Arm::B,
            "PC1" => Arm::PC1,
            "PC4" => Arm::PC4,
            "B3" => Arm::B3,
            _ => panic!("unknown arm {}", s),
        }
    }
    fn btype(&self) -> bool {
        matches!(self, Arm::B | Arm::B3)
    }
    fn pc_m(&self) -> u64 {
        match self {
            Arm::PC1 => 1,
            Arm::PC4 => 4,
            _ => 0,
        }
    }
}

pub fn binom2(n: u64) -> u64 {
    n * (n - 1) / 2
}

/// Instance of one (p, beta, seed): curve, O, FB.
pub struct Inst {
    pub p: u64,
    pub seed: u64,
    pub beta_idx: usize,
    pub curve: Curve,
    pub o: u32,
    pub fb: Vec<u32>,
    pub n_fb: usize,
    pub colof: Vec<i32>, // point id -> FB column or -1
    pub setup: (u64, u64),
}

impl Inst {
    pub fn new(p: u64, beta_idx: usize, seed: u64) -> Inst {
        let fp = Fp::new(p);
        let curve = Curve::generate(&fp, seed);
        let setup = fp.snap();
        let n1 = curve.n1();
        let mut rng = SplitMix64::new(derive(p, seed, 255, 2));
        let mut perm: Vec<u32> = (0..n1 as u32).collect();
        for i in (1..n1).rev() {
            let j = rng.below(i as u64 + 1) as usize;
            perm.swap(i, j);
        }
        let o = perm[0];
        let (num, den) = BETAS[beta_idx];
        let n_fb = std::cmp::max(4, (2 * (n1 as u64 - 1) * num + den) / (2 * den)) as usize;
        let fb: Vec<u32> = perm[1..=n_fb].to_vec();
        let mut colof = vec![-1i32; n1];
        for (k, &id) in fb.iter().enumerate() {
            colof[id as usize] = k as i32;
        }
        Inst { p, seed, beta_idx, curve, o, fb, n_fb, colof, setup }
    }
}

pub struct Row {
    pub kind: u8,
    pub pts: Vec<u32>,
    pub small: Vec<(usize, i64)>,
    pub lps: Vec<u32>,
    pub b: Option<[u64; 4]>,
}

pub fn make_row(inst: &Inst, kind: u8, ids: &[u32], btype: bool, b: Option<[u64; 4]>) -> Row {
    let mut pts = ids.to_vec();
    pts.sort();
    let mut small = vec![];
    let mut lps = vec![];
    for &id in &pts {
        if id == inst.o {
            small.push((inst.n_fb, 1i64));
        } else if inst.colof[id as usize] >= 0 {
            small.push((inst.colof[id as usize] as usize, 1i64));
        } else {
            lps.push(id);
        }
    }
    let k = inst.n_fb;
    match kind {
        0 => {
            if btype {
                small.push((k + 1, -1));
                small.push((k + 2, -1));
            } else {
                small.push((k + 1, -1));
            }
        }
        1 => small.push((k + 1, -1)),
        _ => small.push((k + 2, -1)),
    }
    Row { kind, pts, small, lps, b }
}

pub enum Ins {
    Dup,
    Accepted(usize),
}

pub struct State {
    pub n_cols: usize,
    pub seen: HashSet<(u8, Vec<u32>)>,
    pub g: Graph,
    uvmap: HashMap<u32, usize>,
    pub ex: Option<Exact>,
    pub accepted: u64,
    pub gate_checked: u64,
    p: u64,
}

impl State {
    pub fn new(inst: &Inst, n_cols: usize, exact: bool) -> State {
        State {
            n_cols,
            seen: HashSet::new(),
            g: Graph::new(),
            uvmap: HashMap::new(),
            ex: if exact { Some(Exact::new(n_cols, inst.curve.n1())) } else { None },
            accepted: 0,
            gate_checked: 0,
            p: inst.p,
        }
    }
    fn vert(&mut self, id: u32) -> usize {
        if let Some(&v) = self.uvmap.get(&id) {
            return v;
        }
        let v = self.g.new_vertex();
        self.uvmap.insert(id, v);
        v
    }
    pub fn y(&self) -> i64 {
        self.g.y()
    }
    pub fn insert(&mut self, inst: &Inst, row: &Row) -> Ins {
        let key = (row.kind, row.pts.clone());
        if self.seen.contains(&key) {
            return Ins::Dup;
        }
        self.seen.insert(key);
        self.accepted += 1;
        // G2 row gate
        if inst.p <= 1021 || self.accepted % 97 == 0 {
            self.gate_checked += 1;
            gate_check(inst, row);
        }
        match row.lps.len() {
            0 => self.g.f0 += 1,
            1 => {
                let v = self.vert(row.lps[0]);
                self.g.add_edge(0, v);
            }
            2 => {
                let v = self.vert(row.lps[0]);
                let w = self.vert(row.lps[1]);
                self.g.add_edge(v, w);
            }
            _ => {}
        }
        if let Some(ex) = &mut self.ex {
            ex.add_row(&row.small, &row.lps);
        }
        let _ = self.p;
        Ins::Accepted(row.lps.len())
    }
}

fn gate_check(inst: &Inst, row: &Row) {
    let p = inst.p;
    let cv = &inst.curve;
    for &id in &row.pts {
        let (x, y) = cv.pts[id as usize];
        assert!(raw_eval_f(&cv.a, x, y, p) == 0, "G2: point off curve");
        match row.kind {
            0 => {
                let b = row.b.expect("line row without b");
                let l = (b[0] + b[1] * x % p + b[2] * y % p + b[3] * (x * y % p)) % p;
                assert!(l == 0, "G2: line point off line");
            }
            1 => assert!(x == cv.pts[row.pts[0] as usize].0, "G2: x-fibre coordinate"),
            _ => assert!(y == cv.pts[row.pts[0] as usize].1, "G2: y-fibre coordinate"),
        }
    }
}

pub struct PairStream {
    rng: SplitMix64,
    n: u64,
    total: u64,
    pub used: HashSet<u64>,
    skip: HashSet<u64>,
    list: Option<Vec<u64>>,
    pos: usize,
}

impl PairStream {
    pub fn new(seed: u64, n_fb: usize, skip: HashSet<u64>) -> PairStream {
        let n = n_fb as u64;
        PairStream { rng: SplitMix64::new(seed), n, total: binom2(n), used: HashSet::new(), skip, list: None, pos: 0 }
    }
    pub fn next_pair(&mut self) -> Option<(usize, usize)> {
        let n = self.n;
        loop {
            if self.list.is_none() {
                if self.used.len() as u64 == self.total {
                    return None;
                }
                if self.used.len() as u64 * 10 >= 9 * self.total {
                    let mut l = vec![];
                    for i in 0..n {
                        for j in (i + 1)..n {
                            let key = i * n + j;
                            if !self.used.contains(&key) {
                                l.push(key);
                            }
                        }
                    }
                    for i in (1..l.len()).rev() {
                        let j = self.rng.below(i as u64 + 1) as usize;
                        l.swap(i, j);
                    }
                    self.list = Some(l);
                    self.pos = 0;
                }
            }
            if let Some(l) = &self.list {
                while self.pos < l.len() {
                    let key = l[self.pos];
                    self.pos += 1;
                    self.used.insert(key);
                    if self.skip.contains(&key) {
                        continue;
                    }
                    return Some(((key / n) as usize, (key % n) as usize));
                }
                return None;
            }
            let i = self.rng.below(n);
            let j = self.rng.below(n);
            if i == j {
                continue;
            }
            let (i, j) = (i.min(j), i.max(j));
            let key = i * n + j;
            if self.used.contains(&key) {
                continue;
            }
            self.used.insert(key);
            if self.skip.contains(&key) {
                continue;
            }
            return Some((i as usize, j as usize));
        }
    }
}

/// All split pencil fibres in the SPEC 7.2 order, lazily: calls f(kind, ids) for each; stop if f returns true.
fn pencil_walk(fp: &Fp, inst: &Inst, mut f: impl FnMut(u8, Vec<u32>) -> bool) {
    let p = inst.p;
    for kind in [1u8, 2u8] {
        for t in 0..p {
            let c = fibre_cubic(fp, &inst.curve.a, kind, t);
            if !split_test(fp, &c) {
                continue;
            }
            let rs = roots_split(fp, &c);
            let mut ids = vec![];
            for r in rs {
                let pt = if kind == 1 { (t, r) } else { (r, t) };
                ids.push(*inst.curve.map.get(&pt).expect("fibre point not on curve map"));
            }
            if f(kind, ids) {
                return;
            }
        }
    }
}

fn lp_count(inst: &Inst, ids: &[u32]) -> usize {
    ids.iter().filter(|&&i| i != inst.o && inst.colof[i as usize] < 0).count()
}

/// Counts (n0, n1, n2) of B's pencil rows (<= 2 LP) by LP count, throw-away Fp.
pub fn pencil_counts(inst: &Inst) -> [u64; 3] {
    let fp = Fp::new(inst.p);
    let mut n = [0u64; 3];
    pencil_walk(&fp, inst, |_k, ids| {
        let l = lp_count(inst, &ids);
        if l <= 2 {
            n[l] += 1;
        }
        false
    });
    n
}

#[derive(Default, Clone)]
pub struct Rec {
    pub kind: String,
    pub p: u64,
    pub beta_idx: usize,
    pub seed: u64,
    pub arm: String,
    pub n1: usize,
    pub redraws: u64,
    pub n_fb: usize,
    pub n_cols: usize,
    pub target: i64,
    pub t: i64,
    pub censored: bool,
    pub censor_reason: String,
    pub cap: u64,
    pub t_deg: u64,
    pub t_nosplit: u64,
    pub t_tangent: u64,
    pub t_toomany: u64,
    pub t_dup: u64,
    pub acc: [u64; 3],
    pub pencil_lp: [u64; 4],
    pub pencil_rows: u64,
    pub pc_m: u64,
    pub pc_target: [u64; 3],
    pub pc_got: [u64; 3],
    pub pc_free_trials: u64,
    pub f0: i64,
    pub e: i64,
    pub v: i64,
    pub b: i64,
    pub y: i64,
    pub uf_ops: u64,
    pub fpm_setup: u64,
    pub mul_line: u64,
    pub inv_line: u64,
    pub mul_pencil: u64,
    pub inv_pencil: u64,
    pub mul_pcfree: u64,
    pub inv_pcfree: u64,
    pub gate_rows_checked: u64,
    // exact only
    pub t_uf: i64,
    pub t_exact: i64,
    pub rank_full: i64,
    pub rank_lp: i64,
    pub rank_lp_uf: i64,
    pub dim: i64,
    pub la_ops: u64,
    pub g4_ok: bool,
}

impl Rec {
    pub fn to_json(&self) -> String {
        let (num, den) = BETAS[self.beta_idx];
        let fpm_line = self.mul_line + 10 * self.inv_line;
        let fpm_pencil = self.mul_pencil + 10 * self.inv_pencil;
        let mut s = format!(
            "{{\"kind\":\"{}\",\"p\":{},\"beta_idx\":{},\"beta_num\":{},\"beta_den\":{},\"seed\":{},\"arm\":\"{}\",\"N1\":{},\"redraws\":{},\"n_fb\":{},\"n_cols\":{},\"target\":{},\"T\":{},\"censored\":{},\"censor_reason\":\"{}\",\"cap\":{},",
            self.kind, self.p, self.beta_idx, num, den, self.seed, self.arm, self.n1, self.redraws, self.n_fb, self.n_cols, self.target, self.t, self.censored, self.censor_reason, self.cap
        );
        s += &format!(
            "\"t_degenerate\":{},\"t_nosplit\":{},\"t_tangent\":{},\"t_toomanylp\":{},\"t_duplicate\":{},\"acc_lp0\":{},\"acc_lp1\":{},\"acc_lp2\":{},",
            self.t_deg, self.t_nosplit, self.t_tangent, self.t_toomany, self.t_dup, self.acc[0], self.acc[1], self.acc[2]
        );
        s += &format!(
            "\"pencil_lp0\":{},\"pencil_lp1\":{},\"pencil_lp2\":{},\"pencil_lp3\":{},\"pencil_rows\":{},",
            self.pencil_lp[0], self.pencil_lp[1], self.pencil_lp[2], self.pencil_lp[3], self.pencil_rows
        );
        s += &format!(
            "\"pc_m\":{},\"pc_target0\":{},\"pc_target1\":{},\"pc_target2\":{},\"pc_got0\":{},\"pc_got1\":{},\"pc_got2\":{},\"pc_free_trials\":{},",
            self.pc_m, self.pc_target[0], self.pc_target[1], self.pc_target[2], self.pc_got[0], self.pc_got[1], self.pc_got[2], self.pc_free_trials
        );
        s += &format!("\"F0\":{},\"E\":{},\"V\":{},\"B\":{},\"Y\":{},", self.f0, self.e, self.v, self.b, self.y);
        if self.kind == "exact" {
            s += &format!(
                "\"T_uf\":{},\"T_exact\":{},\"rank_full\":{},\"rank_lp\":{},\"rank_lp_uf\":{},\"dim\":{},\"la_ops\":{},\"g4_ok\":{},",
                self.t_uf, self.t_exact, self.rank_full, self.rank_lp, self.rank_lp_uf, self.dim, self.la_ops, self.g4_ok
            );
        }
        s += &format!(
            "\"uf_ops\":{},\"fpm_setup\":{},\"mul_line\":{},\"inv_line\":{},\"mul_pencil\":{},\"inv_pencil\":{},\"mul_pcfree\":{},\"inv_pcfree\":{},\"fpm_line\":{},\"fpm_pencil\":{},\"fpm_rel\":{},\"gate_rows_checked\":{}}}",
            self.uf_ops, self.fpm_setup, self.mul_line, self.inv_line, self.mul_pencil, self.inv_pencil, self.mul_pcfree, self.inv_pcfree, fpm_line, fpm_pencil, fpm_line + fpm_pencil, self.gate_rows_checked
        );
        s
    }
}

struct Run {
    st: State,
    trials: i64,
    t_uf: i64,
    done: bool,
    exact: bool,
    target: i64,
}
impl Run {
    fn check(&mut self) {
        let y = self.st.y();
        if y >= self.target && self.t_uf < 0 {
            self.t_uf = self.trials;
        }
        if self.exact {
            if self.st.ex.as_ref().unwrap().dim() >= self.target {
                self.done = true;
            }
        } else if y >= self.target {
            self.done = true;
        }
    }
}

pub fn run_arm(inst: &Inst, arm: Arm, exact: bool, pcounts: Option<[u64; 3]>) -> Rec {
    let p = inst.p;
    let fp = Fp::new(p);
    let btype = arm.btype();
    let n_cols = inst.n_fb + if btype { 3 } else { 2 };
    let target = n_cols as i64 - 1;
    let (num, den) = BETAS[inst.beta_idx];
    let cap = std::cmp::min(64 * ((p * den + num - 1) / num), binom2(inst.n_fb as u64));
    let mut rec = Rec::default();
    rec.kind = if exact { "exact".into() } else { "run".into() };
    rec.p = p;
    rec.beta_idx = inst.beta_idx;
    rec.seed = inst.seed;
    rec.arm = arm.name().into();
    rec.n1 = inst.curve.n1();
    rec.redraws = inst.curve.redraws;
    rec.n_fb = inst.n_fb;
    rec.n_cols = n_cols;
    rec.target = target;
    rec.cap = cap;
    rec.pc_m = arm.pc_m();
    rec.fpm_setup = cost(inst.setup);
    let mut run = Run { st: State::new(inst, n_cols, exact), trials: 0, t_uf: -1, done: false, exact, target };

    // pencil phase
    let s0 = fp.snap();
    if btype {
        let keep3 = arm == Arm::B3;
        let mut pen_rows = 0u64;
        let mut pen_lp = [0u64; 4];
        pencil_walk(&fp, inst, |kind, ids| {
            let l = lp_count(inst, &ids);
            pen_lp[l] += 1;
            if l == 3 && !keep3 {
                return false;
            }
            let row = make_row(inst, kind, &ids, true, None);
            if let Ins::Accepted(_) = run.st.insert(inst, &row) {
                pen_rows += 1;
                run.check();
                if run.done {
                    return true;
                }
            }
            false
        });
        rec.pencil_rows = pen_rows;
        rec.pencil_lp = pen_lp;
    }
    let s1 = fp.snap();

    // free comparator rows
    let mut skip: HashSet<u64> = HashSet::new();
    if arm.pc_m() > 0 && !run.done {
        let m = arm.pc_m();
        let nk = pcounts.expect("pc arm needs pencil counts");
        let tg = [m * nk[0], m * nk[1], m * nk[2]];
        rec.pc_target = tg;
        let mut s3 = PairStream::new(derive(p, inst.seed, inst.beta_idx as u64, 13), inst.n_fb, HashSet::new());
        let mut got = [0u64; 3];
        let mut kept: Vec<Row> = vec![];
        let mut local: HashSet<(u8, Vec<u32>)> = HashSet::new();
        let mut free = 0u64;
        while (got[0] < tg[0] || got[1] < tg[1] || got[2] < tg[2]) && free < 64 * p {
            let (i, j) = match s3.next_pair() {
                Some(x) => x,
                None => break,
            };
            free += 1;
            let res = line_trial(&fp, &inst.curve, inst.o, inst.fb[i], inst.fb[j]);
            if res.outcome != Outcome::Ok {
                continue;
            }
            let l = lp_count(inst, &res.pts);
            if l > 2 {
                continue;
            }
            let row = make_row(inst, 0, &res.pts, false, Some(res.b));
            let key = (row.kind, row.pts.clone());
            if got[l] < tg[l] && !local.contains(&key) {
                local.insert(key);
                got[l] += 1;
                kept.push(row);
            }
        }
        rec.pc_got = got;
        rec.pc_free_trials = free;
        skip = s3.used.clone();
        for row in &kept {
            if let Ins::Accepted(_) = run.st.insert(inst, row) {
                run.check();
                if run.done {
                    break;
                }
            }
        }
    }
    let s2 = fp.snap();

    // main line stream
    let stream_tag = if arm == Arm::A2 { 12 } else { 11 };
    let mut stream = PairStream::new(derive(p, inst.seed, inst.beta_idx as u64, stream_tag), inst.n_fb, skip);
    while !run.done {
        if run.trials as u64 >= cap {
            rec.censored = true;
            rec.censor_reason = if cap == binom2(inst.n_fb as u64) { "pairs_exhausted".into() } else { "trial_cap".into() };
            break;
        }
        let (i, j) = match stream.next_pair() {
            Some(x) => x,
            None => {
                rec.censored = true;
                rec.censor_reason = "pairs_exhausted".into();
                break;
            }
        };
        run.trials += 1;
        let res = line_trial(&fp, &inst.curve, inst.o, inst.fb[i], inst.fb[j]);
        match res.outcome {
            Outcome::Degenerate => rec.t_deg += 1,
            Outcome::NoSplit => rec.t_nosplit += 1,
            Outcome::Tangent => rec.t_tangent += 1,
            Outcome::Ok => {
                let l = lp_count(inst, &res.pts);
                if l > 2 {
                    rec.t_toomany += 1;
                } else {
                    let row = make_row(inst, 0, &res.pts, btype, Some(res.b));
                    match run.st.insert(inst, &row) {
                        Ins::Dup => rec.t_dup += 1,
                        Ins::Accepted(k) => {
                            rec.acc[k] += 1;
                            run.check();
                        }
                    }
                }
            }
        }
    }
    let s3 = fp.snap();
    let ph = diff(s0, s1);
    rec.mul_pencil = ph.0;
    rec.inv_pencil = ph.1;
    let pf = diff(s1, s2);
    rec.mul_pcfree = pf.0;
    rec.inv_pcfree = pf.1;
    let ln = diff(s2, s3);
    rec.mul_line = ln.0;
    rec.inv_line = ln.1;
    rec.t = run.trials;
    let g = &run.st.g;
    rec.f0 = g.f0;
    rec.e = g.edges;
    rec.v = g.verts;
    rec.b = g.bip;
    rec.y = g.y();
    rec.uf_ops = g.ops;
    rec.gate_rows_checked = run.st.gate_checked;
    if exact {
        let ex = run.st.ex.as_ref().unwrap();
        rec.t_uf = if arm == Arm::B3 { -2 } else { run.t_uf };
        rec.t_exact = rec.t;
        rec.rank_full = ex.full.rank as i64;
        rec.rank_lp = ex.lp.rank as i64;
        rec.rank_lp_uf = g.rank_lp_uf();
        rec.dim = ex.dim();
        rec.la_ops = ex.la_ops();
        rec.g4_ok = rec.rank_lp == rec.rank_lp_uf;
    }
    rec
}

pub fn cell_output(p: u64, beta_idx: usize, seed_from: u64, seeds: u64, arms: &[Arm], exact: bool) -> String {
    let mut out = String::new();
    for s in seed_from..seed_from + seeds {
        let inst = Inst::new(p, beta_idx, s);
        let needs_pc = arms.iter().any(|a| a.pc_m() > 0);
        let pc = if needs_pc { Some(pencil_counts(&inst)) } else { None };
        for &a in arms {
            let rec = run_arm(&inst, a, exact, pc);
            out += &rec.to_json();
            out.push('\n');
        }
        eprintln!("p={} beta_idx={} seed={} done", p, beta_idx, s);
    }
    out
}

pub fn census_output(p: u64, seed_from: u64, seeds: u64) -> String {
    let mut out = String::new();
    for s in seed_from..seed_from + seeds {
        let inst = Inst::new(p, 5, s);
        let fp = Fp::new(p);
        let (mut split_x, mut split_y, mut pts_x, mut pts_y) = (0u64, 0u64, 0u64, 0u64);
        for kind in [1u8, 2u8] {
            for t in 0..p {
                let c = fibre_cubic(&fp, &inst.curve.a, kind, t);
                if split_test(&fp, &c) {
                    if kind == 1 { split_x += 1 } else { split_y += 1 }
                }
                let n = roots_all(&fp, &c).len() as u64;
                if kind == 1 { pts_x += n } else { pts_y += n }
            }
        }
        assert!(pts_x == inst.curve.n1() as u64 && pts_y == inst.curve.n1() as u64, "census: fibre point sums != N1");
        let ntr = std::cmp::min(20000, binom2(inst.n_fb as u64));
        let mut st = PairStream::new(derive(p, s, 5, 11), inst.n_fb, HashSet::new());
        let (mut nondeg, mut spl) = (0u64, 0u64);
        for _ in 0..ntr {
            let (i, j) = st.next_pair().expect("census stream exhausted");
            let r = line_trial(&fp, &inst.curve, inst.o, inst.fb[i], inst.fb[j]);
            if r.outcome != Outcome::Degenerate {
                nondeg += 1;
                if r.outcome != Outcome::NoSplit {
                    spl += 1;
                }
            }
        }
        out += &format!(
            "{{\"kind\":\"census\",\"p\":{},\"seed\":{},\"N1\":{},\"redraws\":{},\"split_x\":{},\"split_y\":{},\"pts_x\":{},\"pts_y\":{},\"line_trials\":{},\"line_nondegen\":{},\"line_split\":{}}}\n",
            p, s, inst.curve.n1(), inst.curve.redraws, split_x, split_y, pts_x, pts_y, ntr, nondeg, spl
        );
        eprintln!("census p={} seed={} done", p, s);
    }
    out
}
