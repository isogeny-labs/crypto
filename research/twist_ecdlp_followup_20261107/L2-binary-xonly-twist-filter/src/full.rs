// Relation collection to a complete solve: incremental dense zero-skipping elimination mod prime n.
use crate::cells::*;
use crate::model::*;
use crate::solve::*;
use crate::util::*;

fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % n;
        }
        a = a * a % n;
        e >>= 1;
    }
    r
}

pub struct Elim {
    pub n: u64,
    pub nv: usize, // variable columns (rhs at index nv)
    piv: Vec<usize>, // column -> row index+1, 0 none
    rows: Vec<Vec<u64>>,
    nz: Vec<Vec<u32>>,
    pub rank: usize,
    pub madds: u64,
    pub rowops: u64,
    pub inconsistent: u64,
}
impl Elim {
    pub fn new(n: u64, nv: usize) -> Elim {
        Elim { n, nv, piv: vec![0; nv], rows: vec![], nz: vec![], rank: 0, madds: 0, rowops: 0, inconsistent: 0 }
    }
    pub fn insert(&mut self, mut row: Vec<u64>) {
        let n = self.n;
        let nv = self.nv;
        let mut j = 0usize;
        loop {
            while j < nv && row[j] == 0 {
                j += 1;
            }
            if j == nv {
                if row[nv] != 0 {
                    self.inconsistent += 1;
                }
                return;
            }
            let pr = self.piv[j];
            if pr != 0 {
                let f = row[j];
                let prow = &self.rows[pr - 1];
                for &c in &self.nz[pr - 1] {
                    let c = c as usize;
                    row[c] = (row[c] + n - f * prow[c] % n) % n;
                }
                self.madds += self.nz[pr - 1].len() as u64;
                self.rowops += 1;
                j += 1;
            } else {
                let inv = powmod(row[j], n - 2, n);
                let mut nzl = vec![];
                for c in j..=nv {
                    if row[c] != 0 {
                        row[c] = row[c] * inv % n;
                        nzl.push(c as u32);
                    }
                }
                self.madds += nzl.len() as u64;
                self.rows.push(row);
                self.nz.push(nzl);
                self.piv[j] = self.rows.len();
                self.rank += 1;
                return;
            }
        }
    }
    // d is the last variable column; determined iff it has a pivot
    pub fn d_value(&self) -> Option<u64> {
        let pr = self.piv[self.nv - 1];
        if pr == 0 {
            None
        } else {
            Some(self.rows[pr - 1][self.nv])
        }
    }
}

#[derive(Clone, Default)]
pub struct Snap {
    pub rc: u64,
    pub targets: u64,
    pub cu: Cnt,
    pub cf: Cnt,
    pub cl: Cnt,
    pub setup_g: u64,
    pub walk_g: u64,
    pub la_madds: u64,
    pub la_rows: u64,
    pub final_g: u64,
}

fn snap_rec(r: &mut Rec, pfx: &str, s: &Snap) {
    r.put(&format!("{}_rc", pfx), s.rc as f64);
    r.put(&format!("{}_targets", pfx), s.targets as f64);
    r.put(&format!("{}_setup_g", pfx), s.setup_g as f64);
    r.put(&format!("{}_walk_g", pfx), s.walk_g as f64);
    r.put(&format!("{}_la_madds", pfx), s.la_madds as f64);
    r.put(&format!("{}_la_rows", pfx), s.la_rows as f64);
    r.put(&format!("{}_final_g", pfx), s.final_g as f64);
    r.put(&format!("{}_u_native", pfx), s.cu.native() as f64);
    r.put(&format!("{}_f_native", pfx), s.cf.native() as f64);
    r.put(&format!("{}_l_native", pfx), s.cl.native() as f64);
    r.put(&format!("{}_l_g", pfx), s.cl.g as f64);
    r.put(&format!("{}_u_st1", pfx), s.cu.st1 as f64);
    r.put(&format!("{}_f_st1", pfx), s.cf.st1 as f64);
}

pub fn full_solve(m: usize, c: &BinCurve, ci: usize, inst: usize) -> Rec {
    let k = c.k;
    let l = ceil_div(k, m as u32);
    let mut rv = rng_for("V", k, m as u32, ci, 0);
    let vt = if ci % 2 == 0 { Vset::rand(k, l, &mut rv, None) } else { Vset::poly(k, l) };
    let md = BinModel::new(c, vt);
    let mut cc = Cnt::default();
    let mut st = BinStream::new(&md, k, m as u32, ci, 100 + inst, &mut cc);
    let setup_g = cc.g;
    let tag = format!("B {} {}", k, ci);
    let mut arms = Arms::new(&md, m, &tag, 0, true);
    let u = arms.ve.len();
    let nv = u + 1;
    let n = md.n;
    let nneed = (u + 1 + 10) as u64;
    let mut miles = vec![nneed];
    for _ in 0..5 {
        let last = *miles.last().unwrap();
        miles.push(((last as f64) * 1.1).ceil() as u64);
    }
    let mut el = Elim::new(n, nv);
    let mut rc = 0u64;
    let mut walked = 0u64;
    let mut snap_dd: Option<Snap> = None;
    let mut snap_reg: Option<(usize, Snap)> = None;
    let mut reg_fail_snap: Option<Snap> = None;
    let mut next_mile = 0usize;
    let mut d_found: Option<u64> = None;
    let mut unusable = 0u64;
    let max_targets = 400_000u64;
    let mk_snap = |arms: &Arms<BinModel>, el: &Elim, rc: u64, walked: u64, cc: &Cnt| -> Snap {
        Snap { rc, targets: walked, cu: arms.cu.clone(), cf: arms.cf.clone(), cl: arms.cl.clone(), setup_g, walk_g: cc.g - setup_g, la_madds: el.madds, la_rows: el.rowops, final_g: 0 }
    };
    let mut done = false;
    while !done && walked < max_targets {
        let t = st.next(&md, &mut cc);
        walked += 1;
        let rels = arms.target(&t);
        // count relations that include twist coordinates (unusable) = uniq - rels
        for (sol, e) in rels {
            let mut row = vec![0u64; nv + 1];
            for i in 0..m {
                let col = arms.ve.binary_search(&sol[i]).expect("E-type coordinate in V_E");
                arms.cl.hash_lookups += 1;
                arms.cl.l += 1;
                row[col] = (row[col] + if e[i] > 0 { 1 } else { n - 1 }) % n;
            }
            row[u] = (n - (2 * t.b) % n) % n;
            row[nv] = (2 * t.a) % n;
            el.insert(row);
            rc += 1;
            if d_found.is_none() {
                if let Some(d) = el.d_value() {
                    d_found = Some(d);
                    snap_dd = Some(mk_snap(&arms, &el, rc, walked, &cc));
                }
            }
        }
        // registered rounds
        while next_mile < miles.len() && rc >= miles[next_mile] {
            if el.rank == nv && snap_reg.is_none() {
                snap_reg = Some((next_mile, mk_snap(&arms, &el, rc, walked, &cc)));
                done = true;
                break;
            }
            next_mile += 1;
            if next_mile == miles.len() {
                reg_fail_snap = Some(mk_snap(&arms, &el, rc, walked, &cc));
                done = true;
            }
        }
        if reg_fail_snap.is_some() || snap_reg.is_some() {
            done = true;
        }
    }
    // continue (exploratory) until d determined, if not yet: up to 6*N_need relations
    while d_found.is_none() && rc < 6 * nneed && walked < max_targets {
        let t = st.next(&md, &mut cc);
        walked += 1;
        let rels = arms.target(&t);
        for (sol, e) in rels {
            let mut row = vec![0u64; nv + 1];
            for i in 0..m {
                let col = arms.ve.binary_search(&sol[i]).unwrap();
                arms.cl.hash_lookups += 1;
                arms.cl.l += 1;
                row[col] = (row[col] + if e[i] > 0 { 1 } else { n - 1 }) % n;
            }
            row[u] = (n - (2 * t.b) % n) % n;
            row[nv] = (2 * t.a) % n;
            el.insert(row);
            rc += 1;
            if d_found.is_none() {
                if let Some(d) = el.d_value() {
                    d_found = Some(d);
                    snap_dd = Some(mk_snap(&arms, &el, rc, walked, &cc));
                }
            }
        }
    }
    unusable += arms.cu.uniq - rc.min(arms.cu.uniq);
    // verify d (counted G)
    let mut cfin = Cnt::default();
    let mut d_ok = 0.0;
    if let Some(d) = d_found {
        let chk = smul(&md, &mut cfin, d, &st.p);
        if chk == st.q && d == st.d {
            d_ok = 1.0;
        }
    }
    let mut r = Rec::default();
    r.put("k", k as f64);
    r.put("m", m as f64);
    r.put("curve", ci as f64);
    r.put("inst", inst as f64);
    r.put("u", u as f64);
    r.put("nneed", nneed as f64);
    r.put("reg_success", if snap_reg.is_some() { 1.0 } else { 0.0 });
    r.put("reg_round", snap_reg.as_ref().map(|x| x.0 as f64).unwrap_or(-1.0));
    r.put("reg_fail", if reg_fail_snap.is_some() { 1.0 } else { 0.0 });
    r.put("rank_final", el.rank as f64);
    r.put("nvars", nv as f64);
    r.put("dd_found", if d_found.is_some() { 1.0 } else { 0.0 });
    r.put("d_ok", d_ok);
    r.put("inconsistent", el.inconsistent as f64);
    r.put("rc_total", rc as f64);
    r.put("targets_total", walked as f64);
    r.put("unusable_uniq", unusable as f64);
    r.put("f4_mis", arms.cu.f4_mis as f64);
    r.put("rel_bad", arms.cl.rel_bad as f64);
    r.put("final_g", cfin.g as f64);
    // always emit every column (zeros when the event did not happen) so that TSV rows stay column-aligned
    snap_rec(&mut r, "dd", &snap_dd.clone().unwrap_or_default());
    snap_rec(&mut r, "reg", &snap_reg.clone().map(|x| x.1).unwrap_or_default());
    r.put_cnt("u", &arms.cu);
    r.put_cnt("f", &arms.cf);
    r
}
