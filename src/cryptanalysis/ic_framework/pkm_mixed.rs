//! `pkm-mixed`: PKM's Semaev oracle on a **mixed-degree** Vélu chain.
//!
//! The `prime-mixed-isogeny` base builds `V = x(R' + ⟨Q⟩)` as a chain of Vélu
//! steps of degrees `q_1, …, q_n0` (`∏ q_j = S ≈ p^{1/m}`). The paper's system
//! (4) with denominators cleared is
//!
//! ```text
//! S_3(x_1, x_2, x_R) = 0
//! y_(i,j+1)·D_j(y_(i,j)) − N_j(y_(i,j)) = 0
//! N_n0(y_(i,n0)) − c·D_n0(y_(i,n0)) = 0
//! ```
//!
//! For `q_j = 2` this is the quadratic rewrite `pkm-semaev` uses on the tower
//! ring. For `q_j > 2` it is a degree-`q_j` equation and cannot be a rewrite,
//! so the system is solved with the generic F4 on the `m·n0` tower variables
//! (`pkm_smooth_tower::algebraic_decompose`), and its work is reported as
//! native field operations. Every relation returned is checked by point
//! addition inside that routine before it is handed back.
//!
//! Parameters: `m`, `degrees=3;5;7`, `seed`, `max_curves` (default 400 000),
//! `dmax` (default 24, the F4 degree bound), `budget` (default 60 s per
//! target). It refuses a base whose columns are not the chain's zero set.

use std::collections::{BTreeMap, HashSet};
use std::time::Duration;

use super::stages::{DecompositionOracle, InstanceCtx, Params};
use crate::cryptanalysis::ic_boundary::{
    FactorBase, GroupOps, OracleCounters, PrimeCurve, PrimePoint,
};
use crate::cryptanalysis::pkm_smooth_tower as st;

pub struct PkmMixedOracle {
    m: u32,
    degrees: Vec<u64>,
    seed: u64,
    max_curves: u64,
    dmax: u32,
    budget_s: u64,
    tower: Option<st::SmoothTower>,
    curve: Option<st::Curve>,
    native: BTreeMap<String, u64>,
}

impl PkmMixedOracle {
    pub fn new(m: u32) -> Self {
        Self {
            m,
            degrees: Vec::new(),
            seed: 0,
            max_curves: 400_000,
            dmax: 24,
            budget_s: 60,
            tower: None,
            curve: None,
            native: BTreeMap::new(),
        }
    }

    fn bump(&mut self, key: &str, by: u64) {
        *self.native.entry(key.to_string()).or_insert(0) += by;
    }
}

impl DecompositionOracle<PrimeCurve> for PkmMixedOracle {
    fn name(&self) -> &str {
        "pkm-mixed"
    }

    fn summands(&self) -> u32 {
        self.m
    }

    fn describe(&self, params: &Params) -> String {
        format!(
            "PKM isogeny form on a mixed-degree Vélu chain degrees={}, m = {}, generic F4 \
             (degree-q steps are not quadratic rewrites)",
            params.get("degrees").unwrap_or("?"),
            self.m
        )
    }

    fn prepare(
        &mut self,
        ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        params: &Params,
        _ops: &mut GroupOps,
    ) -> Result<(), String> {
        let spec = params.get("degrees").unwrap_or("");
        self.degrees = spec
            .split(|c| c == ',' || c == ';')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>())
            .collect::<Result<_, _>>()
            .map_err(|_| format!("pkm-mixed: degrees must be a list of primes, not `{spec}`"))?;
        if self.degrees.len() < 2 {
            return Err("pkm-mixed: degrees=... needs at least two steps".into());
        }
        self.seed = params.u64_or("seed", 0)?;
        self.max_curves = params.u64_or("max_curves", 400_000)?;
        self.dmax = params.u64_or("dmax", 24)? as u32;
        self.budget_s = params.u64_or("budget", 60)?;

        let curve = ctx.group;
        let s = self
            .degrees
            .iter()
            .try_fold(1u64, |acc, &q| acc.checked_mul(q))
            .ok_or("pkm-mixed: S overflows u64")?;
        let found = st::search_aux_exact(curve.p, s, self.seed, self.max_curves)?;
        let tower = st::isogeny_tower_on(
            found.aux,
            found.order,
            found.q,
            &self.degrees,
            self.seed,
            found.curves_tried,
        )?;
        tower.verify()?;

        // The base must be this chain's zero set, not some other base.
        let v: HashSet<u64> = tower.v.iter().copied().collect();
        for &x in &fb.abscissa_list {
            if !v.contains(&x) {
                return Err(format!(
                    "pkm-mixed: base abscissa {x} is not a zero of the chain (use the \
                     prime-mixed-isogeny base with the same degrees and seed)"
                ));
            }
        }
        self.curve = Some(st::Curve {
            p: curve.p,
            a: curve.a,
            b: curve.b,
        });
        self.tower = Some(tower);
        Ok(())
    }

    fn decompose(
        &mut self,
        _ctx: &InstanceCtx<PrimeCurve>,
        fb: &FactorBase<PrimePoint>,
        _ops: &mut GroupOps,
        _counters: &mut OracleCounters,
        point: PrimePoint,
    ) -> Option<Vec<usize>> {
        if point.infinity {
            return None;
        }
        let tower = self.tower.as_ref()?;
        let curve = self.curve.as_ref()?;
        let dec = st::algebraic_decompose(
            curve,
            tower,
            self.m as usize,
            (point.x, point.y),
            self.dmax,
            Duration::from_secs(self.budget_s),
        );
        self.bump("pkm_mixed_field_ops", dec.field_ops);
        if dec.undetermined {
            self.bump("pkm_mixed_undetermined", 1);
            return None;
        }
        let rel = dec.relations.first()?;
        self.bump("pkm_mixed_relations", 1);
        let mut idx = Vec::with_capacity(rel.len());
        for &(x, y) in rel {
            // The base stores both signs; the summand must be the exact one.
            let i = fb
                .points
                .iter()
                .position(|q| !q.infinity && q.x == x && q.y == y)?;
            idx.push(i);
        }
        Some(idx)
    }

    fn relation_counters(&self) -> BTreeMap<String, u64> {
        self.native.clone()
    }
}
