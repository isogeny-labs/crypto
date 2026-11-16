//! Cell plan, per-base measurement and the run driver (writes one TSV per cell).

use crate::bases::*;
use crate::curve::{build_group, largest_prime_factor, Group};
use crate::enumr::*;
use crate::field::Fq;
use std::io::Write as _;
use std::time::Instant;

pub const ARMS: [&str; 7] = [
    "vpoly", "vrand", "g4poly", "g4rand", "rset", "riota", "wrong",
];
pub const SINGLE_ARMS: [&str; 2] = ["vpoly", "g4poly"];
/// Cap on the number of m-subsets enumerated for one base (about 1 GB at 32 bytes per record). It is
/// never hit by a matched replicate: `draw_matched` redraws the g4rand base (and with it the common item
/// count N of every arm) until ncls >= m and C(N, m) <= ENUM_CAP, identically for all arms.
pub const ENUM_CAP: f64 = 3e7;
/// Arms that are matched on the exact item count within a replicate and enter the decision rule.
pub const MATCHED_ARMS: [&str; 5] = ["vrand", "g4rand", "rset", "riota", "wrong"];
pub const MATCH_TRIES: u64 = 20000;
pub const G_ATTEMPTS: u64 = 200;

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
        (1, 11, &[(2, 6), (2, 7), (3, 5), (4, 5), (5, 5), (6, 5)]),
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
        "vpoly" => Built {
            raw: poly_window(l),
            table: gray_table(&poly_basis(l)),
        },
        "vrand" => {
            let b = random_basis(f.n, l, &mut rng);
            Built {
                raw: span(&b),
                table: gray_table(&b),
            }
        }
        "g4poly" => {
            let b = poly_basis(l - 1);
            Built {
                raw: involution_union(f, &span(&b), 1),
                table: union_table(f, &b, 1),
            }
        }
        "g4rand" => {
            let b = random_basis(f.n, l - 1, &mut rng);
            Built {
                raw: involution_union(f, &span(&b), 1),
                table: union_table(f, &b, 1),
            }
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
            Built {
                raw: involution_union(f, &span(&b), c),
                table: union_table(f, &b, c),
            }
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
pub fn measure(
    g: &Group,
    f: &Fq,
    built: &Built,
    l: u32,
    m: usize,
    order_seed: u64,
) -> Result<Vec<(String, String)>, String> {
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
        if ok {
            "1"
        } else {
            "0"
        }
    } else {
        "NA"
    };
    let (sum_d_g, sum_o_g) = (mg_d.mean * mg_d.n as f64, mg_o.mean * mg_o.n as f64);
    let collapse = if sum_o_g > 0.0 {
        sum_d_g / sum_o_g
    } else {
        f64::NAN
    };
    // rank stream over the odd-order subgroup G in a seeded random order
    let mut order: Vec<u32> = (0..ne).step_by(g.h as usize).collect();
    let mut rng = Rng(order_seed);
    for i in (1..order.len()).rev() {
        order.swap(i, rng.below(i as u64 + 1) as usize);
    }
    let lpf = largest_prime_factor(ne) as u64;
    let (p, pfield) = if lpf >= 101 {
        (lpf, "lpf")
    } else {
        (2147483647u64, "mersenne31")
    };
    let ro = rank_stream(&t, &order, m, base.ncls.max(1), p, order_seed ^ 0xA5A5);
    // MITM cost model (probe counts; a model, not a measured solver cost). Per arm the cheaper table
    // orientation is chosen for the number of targets that arm needs.
    let nit = base.items.len();
    let nq = if base.closure == 1.0 && nit > 0 {
        n_signed_classes(&base)
    } else {
        nit
    };
    let th90 = ((0.9 * ro.active as f64).ceil()).max(1.0);
    // per independent relation: fractional targets (cpr); total to 90% rank: whole targets (tot)
    let t_lb = ro.t90f.unwrap_or(ro.targets_used as f64); // right-censored lower bound when 90% is never reached
    let eval = |n_items: usize| -> (f64, f64, f64, bool) {
        let cpr = match ro.t90f {
            Some(tf) => {
                let c = mitm_total(n_items, m, tf);
                (c.table + tf * c.probe) / th90
            }
            None => f64::NAN,
        };
        let cl = mitm_total(n_items, m, t_lb);
        let cpr_lb = (cl.table + t_lb * cl.probe) / th90;
        let tot = match ro.t90 {
            Some(t90) => {
                let c = mitm_total(n_items, m, t90 as f64);
                c.table + t90 as f64 * c.probe
            }
            None => f64::NAN,
        };
        (
            cpr,
            cpr_lb,
            tot,
            mitm_total(n_items, m, ro.t90f.unwrap_or(t_lb)).flipped,
        )
    };
    let (cpr_blind, cpr_lb_blind, tot_blind, or_blind) = eval(nit);
    let (cpr_best, cpr_lb_best, tot_best, or_best) = eval(nq);
    let sat = mg_o.mean / th90;
    let ms = t0.elapsed().as_secs_f64() * 1e3;
    Ok(vec![
        kv("nx", base.nx),
        kv("nitems", nit),
        kv("nsc", nq),
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
        kv("sat", sat),
        kv("rank_final", ro.rank_final),
        kv("targets_used", ro.targets_used),
        kv("rows_total", ro.rows_total),
        kv("t50", nan_opt(ro.t50)),
        kv("t90", nan_opt(ro.t90)),
        kv(
            "t90f",
            ro.t90f
                .map(|v| v.to_string())
                .unwrap_or_else(|| "NA".into()),
        ),
        kv("t100", nan_opt(ro.t100)),
        kv("rows90", nan_opt(ro.rows90)),
        kv("stalled", ro.stalled as u8),
        kv("rank_field", pfield),
        kv("cpr_blind", cpr_blind),
        kv("cpr_best", cpr_best),
        kv("cpr_lb_blind", cpr_lb_blind),
        kv("cpr_lb_best", cpr_lb_best),
        kv("tot_blind", tot_blind),
        kv("tot_best", tot_best),
        kv("orient_blind", or_blind as u8),
        kv("orient_best", or_best as u8),
        kv("deg", deg),
        kv("param_deg", pdeg),
        kv("anf_terms", terms),
        kv("enum_ms", format!("{ms:.1}")),
    ])
}

pub fn items_of(g: &Group, raw: &[u32]) -> usize {
    build_base(g, &core_set(raw)).items.len()
}

/// (items, classes) of a raw x-set. The enumerator needs m pairwise distinct classes, so a base with
/// fewer than m classes has no decomposition at any target (amendment A1).
pub fn shape_of(g: &Group, raw: &[u32]) -> (usize, usize) {
    let b = build_base(g, &core_set(raw));
    (b.items.len(), b.ncls)
}

/// True iff the base has at least one decomposition (orbit row) at some target of the odd-order subgroup G.
/// A base without one has active = 0 and no finite cost at any target in G, so it would be censored by
/// construction (amendment A2): for a = 0 every item of a class has the same parity, and a selection of
/// exactly m classes with an odd number of odd-parity classes never sums into G.
pub fn has_g_row(g: &Group, raw: &[u32], m: usize) -> bool {
    let base = build_base(g, &core_set(raw));
    if base.ncls < m || binom(base.items.len(), m) > ENUM_CAP {
        return false;
    }
    match enumerate(&base, m, ENUM_CAP) {
        Ok(dec) => dec.iter().any(|&(s, _)| s % g.h == 0),
        Err(_) => false,
    }
}

/// One replicate's matched bases: every arm has exactly `n_items` items (the g4rand base's count).
pub struct Matched {
    pub n_items: usize,
    pub g_attempt: u64,
    /// (arm, base, seed, tries used)
    pub arms: Vec<(&'static str, Built, u64, u64)>,
}

fn seed_of(seed: u64, c: &Cell, arm: &str, rep: u64, gatt: u64, att: u64) -> u64 {
    derive_seed(
        seed,
        &[
            c.a as u64,
            c.n as u64,
            c.m as u64,
            c.l as u64,
            arm_id(arm),
            rep,
            gatt,
            att,
        ],
    )
}

/// Draw the g4rand base of the replicate, then rejection-sample every other matched arm until its item
/// count equals the g4rand base's. A g4rand draw is redrawn (the whole replicate with it) if ncls < m (a stable base has 4 items per class, so N < 4m), if it has no decomposition at any target of G (A2),
/// C(N, m) > ENUM_CAP or any arm cannot be matched in MATCH_TRIES draws; the rule is the same for all
/// arms, so it truncates the common N distribution and cannot favour one arm.
pub fn draw_matched(f: &Fq, g: &Group, c: &Cell, seed: u64, rep: u64) -> Result<Matched, String> {
    'g: for gatt in 0..G_ATTEMPTS {
        let gs = seed_of(seed, c, "g4rand", rep, gatt, 0);
        let gb = make_arm("g4rand", f, c.l, gs, 0);
        let gcore = core_set(&gb.raw).len();
        let (n, gcls) = shape_of(g, &gb.raw);
        if gcls < c.m || binom(n, c.m) > ENUM_CAP || !has_g_row(g, &gb.raw, c.m) {
            continue;
        }
        let mut arms: Vec<(&'static str, Built, u64, u64)> = vec![("g4rand", gb, gs, 0)];
        for arm in ["vrand", "rset", "riota", "wrong"] {
            let mut found = None;
            for att in 0..MATCH_TRIES {
                let s = seed_of(seed, c, arm, rep, gatt, att);
                let b = make_arm(arm, f, c.l, s, gcore);
                let (bi, bc) = shape_of(g, &b.raw);
                if bi == n && bc >= c.m && has_g_row(g, &b.raw, c.m) {
                    found = Some((b, s, att));
                    break;
                }
            }
            match found {
                Some((b, s, att)) => arms.push((arm, b, s, att)),
                None => continue 'g,
            }
        }
        return Ok(Matched {
            n_items: n,
            g_attempt: gatt,
            arms,
        });
    }
    Err(format!(
        "replicate {rep}: no matched draw in {G_ATTEMPTS} attempts"
    ))
}

pub struct RunOpts {
    pub seed: u64,
    pub reps: Option<usize>,
    pub out: String,
}

/// Run one cell: matched arms, interleaved by replicate, 2 halves (main, aa). On any error the rows
/// flushed so far stay on disk and `analyze` classifies the cell INCOMPLETE (never silently complete).
pub fn run_cell(c: &Cell, o: &RunOpts) -> Result<String, String> {
    let f = Fq::new(c.n);
    let g = build_group(&f, c.a)?;
    let reps = o.reps.unwrap_or_else(|| reps_for(c.n));
    let path = format!("{}/rows_a{}_n{}_m{}_l{}.tsv", o.out, c.a, c.n, c.m, c.l);
    let mut w = std::io::BufWriter::new(std::fs::File::create(&path).map_err(|e| e.to_string())?);
    let mut header_done = false;
    for rep in 0..2 * reps {
        let (half, ridx) = if rep < reps {
            ("main", rep)
        } else {
            ("aa", rep - reps)
        };
        let mt = draw_matched(&f, &g, c, o.seed, rep as u64)
            .map_err(|e| format!("cell a{} n{} m{} l{}: {e}", c.a, c.n, c.m, c.l))?;
        let mut order: Vec<&str> = ARMS.to_vec();
        let mut rng = Rng(derive_seed(
            o.seed,
            &[
                999, rep as u64, c.n as u64, c.m as u64, c.l as u64, c.a as u64,
            ],
        ));
        for i in (1..order.len()).rev() {
            order.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let gcore = {
            let gb = &mt.arms[0].1;
            core_set(&gb.raw).len()
        };
        let mut single: Vec<(&'static str, Built, u64, u64)> = vec![];
        if half == "main" && ridx == 0 {
            for arm in SINGLE_ARMS {
                let s = seed_of(o.seed, c, arm, rep as u64, 0, 0);
                let b = make_arm(arm, &f, c.l, s, gcore);
                // unmatched reference instances: skipped (not an error) if over the enumeration cap
                if binom(items_of(&g, &b.raw), c.m) <= ENUM_CAP {
                    single.push((arm, b, s, 0));
                } else {
                    eprintln!("SINGLE_SKIPPED {arm}: over enumeration cap");
                }
            }
        }
        for arm in order {
            let entry = mt.arms.iter().chain(single.iter()).find(|e| e.0 == arm);
            let (arm, built, seed, tries) = match entry {
                Some(e) => (e.0, &e.1, e.2, e.3),
                None => continue,
            };
            let raw = &built.raw;
            let stable = is_iota_stable(&f, &core_set(raw));
            let cols = measure(&g, &f, built, c.l, c.m, seed ^ 0x5151)
                .map_err(|e| format!("cell a{} n{} m{} l{} arm {arm}: {e}", c.a, c.n, c.m, c.l))?;
            let mut row = vec![
                kv("a", c.a),
                kv("n", c.n),
                kv("m", c.m),
                kv("l", c.l),
                kv("arm", arm),
                kv("half", half),
                kv("rep", ridx),
                kv("seed", seed),
                kv(
                    "n_target",
                    if MATCHED_ARMS.contains(&arm) {
                        mt.n_items.to_string()
                    } else {
                        "NA".into()
                    },
                ),
                kv("g_attempt", mt.g_attempt),
                kv("tries", tries),
                kv("ne", g.ne),
                kv("h", g.h),
                kv("stable_set", stable as u8),
                kv("size_raw", raw.len()),
            ];
            row.extend(cols);
            if !header_done {
                writeln!(
                    w,
                    "{}",
                    row.iter()
                        .map(|r| r.0.as_str())
                        .collect::<Vec<_>>()
                        .join("\t")
                )
                .unwrap();
                header_done = true;
            }
            writeln!(
                w,
                "{}",
                row.iter()
                    .map(|r| r.1.as_str())
                    .collect::<Vec<_>>()
                    .join("\t")
            )
            .unwrap();
        }
        w.flush().unwrap();
    }
    Ok(path)
}
