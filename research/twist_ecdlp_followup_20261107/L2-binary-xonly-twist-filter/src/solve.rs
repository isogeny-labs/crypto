// Enumeration solver for Semaev S_{m+1}, arms U/F/S, classification, lifts and sign verification.
use crate::model::*;
use crate::util::*;

#[derive(Clone)]
pub struct Target {
    pub xr: u64,
    pub rpt: Pt,
    pub a: u64,
    pub b: u64,
}

#[inline]
fn quad_c<M: Model>(md: &M, c: &mut Cnt, u: u64, v: u64) -> Quad {
    let n0 = c.native();
    let q = md.quad(c, u, v);
    let ops = c.native() - n0;
    if q.pass {
        c.n_pass += 1;
        c.ops_pass += ops;
        c.hroot += 1;
    } else {
        c.n_fail += 1;
        c.ops_fail += ops;
        c.tfail += 1;
    }
    q
}
fn sort2(a: u64, b: u64) -> [u64; 3] {
    if a <= b { [a, b, 0] } else { [b, a, 0] }
}
fn sort3(a: u64, b: u64, c: u64) -> [u64; 3] {
    let mut v = [a, b, c];
    v.sort_unstable();
    v
}

pub fn solve<M: Model>(md: &M, c: &mut Cnt, m: usize, list: &[u64], xr: u64) -> Vec<[u64; 3]> {
    let mut out = vec![];
    if m == 2 {
        for &x1 in list {
            c.st1 += 1;
            let q = quad_c(md, c, x1, xr);
            if !q.pass {
                continue;
            }
            for i in 0..q.nr as usize {
                let r = q.r[i];
                if r != 0 && md.in_v(c, r) {
                    out.push(sort2(x1, r));
                }
            }
        }
    } else {
        let n = list.len();
        for i in 0..n {
            let x1 = list[i];
            for j in i..n {
                let x2 = list[j];
                c.st1 += 1;
                let q1 = quad_c(md, c, x1, x2);
                if !q1.pass {
                    continue;
                }
                for t in 0..q1.nr as usize {
                    let xx = q1.r[t];
                    c.st2 += 1;
                    let q2 = quad_c(md, c, xr, xx);
                    if !q2.pass {
                        continue;
                    }
                    for s in 0..q2.nr as usize {
                        let r = q2.r[s];
                        if r != 0 && md.in_v(c, r) {
                            out.push(sort3(x1, x2, r));
                        }
                    }
                }
                if q1.inf {
                    c.st2 += 1;
                    if md.in_v(c, xr) {
                        out.push(sort3(x1, x2, xr));
                    }
                }
            }
        }
    }
    out
}

pub fn dedup(mut v: Vec<[u64; 3]>) -> Vec<[u64; 3]> {
    v.sort_unstable();
    v.dedup();
    v
}

// degenerate-family match (Section 2.1 family D)
pub fn dmatch<M: Model>(md: &M, sol: &[u64; 3], m: usize, xr: u64) -> bool {
    if m != 3 {
        return false;
    }
    let pairs = [(1usize, 2usize), (0, 2), (0, 1)];
    for k in 0..3 {
        let (i, j) = pairs[k];
        if sol[i] == sol[j] && sol[k] == xr {
            return true;
        }
    }
    if let Some(sbxr) = md.sb_over(xr) {
        for k in 0..3 {
            let (i, j) = pairs[k];
            if let Some(y) = md.sb_over(sol[i]) {
                if sol[j] == y && sol[k] == sbxr {
                    return true;
                }
            }
        }
    }
    false
}

// lift coordinates and find signs with sum e_i P_i = R. returns signs.
pub fn verify_sol<M: Model>(md: &M, cl: &mut Cnt, m: usize, sol: &[u64; 3], rpt: &Pt) -> Option<[i8; 3]> {
    let mut pts = [INF; 3];
    for i in 0..m {
        match md.lift(cl, sol[i]) {
            Some(p) => pts[i] = p,
            None => {
                cl.rel_unlift += 1;
                return None;
            }
        }
    }
    let g0 = cl.g;
    let nr = md.neg(rpt);
    let mut res: Option<[i8; 3]> = None;
    let chk = |s: &Pt, e: [i8; 3]| -> Option<[i8; 3]> {
        if *s == *rpt {
            Some(e)
        } else if *s == nr {
            Some([-e[0], -e[1], -e[2]])
        } else {
            None
        }
    };
    if m == 2 {
        for s1 in [1i8, -1] {
            let p1 = if s1 == 1 { pts[1] } else { md.neg(&pts[1]) };
            let s = md.add(cl, &pts[0], &p1);
            if let Some(e) = chk(&s, [1, s1, 0]) {
                res = Some(e);
                break;
            }
        }
    } else {
        'o: for s1 in [1i8, -1] {
            let p1 = if s1 == 1 { pts[1] } else { md.neg(&pts[1]) };
            let a = md.add(cl, &pts[0], &p1);
            for s2 in [1i8, -1] {
                let p2 = if s2 == 1 { pts[2] } else { md.neg(&pts[2]) };
                let s = md.add(cl, &a, &p2);
                if let Some(e) = chk(&s, [1, s1, s2]) {
                    res = Some(e);
                    break 'o;
                }
            }
        }
    }
    cl.signchk_g += cl.g - g0;
    if res.is_some() {
        cl.rel_ok += 1;
    } else {
        cl.rel_bad += 1;
    }
    res
}

pub struct ArmsOut {
    pub cu: Cnt,
    pub cf: Cnt,
    pub cs: Cnt,
    pub cl: Cnt,
    pub samples: Vec<String>,
    pub ve: Vec<u64>,
    pub rels: Vec<(usize, [u64; 3], [i8; 3])>,
}

pub fn etab_of<M: Model>(md: &M) -> Vec<bool> {
    md.star().iter().map(|&x| md.is_e(x)).collect()
}

pub struct Arms<'a, M: Model> {
    pub md: &'a M,
    pub m: usize,
    pub etab: Vec<bool>,
    pub stab: Vec<bool>,
    pub ve: Vec<u64>,
    pub cu: Cnt,
    pub cf: Cnt,
    pub cs: Cnt,
    pub cl: Cnt,
    pub samples: Vec<String>,
    want: usize,
    tag: String,
    verify: bool,
}
impl<'a, M: Model> Arms<'a, M> {
    pub fn new(md: &'a M, m: usize, tag: &str, want: usize, verify: bool) -> Self {
        let mut cf = Cnt::default();
        let mut cs = Cnt::default();
        let star = md.star();
        let etab = etab_of(md);
        let mut ve = vec![];
        for (i, &x) in star.iter().enumerate() {
            let e = md.pre_e(&mut cf, x);
            if e != etab[i] {
                cf.pre_mis += 1;
            }
            if e {
                ve.push(x);
            }
        }
        let mut stab = vec![false; star.len()];
        for (i, &x) in star.iter().enumerate() {
            stab[i] = md.pre_e(&mut cs, x);
        }
        Arms { md, m, etab, stab, ve, cu: Cnt::default(), cf, cs, cl: Cnt::default(), samples: vec![], want, tag: tag.to_string(), verify }
    }
    fn lab(&self, x: u64) -> bool {
        match self.md.star().binary_search(&x) {
            Ok(i) => self.etab[i],
            Err(_) => self.md.is_e(x),
        }
    }
    fn slab(&self, x: u64) -> bool {
        match self.md.star().binary_search(&x) {
            Ok(i) => self.stab[i],
            Err(_) => self.md.is_e(x),
        }
    }
    // process one target; returns usable (all E-type, verified) relations (solution, signs)
    pub fn target(&mut self, t: &Target) -> Vec<([u64; 3], [i8; 3])> {
        let md = self.md;
        let m = self.m;
        let flagged = md.in_v_free(t.xr) || md.sb_over(t.xr).map_or(false, |y| md.in_v_free(y));
        self.cu.targets += 1;
        self.cf.targets += 1;
        if flagged {
            self.cu.flagged += 1;
        }
        let raw_u = solve(md, &mut self.cu, m, md.star(), t.xr);
        let raw_f = solve(md, &mut self.cf, m, &self.ve, t.xr);
        self.cu.raw += raw_u.len() as u64;
        self.cf.raw += raw_f.len() as u64;
        for r in &raw_u {
            let mut bad = false;
            for i in 0..m {
                self.cs.t += 1;
                if !self.slab(r[i]) {
                    bad = true;
                }
            }
            if bad {
                self.cs.s_flag_raw += 1;
            }
        }
        let uu = dedup(raw_u);
        let uf = dedup(raw_f);
        self.cu.uniq += uu.len() as u64;
        self.cf.uniq += uf.len() as u64;
        if !flagged && uu != uf {
            self.cu.f4_mis += 1;
        }
        let mut rels = vec![];
        for s in &uu {
            let ntw = (0..m).filter(|&i| !self.lab(s[i])).count();
            let d = dmatch(md, s, m, t.xr);
            if (0..m).any(|i| s[i] == 0) {
                self.cu.cls_neutral += 1;
            }
            if ntw == 0 {
                self.cu.cls_alle += 1;
            } else if ntw == m {
                if d { self.cu.cls_pure_d += 1 } else { self.cu.cls_pure_nd += 1 }
            } else if d {
                self.cu.cls_mix_d += 1
            } else {
                self.cu.cls_mix_nd += 1
            }
            if ntw > 0 {
                self.cs.s_avoid_lifts += ntw as u64;
            }
            if self.samples.len() < self.want {
                let labs: Vec<String> = (0..m).map(|i| if self.lab(s[i]) { "1".into() } else { "0".to_string() }).collect();
                let cs_: Vec<String> = (0..m).map(|i| s[i].to_string()).collect();
                self.samples.push(format!("{} {} {} {} {} {}", self.tag, m, t.xr, cs_.join(","), labs.join(","), flagged as u8));
            }
            if self.verify {
                let sg = verify_sol(md, &mut self.cl, m, s, &t.rpt);
                if let Some(e) = sg {
                    if ntw == 0 {
                        rels.push((*s, e));
                    }
                }
            }
        }
        rels
    }
}

pub fn run_arms<M: Model>(md: &M, m: usize, targets: &[Target], tag: &str, want_samples: usize, verify: bool, keep_rels: bool) -> ArmsOut {
    let mut a = Arms::new(md, m, tag, want_samples, verify);
    let mut rels = vec![];
    for (ti, t) in targets.iter().enumerate() {
        let r = a.target(t);
        if keep_rels {
            for (s, e) in r {
                rels.push((ti, s, e));
            }
        }
    }
    ArmsOut { cu: a.cu, cf: a.cf, cs: a.cs, cl: a.cl, samples: a.samples, ve: a.ve, rels }
}

// twist-target solver run (PC-B): all unique solutions classified; returns counts
pub struct TwistOut {
    pub nsol: u64,
    pub n_all_e: u64,
    pub n_mixed: u64,
    pub n_all_tw: u64,
    pub samples: Vec<String>,
    pub c: Cnt,
}
pub fn run_twist_targets<M: Model>(md: &M, m: usize, xrs: &[u64], tag: &str, want: usize) -> TwistOut {
    let mut c = Cnt::default();
    let star = md.star();
    let mut out = TwistOut { nsol: 0, n_all_e: 0, n_mixed: 0, n_all_tw: 0, samples: vec![], c: Cnt::default() };
    for &xr in xrs {
        let u = dedup(solve(md, &mut c, m, star, xr));
        for s in &u {
            out.nsol += 1;
            let ne = (0..m).filter(|&i| md.is_e(s[i])).count();
            if ne == 0 {
                out.n_all_tw += 1;
            } else if ne == m {
                out.n_all_e += 1;
            } else {
                out.n_mixed += 1;
            }
            if out.samples.len() < want {
                let cs_: Vec<String> = (0..m).map(|i| s[i].to_string()).collect();
                let labs: Vec<String> = (0..m).map(|i| if md.is_e(s[i]) { "1".into() } else { "0".to_string() }).collect();
                out.samples.push(format!("{} {} {} {} {}", tag, m, xr, cs_.join(","), labs.join(",")));
            }
        }
    }
    out.c = c;
    out
}
