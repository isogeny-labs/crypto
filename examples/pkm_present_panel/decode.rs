//! Loading a `system.json`, decoding solutions to abscissa multisets and verifying them by
//! counted point additions (`[pcc:unverified]` instrument construction).
use crate::ec::{lift_small, Ec};
use crate::present::{eval_terms, Decode, Terms};
use crate::targets::signed_json;
use crate::util::*;
use crypto_lib::cryptanalysis::glv_invariant_base::poly_roots_fp_counted_until;
use crypto_lib::cryptanalysis::ic_boundary::{GroupOps, PrimePoint};
use serde_json::{json, Value};
use std::collections::{BTreeSet, HashMap};
use std::path::Path;

pub struct Loaded {
    pub presentation: String,
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub g: u64,
    pub m: usize,
    pub t: usize,
    pub x_r: u64,
    pub vars: Vec<String>,
    pub polys: Vec<Terms>,
    pub decode: Decode,
    pub base: Option<Vec<(u64, u64)>>,
    pub classifier_real_count: Option<u64>,
    pub classifier_distinct: Option<u64>,
    pub json: Value,
}

pub fn load(path: &str) -> Result<Loaded, String> {
    let j = read_json(Path::new(path))?;
    if j.get("sha256").is_some() && !check_seal(&j) {
        return Err(format!("{path}: seal mismatch"));
    }
    let (p, m, t, polys) = crate::system::polys_from_json(&j)?;
    let vars: Vec<String> = match j["variables"].as_array() {
        Some(v) => v.iter().map(|x| x.as_str().unwrap_or("").to_string()).collect(),
        None => crate::system::var_names(m, t),
    };
    let g = u64_of(&j, "g")?;
    let decode = match j["decode"]["kind"].as_str() {
        Some("elem") => Decode::Elem { idx: u64_vec(&j["decode"]["idx"])?.into_iter().map(|x| x as usize).collect(), scale: j["decode"]["scale"].as_u64().unwrap_or(1) },
        Some("vars") => Decode::Vars {
            idx: u64_vec(&j["decode"]["idx"])?.into_iter().map(|x| x as usize).collect(),
            scale: u64_of(&j["decode"], "scale")?,
        },
        _ => Decode::Vars { idx: (0..m).map(|i| i * t).collect(), scale: g },
    };
    let vd = &j["verify_data"];
    let base = vd["real_base"].as_array().map(|a| {
        a.iter().map(|e| (e["x"].as_u64().unwrap_or(0), e["y"].as_u64().unwrap_or(0))).collect::<Vec<_>>()
    });
    Ok(Loaded {
        presentation: j["presentation"].as_str().unwrap_or("chain-tower").to_string(),
        p,
        a: u64_of(&j, "a")?,
        b: u64_of(&j, "b")?,
        g,
        m,
        t,
        x_r: u64_of(&j, "x_r")?,
        vars,
        polys,
        decode,
        base,
        classifier_real_count: vd["real_count"].as_u64(),
        classifier_distinct: vd["real_count_distinct"].as_u64(),
        json: j,
    })
}

pub struct Lifter {
    ec: Ec,
    m: usize,
    r: PrimePoint,
    base: Vec<(u64, u64)>,
    idx: HashMap<u64, usize>,
}

impl Lifter {
    pub fn new(ld: &Loaded) -> Result<Lifter, String> {
        let base = ld.base.clone().ok_or("system.json carries no verify_data.real_base (re-export it)")?;
        let (xr, yr) = lift_small(ld.p, ld.a, ld.b, ld.x_r).ok_or("x_R is not an abscissa of the curve")?;
        let ec = Ec::new(ld.p, ld.a, ld.b);
        let r = ec.pt(xr, yr);
        let idx = base.iter().enumerate().map(|(i, &(x, _))| (x, i)).collect();
        Ok(Lifter { ec, m: ld.m, r, base, idx })
    }
    /// Sorted symbol lists (`2i` = `+P_i`, `2i+1` = `-P_i`) whose point sum is exactly `R`;
    /// `None` if some abscissa is not a real base abscissa.
    pub fn decomps(&self, xs: &[u64], ops: &mut GroupOps) -> Option<BTreeSet<Vec<u8>>> {
        let p = self.ec.c.p;
        let mut cand: Vec<(PrimePoint, PrimePoint, u8)> = Vec::new();
        for x in xs {
            let i = *self.idx.get(x)?;
            let (bx, by) = self.base[i];
            cand.push((self.ec.pt(bx, by), self.ec.pt(bx, (p - by) % p), (2 * i) as u8));
        }
        let mut out = BTreeSet::new();
        let mut stack: Vec<(usize, PrimePoint, Vec<u8>)> = vec![(0, PrimePoint::INFINITY, vec![])];
        while let Some((d, acc, syms)) = stack.pop() {
            if d == self.m {
                if acc == self.r {
                    let mut s = syms.clone();
                    s.sort_unstable();
                    out.insert(s);
                }
                continue;
            }
            let (pos, neg, s0) = &cand[d];
            for (q, s) in [(*pos, *s0), (*neg, *s0 + 1)] {
                let nacc = self.ec.add(ops, acc, q);
                let mut ns = syms.clone();
                ns.push(s);
                stack.push((d + 1, nacc, ns));
            }
        }
        Some(out)
    }
}

pub struct Extract {
    pub assignments: usize,
    pub eq_check_failures: usize,
    pub non_split_elem: usize,
    pub multisets: BTreeSet<Vec<u64>>,
    pub verified: BTreeSet<Vec<u8>>,
    pub verified_multisets: usize,
    pub not_real_multisets: usize,
    pub root_muls: u64,
    pub ops: GroupOps,
}

/// Decode, equation-check and lift full assignments of the system's variables.
pub fn process(ld: &Loaded, sols: &[Vec<u64>], seed: u64) -> Result<Extract, String> {
    let lifter = Lifter::new(ld)?;
    let mut ex = Extract {
        assignments: 0,
        eq_check_failures: 0,
        non_split_elem: 0,
        multisets: BTreeSet::new(),
        verified: BTreeSet::new(),
        verified_multisets: 0,
        not_real_multisets: 0,
        root_muls: 0,
        ops: GroupOps::default(),
    };
    let p = ld.p;
    for s in sols {
        ex.assignments += 1;
        if ld.polys.iter().any(|f| eval_terms(f, s, p) != 0) {
            ex.eq_check_failures += 1;
        }
        let mut xs: Vec<u64> = match &ld.decode {
            Decode::Vars { idx, scale } => idx.iter().map(|&i| mulm(*scale, s[i], p)).collect(),
            Decode::Elem { idx, scale } => {
                let e: Vec<u64> = idx.iter().map(|&i| s[i]).collect();
                let coeffs = crate::mpoly::p_poly_from_e(&e, p);
                let roots = poly_roots_fp_counted_until(&coeffs, p, seed, &mut ex.root_muls, None).unwrap_or_default();
                if roots.len() != ld.m {
                    ex.non_split_elem += 1;
                    continue;
                }
                roots.iter().map(|&w| mulm(*scale, w, p)).collect()
            }
        };
        xs.sort_unstable();
        if !ex.multisets.insert(xs.clone()) {
            continue;
        }
        match lifter.decomps(&xs, &mut ex.ops) {
            None => ex.not_real_multisets += 1,
            Some(ds) => {
                if !ds.is_empty() {
                    ex.verified_multisets += 1;
                }
                ex.verified.extend(ds);
            }
        }
    }
    Ok(ex)
}

fn distinct_abscissae(d: &[u8]) -> bool {
    let mut v: Vec<u8> = d.iter().map(|s| s / 2).collect();
    v.sort_unstable();
    v.windows(2).all(|w| w[0] != w[1])
}

pub fn extract_json(ld: &Loaded, ex: &Extract) -> Value {
    let sym = crate::present::is_symmetric(&ld.presentation);
    let expected = if sym { ld.classifier_distinct } else { ld.classifier_real_count };
    let base = ld.base.clone().unwrap_or_default();
    let mut decs: Vec<Vec<(u64, i64)>> = ex
        .verified
        .iter()
        .map(|d| {
            let mut v: Vec<(u64, i64)> = d.iter().map(|&s| (base[(s / 2) as usize].0, if s % 2 == 0 { 1 } else { -1 })).collect();
            v.sort_unstable();
            v
        })
        .collect();
    decs.sort();
    let unlifted = ex.multisets.len() - ex.verified_multisets;
    json!({
        "fp_rational_solutions": ex.assignments,
        "distinct_x_multisets": ex.multisets.len(),
        "verified_real_decompositions": decs,
        "verified_real_decompositions_count": ex.verified.len(),
        "verified_decomposition_format": "list of decompositions, each a list of [x, sign] pairs (sign +1 = the point with y <= (p-1)/2, -1 = its negative), pairs sorted by (x, sign), decompositions sorted lexicographically",
        "verified_distinct_abscissa": ex.verified.iter().filter(|d| distinct_abscissae(d)).count(),
        "unlifted_solutions": unlifted,
        "unlifted_multisets_not_real_base": ex.not_real_multisets,
        "equation_check_failures": ex.eq_check_failures,
        "non_split_elem_vectors": ex.non_split_elem,
        "classifier_real_count": ld.classifier_real_count,
        "classifier_real_count_distinct": ld.classifier_distinct,
        "classifier_expected": expected,
        "matches_classifier": expected.map(|e| e as usize == if sym { ex.verified.iter().filter(|d| distinct_abscissae(d)).count() } else { ex.verified.len() }),
        "lift_field_multiplications": ex.ops.field_ops.muls + ex.ops.field_ops.sqrs,
        "lift_and_root_field_multiplications": ex.ops.field_ops.muls + ex.ops.field_ops.sqrs + ex.root_muls,
        "decompositions_signed_base": ex.verified.iter().map(|d| signed_json(d)).collect::<Vec<_>>(),
        "solution_semantics": crate::present::semantics_class(&ld.presentation),
        "solution_semantics_detail": crate::present::semantics(&ld.presentation),
        "work": {
            "point_adds": ex.ops.adds, "point_doubles": ex.ops.doubles,
            "point_field_ops": {"muls": ex.ops.field_ops.muls, "sqrs": ex.ops.field_ops.sqrs, "invs": ex.ops.field_ops.invs},
            "root_finding_field_muls": ex.root_muls,
        },
        "lift_note": "x_i must be a real base abscissa; decomposition = sorted signed-symbol list whose point sum is exactly R (R lifted to y <= (p-1)/2; counts are invariant under R -> -R)",
    })
}
