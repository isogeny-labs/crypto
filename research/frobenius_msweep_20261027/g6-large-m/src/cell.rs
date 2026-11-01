//! One cell (n, a, m): size census of the arms, frozen selection of feasible draws, paired enumeration, controls.
//! Everything frozen in PROTOCOL.md section 6 is a constant here.

use crate::arms::{self, Arm};
use crate::census;
use crate::curve::Curve;
use crate::field::Fq;
use crate::rng::{derive_seed, Rng};
use std::fmt::Write as _;

pub const DRAWS: usize = 3000; // size-census draws per arm per cell
pub const K_MAX: usize = 32; // paired NI/UP instances per cell
pub const K_AA: usize = 16; // UP2 (A/A) instances
pub const K_CTRL: usize = 8; // DUP (positive control) instances
pub const K_SEC: usize = 32; // NC / NR instances
pub const MIN_PAIRS: usize = 8; // fewer feasible draws than this: the cell is UNMATCHABLE
pub const LMIN: f64 = -10.0; // log2 mu window (before the hits floor)
pub const LMAX: f64 = 5.0;
pub const MIN_HITS: f64 = 1024.0; // expected number of group elements with H >= 1 at the lower edge
pub const CAP_LOG2_TUPLES: f64 = 28.0; // enumeration refuses more than 2^28 tuples
pub const TUPLE_BUDGET_LOG2: f64 = 33.0; // per-cell budget: largest k_eff <= K_MAX with cell_tuple_units(k_eff) * 2^median(log2 T) <= 2^33 (floor MIN_PAIRS)
pub const SEC_WINDOW_FACTOR: f64 = 1.4143; // NC/NR instances sit up to WINDOW_HALF = 0.5 bit above the median: 2^0.5
pub const WINDOW_HALF: f64 = 0.5; // secondary arms: |log2 mu - median(NI)| <= this
pub const PLANTS: usize = 256;
pub const SEED_DEFAULT: u64 = 20261027;

pub struct CellParams {
    pub n: u32,
    pub a: u32,
    pub m: usize,
    pub draws: usize,
    pub k: usize,
    pub seed: u64,
    pub out: String,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub arm: String,
    pub idx: u64,
    pub l2mu: f64,
    pub log2t: f64,
    pub p: f64,
    pub eff: f64,
    pub disp: f64,
    pub fano: f64,
    pub fano_tamper: f64,
    pub planted: usize,
    pub conserved: bool,
    pub a_len: usize,
    pub b_len: usize,
    pub adds_build: u64,
    /// digit-model prediction of p and disp (NaN when the slot system is not a Frobenius chain; PROTOCOL section 4.4)
    pub pred_p: f64,
    pub pred_disp: f64,
}

pub const ROW_HEADER: &str =
    "arm\tidx\tl2mu\tlog2T\tp\teff\tdisp\tfano\tfano_tamper\tplanted\tconserved\ta_len\tb_len\tadds_build\tpred_p\tpred_disp";

impl Row {
    pub fn tsv(&self) -> String {
        format!(
            "{}\t{}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{:.9e}\t{}\t{}\t{}\t{}\t{}\t{:.12e}\t{:.12e}",
            self.arm, self.idx, self.l2mu, self.log2t, self.p, self.eff, self.disp, self.fano, self.fano_tamper,
            self.planted, self.conserved as u8, self.a_len, self.b_len, self.adds_build, self.pred_p, self.pred_disp
        )
    }
}

fn median(v: &mut Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

/// Enumerate one instance and compute every statistic and the instrument checks.
pub fn enumerate(
    c: &Curve,
    om: &census::OrbitMap,
    ne: u32,
    slots: &[Vec<crate::curve::Pt>],
    arm: &str,
    idx: u64,
    h: &mut Vec<u32>,
    rng: &mut Rng,
) -> Row {
    let cen = census::histogram_into(c, slots, h);
    let st = census::stats(h, ne, cen.t);
    let planted = census::planted_hits(c, slots, h, PLANTS, rng);
    let pred = crate::digits::predict_slots(c, slots, ne);
    Row {
        arm: arm.to_string(),
        idx,
        l2mu: st.mu.log2(),
        log2t: (cen.t as f64).log2(),
        p: st.p,
        eff: st.eff,
        disp: st.disp,
        fano: census::orbit_fano(h, om, false),
        fano_tamper: census::orbit_fano(h, om, true),
        planted,
        conserved: st.conserved,
        a_len: cen.a_len,
        b_len: cen.b_len,
        adds_build: cen.adds_build,
        pred_p: pred.map_or(f64::NAN, |q| q.p),
        pred_disp: pred.map_or(f64::NAN, |q| q.disp),
    }
}

pub fn run_cell(p: &CellParams) -> Result<String, String> {
    let n = p.n as usize;
    if p.m < 2 || p.m >= n {
        return Err(format!(
            "m={} must satisfy 2 <= m < n={n} (m >= n is a Burnside artefact)",
            p.m
        ));
    }
    let f = Fq::new(p.n);
    let c = Curve::koblitz(&f, p.a);
    let ne = c.count();
    let lne = (ne as f64).log2();
    let id = format!("n{}a{}m{}", p.n, p.a, p.m);
    let lmin = LMIN.max((MIN_HITS / ne as f64).log2());
    let lmax = LMAX;
    let mut meta = String::new();
    writeln!(meta, "cell\t{id}").unwrap();
    writeln!(
        meta,
        "n\t{}\na\t{}\nm\t{}\nE\t{ne}\nseed\t{}\ndraws\t{}",
        p.n, p.a, p.m, p.seed, p.draws
    )
    .unwrap();
    writeln!(meta, "lmin\t{lmin:.4}\nlmax\t{lmax:.4}").unwrap();

    // ---- size census (no group sums) ----
    let mut sizes_tsv = String::from("arm\tidx\tl2mu\tfeasible\n");
    let mut ni: Vec<(u64, f64)> = Vec::new();
    let mut u_feasible = 0usize;
    let mut sec: Vec<(Arm, Vec<(u64, f64)>)> = vec![(Arm::NC, vec![]), (Arm::NR, vec![])];
    for idx in 0..p.draws as u64 {
        let slots = arms::build_normal(&f, &c, Arm::NI, p.m, derive_seed(p.seed, &id, "NI", idx));
        let l = arms::log2_tuples(&arms::sizes(&slots)) - lne;
        writeln!(
            sizes_tsv,
            "NI\t{idx}\t{l:.6}\t{}",
            (l >= lmin && l <= lmax) as u8
        )
        .unwrap();
        ni.push((idx, l));
        let ug = arms::build_generic(&f, &c, p.m, derive_seed(p.seed, &id, "U", idx));
        let lu = arms::log2_tuples(&arms::sizes(&ug)) - lne;
        writeln!(
            sizes_tsv,
            "U\t{idx}\t{lu:.6}\t{}",
            (lu >= lmin && lu <= lmax) as u8
        )
        .unwrap();
        u_feasible += (lu >= lmin && lu <= lmax) as usize;
        for (arm, v) in sec.iter_mut() {
            let s =
                arms::build_normal(&f, &c, *arm, p.m, derive_seed(p.seed, &id, arm.name(), idx));
            let l = arms::log2_tuples(&arms::sizes(&s)) - lne;
            writeln!(
                sizes_tsv,
                "{}\t{idx}\t{l:.6}\t{}",
                arm.name(),
                (l >= lmin && l <= lmax) as u8
            )
            .unwrap();
            v.push((idx, l));
        }
    }
    let feasible_ni: Vec<(u64, f64)> = ni
        .iter()
        .cloned()
        .filter(|&(_, l)| l >= lmin && l <= lmax)
        .collect();
    writeln!(meta, "feasible_NI\t{}", feasible_ni.len()).unwrap();
    writeln!(meta, "feasible_U\t{u_feasible}").unwrap();
    for (arm, v) in &sec {
        let k = v.iter().filter(|&&(_, l)| l >= lmin && l <= lmax).count();
        writeln!(meta, "feasible_{}\t{k}", arm.name()).unwrap();
    }
    std::fs::create_dir_all(&p.out).map_err(|e| e.to_string())?;
    std::fs::write(format!("{}/sizes_{id}.tsv", p.out), &sizes_tsv).map_err(|e| e.to_string())?;
    // adaptive pair count: bounded total enumeration work per cell (frozen rule, PROTOCOL section 6)
    let k_eff = if feasible_ni.is_empty() {
        0
    } else {
        let mut lt: Vec<f64> = feasible_ni.iter().map(|&(_, l)| l + lne).collect();
        let med_t = median(&mut lt);
        let mut k = p.k;
        while k > MIN_PAIRS
            && cell_tuple_units(k) * (2f64).powf(med_t) > (2f64).powf(TUPLE_BUDGET_LOG2)
        {
            k -= 1;
        }
        k
    };
    writeln!(meta, "k_eff\t{k_eff}").unwrap();
    let (k_aa, k_ctrl, k_sec) = (K_AA.min(k_eff), K_CTRL.min(k_eff), K_SEC.min(k_eff));
    let selected: Vec<(u64, f64)> = feasible_ni.iter().cloned().take(k_eff).collect();
    writeln!(meta, "selected_pairs\t{}", selected.len()).unwrap();
    if selected.len() < MIN_PAIRS {
        writeln!(meta, "status\tUNMATCHABLE").unwrap();
        std::fs::write(format!("{}/meta_{id}.txt", p.out), &meta).map_err(|e| e.to_string())?;
        std::fs::write(
            format!("{}/rows_{id}.tsv", p.out),
            format!("{ROW_HEADER}\n"),
        )
        .map_err(|e| e.to_string())?;
        return Ok(format!(
            "{id} UNMATCHABLE feasible_NI={}",
            feasible_ni.len()
        ));
    }

    // ---- enumeration ----
    let om = census::build_orbits(&c);
    let mut h: Vec<u32> = Vec::new();
    let mut rows: Vec<Row> = Vec::new();
    let mut skipped = String::new();
    let mut rng = Rng::new(derive_seed(p.seed, &id, "plant", 0));
    let cap = CAP_LOG2_TUPLES;
    for (rank, &(idx, _)) in selected.iter().enumerate() {
        let slots = arms::build_normal(&f, &c, Arm::NI, p.m, derive_seed(p.seed, &id, "NI", idx));
        let prof = arms::sizes(&slots);
        rows.push(enumerate(&c, &om, ne, &slots, "NI", idx, &mut h, &mut rng));
        match arms::build_matched_generic(&f, &c, p.m, derive_seed(p.seed, &id, "UP", idx), &prof) {
            None => writeln!(skipped, "UP_BUILD_FAILED idx={idx}").unwrap(),
            Some(up) => {
                rows.push(enumerate(&c, &om, ne, &up, "UP", idx, &mut h, &mut rng));
                if rank < k_ctrl {
                    let d = arms::build_dup(&up);
                    if d == up {
                        writeln!(
                            skipped,
                            "DUP_NOOP idx={idx} (the duplicated slots have one point each)"
                        )
                        .unwrap();
                    } else if arms::log2_tuples(&arms::sizes(&d)) <= cap {
                        rows.push(enumerate(&c, &om, ne, &d, "DUP", idx, &mut h, &mut rng));
                    } else {
                        writeln!(skipped, "DUP_OVER_CAP idx={idx}").unwrap();
                    }
                }
            }
        }
        if rank < k_aa {
            if let Some(u2) = arms::build_matched_generic(
                &f,
                &c,
                p.m,
                derive_seed(p.seed, &id, "UP2", idx),
                &prof,
            ) {
                rows.push(enumerate(&c, &om, ne, &u2, "UP2", idx, &mut h, &mut rng));
            } else {
                writeln!(skipped, "UP2_BUILD_FAILED idx={idx}").unwrap();
            }
        }
    }
    // secondary arms: unpaired, same mu window as the NI selection
    let mut ls: Vec<f64> = selected.iter().map(|&(_, l)| l).collect();
    let med = median(&mut ls);
    writeln!(meta, "window_center\t{med:.4}").unwrap();
    for (arm, v) in &sec {
        let pick: Vec<u64> = v
            .iter()
            .filter(|&&(_, l)| l >= lmin && l <= lmax && (l - med).abs() <= WINDOW_HALF)
            .map(|&(i, _)| i)
            .take(k_sec)
            .collect();
        writeln!(meta, "selected_{}\t{}", arm.name(), pick.len()).unwrap();
        if pick.len() < MIN_PAIRS {
            writeln!(
                skipped,
                "{}_NOT_RUN window has {} draws",
                arm.name(),
                pick.len()
            )
            .unwrap();
            continue;
        }
        for idx in pick {
            let slots =
                arms::build_normal(&f, &c, *arm, p.m, derive_seed(p.seed, &id, arm.name(), idx));
            rows.push(enumerate(
                &c,
                &om,
                ne,
                &slots,
                arm.name(),
                idx,
                &mut h,
                &mut rng,
            ));
        }
    }
    // the type-II ONB (single instance) and its profile-matched generic partner
    if let Some(slots) = arms::build_onb(&f, &c, p.m) {
        let sz = arms::sizes(&slots);
        let t = arms::log2_tuples(&sz);
        writeln!(meta, "onb_l2mu\t{:.4}", t - lne).unwrap();
        if t <= cap {
            rows.push(enumerate(&c, &om, ne, &slots, "ONB", 0, &mut h, &mut rng));
            match arms::build_matched_generic(
                &f,
                &c,
                p.m,
                derive_seed(p.seed, &id, "ONBUP", 0),
                &sz,
            ) {
                Some(up) => rows.push(enumerate(&c, &om, ne, &up, "ONBUP", 0, &mut h, &mut rng)),
                None => writeln!(skipped, "ONBUP_BUILD_FAILED").unwrap(),
            }
        } else {
            writeln!(skipped, "ONB_OVER_CAP log2T={t:.2}").unwrap();
        }
    } else {
        writeln!(meta, "onb\tnot available at this n").unwrap();
    }
    writeln!(meta, "status\tOK").unwrap();
    for l in skipped.lines() {
        writeln!(meta, "skipped\t{l}").unwrap();
    }
    let mut body = format!("{ROW_HEADER}\n");
    for r in &rows {
        body.push_str(&r.tsv());
        body.push('\n');
    }
    std::fs::write(format!("{}/rows_{id}.tsv", p.out), body).map_err(|e| e.to_string())?;
    std::fs::write(format!("{}/meta_{id}.txt", p.out), &meta).map_err(|e| e.to_string())?;
    Ok(format!(
        "{id} OK rows={} feasible_NI={}",
        rows.len(),
        feasible_ni.len()
    ))
}

/// The frozen cell list: (n, a, m).
pub fn cell_list() -> Vec<(u32, u32, usize)> {
    let mut v = vec![];
    for &m in &[9usize, 10, 11, 12, 14, 16, 18, 20] {
        v.push((23, 0, m));
    }
    for &m in &[9usize, 10, 11, 12, 14, 16, 18] {
        v.push((19, 0, m));
        v.push((19, 1, m));
    }
    // slot-size control (PROTOCOL section 4.5): fixed m, smaller n, so that slot size varies WITHOUT m varying
    for &n in &[13u32, 17] {
        for &m in &[9usize, 10] {
            v.push((n, 0, m));
        }
    }
    v
}

/// Enumerated instances of a cell with k pairs, in units of "median-T instances" (NI, UP, DUP, UP2, NC, NR, ONB pair), with the
/// NC/NR window factor applied to those 2 k_sec instances.
pub fn cell_tuple_units(k: usize) -> f64 {
    let main = 2 * k + K_CTRL.min(k) + K_AA.min(k) + 2;
    main as f64 + 2.0 * K_SEC.min(k) as f64 * SEC_WINDOW_FACTOR
}

/// Upper bound of instances per cell and of tuples enumerated per instance.
pub fn instances_per_cell() -> usize {
    K_MAX + K_MAX + K_CTRL + K_AA + 2 * K_SEC + 2
}
