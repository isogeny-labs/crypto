//! Cell plan, per-base measurement and the run drivers (one TSV per cell). Every arm of a cell goes through
//! the same `measure` function; arms are interleaved per replicate in a seeded random order with derived seeds.

use crate::bases::*;
use crate::census::*;
use crate::chain::member;
use crate::curve::{build_group, curve_summary, Group};
use crate::field::Fq;
use crate::proptt::*;
use crate::rank::*;
use crate::rng::{derive_seed, Rng};
use std::io::Write as _;
use std::time::Instant;

/// (a, n): the four primary curves (r = 991, 2003, 65587, 130873) and one secondary whose order is decided by the gate.
pub const CURVES: [(u32, u32); 5] = [(1, 11), (0, 13), (1, 17), (0, 19), (1, 19)];
pub const BASE_OPS_CAP: f64 = 3.0e8;
pub const NU_TARGET: f64 = 4.0;
pub const SEED_DEFAULT: u64 = 20261027;

#[derive(Clone, Debug)]
pub struct Cell {
    pub a: u32,
    pub n: u32,
    pub m: usize,
    pub l: u32,
    pub reps: usize,
}

impl Cell {
    pub fn name(&self) -> String {
        format!("a{}_n{}_m{}_l{}", self.a, self.n, self.m, self.l)
    }
}

pub fn nu_exp(nx: f64, m: usize, r: u32) -> f64 {
    // continuous binomial via rounding nx
    binom(nx.round() as usize, m) * (1u64 << m) as f64 / (r as f64 - 1.0)
}

/// Smallest N with nu_exp(N, m) >= NU_TARGET.
pub fn tuned_size(m: usize, r: u32) -> usize {
    let mut n = m + 1;
    while nu_exp(n as f64, m, r) < NU_TARGET {
        n += 1;
    }
    n
}

/// P(X < m) for X ~ Binomial(2^l - 1, dens): the chance that a random l-dimensional subspace holds fewer than m
/// factor-base classes (such a base is skipped as TOO_FEW_CLASSES, which censors the cell).
pub fn p_too_few(l: u32, m: usize, dens: f64) -> f64 {
    let nn = ((1u64 << l) - 1) as f64;
    let mut total = 0.0;
    for k in 0..m {
        let mut lnc = 0.0;
        for i in 0..k {
            lnc += ((nn - i as f64) / (i as f64 + 1.0)).ln();
        }
        let lp = lnc + k as f64 * dens.ln() + (nn - k as f64) * (1.0 - dens).ln();
        total += lp.exp();
    }
    total
}

/// Cells are dropped from the plan unless the censoring probability is at most this (PROTOCOL 4.2).
pub const MAX_P_TOO_FEW: f64 = 0.02;

pub fn cell_list() -> Vec<Cell> {
    let mut cells: Vec<Cell> = vec![];
    for &(a, n) in CURVES.iter() {
        let f = Fq::new(n);
        let (_ne, r, chk) = curve_summary(&f, a);
        if chk.is_err() {
            continue;
        }
        let q = f.q as f64;
        let dens = (r as f64 - 1.0) / (2.0 * q);
        let jn = ((n - 1) / 2) as usize;
        for m in 2..=8usize {
            let nx_of = |l: u32| ((1u64 << l) as f64 * dens).round();
            let mut chosen: Vec<u32> = vec![];
            let mut star: Option<u32> = None;
            for l in 3..=(n - 2) {
                let nx = nx_of(l);
                if nx < m as f64 {
                    continue;
                }
                let ops = est_ops(nx as usize, m, jn);
                if ops > BASE_OPS_CAP {
                    break;
                }
                if nu_exp(nx, m, r) >= 1.0
                    && star.is_none()
                    && p_too_few(l, m, dens) <= MAX_P_TOO_FEW
                {
                    star = Some(l);
                }
            }
            match star {
                Some(l) => {
                    chosen.push(l);
                    let nx2 = nx_of(l + 1);
                    if l + 1 <= n - 2
                        && est_ops(nx2 as usize, m, jn) <= BASE_OPS_CAP
                        && nu_exp(nx2, m, r) <= 3000.0
                    {
                        chosen.push(l + 1);
                    }
                }
                None => {
                    // nothing reaches nu >= 1 inside the cap: take the largest l inside the cap (low-yield cell)
                    let mut best = None;
                    for l in 3..=(n - 2) {
                        let nx = nx_of(l);
                        if nx >= m as f64
                            && est_ops(nx as usize, m, jn) <= BASE_OPS_CAP
                            && p_too_few(l, m, dens) <= MAX_P_TOO_FEW
                        {
                            best = Some(l);
                        }
                    }
                    if let Some(l) = best {
                        chosen.push(l);
                    }
                }
            }
            if a == 1 && n == 17 && (2..=4).contains(&m) && !chosen.contains(&8) {
                chosen.push(8); // stable dimension at n = 17
            }
            chosen.sort_unstable();
            chosen.dedup();
            for l in chosen {
                let nx = nx_of(l) as usize;
                let ops = est_ops(nx.max(m), m, jn).max(1.0);
                let reps = ((4.0e8 / ops) as usize).clamp(8, 200);
                cells.push(Cell { a, n, m, l, reps });
            }
        }
    }
    cells
}

/// Planning-unit operation count of one overlap invocation with `reps` replicates per half (est_ops counts two
/// multiplier sets; the run uses four, which the measured pre-flight ns/op absorbs because it is taken against the same
/// formula). Arms: poly (1 base), stable (2 bases at n = 17, l = 8), rand/rset/orbunion at the cell's size, tuned.
pub fn cell_ops(c: &Cell, reps: usize) -> f64 {
    let f = Fq::new(c.n);
    let (_ne, r, _) = curve_summary(&f, c.a);
    let nx = ((1u64 << c.l) as f64 * (r as f64 - 1.0) / (2.0 * f.q as f64)).round() as usize;
    let jn = ((c.n - 1) / 2) as usize;
    let ops = est_ops(nx.max(c.m), c.m, jn);
    let tops = est_ops(tuned_size(c.m, r), c.m, jn);
    let stable = if c.n == 17 && c.l == 8 { 2.0 } else { 0.0 };
    ops * (1.0 + stable) + reps as f64 * 2.0 * (3.0 * ops + tops)
}

// ---------------------------------------------------------------------------------------------------------------
pub const OVERLAP_COLS: [&str; 34] = [
    "a",
    "n",
    "m",
    "l",
    "r",
    "arm",
    "half",
    "rep",
    "seed",
    "status",
    "nx",
    "nsub",
    "nsol",
    "nu",
    "u_frob",
    "u_null",
    "u_pred",
    "corr_frob",
    "corr_null",
    "concord_frob",
    "concord_null",
    "p_solved",
    "count_equal_frob",
    "halving_n",
    "halving_bad",
    "chain_pos_n",
    "chain_pos_bad",
    "chain_neg_n",
    "chain_neg_bad",
    "lb_checked",
    "lb_bad",
    "clos_targets",
    "clos_pairs",
    "clos_closed",
];
pub const OVERLAP_COLS2: [&str; 23] = [
    "enum_ms",
    "u_by_j_frob",
    "u_by_j_null",
    "s1_nsub",
    "s1_nsol",
    "s1_uf",
    "s1_un",
    "s2_nsub",
    "s2_nsol",
    "s2_uf",
    "s2_un",
    "s3_nsub",
    "s3_nsol",
    "s3_uf",
    "s3_un",
    "u_null2",
    "u_plant",
    "plant_lags_ok",
    "s1l_nsub",
    "s1l_pairs",
    "s1l_cc1",
    "s1l_uf",
    "s1l_un",
];

fn member_any_order(f: &Fq, xr: u32, xs: &[u32], rng: &mut Rng) -> bool {
    if member(f, 1, xr, xs) {
        return true;
    }
    let mut v = xs.to_vec();
    for _ in 0..60 {
        rng.shuffle(&mut v);
        if member(f, 1, xr, &v) {
            return true;
        }
    }
    false
}

pub struct Ctx<'a> {
    pub f: &'a Fq,
    pub g: &'a Group,
    pub fr: &'a Mult,
    pub ob: &'a Orbits,
}

/// One base, one row. The same function serves every arm.
pub fn measure(
    ctx: &Ctx,
    base: &BaseSet,
    m: usize,
    null: &Mult,
    aux_seed: u64,
    rng: &mut Rng,
    lead: Vec<String>,
) -> Vec<String> {
    let g = ctx.g;
    let jn = ctx.fr.len();
    let mut row = lead;
    let nx = base.len();
    let ops = est_ops(nx.max(1), m, jn);
    let blank = |row: &mut Vec<String>, status: &str, nx: usize| {
        row.push(status.to_string());
        row.push(nx.to_string());
        for _ in 0..(OVERLAP_COLS.len() + OVERLAP_COLS2.len() - 11) {
            row.push("NA".into());
        }
    };
    if nx < m {
        blank(&mut row, "TOO_FEW_CLASSES", nx);
        return row;
    }
    if ops > BASE_OPS_CAP * 5.0 {
        blank(&mut row, "OVER_BUDGET", nx);
        return row;
    }
    let t0 = Instant::now();
    let orbs: Vec<u32> = base
        .ds
        .iter()
        .map(|&d| ctx.ob.orbit[(d - 1) as usize])
        .collect();
    let ks: Vec<u32> = base
        .ds
        .iter()
        .map(|&d| ctx.ob.k[(d - 1) as usize])
        .collect();
    let oi = OrbInfo {
        orbit: &orbs,
        k: Some(&ks),
        n: g.n,
    };
    // two further multiplier sets on the same enumeration: an independent second null (truth D_nn = 1, calibrates the
    // whole decision pipeline) and the planted positive control (at least one guaranteed hit per lag)
    let mut null2_rng = Rng::new(crate::rng::mix(aux_seed, 0x6E32));
    let null2 = Mult::random_null(g.r, jn, &mut null2_rng);
    let mut plant_rng = Rng::new(crate::rng::mix(aux_seed, 0x706C));
    let plant = Mult::planted(g.r, &base.ds, m, jn, &mut plant_rng);
    let plant_ref = plant.as_ref().unwrap_or(&null2);
    let out = census_ex(
        &base.ds,
        g.r,
        m,
        &[ctx.fr, null, &null2, plant_ref],
        16,
        8,
        Some(&oi),
    );
    let plant_lags_ok = if plant.is_some() {
        out.u[3].iter().filter(|&&h| h >= 1).count()
    } else {
        0
    };
    let enum_ms = t0.elapsed().as_secs_f64() * 1000.0;
    let ind_f = ind_stats(&out.cnt, ctx.fr);
    let ind_n = ind_stats(&out.cnt, null);
    let mean = |v: Vec<f64>| -> f64 {
        let ok: Vec<f64> = v.into_iter().filter(|x| x.is_finite()).collect();
        if ok.is_empty() {
            f64::NAN
        } else {
            ok.iter().sum::<f64>() / ok.len() as f64
        }
    };
    let corr_f = mean(
        ind_f
            .joint
            .iter()
            .map(|&j| corr(ind_f.p, j, ind_f.ncls))
            .collect(),
    );
    let corr_n = mean(
        ind_n
            .joint
            .iter()
            .map(|&j| corr(ind_n.p, j, ind_n.ncls))
            .collect(),
    );
    let conc_f = ind_f.concord.iter().sum::<u64>() as f64 / (ind_f.ncls as f64 * jn as f64);
    let conc_n = ind_n.concord.iter().sum::<u64>() as f64 / (ind_n.ncls as f64 * jn as f64);
    let ceq_f = ind_f.count_equal.iter().sum::<u64>() as f64 / (ind_f.ncls as f64 * jn as f64);
    // instrument checks on the recorded samples
    let (mut hn, mut hb) = (0u32, 0u32);
    let (mut cpn, mut cpb, mut cnn, mut cnb) = (0u32, 0u32, 0u32, 0u32);
    for (si, s) in out.samples.iter().enumerate() {
        let (pairs, ok) = halving_check(&base.ds, g.r, &s.idx, s.a, s.b, ctx.fr.mus[s.t]);
        hn += pairs;
        if !ok {
            hb += 1;
        }
        if si < 6 {
            let xs: Vec<u32> = s.idx.iter().map(|&k| base.xs[k]).collect();
            for t in [s.a, s.b] {
                cpn += 1;
                if !member_any_order(ctx.f, g.xs[(t - 1) as usize], &xs, rng) {
                    cpb += 1;
                }
            }
        }
    }
    for (idx, pos, neg) in out.sol_samples.iter() {
        let xs: Vec<u32> = idx.iter().map(|&k| base.xs[k]).collect();
        cpn += 1;
        if !member_any_order(ctx.f, g.xs[(*pos - 1) as usize], &xs, rng) {
            cpb += 1;
        }
        cnn += 1;
        if member(ctx.f, 1, g.xs[(*neg - 1) as usize], &xs) {
            cnb += 1;
        }
    }
    // shared-leg lower bound: joint(j) >= #{a : exists S inside X_j solving a}
    let (mut lbn, mut lbb) = (0u32, 0u32);
    if !base.is_sigma_closed(ctx.f) {
        for j in 1..=(g.n - 1) / 2 {
            let sj = base.shared_with_shift(ctx.f, g, j);
            if sj.len() >= m && est_ops(sj.len(), m, 0) <= 2.0e6 {
                let cj = solution_counts(&sj.ds, g.r, m);
                let lowers = (1..cj.len()).filter(|&a| cj[a] > 0).count() as u64;
                lbn += 1;
                if ind_f.joint[(j - 1) as usize] < lowers {
                    lbb += 1;
                }
            }
        }
    }
    let (ct, cp, cc) = if out.nsol <= 20_000 {
        let (a, b, c) = sigma_closure(base, ctx.f, g.r, m);
        (a.to_string(), b.to_string(), c.to_string())
    } else {
        ("NA".into(), "NA".into(), "NA".into())
    };
    let half = g.half() as f64;
    let upred = u_pred(&out, g.r, jn);
    row.push("OK".into());
    row.push(nx.to_string());
    row.push(out.nsub.to_string());
    row.push(out.nsol.to_string());
    row.push(format!("{:.6}", out.nsol as f64 / half));
    row.push(out.u[0].iter().sum::<u64>().to_string());
    row.push(out.u[1].iter().sum::<u64>().to_string());
    row.push(format!("{:.4}", upred));
    row.push(format!("{:.6}", corr_f));
    row.push(format!("{:.6}", corr_n));
    row.push(format!("{:.6}", conc_f));
    row.push(format!("{:.6}", conc_n));
    row.push(format!("{:.6}", ind_f.p));
    row.push(format!("{:.6}", ceq_f));
    row.push(hn.to_string());
    row.push(hb.to_string());
    row.push(cpn.to_string());
    row.push(cpb.to_string());
    row.push(cnn.to_string());
    row.push(cnb.to_string());
    row.push(lbn.to_string());
    row.push(lbb.to_string());
    row.push(ct);
    row.push(cp);
    row.push(cc);
    row.push(format!("{:.2}", enum_ms));
    row.push(
        out.u[0]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    );
    row.push(
        out.u[1]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(","),
    );
    for sx in 0..3 {
        row.push(out.strat_nsub[sx].to_string());
        row.push(out.strat_nsol[sx].to_string());
        row.push(out.strat_u[0][sx].to_string());
        row.push(out.strat_u[1][sx].to_string());
    }
    row.push(out.u[2].iter().sum::<u64>().to_string());
    row.push(if plant.is_some() {
        out.u[3].iter().sum::<u64>().to_string()
    } else {
        "NA".into()
    });
    row.push(if plant.is_some() {
        plant_lags_ok.to_string()
    } else {
        "NA".into()
    });
    row.push(out.s1l_nsub.to_string());
    row.push(out.s1l_pairs.to_string());
    row.push(out.s1l_cc1.to_string());
    row.push(out.s1l_u[0].to_string());
    row.push(out.s1l_u[1].to_string());
    row
}

fn open_out(dir: &str, file: &str, header: &[&str]) -> Result<std::fs::File, String> {
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = format!("{dir}/{file}");
    let mut fh = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    writeln!(fh, "{}", header.join("\t")).map_err(|e| e.to_string())?;
    Ok(fh)
}

pub struct RunOpts {
    pub seed: u64,
    pub reps: Option<usize>,
    pub out: String,
}

/// Overlap census of one cell: arms poly, stable (n = 17, l = 8), rand (main + aa halves), rset (paired size),
/// orbunion (sigma-closed union of whole orbits, paired size), tuned (yield-matched random class set, main + aa halves). Frobenius and null multipliers share each base.
pub fn run_overlap(cell: &Cell, o: &RunOpts) -> Result<String, String> {
    let f = Fq::new(cell.n);
    let g = build_group(&f, cell.a)?;
    let fr = Mult::frobenius(&g);
    let jn = fr.len();
    let ob = build_orbits(&f, &g);
    let ctx = Ctx {
        f: &f,
        g: &g,
        fr: &fr,
        ob: &ob,
    };
    let cname = cell.name();
    let reps = o.reps.unwrap_or(cell.reps);
    let tuned_n = tuned_size(cell.m, g.r);
    let mut hdr: Vec<&str> = OVERLAP_COLS.to_vec();
    hdr.extend_from_slice(&OVERLAP_COLS2);
    let file = format!("overlap_{cname}.tsv");
    let mut fh = open_out(&o.out, &file, &hdr)?;
    let stable: Vec<Vec<u32>> = if cell.n == 17 && cell.l == 8 {
        stable_subspaces(&f, 8)
    } else {
        vec![]
    };
    // work items: (arm, half, rep)
    let mut items: Vec<(&'static str, &'static str, usize)> = vec![];
    items.push(("poly", "main", 0));
    for (i, _) in stable.iter().enumerate() {
        items.push(("stable", "main", i));
    }
    for rep in 0..reps {
        for half in ["main", "aa"] {
            for arm in ["rand", "rset", "orbunion", "tuned"] {
                items.push((arm, half, rep));
            }
        }
    }
    // interleave: shuffle within each (half, rep) group, keeping the single-instance arms first
    let mut order_rng = Rng::new(derive_seed(o.seed, &cname, "order", "-", 0));
    let (single, rest): (Vec<_>, Vec<_>) = items
        .into_iter()
        .partition(|it| it.0 == "poly" || it.0 == "stable");
    let mut groups: Vec<Vec<(&'static str, &'static str, usize)>> = vec![];
    for chunk in rest.chunks(4) {
        let mut c = chunk.to_vec();
        order_rng.shuffle(&mut c);
        groups.push(c);
    }
    let mut seq: Vec<(&'static str, &'static str, usize)> = single;
    for gp in groups {
        seq.extend(gp);
    }
    // the paired rset needs the rand base of the same (half, rep): rebuild it from its own derived seed
    let rand_base = |half: &str, rep: usize| -> BaseSet {
        let mut rng = Rng::new(derive_seed(o.seed, &cname, "rand", half, rep as u64));
        let basis = random_subspace(&f, cell.l, &mut rng);
        BaseSet::from_elems(&g, &span(&basis))
    };
    for (arm, half, rep) in seq {
        let seed = derive_seed(o.seed, &cname, arm, half, rep as u64);
        let mut rng = Rng::new(seed);
        let base = match arm {
            "poly" => BaseSet::from_elems(&g, &span(&poly_window(cell.l))),
            "stable" => BaseSet::from_elems(&g, &span(&stable[rep])),
            "rand" => rand_base(half, rep),
            "rset" => {
                let nx = rand_base(half, rep).len();
                random_class_set(&g, nx, &mut rng)
            }
            "orbunion" => {
                let nx = rand_base(half, rep).len();
                let t = ((nx as f64 / cell.n as f64).round() as usize).max(1);
                orbit_union(&f, &g, t, &mut rng)
            }
            "tuned" => random_class_set(&g, tuned_n, &mut rng),
            _ => unreachable!(),
        };
        let mut null_rng = Rng::new(mix_null(seed));
        let null = Mult::random_null(g.r, jn, &mut null_rng);
        let lead = vec![
            cell.a.to_string(),
            cell.n.to_string(),
            cell.m.to_string(),
            cell.l.to_string(),
            g.r.to_string(),
            arm.to_string(),
            half.to_string(),
            rep.to_string(),
            seed.to_string(),
        ];
        let row = measure(
            &ctx,
            &base,
            cell.m,
            &null,
            mix_null(seed) ^ 0x1234,
            &mut rng,
            lead,
        );
        writeln!(fh, "{}", row.join("\t")).map_err(|e| e.to_string())?;
    }
    Ok(format!("{}/{}", o.out, file))
}

fn mix_null(seed: u64) -> u64 {
    crate::rng::mix(seed, 0x6E75_6C6C)
}

// ---------------------------------------------------------------------------------------------------------------
pub const PROPT_COLS: [&str; 20] = [
    "n",
    "m",
    "arm",
    "half",
    "block",
    "seed",
    "trials",
    "assigns",
    "trials_eq_fail",
    "assigns_eq_fail",
    "trials_bool_fail",
    "rec_singular",
    "planted",
    "plant_bad",
    "conj_member_n",
    "conj_member_fail",
    "dom_n",
    "dom_fail",
    "neg_n",
    "neg_disagree",
];

/// Proposition T cell (n, m): arms koblitz, wrongdir, ordinary (+ stable at n = 17), two halves, blocks of trials.
pub fn run_propt(
    n: u32,
    m: usize,
    trials: usize,
    k_assign: usize,
    o: &RunOpts,
) -> Result<String, String> {
    let f = Fq::new(n);
    let cname = format!("propt_n{n}_m{m}");
    let mut hdr: Vec<&str> = PROPT_COLS.to_vec();
    hdr.extend_from_slice(&["stable_n", "stable_cob_fail"]);
    let file = format!("{cname}.tsv");
    let mut fh = open_out(&o.out, &file, &hdr)?;
    let stable = if n == 17 {
        stable_subspaces(&f, 8)
    } else {
        vec![]
    };
    let mut arms: Vec<Arm> = vec![Arm::Koblitz, Arm::WrongDir, Arm::Ordinary];
    if !stable.is_empty() {
        arms.push(Arm::Stable);
    }
    let block = 500usize;
    let nblocks = (trials + block - 1) / block;
    let mut order_rng = Rng::new(derive_seed(o.seed, &cname, "order", "-", 0));
    for b in 0..nblocks {
        for half in ["main", "aa"] {
            let mut order = arms.clone();
            order_rng.shuffle(&mut order);
            for arm in order {
                let seed = derive_seed(o.seed, &cname, arm.name(), half, b as u64);
                let mut rng = Rng::new(seed);
                let mut tal = CellTally::default();
                let nt = block.min(trials - b * block);
                for _ in 0..nt {
                    if let Some(t) = run_trial(&f, arm, m, k_assign, &stable, &mut rng) {
                        tal.add(&t);
                    }
                }
                let v: Vec<String> = vec![
                    n.to_string(),
                    m.to_string(),
                    arm.name().to_string(),
                    half.to_string(),
                    b.to_string(),
                    seed.to_string(),
                    tal.trials.to_string(),
                    tal.assigns.to_string(),
                    tal.trials_eq_fail.to_string(),
                    tal.assigns_eq_fail.to_string(),
                    tal.trials_bool_fail.to_string(),
                    tal.rec_singular.to_string(),
                    tal.planted.to_string(),
                    tal.plant_bad.to_string(),
                    tal.conj_member_n.to_string(),
                    tal.conj_member_fail.to_string(),
                    tal.dom_n.to_string(),
                    tal.dom_fail.to_string(),
                    tal.neg_n.to_string(),
                    tal.neg_disagree.to_string(),
                    tal.stable_n.to_string(),
                    tal.stable_cob_fail.to_string(),
                ];
                writeln!(fh, "{}", v.join("\t")).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(format!("{}/{}", o.out, file))
}

// ---------------------------------------------------------------------------------------------------------------
pub const RANK_COLS: [&str; 22] = [
    "a",
    "n",
    "m",
    "l",
    "r",
    "arm",
    "half",
    "rep",
    "seed",
    "nx",
    "n_rel",
    "n_rel_indep",
    "orbits_touched",
    "cols_orbit",
    "cols_plain_n",
    "rank_plain_1",
    "rank_plain_n",
    "rank_orbit_1",
    "rank_orbit_n",
    "rank_orbit_wrong",
    "rank_orbit_indep",
    "rows_bad",
];

/// Rank census of one cell on rand (subspace) and tuned bases; two halves.
pub fn run_rank(cell: &Cell, t1: usize, row_cap: usize, o: &RunOpts) -> Result<String, String> {
    let f = Fq::new(cell.n);
    let g = build_group(&f, cell.a)?;
    let ob = build_orbits(&f, &g);
    let cname = cell.name();
    let reps = o.reps.unwrap_or(8);
    let tuned_n = tuned_size(cell.m, g.r);
    let mut hdr: Vec<&str> = RANK_COLS.to_vec();
    hdr.extend_from_slice(&["incons", "nonproportional", "row_cap_eff", "indep_need"]);
    let file = format!("rank_{cname}.tsv");
    let mut fh = open_out(&o.out, &file, &hdr)?;
    let mut order_rng = Rng::new(derive_seed(o.seed, &cname, "rank-order", "-", 0));
    for rep in 0..reps {
        for half in ["main", "aa"] {
            let mut arms = vec!["rand", "tuned"];
            order_rng.shuffle(&mut arms);
            for arm in arms {
                let seed = derive_seed(o.seed, &cname, &format!("rank-{arm}"), half, rep as u64);
                let mut rng = Rng::new(seed);
                let base = if arm == "rand" {
                    let basis = random_subspace(&f, cell.l, &mut rng);
                    BaseSet::from_elems(&g, &span(&basis))
                } else {
                    random_class_set(&g, tuned_n, &mut rng)
                };
                let nx = base.len();
                let est = est_ops(nx.max(1), cell.m, 0);
                let v: Vec<String> = if nx < cell.m || est > BASE_OPS_CAP {
                    vec![]
                } else {
                    let ro = rank_census(&f, &g, &ob, &base.ds, cell.m, t1, row_cap, &mut rng);
                    vec![
                        nx.to_string(),
                        ro.n_rel.to_string(),
                        ro.n_rel_indep.to_string(),
                        ro.orbits_touched.to_string(),
                        ro.cols_orbit.to_string(),
                        ro.cols_plain_n.to_string(),
                        ro.rank_plain_1.to_string(),
                        ro.rank_plain_n.to_string(),
                        ro.rank_orbit_1.to_string(),
                        ro.rank_orbit_n.to_string(),
                        ro.rank_orbit_wrong.to_string(),
                        ro.rank_orbit_indep.to_string(),
                        ro.rows_bad.to_string(),
                        ro.incons.to_string(),
                        ro.nonproportional.to_string(),
                        ro.row_cap_eff.to_string(),
                        ro.indep_need.to_string(),
                    ]
                };
                let lead = vec![
                    cell.a.to_string(),
                    cell.n.to_string(),
                    cell.m.to_string(),
                    cell.l.to_string(),
                    g.r.to_string(),
                    arm.to_string(),
                    half.to_string(),
                    rep.to_string(),
                    seed.to_string(),
                ];
                let mut row = lead;
                if v.is_empty() {
                    for _ in 0..(RANK_COLS.len() - 9 + 4) {
                        row.push("NA".into());
                    }
                } else {
                    row.extend(v);
                }
                writeln!(fh, "{}", row.join("\t")).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(format!("{}/{}", o.out, file))
}
