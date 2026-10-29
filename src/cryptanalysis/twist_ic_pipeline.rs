//! Bounded twist-route ablations for public small prime curves. Construction
//! runs afresh in every cold pipeline. Native arithmetic and symbolic work
//! remain explicitly unpriced; these adapters establish no speed claim.
use super::f4_fp::{self as f4, F4Options, Ordering};
use super::ic_boundary::{
    lift_abscissae, CountedGroup, FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimePoint,
};
use super::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, SystemShape,
};
use super::twist_ic_algebra as alg;
use super::twist_ic_field::{inv, reference_calls, sqrt, Quad, Q};
use std::collections::{BTreeMap, BTreeSet};

pub struct TwistExperimentBase;

/// First translate in ascending order whose *coefficient isomorphism* identifies
/// the complementary elliptic curve with the requested quadratic twist.
fn select_cover(e: Quad, work: &mut u64) -> Option<u64> {
    let p = e.p;
    let (a, b, d) = (e.a, e.b, e.d);
    let ta = d * d % p * a % p;
    let tb = d * d % p * d % p * b % p;
    let i3 = inv(3, p);
    let i27 = inv(27, p);
    for c in 0..p {
        *work += 1;
        if e.rhs(Q(c, 0)) == Q(0, 0) {
            continue;
        }
        let ad = ((3 * a % p * c % p * c + 9 * b % p * c + p - a * a % p) % p) * i3 % p;
        let bd = ((p - 27 * b % p * c % p * c % p * c % p
            + 18 * a % p * a % p * c % p * c % p
            + 27 * a % p * b % p * c % p
            + 27 * b % p * b % p
            + 2 * a % p * a % p * a % p)
            % p)
            * i27
            % p;
        if ad == 0 || bd == 0 {
            continue;
        }
        let s = bd * ta % p * inv(tb * ad % p, p) % p;
        if ta * s % p * s % p == ad && tb * s % p * s % p * s % p == bd && sqrt(s, p).is_some() {
            return Some(c);
        }
    }
    None
}

impl FactorBaseBuilder<PrimeCurve> for TwistExperimentBase {
    fn name(&self) -> &str {
        "prime-twist-experiment"
    }
    fn describe(&self, p: &Params) -> String {
        format!(
            "cold trace={}, cover={}, H={}, at most {} negation columns; undersized bases retained",
            p.flag("trace"),
            p.flag("cover"),
            p.get("h").unwrap_or("8"),
            p.get("size").unwrap_or("8")
        )
    }
    fn build(
        &self,
        ctx: &InstanceCtx<PrimeCurve>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<FactorBase<PrimePoint>, String> {
        let g = ctx.group;
        if g.p <= 3 || g.p >= 32768 || g.a == 0 || g.b == 0 || ctx.cofactor != 1 || ctx.r % 2 == 0 {
            return Err("twist experiment requires its registered small, generic-j, odd prime-order fixtures".into());
        }
        let size = params.u64_or("size", 8)? as usize;
        let h = params.u64_or("h", 8)?;
        if size == 0 || size > 16 || h == 0 || h > 16 || h >= g.p {
            return Err("twist experiment: 1<=size,H<=16 and H<p required".into());
        }
        let before = reference_calls();
        let e = Quad::new(g.p, g.a, g.b);
        let mut probes = 0;
        let mut trace = BTreeSet::new();
        if params.flag("trace") {
            for u in 0..h {
                for v in 0..h {
                    probes += 1;
                    let x = Q(u, v);
                    if let Some(y) = e.sqrt(e.rhs(x)) {
                        if let Some((x, y)) = e.trace(Some((x, y))) {
                            assert_eq!((x.1, y.1), (0, 0));
                            trace.insert(x.0);
                        }
                    }
                }
            }
        }
        let mut cover = BTreeSet::new();
        let mut cover_candidates = 0;
        if params.flag("cover") {
            let c = select_cover(e, &mut cover_candidates)
                .ok_or("no coefficient-verified twist complement in this cover family")?;
            for u in 0..h {
                probes += 1;
                let x = (u * u + c) % g.p;
                if sqrt(e.rhs(Q(x, 0)).0, g.p).is_some() {
                    cover.insert(x);
                }
            }
        }
        let xs: Vec<_> = match (params.flag("trace"), params.flag("cover")) {
            (true, true) => trace.intersection(&cover).copied().take(size).collect(),
            (true, false) => trace.into_iter().take(size).collect(),
            (false, true) => cover.into_iter().take(size).collect(),
            (false, false) => {
                let mut xs = vec![];
                for x in 1..g.p {
                    probes += 1;
                    if sqrt(e.rhs(Q(x, 0)).0, g.p).is_some() {
                        xs.push(x);
                    }
                    if xs.len() == size {
                        break;
                    }
                }
                xs
            }
        };
        if xs.is_empty() {
            return Err(format!("empty trace/cover intersection; {} discovery calls, {probes} probes, {cover_candidates} cover candidates",
                reference_calls() - before));
        }
        let mut points = vec![];
        let mut cols = vec![];
        let mut coefficients = vec![];
        for (col, &x) in xs.iter().enumerate() {
            let y = sqrt(e.rhs(Q(x, 0)).0, g.p).ok_or("non-rational abscissa")?;
            if y == 0 {
                return Err("unexpected two-torsion in odd prime-order fixture".into());
            }
            let point = PrimePoint::affine(x, y.min(g.p - y));
            assert!(g.is_on_curve(point));
            points.extend([point, g.neg(point)]);
            cols.extend([col, col]);
            coefficients.extend([1, ctx.r - 1]);
        }
        let mut fb = FactorBase::from_column_map(
            self.describe(params),
            points,
            cols,
            coefficients,
            xs.len(),
            |p| g.key(p),
            |p| g.key(&g.neg(*p)),
            |p| p.x,
        )?;
        fb.cost.count(
            "reference_arithmetic_calls_uncharged",
            reference_calls() - before,
        );
        fb.cost.count("construction_probes_uncharged", probes);
        fb.cost
            .count("cover_coefficient_candidates_uncharged", cover_candidates);
        fb.cost
            .count("index_entries_uncharged", (xs.len() * 2) as u64);
        fb.cost.count("requested_columns", size as u64);
        fb.cost.count("undersized_base", u64::from(xs.len() < size));
        Ok(fb)
    }
}

pub struct TwistF4Oracle {
    symmetric: bool,
    degree: u32,
    counts: BTreeMap<String, u64>,
    shape: Option<SystemShape>,
}
impl TwistF4Oracle {
    pub fn new(symmetric: bool) -> Self {
        Self {
            symmetric,
            degree: 16,
            counts: BTreeMap::new(),
            shape: None,
        }
    }
    fn count(&mut self, key: &str, value: u64) {
        *self.counts.entry(key.into()).or_default() += value;
    }
}
impl DecompositionOracle<PrimeCurve> for TwistF4Oracle {
    fn name(&self) -> &str {
        if self.symmetric {
            "twist-f4-symmetric"
        } else {
            "twist-f4-product"
        }
    }
    fn summands(&self) -> u32 {
        2
    }
    fn describe(&self, _: &Params) -> String {
        format!("{}: finite-base F4 followed by explicit, charged finite-domain root extraction and point lifting", self.name())
    }
    fn prepare(
        &mut self,
        _: &InstanceCtx<PrimeCurve>,
        _: &FactorBase<PrimePoint>,
        p: &Params,
        _: &mut GroupOps,
    ) -> Result<(), String> {
        self.degree = p.u64_or("degree", 16)? as u32;
        if self.degree == 0 || self.degree > 24 {
            return Err("degree must lie in 1..=24".into());
        }
        self.counts.clear();
        self.shape = None;
        Ok(())
    }
    fn decompose(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        ops: &mut GroupOps,
        _: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        // The symmetric chart excludes repeated first abscissae, and x has no
        // affine chart at O. Check both branches with real group additions.
        if point.infinity {
            return Some(vec![0, fb.neg_index[0]]);
        }
        if self.symmetric {
            for &x in &fb.abscissa_list {
                self.count("exceptional_branch_probes_uncharged", 1);
                if let Some(indices) = lift_abscissae(ctx.group, fb, ops, &[x, x], point) {
                    return Some(indices);
                }
            }
        }
        let p = ctx.group.p;
        let case = alg::build(
            p,
            ctx.group.a,
            ctx.group.b,
            point.x,
            &fb.abscissa_list,
            2,
            if self.symmetric {
                "symmetric"
            } else {
                "product"
            },
        );
        self.count(
            "system_input_terms_uncharged",
            case.equations.iter().map(|p| p.len() as u64).sum(),
        );
        self.shape = Some(SystemShape {
            n_vars: case.names.len(),
            n_equations: case.equations.len(),
            degrees: case
                .equations
                .iter()
                .map(|f| f.iter().map(|(e, _)| e.iter().sum()).max().unwrap_or(0))
                .collect(),
            semi_regular_degree: None,
        });
        let rep = f4::f4(
            &case.equations,
            case.names.len(),
            p,
            &F4Options::new(Ordering::Grevlex, self.degree),
        );
        self.count("f4_calls", 1);
        self.count("f4_field_muls_uncharged", rep.field_ops);
        self.count("f4_steps_uncharged", rep.steps as u64);
        self.count("f4_pairs_above_bound", rep.pairs_above_bound as u64);
        let degree = self
            .counts
            .entry("f4_solving_degree_max".into())
            .or_default();
        *degree = (*degree).max(rep.solving_degree_max as u64);
        if rep.inconsistent {
            self.count("f4_refutations", 1);
            return None;
        }
        if rep.timed_out || rep.pairs_above_bound > 0 || rep.staircase_at_stop.is_some() {
            self.count("f4_unresolved_calls", 1);
            return None;
        }
        for (i, &x) in fb.abscissa_list.iter().enumerate() {
            for (j, &y) in fb.abscissa_list.iter().enumerate().skip(i) {
                if self.symmetric && i == j {
                    continue;
                }
                self.count("root_extraction_candidates_uncharged", 1);
                let values = if self.symmetric {
                    vec![(x + y) % p, x * y % p]
                } else {
                    vec![x, y]
                };
                let mut valid = true;
                for poly in rep.basis.iter().chain(&case.equations) {
                    self.count("polynomial_terms_evaluated_uncharged", poly.len() as u64);
                    if f4::eval(poly, &values, p) != 0 {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    if let Some(indices) = lift_abscissae(ctx.group, fb, ops, &[x, y], point) {
                        self.count("verified_decompositions", 1);
                        return Some(indices);
                    }
                }
                let _ = j;
            }
        }
        None
    }
    fn relation_counters(&self) -> BTreeMap<String, u64> {
        self.counts.clone()
    }
    fn last_system(&self) -> Option<SystemShape> {
        self.shape.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independently_frozen_cover_choices_and_all_small_target_decompositions() {
        for (p, a, b, r, c) in [(59, 1, 13, 67, 55), (827, 1, 15, 823, 431)] {
            let e = Quad::new(p, a, b);
            assert_eq!(select_cover(e, &mut 0), Some(c));
            let g = PrimeCurve { p, a, b };
            let y = sqrt(e.rhs(Q(1, 0)).0, p);
            let generator = if let Some(y) = y {
                PrimePoint::affine(1, y)
            } else {
                let (x, y) = e.rational().into_iter().flatten().next().unwrap();
                PrimePoint::affine(x.0, y.0)
            };
            let ctx = InstanceCtx {
                group: &g,
                generator,
                target: generator,
                r,
                cofactor: 1,
                group_order: r,
                name: "test".into(),
                field_degree: None,
            };
            for t in [false, true] {
                for c in [false, true] {
                    let mut params = Params::default();
                    params
                        .set("size", "4")
                        .set("h", "8")
                        .set("trace", t.to_string())
                        .set("cover", c.to_string());
                    let Ok(fb) = TwistExperimentBase.build(&ctx, &params, &mut GroupOps::default())
                    else {
                        continue;
                    };
                    let targets: Vec<_> = e
                        .rational()
                        .into_iter()
                        .map(|q| {
                            q.map_or(PrimePoint::INFINITY, |(x, y)| PrimePoint::affine(x.0, y.0))
                        })
                        .collect();
                    // Complete p=59 group; a held-out deterministic prefix at p=827.
                    for target in targets
                        .into_iter()
                        .take(if p == 59 { usize::MAX } else { 32 })
                    {
                        let mut scratch = GroupOps::default();
                        let exists = fb.points.iter().any(|x| {
                            fb.points
                                .iter()
                                .any(|y| g.add(&mut scratch, *x, *y) == target)
                        });
                        for symmetric in [false, true] {
                            let mut oracle = TwistF4Oracle::new(symmetric);
                            let answer = oracle.decompose(
                                &ctx,
                                &fb,
                                &mut scratch,
                                &mut OracleCounters::default(),
                                target,
                            );
                            assert_eq!(answer.is_some(), exists, "p={p}, trace={t}, cover={c}, symmetric={symmetric}, target={target:?}");
                            if let Some(indices) = answer {
                                let sum = indices.into_iter().fold(PrimePoint::INFINITY, |s, i| {
                                    g.add(&mut scratch, s, fb.points[i])
                                });
                                assert_eq!(sum, target);
                            }
                        }
                    }
                }
            }
        }
    }
}
