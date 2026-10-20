use crate::{arithmetic::*, recipe::*, store::*};
use anyhow::{ensure, Result};
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
    time::Instant,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Workload {
    pub split: String,
    pub seed: u64,
    pub queries: usize,
    pub arity: usize,
    pub relation_state_cap: usize,
    pub regularity_variable_cap: usize,
    pub regularity_degree_cap: u32,
    pub regularity_targets: usize,
    #[serde(default)]
    pub targets: Option<Vec<PointSpec>>,
}
struct Search<'a> {
    curve: &'a Curve,
    base: &'a [Point],
    targets: BTreeMap<Point, Vec<usize>>,
    witnesses: Vec<Option<Vec<usize>>>,
    visited: usize,
    leaves: usize,
    cap: usize,
}
impl Search<'_> {
    fn walk(&mut self, left: usize, start: usize, sum: P, tuple: &mut Vec<usize>) -> bool {
        if self.visited >= self.cap {
            return false;
        }
        self.visited += 1;
        if left == 0 {
            self.leaves += 1;
            if let Some(p) = sum {
                if let Some(indices) = self.targets.get(&p) {
                    for &i in indices {
                        if self.witnesses[i].is_none() {
                            self.witnesses[i] = Some(tuple.clone());
                        }
                    }
                }
            }
            return true;
        }
        for i in start..self.base.len() {
            if tuple
                .iter()
                .any(|&j| self.curve.neg(&self.base[j]) == self.base[i])
            {
                continue;
            }
            tuple.push(i);
            let next = self.curve.add(&sum, &Some(self.base[i].clone()));
            let complete = self.walk(left - 1, i, next, tuple);
            tuple.pop();
            if !complete {
                return false;
            }
        }
        true
    }
}
pub fn evaluate(root: &Path, id: &str, w: &Workload) -> Result<Value> {
    ensure!((2..=8).contains(&w.arity), "arity must be 2..=8");
    ensure!(
        (1..=10_000_000).contains(&w.relation_state_cap),
        "invalid relation state cap"
    );
    ensure!(
        w.regularity_variable_cap <= 14
            && w.regularity_degree_cap <= 15
            && w.regularity_targets <= 8,
        "regularity caps exceed reference engine limits"
    );
    ensure!(
        w.split == "discovery" || w.split == "holdout",
        "invalid workload split"
    );
    ensure!((1..=1024).contains(&w.queries), "invalid query count");
    let t = Instant::now();
    let (_, g) = verify(root, id)?;
    let verify_ms = t.elapsed().as_secs_f64() * 1000.;
    let targets = if let Some(targets) = &w.targets {
        ensure!(targets.len() == w.queries, "supplied target count mismatch");
        targets
            .iter()
            .map(Point::from_spec)
            .collect::<Result<Vec<_>>>()?
    } else {
        sample_targets(&g.curve, w.queries, w.seed, &w.split)?
    };
    ensure!(
        targets.iter().all(|p| g.curve.subgroup(p)),
        "target outside specified subgroup"
    );
    let mut lookup = BTreeMap::<Point, Vec<usize>>::new();
    for (i, p) in targets.iter().enumerate() {
        lookup.entry(p.clone()).or_default().push(i);
    }
    let mut search = Search {
        curve: &g.curve,
        base: &g.points,
        targets: lookup,
        witnesses: vec![None; targets.len()],
        visited: 0,
        leaves: 0,
        cap: w.relation_state_cap,
    };
    let t = Instant::now();
    let complete = search.walk(w.arity, 0, None, &mut Vec::new());
    let relation_ms = t.elapsed().as_secs_f64() * 1000.;
    for (i, witness) in search.witnesses.iter().enumerate() {
        if let Some(indices) = witness {
            let sum = indices
                .iter()
                .fold(None, |p, &j| g.curve.add(&p, &Some(g.points[j].clone())));
            ensure!(sum == Some(targets[i].clone()), "relation replay failed");
        }
    }
    let hits = search.witnesses.iter().filter(|x| x.is_some()).count();
    let yield_value = if complete || hits == targets.len() {
        Some(hits as f64 / targets.len() as f64)
    } else {
        None
    };
    let t = Instant::now();
    let regularity = targets
        .iter()
        .take(w.regularity_targets)
        .map(|target| regularity(&g, target, w))
        .collect::<Vec<_>>();
    let regularity_ms = t.elapsed().as_secs_f64() * 1000.;
    let semantics = json!({"schema":"factor-base-evaluation/v1","manifest_sha256":id,"curve_sha256":digest(&canonical(&g.curve.spec)?),"workload":w,"workload_sha256":digest(&canonical(w)?),"targets":targets.iter().map(Point::spec).collect::<Vec<_>>(),
        "sampling":if w.targets.is_some(){"supplied_targets_no_population_inference"}else{"seeded_uniform_finite_subgroup_points_with_replacement"},
        "relation_status":if complete{"complete"}else{"censored_state_cap"},"base_points":g.points.len(),"visited_states":search.visited,"admissible_multisets_visited":search.leaves,"verified_hits":hits,"queries":targets.len(),"relation_yield":yield_value,"witnessed_yield_lower_bound":hits as f64/targets.len()as f64,"witnesses":search.witnesses,
        "wilson_95":if yield_value.is_some()&&w.targets.is_none(){Some(wilson(hits,targets.len()))}else{None},
        "regularity":regularity,"useful_relation_rank":null,"selected_best_total_runtime":null,"cold_total_ecdlp_cost":null,"online_ecdlp_cost":null,
        "relation_policy":"unordered_multisets_repetition_allowed_inverse_pairs_excluded",
        "accounting_scope":"factor_base_precomputation_and_relation_diagnostics"});
    Ok(
        json!({"semantic":semantics,"semantic_sha256":digest(&canonical(&semantics)?),"runtime":{"verification_ms":verify_ms,"relation_enumeration_ms":relation_ms,"regularity_ms":regularity_ms,"wall_time_qualification":"unqualified_diagnostic","source_sha256":source_digest()}}),
    )
}
fn wilson(h: usize, n: usize) -> [f64; 2] {
    let p = h as f64 / n as f64;
    let z2 = 1.96 * 1.96;
    let den = 1. + z2 / n as f64;
    let center = (p + z2 / (2. * n as f64)) / den;
    let half = 1.96 * ((p * (1. - p) + z2 / (4. * n as f64)) / n as f64).sqrt() / den;
    [(center - half).max(0.), (center + half).min(1.)]
}
pub fn regularity(g: &Generated, target: &Point, w: &Workload) -> Value {
    let unsupported = |status: &str| json!({"target":target.spec(),"status":status,"degree_of_regularity":null,"affine_solving_degree":null});
    let Some(v) = &g.basis else {
        return unsupported("unsupported_factor_base_encoding");
    };
    let Some(os) = offsets(&g.recipe) else {
        return unsupported("unsupported_factor_base_encoding");
    };
    if os.len() != 1 {
        return unsupported("unsupported_union_encoding");
    }
    let f = &g.curve.f;
    let field_degree = f.degree().unwrap();
    let vars = w.arity * v.len() + (w.arity - 2) * field_degree;
    if vars > w.regularity_variable_cap {
        return unsupported("censored_variable_cap");
    }
    let offset = parse(&os[0]).unwrap();
    let mut values = Vec::with_capacity(1 << vars);
    for mask in 0..1usize << vars {
        let mut xs = vec![offset.clone(); w.arity];
        for (j, x) in xs.iter_mut().enumerate() {
            for (i, b) in v.iter().enumerate() {
                if mask & (1 << (j * v.len() + i)) != 0 {
                    *x ^= b;
                }
            }
        }
        let aux = (0..w.arity - 2)
            .map(|j| {
                n(((mask >> (w.arity * v.len() + j * field_degree))
                    & ((1usize << field_degree) - 1)) as u64)
            })
            .collect::<Vec<_>>();
        let mut packed = N::zero();
        for i in 0..w.arity - 1 {
            let x = if i == 0 { &xs[0] } else { &aux[i - 1] };
            let y = &xs[i + 1];
            let z = if i == w.arity - 2 { &target.x } else { &aux[i] };
            packed |= semaev_s3(&g.curve, x, y, z) << (i * field_degree);
        }
        values.push(packed);
    }
    // Truth table -> exact Boolean ANF by Mobius transform, coefficient by coefficient.
    for bit in 0..vars {
        for mask in 0..1usize << vars {
            if mask & (1 << bit) != 0 {
                let other = values[mask ^ (1 << bit)].clone();
                values[mask] ^= other;
            }
        }
    }
    let equations = (0..(w.arity - 1) * field_degree)
        .map(|bit| {
            values
                .iter()
                .enumerate()
                .filter_map(|(mask, c)| c.bit(bit as u64).then_some(mask as u64))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let degree = equations
        .iter()
        .flatten()
        .map(|m| m.count_ones())
        .max()
        .unwrap_or(0);
    let d = leading_regularity(&equations, vars, w.regularity_degree_cap);
    json!({"target":target.spec(),"status":if d.is_some(){"measured"}else{"censored_degree_cap"},"variables":vars,"equations":equations.len(),"system_total_degree":degree,"degree_of_regularity":d,"affine_solving_degree":null,
        "definition":"first_saturated_degree_of_homogeneous_leading_form_ideal_in_F2[z]/(z_i^2)","encoding":"binary_Semaev_S3_chain_on_affine_x_subspace","equation_sha256":digest(&canonical(&equations).unwrap()),"rational_lifting_and_subgroup_constraints_in_ideal":false,"intermediate_infinity_branches_encoded":false})
}
pub fn semaev_s3(c: &Curve, x: &N, y: &N, z: &N) -> N {
    let f = &c.f;
    let xy = f.mul(x, y);
    let cross = f.add(&f.add(&xy, &f.mul(x, z)), &f.mul(y, z));
    f.add(&f.add(&f.sq(&cross), &f.mul(&xy, z)), &c.b)
}
pub fn leading_regularity(equations: &[Vec<u64>], vars: usize, cap: u32) -> Option<u32> {
    let generators = equations
        .iter()
        .filter_map(|e| {
            e.iter().map(|t| t.count_ones()).max().map(|d| {
                (
                    d,
                    e.iter()
                        .copied()
                        .filter(|t| t.count_ones() == d)
                        .collect::<Vec<_>>(),
                )
            })
        })
        .collect::<Vec<_>>();
    if generators.iter().any(|(d, _)| *d == 0) {
        return Some(0);
    }
    let limit = 1u64 << vars;
    for d in 1..=cap.min(vars as u32) {
        let cols = (0..limit)
            .filter(|m| m.count_ones() == d)
            .collect::<Vec<_>>();
        let lookup = cols
            .iter()
            .enumerate()
            .map(|(i, &m)| (m, i))
            .collect::<HashMap<_, _>>();
        let mut pivots = vec![None::<N>; cols.len()];
        let mut rank = 0;
        for (gd, terms) in &generators {
            if *gd > d {
                continue;
            }
            for multiplier in (0..limit).filter(|m| m.count_ones() == d - gd) {
                let mut row = N::zero();
                for term in terms {
                    if term & multiplier == 0 {
                        row ^= N::one() << lookup[&(term | multiplier)];
                    }
                }
                while !row.is_zero() {
                    let k = row.bits() as usize - 1;
                    if let Some(p) = &pivots[k] {
                        row ^= p;
                    } else {
                        pivots[k] = Some(row);
                        rank += 1;
                        break;
                    }
                }
                if rank == cols.len() {
                    return Some(d);
                }
            }
        }
    }
    if cap > vars as u32 {
        Some(vars as u32 + 1)
    } else {
        None
    }
}
pub fn replay(root: &Path, id: &str, receipt: &Value) -> Result<Value> {
    let w: Workload = serde_json::from_value(receipt["semantic"]["workload"].clone())?;
    let actual = evaluate(root, id, &w)?;
    ensure!(
        actual["semantic"] == receipt["semantic"]
            && actual["semantic_sha256"] == receipt["semantic_sha256"],
        "evaluation replay differs"
    );
    Ok(
        json!({"status":"replayed","semantic_sha256":actual["semantic_sha256"],"independence":"same_native_implementation"}),
    )
}
