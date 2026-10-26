//! Cell plan, per-base measurement and the run driver (writes one TSV per cell).

use crate::bases::*;
use crate::curve::{build_group, largest_prime_factor, Group};
use crate::enumr::*;
use crate::field::Fq;
use std::io::Write as _;
use std::time::Instant;

pub const ARMS: [&str; 7] = ["vpoly", "vrand", "g4poly", "g4rand", "rset", "riota", "wrong"];
pub const SINGLE_ARMS: [&str; 2] = ["vpoly", "g4poly"];
pub const ENUM_CAP: f64 = 2.5e6;

#[derive(Clone, Debug)]
pub struct Cell {
    pub a: u32,
    pub n: u32,
    pub m: usize,
    pub l: u32,
}

/// The frozen cell list (nominal lambda = C(2^l, m) / #E, see PROTOCOL.md section 6).
/// (n, a) = (11, 0) is excluded: E(F_2^11) for a = 0 has order 2116 = 4 * 23^2 and is not cyclic,
/// so the single-generator log table does not apply (the instrument returns an error there).
pub fn cell_list() -> Vec<Cell> {
    type Spec = (u32, u32, &'static [(usize, u32)]);
    let spec: [Spec; 6] = [
        (0, 13, &[(2, 7), (2, 8), (3, 6), (4, 5), (5, 5), (6, 5)]),
        (0, 17, &[(2, 9), (3, 7), (4, 6), (5, 5), (6, 5)]),
        (0, 19, &[(2, 10), (3, 7), (4, 6), (5, 5), (6, 5)]),
        (1, 11, &[(2, 6), (2, 7), (3, 5), (4, 4), (4, 5), (5, 4), (5, 5), (6, 4), (6, 5)]),
        (1, 13, &[(2, 7), (2, 8), (3, 6), (4, 5), (5, 5), (6, 5)]),
        (1, 17, &[(2, 9), (3, 7), (4, 6), (5, 5), (6, 5)]),
    ];
    let mut v = vec![];
    for (a, n, ml) in spec.iter() {
        for &(m, l) in ml.iter() {
            v.push(Cell { a: *a, n: *n, m, l });
        }
    }
    v
}

/// Replicates per half (main / A-A) by field size.
pub fn reps_for(n: u32) -> usize {
    match n {
        11 => 100,
        13 => 40,
        17 => 24,
        _ => 16,
    }
}

pub fn arm_id(arm: &str) -> u64 {
    ARMS.iter().position(|a| *a == arm).unwrap() as u64 + 1
}

/// A base together with a parametrisation table: entry b (l free bits, top bit = selector for the
/// iota-union arms) is the x-coordinate the free bits select. It is what a solver encoding would
/// substitute for x, so its algebraic degree is the degree price of the base.
pub struct Built {
    pub raw: Vec<u32>,
    pub table: Vec<u32>,
}

fn gray_table(basis: &[u32]) -> Vec<u32> {
    let mut t = vec![0u32; 1 << basis.len()];
    for i in 1..t.len() {
        t[i] = t[i & (i - 1)] ^ basis[i.trailing_zeros() as usize];
    }
    t
}

fn union_table(f: &Fq, wbasis: &[u32], c: u32) -> Vec<u32> {
    let wt = gray_table(wbasis);
    let mut t = wt.clone();
    for &w in &wt {
        t.push(if w == 0 { 0 } else { f.mul(c, f.inv(w)) });
    }
    t
}

fn shuffled_padded(core: &[u32], l: u32, rng: &mut Rng) -> Vec<u32> {
    let mut v = core.to_vec();
    for i in (1..v.len()).rev() {
        v.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let sz = 1usize << l;
    let mut t = Vec::with_capacity(sz);
    for i in 0..sz {
        t.push(v[i % v.len()]);
    }
    t
}

/// Build an arm. `g4_core_size` is the size of the paired g4rand core set (sets the size of the
/// rset / riota controls). Raw sets may contain 0 and 1 (removed later by `core_set`).
pub fn make_arm(arm: &str, f: &Fq, l: u32, seed: u64, g4_core_size: usize) -> Built {
    let mut rng = Rng(seed);
    let poly_basis = |k: u32| (0..k).map(|i| 1u32 << i).collect::<Vec<u32>>();
    match arm {
        "vpoly" => Built { raw: poly_window(l), table: gray_table(&poly_basis(l)) },
        "vrand" => {
            let b = random_basis(f.n, l, &mut rng);
            Built { raw: span(&b), table: gray_table(&b) }
        }
        "g4poly" => {
            let b = poly_basis(l - 1);
            Built { raw: involution_union(f, &span(&b), 1), table: union_table(f, &b, 1) }
        }
        "g4rand" => {
            let b = random_basis(f.n, l - 1, &mut rng);
            Built { raw: involution_union(f, &span(&b), 1), table: union_table(f, &b, 1) }
        }
        "rset" => {
            let raw = random_set(f.q, g4_core_size, &mut rng);
            let table = shuffled_padded(&raw, l, &mut rng);
            Built { raw, table }
        }
        "riota" => {
            assert!(g4_core_size % 2 == 0);
            let raw = random_iota_set(f, g4_core_size / 2, &mut rng);
            let table = shuffled_padded(&raw, l, &mut rng);
            Built { raw, table }
        }
        "wrong" => {
            let b = random_basis(f.n, l - 1, &mut rng);
            let c = 2 + rng.below((f.q - 2) as u64) as u32; // c != 0, 1
            Built { raw: involution_union(f, &span(&b), c), table: union_table(f, &b, c) }
        }
        _ => panic!("unknown arm {arm}"),
    }
}

pub fn make_raw(arm: &str, f: &Fq, l: u32, seed: u64, g4_core_size: usize) -> Vec<u32> {
    make_arm(arm, f, l, seed, g4_core_size).raw
}

fn kv<T: ToString>(k: &str, v: T) -> (String, String) {
    (k.to_string(), v.to_string())
}

pub fn nan_opt(o: Option<u64>) -> String {
    o.map(|v| v.to_string()).unwrap_or_else(|| "NA".into())
}

/// Measure one base exactly. Everything is a deterministic function of (group, set, m, order_seed).
pub fn measure(g: &Group, f: &Fq, built: &Built, l: u32, m: usize, order_seed: u64) -> Result<Vec<(String, String)>, String> {
    let t0 = Instant::now();
    let raw = &built.raw;
    let (deg, terms) = anf(f.n, raw);
    let pdeg = param_degree(f.n, l, &built.table);
    let core = core_set(raw);
    let base = build_base(g, &core);
    let dec = enumerate(&base, m, ENUM_CAP)?;
    let ne = g.ne;
    let t = tally(ne, dec);
    let mg_d = moments(&t.d, g.h);
    let mg_o = moments(&t.o, g.h);
    let ma_d = moments(&t.d, 1);
    let ma_o = moments(&t.o, 1);
    let ident = identity_exact(&t, m);
    let quot_ok = if base.closure == 1.0 && !base.items.is_empty() {
        let q = quot_counts(&base, m);
        let ok = (0..ne as usize).all(|s| q[s % g.m as usize] == t.o[s]);
        if ok { "1" } else { "0" }
    } else {
        "NA"
    };
    let (sum_d_g, sum_o_g) = (mg_d.mean * mg_d.n as f64, mg_o.mean * mg_o.n as f64);
    let collapse = if sum_o_g > 0.0 { sum_d_g / sum_o_g } else { f64::NAN };
    // rank stream over the odd-order subgroup G in a seeded random order
    let mut order: Vec<u32> = (0..ne).step_by(g.h as usize).collect();
    let mut rng = Rng(order_seed);
    for i in (1..order.len()).rev() {
        order.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let lpf = largest_prime_factor(ne) as u64;
    let (p, pfield) = if lpf >= 101 { (lpf, "lpf") } else { (2147483647u64, "mersenne31") };
    let ro = rank_stream(&t, &order, m, base.ncls.max(1), p);
    // MITM cost model (probe counts; a model, not a measured solver cost)
    let nit = base.items.len();
    let cb = mitm_cost(nit, m);
    let (cq_table, cq_probe) = if base.closure == 1.0 && nit > 0 {
        let c = mitm_cost(n_signed_classes(&base), m);
        (c.table, c.probe)
    } else {
        (f64::NAN, f64::NAN)
    };
    let th90 = ((0.9 * ro.active as f64).ceil()).max(1.0);
    let cpr = |table: f64, probe: f64| -> f64 {
        match ro.t90 {
            Some(t90) => (table + t90 as f64 * probe) / th90,
            None => f64::NAN,
        }
    };
    let cpr_blind = cpr(cb.table, cb.probe);
    let cpr_best = if cq_probe.is_nan() { cpr_blind } else { cpr(cq_table, cq_probe) };
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    Ok(vec![
        kv("nx", base.nx),
        kv("nitems", nit),
        kv("ncls", base.ncls),
        kv("closure", base.closure),
        kv("total_subsets", t.total),
        kv("lam_pred", t.total as f64 / ne as f64),
        kv("mean_d", mg_d.mean),
        kv("fano_d", mg_d.fano),
        kv("mean_o", mg_o.mean),
        kv("fano_o", mg_o.fano),
        kv("hit_d", mg_d.hit),
        kv("hit_pois_o", 1.0 - (-mg_o.mean).exp()),
        kv("collapse", collapse),
        kv("mean_d_all", ma_d.mean),
        kv("fano_d_all", ma_d.fano),
        kv("fano_o_all", ma_o.fano),
        kv("ident_ok", ident as u8),
        kv("quot_ok", quot_ok),
        kv("ntargets_g", mg_d.n),
        kv("active", ro.active),
        kv("rank_final", ro.rank_final),
        kv("targets_used", ro.targets_used),
        kv("t50", nan_opt(ro.t50)),
        kv("t90", nan_opt(ro.t90)),
        kv("t100", nan_opt(ro.t100)),
        kv("rows90", nan_opt(ro.rows90)),
        kv("stalled", ro.stalled as u8),
        kv("rank_field", pfield),
        kv("blind_table", cb.table),
        kv("blind_probe", cb.probe),
        kv("quot_table", cq_table),
        kv("quot_probe", cq_probe),
        kv("cpr_blind", cpr_blind),
        kv("cpr_best", cpr_best),
        kv("deg", deg),
        kv("param_deg", pdeg),
        kv("anf_terms", terms),
        kv("enum_ms", format!("{ms:.1}")),
    ])
}

pub struct RunOpts {
    pub seed: u64,
    pub reps: Option<usize>,
    pub out: String,
}

/// Run one cell: all arms, interleaved by replicate, 2 halves (main, aa) per replicate arm.
pub fn run_cell(c: &Cell, o: &RunOpts) -> Result<String, String> {
    let f = Fq::new(c.n);
    let g = build_group(&f, c.a)?;
    let reps = o.reps.unwrap_or_else(|| reps_for(c.n));
    let path = format!("{}/rows_a{}_n{}_m{}_l{}.tsv", o.out, c.a, c.n, c.m, c.l);
    let mut w = std::io::BufWriter::new(std::fs::File::create(&path).map_err(|e| e.to_string())?);
    let mut header_done = false;
    let cellkey = [c.a as u64, c.n as u64, c.m as u64, c.l as u64];
    for rep in 0..2 * reps {
        let (half, ridx) = if rep < reps { ("main", rep) } else { ("aa", rep - reps) };
        // paired size for the random controls: the g4rand base of this replicate
        let sd = |arm: &str| derive_seed(o.seed, &[cellkey[0], cellkey[1], cellkey[2], cellkey[3], arm_id(arm), rep as u64]);
        let g4_core = core_set(&make_raw("g4rand", &f, c.l, sd("g4rand"), 0)).len();
        let mut order: Vec<&str> = ARMS.to_vec();
        let mut rng = Rng(derive_seed(o.seed, &[999, rep as u64, cellkey[1], cellkey[2]]));
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i as u64 + 1) as usize);
        }
        for arm in order {
            if SINGLE_ARMS.contains(&arm) && !(half == "main" && ridx == 0) {
                continue;
            }
            let built = make_arm(arm, &f, c.l, sd(arm), g4_core);
            let raw = &built.raw;
            let stable = is_iota_stable(&f, &core_set(raw));
            let cols = match measure(&g, &f, &built, c.l, c.m, sd(arm) ^ 0x5151) {
                Ok(v) => v,
                Err(e) => return Err(format!("cell a{} n{} m{} l{} arm {arm}: {e}", c.a, c.n, c.m, c.l)),
            };
            let mut row = vec![
                kv("a", c.a),
                kv("n", c.n),
                kv("m", c.m),
                kv("l", c.l),
                kv("arm", arm),
                kv("half", half),
                kv("rep", ridx),
                kv("seed", sd(arm)),
                kv("ne", g.ne),
                kv("h", g.h),
                kv("stable_set", stable as u8),
                kv("size_raw", raw.len()),
            ];
            row.extend(cols);
            if !header_done {
                writeln!(w, "{}", row.iter().map(|r| r.0.as_str()).collect::<Vec<_>>().join("\t")).unwrap();
                header_done = true;
            }
            writeln!(w, "{}", row.iter().map(|r| r.1.as_str()).collect::<Vec<_>>().join("\t")).unwrap();
        }
        w.flush().unwrap();
    }
    Ok(path)
}
