//! `targets`: natural targets `R_j = a_j P + b_j Q` (hidden `Q = kP`), classified by
//! exhaustive enumeration of the real signed multisets of size `m` and by the
//! algebraic count over `F_{p^2}`; plus planted interface controls.
use crate::cell::{seeded_rng, Cell};
use crate::ec::*;
use crate::util::*;
use crypto_lib::cryptanalysis::ic_boundary::GroupOps;
use rand::Rng;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;
use std::time::Instant;

pub const MAXM: usize = 12;

pub fn pack(syms: &[u8]) -> u64 {
    syms.iter()
        .enumerate()
        .fold(0u64, |acc, (k, &s)| acc | (u64::from(s) << (6 * k)))
}
pub fn unpack(code: u64, m: usize) -> Vec<u8> {
    (0..m).map(|k| ((code >> (6 * k)) & 63) as u8).collect()
}
fn binom(n: u128, k: u128) -> u128 {
    let mut r = 1u128;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

/// Positive and negative symbols of the real base: `2i` is `+P_i`, `2i+1` is `-P_i`.
pub fn symbols(cell: &Cell) -> Vec<Aff> {
    let mut v = Vec::new();
    for e in &cell.real {
        v.push((e.x, e.y));
        v.push((e.x, (cell.p - e.y) % cell.p));
    }
    v
}

pub struct RealTable {
    pub entries: Vec<(u64, u64)>,
    pub leaves: u64,
}

struct RealEnum<'a> {
    p: u64,
    a: u64,
    sym: &'a [Aff],
    m: usize,
    cur: [u8; MAXM],
    out: Vec<(u64, u64)>,
    leaves: u64,
}
impl RealEnum<'_> {
    fn rec(&mut self, depth: usize, start: usize, acc: Option<Aff>) {
        if depth == self.m {
            self.leaves += 1;
            if let Some((x, _)) = acc {
                let m = self.m;
                let mut f = [0u8; MAXM];
                for k in 0..m {
                    f[k] = self.cur[k] ^ 1;
                }
                f[..m].sort_unstable();
                if self.cur[..m] <= f[..m] {
                    self.out.push((x, pack(&self.cur[..m])));
                }
            }
            return;
        }
        for s in start..self.sym.len() {
            let nacc = add_opt(self.p, self.a, acc, Some(self.sym[s]));
            self.cur[depth] = s as u8;
            self.rec(depth + 1, s, nacc);
        }
    }
}

/// All signed multisets of size `m` of the real base, one representative of each
/// pair `{S, -S}`, keyed by the abscissa of the sum (identity sums skipped).
pub fn enumerate_real(cell: &Cell, m: usize, max_entries: u128) -> Result<RealTable, String> {
    let sym = symbols(cell);
    if sym.len() > 64 || m > MAXM {
        return Err("real base or arity too large for the 6-bit packing".into());
    }
    let est = binom((sym.len() + m - 1) as u128, m as u128) / 2;
    if est > max_entries {
        return Err(format!(
            "classifier enumeration would store about {est} entries (> --max-enum-entries {max_entries})"
        ));
    }
    let mut e = RealEnum {
        p: cell.p,
        a: cell.a,
        sym: &sym,
        m,
        cur: [0; MAXM],
        out: Vec::with_capacity(est.min(1 << 28) as usize),
        leaves: 0,
    };
    e.rec(0, 0, None);
    let mut out = e.out;
    out.sort_unstable();
    Ok(RealTable {
        entries: out,
        leaves: e.leaves,
    })
}

pub struct AlgTable {
    pub entries: Vec<(u64, u64)>,
    pub leaves: u64,
    pub pts: Vec<F2pt>,
    pub f2: F2,
}
struct AlgEnum<'a> {
    f: &'a F2,
    a: u64,
    pts: &'a [F2pt],
    m: usize,
    cur: [u8; MAXM],
    out: Vec<(u64, u64)>,
    leaves: u64,
}
impl AlgEnum<'_> {
    fn rec(&mut self, depth: usize, start: usize, acc: Option<F2pt>) {
        if depth == self.m {
            self.leaves += 1;
            if let Some((x, _)) = acc {
                if x.1 == 0 {
                    let m = self.m;
                    let mut fl = [0u8; MAXM];
                    for k in 0..m {
                        fl[k] = self.cur[k] ^ 1;
                    }
                    fl[..m].sort_unstable();
                    if self.cur[..m] <= fl[..m] {
                        self.out.push((x.0, pack(&self.cur[..m])));
                    }
                }
            }
            return;
        }
        for s in start..self.pts.len() {
            self.cur[depth] = s as u8;
            let nacc = add_f2(self.f, self.a, acc, Some(self.pts[s]));
            self.rec(depth + 1, s, nacc);
        }
    }
}

/// ALGEBRAIC DECOMPOSITIONS.  Signed multisets of size `m` over the `2B` lifts
/// `(+-)(x, y)` of `V` to `E(F_{p^2})` whose sum has an abscissa in `F_p`
/// (one representative of each pair `{T, -T}`), keyed by that abscissa.
/// For a real target `R` the classes with abscissa `x_R` are exactly the signed
/// multisets `T` with sum `+R` after orientation.  `algebraic_count` is the number of
/// distinct unsigned multisets of `V` among them (gen.py's `decomposable` hits).
///
/// ORDERED SOLUTION COUNT (frozen formula).  The system's solutions over `F-bar_p`
/// are the chains `(x_0..x_{m-1}; u_0..u_{m-3})` with `x_i in V` and `u_k` the abscissa
/// of the prefix sum `s_0+..+s_{k+1}` of signed points `s_i` over `x_i`, final sum
/// abscissa `x_R`.  A chain is the class `{(s_i), (-s_i)}` of a sequence of signed
/// points, so the solutions correspond one-to-one to sequences of signed points with
/// sum exactly `+R`, in which no prefix of length `2..m-1` sums to `O` (`S_3` has no
/// root for an identity partial sum).  For a signed multiset `T` of multiplicities
/// `n_s` the number of such sequences is counted exactly by a dynamic programme over
/// sub-multisets (`ordered_count`); the ordered count of a target is the sum over its
/// classes.  Assumption: distinct sequences give distinct chains (generic
/// non-degeneracy); msolve's distinct-solution count must be `>=` this number.
pub fn enumerate_alg(cell: &Cell, m: usize, max_leaves: u128) -> Result<AlgTable, String> {
    let b = cell.v.len();
    if 2 * b > 64 || m > MAXM {
        return Err("base or arity too large".into());
    }
    let leaves = binom((2 * b + m - 1) as u128, m as u128);
    if leaves > max_leaves {
        return Err(format!(
            "algebraic enumeration needs {leaves} leaves (> --max-alg-leaves {max_leaves})"
        ));
    }
    let f = F2::new(cell.p);
    let mut pts: Vec<F2pt> = Vec::new();
    for &x in &cell.v {
        let q = lift_f2(&f, cell.a, cell.b, x);
        pts.push(q);
        pts.push((q.0, f.neg(q.1)));
    }
    let mut e = AlgEnum {
        f: &f,
        a: cell.a,
        pts: &pts,
        m,
        cur: [0; MAXM],
        out: Vec::new(),
        leaves: 0,
    };
    e.rec(0, 0, None);
    let mut out = e.out;
    out.sort_unstable();
    out.dedup();
    let leaves = e.leaves;
    Ok(AlgTable {
        entries: out,
        leaves,
        pts,
        f2: f,
    })
}

/// Number of sequences of the signed multiset `syms` (symbols of `tab.pts`) with no
/// prefix of length `2..` summing to the identity.
pub fn ordered_count(tab: &AlgTable, a: u64, syms: &[u8]) -> u64 {
    let mut distinct: Vec<(u8, usize)> = Vec::new();
    for &s in syms {
        match distinct.iter_mut().find(|d| d.0 == s) {
            Some(d) => d.1 += 1,
            None => distinct.push((s, 1)),
        }
    }
    let radix: Vec<usize> = distinct
        .iter()
        .scan(1usize, |acc, d| {
            let r = *acc;
            *acc *= d.1 + 1;
            Some(r)
        })
        .collect();
    let nstates: usize = distinct.iter().map(|d| d.1 + 1).product();
    let counts = |st: usize| -> Vec<usize> {
        distinct.iter().zip(&radix).map(|(d, r)| (st / r) % (d.1 + 1)).collect()
    };
    let mut sums: Vec<Option<F2pt>> = vec![None; nstates];
    let mut dp = vec![0u64; nstates];
    dp[0] = 1;
    for st in 1..nstates {
        let c = counts(st);
        let j = c.iter().position(|&x| x > 0).expect("non-empty state");
        let prev = st - radix[j];
        sums[st] = add_f2(&tab.f2, a, sums[prev], Some(tab.pts[distinct[j].0 as usize]));
    }
    for st in 0..nstates {
        if dp[st] == 0 {
            continue;
        }
        let c = counts(st);
        for j in 0..distinct.len() {
            if c[j] < distinct[j].1 {
                let nx = st + radix[j];
                if sums[nx].is_some() {
                    dp[nx] += dp[st];
                }
            }
        }
    }
    dp[nstates - 1]
}

pub fn range_of(entries: &[(u64, u64)], x: u64) -> &[(u64, u64)] {
    let lo = entries.partition_point(|e| e.0 < x);
    let hi = entries.partition_point(|e| e.0 <= x);
    &entries[lo..hi]
}

/// Sum of the signed symbols (uncounted).
pub fn sum_syms(cell: &Cell, sym: &[Aff], s: &[u8]) -> Option<Aff> {
    s.iter()
        .fold(None, |acc, &k| add_opt(cell.p, cell.a, acc, Some(sym[k as usize])))
}
/// Net coefficient vector over the real base.
pub fn net_vector(b_real: usize, s: &[u8]) -> Vec<i64> {
    let mut c = vec![0i64; b_real];
    for &k in s {
        c[(k / 2) as usize] += if k % 2 == 0 { 1 } else { -1 };
    }
    c
}
/// Decompositions of the point `r` (sum exactly `+r`), as sorted symbol lists.
pub fn decompositions_of(cell: &Cell, sym: &[Aff], tab: &RealTable, m: usize, r: Aff) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for &(_, code) in range_of(&tab.entries, r.0) {
        let mut s = unpack(code, m);
        if sum_syms(cell, sym, &s) != Some(r) {
            for k in s.iter_mut() {
                *k ^= 1;
            }
            s.sort_unstable();
        }
        out.push(s);
    }
    out.sort();
    out
}

pub fn signed_json(s: &[u8]) -> Vec<i64> {
    s.iter()
        .map(|&k| {
            let i = i64::from(k / 2) + 1;
            if k % 2 == 0 {
                i
            } else {
                -i
            }
        })
        .collect()
}

/// Algebraic classes of abscissa `x`: signed symbol lists, the distinct unsigned
/// multisets of `V` indices, and the ordered solution count (see `enumerate_alg`).
pub fn alg_info(alg: &AlgTable, a: u64, m: usize, x: u64) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, u64) {
    let classes: Vec<Vec<u8>> = range_of(&alg.entries, x).iter().map(|e| unpack(e.1, m)).collect();
    let mut uns: BTreeSet<Vec<u8>> = BTreeSet::new();
    let mut ordered = 0u64;
    for c in &classes {
        let mut v: Vec<u8> = c.iter().map(|s| s / 2).collect();
        v.sort_unstable();
        uns.insert(v);
        ordered += ordered_count(alg, a, c);
    }
    (classes, uns.into_iter().collect(), ordered)
}

pub fn run(args: &[String]) -> Result<(), String> {
    let t0 = Instant::now();
    let cdir = crate::arg_req(args, "--cell")?;
    let cell = Cell::load(Path::new(&cdir))?;
    let n_sat: usize = crate::arg_req(args, "--n-sat")?.parse().map_err(|_| "n-sat")?;
    let n_unsat: usize = crate::arg_req(args, "--n-unsat")?.parse().map_err(|_| "n-unsat")?;
    let n_pl: usize = crate::arg_opt(args, "--planted").map_or(Ok(1), |s| s.parse()).map_err(|_| "planted")?;
    let seed: u64 = crate::arg_req(args, "--seed")?.parse().map_err(|_| "seed")?;
    let m: usize = crate::arg_opt(args, "--m").map_or(Ok(8), |s| s.parse()).map_err(|_| "m")?;
    let max_draws: u64 = crate::arg_opt(args, "--max-draws").map_or(Ok(10_000_000), |s| s.parse()).map_err(|_| "max-draws")?;
    let max_enum: u128 = crate::arg_opt(args, "--max-enum-entries").map_or(Ok(250_000_000), |s| s.parse()).map_err(|_| "max-enum")?;
    let max_alg: u128 = crate::arg_opt(args, "--max-alg-leaves").map_or(Ok(1_000_000_000), |s| s.parse()).map_err(|_| "max-alg")?;
    let out = crate::arg_opt(args, "--out").unwrap_or_else(|| format!("{cdir}/targets-s{seed}"));
    let r = cell.r;
    let ec = cell.ec();
    let sym = symbols(&cell);

    let te = Instant::now();
    let real = enumerate_real(&cell, m, max_enum)?;
    let alg = enumerate_alg(&cell, m, max_alg)?;
    let enum_secs = te.elapsed().as_secs_f64();

    let mut rng = seeded_rng(&cell.slug, seed, "targets");
    let k = rng.gen_range(1..r);
    let mut ops = GroupOps::default();
    let pp = ec.pt(cell.px, cell.py);
    let q = ec.mul(&mut ops, pp, k);
    if q.infinity {
        return Err("Q = O".into());
    }

    struct Sel {
        name: String,
        label: &'static str,
        draw: Option<u64>,
        x_r: u64,
        a: Option<u64>,
        b: Option<u64>,
        decomps: Vec<Vec<u8>>,
        alg_multisets: Vec<Vec<u8>>,
        alg_classes: usize,
        ordered: u64,
        planted_syms: Option<Vec<u8>>,
    }
    let mut sel: Vec<Sel> = Vec::new();
    let (mut n_s, mut n_u) = (0usize, 0usize);
    let mut sat_draws: Vec<u64> = Vec::new();
    let mut identity_draws = 0u64;
    let mut draws = 0u64;
    while (n_s < n_sat || n_u < n_unsat) && draws < max_draws {
        let j = draws;
        draws += 1;
        let a = rng.gen_range(1..r);
        let b = rng.gen_range(1..r);
        let rr = ec.combo(&mut ops, pp, q, a, b);
        if rr.infinity {
            identity_draws += 1;
            continue;
        }
        let rc = range_of(&real.entries, rr.x).len();
        let is_sat = rc >= 1;
        if is_sat {
            sat_draws.push(j);
        }
        let keep = if is_sat { n_s < n_sat } else { n_u < n_unsat };
        if !keep {
            continue;
        }
        let decomps = decompositions_of(&cell, &sym, &real, m, (rr.x, rr.y));
        let (classes, alg_multisets, ordered) = alg_info(&alg, cell.a, m, rr.x);
        if is_sat {
            n_s += 1;
        } else {
            n_u += 1;
        }
        sel.push(Sel {
            name: format!("d{j:06}"),
            label: if is_sat { "sat" } else { "unsat" },
            draw: Some(j),
            x_r: rr.x,
            a: Some(a),
            b: Some(b),
            decomps,
            alg_multisets,
            alg_classes: classes.len(),
            ordered,
            planted_syms: None,
        });
    }
    if n_s < n_sat || n_u < n_unsat {
        return Err(format!(
            "draw cap {max_draws} reached with {n_s}/{n_sat} real-SAT and {n_u}/{n_unsat} real-UNSAT targets"
        ));
    }
    // planted interface controls: sum of m random real signed base points
    let mut prng = seeded_rng(&cell.slug, seed, "planted");
    for i in 0..n_pl {
        let (s, pt) = loop {
            let s: Vec<u8> = (0..m).map(|_| prng.gen_range(0..sym.len()) as u8).collect();
            let mut acc: Option<Aff> = None;
            let mut ok = true;
            for &k in &s {
                acc = add_opt(cell.p, cell.a, acc, Some(sym[k as usize]));
                if acc.is_none() {
                    ok = false;
                    break;
                }
            }
            if let (true, Some(pt)) = (ok, acc) {
                break (s, pt);
            }
        };
        let mut sorted = s.clone();
        sorted.sort_unstable();
        let decomps = decompositions_of(&cell, &sym, &real, m, pt);
        let (classes, alg_multisets, ordered) = alg_info(&alg, cell.a, m, pt.0);
        sel.push(Sel {
            name: format!("planted{i}"),
            label: "planted",
            draw: None,
            x_r: pt.0,
            a: None,
            b: None,
            decomps,
            alg_multisets,
            alg_classes: classes.len(),
            ordered,
            planted_syms: Some(sorted),
        });
    }

    let b_real = cell.real.len();
    let hidden_targets: Vec<Value> = sel
        .iter()
        .map(|s| {
            let cs: BTreeSet<Vec<i64>> = s.decomps.iter().map(|d| net_vector(b_real, d)).collect();
            json!({
                "name": s.name,
                "a": s.a, "b": s.b,
                "decompositions": s.decomps.iter().map(|d| json!({"signed_base": signed_json(d), "c": net_vector(b_real, d)})).collect::<Vec<_>>(),
                "distinct_net_vectors": cs.len(),
                "algebraic_v_multisets": s.alg_multisets,
                "planted_symbols": s.planted_syms.as_ref().map(|d| signed_json(d)),
            })
        })
        .collect();
    let hidden = seal(json!({
        "schema": "pkm.m8-panel.hidden/v1",
        "cell_sha256": cell.sha256,
        "k": k,
        "note": "HIDDEN: the solver pipeline never reads this file; the protocol seals it by sha256. signed_base lists +-(1-based index into the cell's real_base).",
        "targets": hidden_targets,
    }));
    let public_targets: Vec<Value> = sel
        .iter()
        .map(|s| {
            json!({
                "name": s.name, "label": s.label, "planted": s.label == "planted",
                "draw_index": s.draw, "x_r": s.x_r, "a": s.a, "b": s.b,
                "real_count": s.decomps.len(), "algebraic_count": s.alg_multisets.len(),
                "algebraic_signed_classes": s.alg_classes, "ordered_algebraic_solutions": s.ordered,
            })
        })
        .collect();
    let sat_total = sat_draws.len() as u64;
    let public = seal(json!({
        "schema": "pkm.m8-panel.targets/v1",
        "cell_sha256": cell.sha256,
        "seed": seed, "m": m,
        "Q": {"x": q.x, "y": q.y},
        "hidden_sha256": hidden["sha256"],
        "label_leak_note": "label (sat/unsat/planted) and the real/algebraic counts are classifier metadata needed for the panel's stratification; they are NOT inputs to a solver. a, b are the attacker's own multipliers of R = aP + bQ (needed to form relation rows), k is hidden.",
        "draws": {
            "draws_total": draws, "identity_draws": identity_draws,
            "real_sat_draws": sat_total,
            "empirical_natural_sat_rate": sat_total as f64 / draws.max(1) as f64,
            "real_sat_draw_indices": sat_draws,
            "selected_draw_indices": sel.iter().filter_map(|s| s.draw).collect::<Vec<_>>(),
            "stopping_rule": format!("draw in order until {n_sat} real-SAT and {n_unsat} real-UNSAT are kept; SAT = at least one real signed multiset of size {m} sums to R"),
        },
        "enumeration": {
            "real_leaves": real.leaves, "real_entries": real.entries.len(),
            "alg_leaves": alg.leaves, "alg_entries": alg.entries.len(),
            "seconds": enum_secs, "total_seconds": t0.elapsed().as_secs_f64(),
            "definition": "real decomposition = signed multiset of m real base points with sum exactly R; algebraic = signed multisets over the 2B lifts of V to E(F_{p^2}) whose sum has abscissa x_R; algebraic_count = distinct unsigned multisets of V among them (gen.py decomposable); ordered_algebraic_solutions = sequences with no identity prefix (formula: targets.rs enumerate_alg)",
        },
        "targets": public_targets,
        "tool": crate::tool_info(),
    }));
    write_json(&Path::new(&out).join("hidden.json"), &hidden)?;
    write_json(&Path::new(&out).join("targets.json"), &public)?;
    println!(
        "targets: {} draws, {} real-SAT draws, rate {:.3e}; wrote {out}",
        draws,
        sat_total,
        sat_total as f64 / draws.max(1) as f64
    );
    Ok(())
}
