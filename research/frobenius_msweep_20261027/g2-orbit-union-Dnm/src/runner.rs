//! One cell = (curve a, n, m, l): NV independent base draws per arm, NT shared
//! targets, all arms through the same enumerator.

use std::time::Instant;

use crate::combi::{shift_pattern, PatternTable};
use crate::curve::{cofactor, koblitz_order, Curve, INF};
use crate::enumr::Enumerator;
use crate::family::{self, ARMS};
use crate::field::Fq;
use crate::{derive_seed, Rng};

#[derive(Clone, Debug)]
pub struct Cfg {
    pub a: u32,
    pub n: u32,
    pub m: usize,
    pub l: usize,
    pub nv: usize,
    pub nt: usize,
    pub seed: u64,
    pub arms: Vec<String>,
    pub max_left: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Row {
    pub a: u32,
    pub n: u32,
    pub m: usize,
    pub l: usize,
    pub arm: String,
    pub vd: usize,
    pub nt: usize,
    pub nb: usize,
    pub d: usize,
    pub order: u64,
    pub npts: usize,
    pub nleft: usize,
    pub total: u64,
    pub unamb: u64,
    pub overflow: u64,
    pub audit: i32,
    pub ms: f64,
    pub xs: Vec<u64>,
    pub ycov: Vec<f64>,
    pub yfrac: Vec<f64>,
    pub yrep: Vec<f64>,
}

pub const HEADER: &str =
    "a\tn\tm\tl\tarm\tvd\tnt\tnb\tD\torder\tnpts\tnleft\ttotal\tunamb\toverflow\taudit\tms\txs\tycov\tyfrac\tyrep";

fn join<T: ToString>(v: &[T]) -> String {
    v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
}
fn split<T: std::str::FromStr>(s: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    if s.is_empty() {
        return vec![];
    }
    s.split(',').map(|x| x.parse().unwrap()).collect()
}

impl Row {
    pub fn to_tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.3}\t{}\t{}\t{}\t{}",
            self.a, self.n, self.m, self.l, self.arm, self.vd, self.nt, self.nb, self.d, self.order,
            self.npts, self.nleft, self.total, self.unamb, self.overflow, self.audit, self.ms,
            join(&self.xs), join(&self.ycov), join(&self.yfrac), join(&self.yrep)
        )
    }
    pub fn from_tsv(line: &str) -> Row {
        let t: Vec<&str> = line.split('\t').collect();
        assert_eq!(t.len(), 21, "bad row");
        Row {
            a: t[0].parse().unwrap(),
            n: t[1].parse().unwrap(),
            m: t[2].parse().unwrap(),
            l: t[3].parse().unwrap(),
            arm: t[4].to_string(),
            vd: t[5].parse().unwrap(),
            nt: t[6].parse().unwrap(),
            nb: t[7].parse().unwrap(),
            d: t[8].parse().unwrap(),
            order: t[9].parse().unwrap(),
            npts: t[10].parse().unwrap(),
            nleft: t[11].parse().unwrap(),
            total: t[12].parse().unwrap(),
            unamb: t[13].parse().unwrap(),
            overflow: t[14].parse().unwrap(),
            audit: t[15].parse().unwrap(),
            ms: t[16].parse().unwrap(),
            xs: split(t[17]),
            ycov: split(t[18]),
            yfrac: split(t[19]),
            yrep: split(t[20]),
        }
    }
}

pub fn arm_code(arm: &str) -> u64 {
    ARMS.iter().position(|&a| a == arm).expect("arm") as u64 + 1
}

/// Targets: R = h * P for uniformly random affine P (h = cofactor), so R lies
/// in the odd-or-2-power-free subgroup hE; R = O is skipped.
pub fn make_targets(f: &Fq, c: &Curve, a: u32, nt: usize, seed: u64) -> Vec<crate::curve::P> {
    let pts = c.all_points(f);
    let mut rng = Rng(seed);
    let mut out = vec![];
    while out.len() < nt {
        let p = pts[rng.below(pts.len() as u64) as usize];
        let r = c.scalar(f, p, cofactor(a));
        if r != INF {
            out.push(r);
        }
    }
    out
}

pub fn liftable_counts(f: &Fq, c: &Curve, fam: &family::Family) -> Vec<u64> {
    fam.blocks
        .iter()
        .map(|b| b.iter().filter(|&&x| c.lift(f, x).is_some()).count() as u64)
        .collect()
}

/// Does count(sigma R, pattern + 1) == count(R, pattern) hold for every pattern?
pub fn gauge_identity_holds(
    e: &Enumerator,
    c: &Curve,
    f: &Fq,
    pt: &PatternTable,
    r: crate::curve::P,
) -> bool {
    let n = pt.nb;
    let c0 = e.count_target(r, pt);
    let c1 = e.count_target(c.sigma(f, r), pt);
    for (i, p) in pt.pats.iter().enumerate() {
        let q = shift_pattern(p, 1, n);
        let j = pt.index[&crate::combi::pack(&q)];
        if c0.cover[i] != c1.cover[j] {
            return false;
        }
    }
    c0.total == c1.total
}

pub fn run_cell(cfg: &Cfg, mut sink: impl FnMut(&Row)) {
    let f = Fq::new(cfg.n);
    let c = Curve::koblitz(cfg.a);
    let order = koblitz_order(cfg.a, cfg.n);
    assert_eq!(c.all_points(&f).len() as u64 + 1, order, "point count != Koblitz formula");
    let n = cfg.n as usize;
    let targets = make_targets(&f, &c, cfg.a, cfg.nt, derive_seed(cfg.seed, &[cfg.a as u64, cfg.n as u64, cfg.m as u64, cfg.l as u64, 0x7A12]));
    let pt_union = PatternTable::new(n, cfg.m, true);
    let pt_one = PatternTable::new(1, cfg.m, false);
    for vd in 0..cfg.nv {
        // arm order is shuffled per draw (interleaving); seeds do not depend on the order
        let mut arms: Vec<&String> = cfg.arms.iter().collect();
        let mut orng = Rng(derive_seed(cfg.seed, &[cfg.n as u64, cfg.m as u64, vd as u64, 0x0DE7]));
        for i in (1..arms.len()).rev() {
            arms.swap(i, orng.below(i as u64 + 1) as usize);
        }
        for arm in arms {
            let t0 = Instant::now();
            let mut rng = Rng(derive_seed(
                cfg.seed,
                &[cfg.a as u64, cfg.n as u64, cfg.m as u64, cfg.l as u64, arm_code(arm), vd as u64],
            ));
            let fam = family::build(arm, &f, &mut rng, cfg.l);
            let pt = if fam.nb == 1 { &pt_one } else { &pt_union };
            let e = Enumerator::new(&f, &c, &fam, cfg.m, cfg.max_left);
            let d = pt.n_classes();
            let (mut ycov, mut yfrac, mut yrep) = (vec![0.0; d], vec![0.0; d], vec![0.0; d]);
            let (mut total, mut unamb, mut overflow) = (0u64, 0u64, 0u64);
            for &r in &targets {
                let k = e.count_target(r, pt);
                total += k.total;
                unamb += k.unamb;
                overflow += k.overflow;
                for (ci, ps) in pt.class_pats.iter().enumerate() {
                    for &p in ps {
                        ycov[ci] += k.cover[p] as f64;
                        yfrac[ci] += k.frac[p];
                    }
                    yrep[ci] += k.cover[pt.rep[ci]] as f64;
                }
            }
            let audit = if vd == 0 && fam.nb == n && matches!(arm.as_str(), "frob" | "geom" | "rsub" | "rset") {
                targets.iter().take(3).all(|&r| gauge_identity_holds(&e, &c, &f, pt, r)) as i32
            } else {
                -1
            };
            let row = Row {
                a: cfg.a,
                n: cfg.n,
                m: cfg.m,
                l: cfg.l,
                arm: arm.clone(),
                vd,
                nt: cfg.nt,
                nb: fam.nb,
                d,
                order,
                npts: e.n_points(),
                nleft: e.n_left(),
                total,
                unamb,
                overflow,
                audit,
                ms: t0.elapsed().as_secs_f64() * 1e3,
                xs: liftable_counts(&f, &c, &fam),
                ycov,
                yfrac,
                yrep,
            };
            sink(&row);
        }
    }
}
