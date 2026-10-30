//! # Hyperelliptic Jacobians in ecbench: the thread's instances, index
//! calculus and its rho reference
//!
//! The hyperelliptic head-to-head (`hyperelliptic_ic_bench::head_to_head`,
//! study `research/hec_bitop_20261020/`) counted both arms in `bitop/v1`
//! primitives (`prime_hyperelliptic::ops`) but ran outside the harness: no
//! sealed record, no replay, no fit.  This module runs the same two
//! algorithms as ecbench methods, on the same instances, with the same
//! counters, so each run is one sealed, replayable record that `ecbench
//! work` prices.
//!
//! - **Curve.** [`HecInstance::build`] is the thread's instance rule
//!   (`hyperelliptic_ic_bench::thread_instance`): the first `c` whose
//!   Jacobian has a prime subgroup above 1000, `D₁` from the first eight
//!   factor-base places.  Genus 2 to 4, `p` prime below `2^15`.  `#Jac` is
//!   not computed (only genus 2 has a routine), so the facts carry
//!   `group_order = cofactor = 0`, meaning unknown, and only planted targets
//!   exist: no public-target law is defined here.
//! - **Elements.** A reduced divisor `(u, v)` is two 64-bit words: `u`'s
//!   `deg u` low coefficients at 15 bits each with `deg u` in bits 60–62
//!   (`u` is monic), and `v`'s coefficients at 15 bits each.  The identity
//!   `(1, 0)` packs to `(0, 0)` and, like the point at infinity, has no hex
//!   form.
//! - **Methods.** `hec.ic_fb_walk` is `hec_index_calculus_dlp` with the
//!   factor-base walk, sparse elimination and the gcd smoothness test (the
//!   study's `fb-walk` arm); `hec.rho_full_width` is
//!   `pollard_rho_jacobian_with` with the full-width distinguished-point
//!   walk (the study's reference).  Neither sees the planted scalar.
//!
//! **The charge.**  Every phase of [`ops::Phase`] is one record phase; its
//! `native` map holds that phase's primitive counts under their `bitop/v1`
//! names, which `ecbench work` prices one to one at `bits(p)` (field
//! primitives, hash probes) and `bits(r)` (modular primitives).  The total
//! includes the checked arithmetic's operand re-validation, as the study's
//! primary measure does; the validation subset is reported beside it as
//! top-level `validation_<prim>` counters, which `work` does not add in
//! (`docs/units/BITOPS.md`, "Self-check work").  Each phase's `gae` is the
//! Jacobian group operations the algorithm counts in it (walk-step and
//! branch precomputation, walk steps); the field, ring and word work outside
//! them has no pinned ratio to a Jacobian addition, so the group-operation
//! total is a lower bound and names it in `unpriced`.  The `bitop/v1` work
//! block is the complete figure.
//!
//! The counters are thread-local and both algorithms run on the calling
//! thread, so a run's counts are a function of (instance, target, seed):
//! replays reproduce them exactly.  No wall-clock calibration enters a
//! record.

use std::collections::BTreeMap;
use std::time::Instant;

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive};
use serde_json::{json, Value};

use crate::cryptanalysis::ecbench::methods::{PhaseRecord, ResolvedMethod, SolveReport};
use crate::cryptanalysis::hyperelliptic_ic_bench::{
    pollard_rho_jacobian_with, thread_instance, RhoVariant,
};
use crate::cryptanalysis::hyperelliptic_index_calculus::{
    hec_index_calculus_dlp, HecIndexCalculusParams, LinearAlgebra, RelationSearch, SmoothnessTest,
};
use crate::cryptanalysis::residual_walk::is_prime_u64;
use crate::hash::sha256::sha256;
use crate::prime_hyperelliptic::ops::{self, Kind, Phase, PhaseTally};
use crate::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};

/// Bits per packed coefficient.
const LIMB: u32 = 15;

/// Name of the group-operation total's gap (see the module docs).
pub const GAE_GAP: &str = "jacobian_field_ring_and_word_work_outside_group_operations";

/// A thread instance: the curve, `D₁` and its prime order.
pub struct HecInstance {
    pub p: u64,
    pub genus: u32,
    /// The `c` the instance rule chose.
    pub c: u64,
    pub curve: HyperellipticCurveP,
    pub d1: MumfordDivisorP,
    pub n: BigUint,
    pub r: u64,
}

impl HecInstance {
    /// The thread's instance on genus `genus` over `F_p`.
    pub fn build(p: u64, genus: u32) -> Result<Self, String> {
        if !(2..=4).contains(&genus) {
            return Err(format!("hec_thread takes genus 2, 3 or 4, not {genus}"));
        }
        if !(5..1 << LIMB).contains(&p) || !is_prime_u64(p) {
            return Err(format!("hec_thread takes a prime 5 ≤ p < 2^15, not {p}"));
        }
        let (curve, d1, n, c) = thread_instance(p, genus).ok_or_else(|| {
            format!(
                "the thread's instance rule finds no usable subgroup for genus {genus}, p = {p}"
            )
        })?;
        let r = n.to_u64().ok_or("subgroup order past one word")?;
        Ok(HecInstance {
            p,
            genus,
            c,
            curve,
            d1,
            n,
            r,
        })
    }

    /// Coefficient `i` of `f` as a word.
    fn f_coeffs(&self) -> Vec<u64> {
        self.curve
            .f
            .coeffs
            .iter()
            .map(|c| c.to_u64().unwrap_or(0))
            .collect()
    }

    /// The canonical model JSON and its SHA-256.
    pub fn model(&self) -> (String, String) {
        let f: Vec<String> = self.f_coeffs().iter().map(u64::to_string).collect();
        let json = format!(
            "{{\"form\":\"y^2=f(x)\",\"genus\":{},\"p\":{},\"f\":[{}]}}",
            self.genus,
            self.p,
            f.join(",")
        );
        let h: String = sha256(json.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        (json, h)
    }

    /// `(icv1, slug)`.  ICV1 has no Jacobian form, so the name keeps
    /// ICV1's shape with a `hec` field tag, `-` for the unknown trace and
    /// order, and is never registered.
    pub fn names(&self) -> (String, String) {
        let (_, h) = self.model();
        (
            format!(
                "ICV1:hecg{}-p{}:-:-:-:-:-:-:{}",
                self.genus,
                self.p,
                &h[..12]
            ),
            format!("icv1-hecg{}p{}-c{}-{}", self.genus, self.p, self.c, &h[..8]),
        )
    }

    /// `D` as two words, or `None` for the identity.
    pub fn encode(&self, d: &MumfordDivisorP) -> Option<[String; 2]> {
        let du = d.u.degree()?;
        if du == 0 {
            return None;
        }
        let word = |coeffs: &[BigUint]| {
            coeffs.iter().enumerate().fold(0u64, |w, (i, c)| {
                w | (c.to_u64().unwrap_or(0) << (LIMB * i as u32))
            })
        };
        let u = word(&d.u.coeffs[..du]) | ((du as u64) << 60);
        let v = word(&d.v.coeffs);
        Some([format!("0x{u:x}"), format!("0x{v:x}")])
    }

    /// The divisor two words name, checked to be reduced on the curve.
    pub fn decode(&self, u: u64, v: u64) -> Result<MumfordDivisorP, String> {
        let p = BigUint::from(self.p);
        let mask = (1u64 << LIMB) - 1;
        let du = (u >> 60) as usize;
        if du == 0 || du > self.genus as usize || u >> 63 != 0 {
            return Err("u's degree is not in 1..=genus".into());
        }
        let limbs = |w: u64, k: usize| -> Vec<BigUint> {
            (0..k)
                .map(|i| BigUint::from((w >> (LIMB * i as u32)) & mask))
                .collect()
        };
        let mut uc = limbs(u, du);
        uc.push(BigUint::one());
        let vc = limbs(v, du);
        let low = u & ((1u64 << 60) - 1);
        if low >> (LIMB * du as u32) != 0
            || v >> (LIMB * du as u32) != 0
            || uc.iter().chain(&vc).any(|c| c >= &p)
        {
            return Err("a coefficient is out of range".into());
        }
        MumfordDivisorP::try_from_polynomials(
            &self.curve,
            FpPoly::from_coeffs(uc, p.clone()),
            FpPoly::from_coeffs(vc, p),
        )
        .map_err(|e| format!("the target is not a reduced divisor: {e}"))
    }

    /// `[k]D₁` as hex, `None` for the identity.
    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        self.encode(&self.d1.scalar_mul(&BigUint::from(k % self.r), &self.curve))
    }

    /// The facts' generator: `D₁`, which is never the identity.
    pub fn generator_hex(&self) -> [String; 2] {
        self.encode(&self.d1)
            .expect("D₁ has prime order, so it is not the identity")
    }
}

/// `k` of the thread target law (`hec_thread_scalar_v1`): `⌊r/3⌋ + 7 +
/// index`, the study's `k = N/3 + 7` at index 0.
pub fn thread_scalar(r: u64, index: u64) -> u128 {
    u128::from((r / 3 + 7 + index) % r)
}

fn param_u64(m: &ResolvedMethod, name: &str) -> Result<u64, String> {
    m.params[name]
        .parse()
        .map_err(|_| format!("parameter `{name}` is not an integer: `{}`", m.params[name]))
}

/// The solver's seed: the harness's algorithm seed, or the arm's
/// `fixed_seed` (a study's seed list, one arm per seed).
fn run_seed(m: &ResolvedMethod, seed: u64) -> Result<u64, String> {
    match m.params["fixed_seed"].as_str() {
        "none" => Ok(seed),
        _ => param_u64(m, "fixed_seed"),
    }
}

/// The record phases of one run: each `ops::Phase` with its primitive
/// counts as `native`, and its Jacobian group operations as `gae`.
fn phases_of(run: &PhaseTally, group_ops: &[(Phase, u64)]) -> Vec<PhaseRecord> {
    Phase::ALL
        .iter()
        .map(|&ph| {
            let t = run.phase(ph);
            let mut native = BTreeMap::new();
            for k in Kind::ALL {
                if t.get(k) > 0 {
                    native.insert(k.prim().name().to_string(), t.get(k));
                }
            }
            if t.rng_words > 0 {
                native.insert("rng_words".to_string(), t.rng_words);
            }
            let ops = group_ops
                .iter()
                .find(|(p, _)| *p == ph)
                .map_or(0, |(_, n)| *n);
            PhaseRecord {
                name: ph.name().into(),
                gae: ops as f64,
                native,
                ..Default::default()
            }
        })
        .collect()
}

/// The validation subset, summed over phases, as `validation_<prim>`.
fn validation_counters(checks: &PhaseTally, counters: &mut BTreeMap<String, u64>) {
    let t = checks.total();
    for k in Kind::ALL {
        if t.get(k) > 0 {
            counters.insert(format!("validation_{}", k.prim().name()), t.get(k));
        }
    }
}

fn report(
    recovered: Option<BigUint>,
    phases: Vec<PhaseRecord>,
    counters: BTreeMap<String, u64>,
    automorphisms_used: u32,
    wall: u64,
    detail: Value,
) -> SolveReport {
    let total = phases.iter().map(|p| p.gae).sum();
    let recovered = recovered.and_then(|k| k.to_u128());
    SolveReport {
        recovered,
        exhausted: recovered.is_none(),
        phases,
        total_gae: total,
        automorphisms_used,
        counters,
        unpriced: vec![GAE_GAP.into()],
        deterministic: true,
        nondeterminism: vec![],
        solve_wall_ns: wall,
        factor_base: None,
        detail,
        online: None,
        online_error: None,
        solver: None,
        field_ops: None,
    }
}

/// Solve for the target `(tu, tv)` with a `hec.*` method.
pub fn solve(
    m: &ResolvedMethod,
    i: &HecInstance,
    tu: u64,
    tv: u64,
    seed: u64,
) -> Result<SolveReport, String> {
    // Decoding validates the target: outside the counted interval.
    let d2 = i.decode(tu, tv)?;
    let seed = run_seed(m, seed)?;
    match m.id.as_str() {
        "hec.ic_fb_walk" => {
            let params = HecIndexCalculusParams {
                fb_size: usize::MAX,
                extra_relations: param_u64(m, "extra_relations")? as usize,
                max_trials: param_u64(m, "max_trials")? as usize,
                seed,
                search: RelationSearch::FactorBaseWalk {
                    branches: param_u64(m, "branches")? as usize,
                    d_step_in: param_u64(m, "d_step_in")?,
                },
                linear_algebra: LinearAlgebra::Sparse,
                smoothness: SmoothnessTest::Gcd,
                large_primes: false,
            };
            if param_u64(m, "branches")? == 0 || param_u64(m, "d_step_in")? == 0 {
                return Err("branches and d_step_in must be positive".into());
            }
            let (t0, v0) = (ops::snapshot(), ops::snapshot_validation());
            let t = Instant::now();
            let (k, rep) = hec_index_calculus_dlp(&i.curve, &i.d1, &d2, &i.n, &params);
            let wall = t.elapsed().as_nanos() as u64;
            let run = ops::snapshot().since(&t0);
            let checks = ops::snapshot_validation().since(&v0);
            let group = [
                (Phase::Precompute, rep.precompute_ops as u64),
                (
                    Phase::Walk,
                    rep.jacobian_ops.saturating_sub(rep.precompute_ops) as u64,
                ),
            ];
            let mut counters = BTreeMap::from([
                ("jacobian_ops".to_string(), rep.jacobian_ops as u64),
                ("precompute_ops".to_string(), rep.precompute_ops as u64),
                ("factor_base_size".to_string(), rep.factor_base_size as u64),
                ("trials".to_string(), rep.trials as u64),
                ("smooth_trials".to_string(), rep.smooth_trials as u64),
                ("relations".to_string(), rep.relations as u64),
                ("solve_row_ops".to_string(), rep.solve_row_ops as u64),
                (
                    "smoothness_field_ops".to_string(),
                    rep.smoothness_field_ops as u64,
                ),
            ]);
            validation_counters(&checks, &mut counters);
            let detail = json!({
                "algorithm": "hec_index_calculus_dlp",
                "search": format!("{:?}", params.search),
                "linear_algebra": "Sparse",
                "smoothness": "Gcd",
                "large_primes": false,
                "solver_seed": seed.to_string(),
                "solved": rep.solved,
            });
            Ok(report(
                k,
                phases_of(&run, &group),
                counters,
                2,
                wall,
                detail,
            ))
        }
        "hec.rho_full_width" => {
            let max_steps = param_u64(m, "max_steps")? as usize;
            let (t0, v0) = (ops::snapshot(), ops::snapshot_validation());
            let t = Instant::now();
            let res = pollard_rho_jacobian_with(
                &i.curve,
                &i.d1,
                &d2,
                &i.n,
                seed,
                max_steps,
                &RhoVariant::DistinguishedPointsFullWidth,
            );
            let wall = t.elapsed().as_nanos() as u64;
            let run = ops::snapshot().since(&t0);
            let checks = ops::snapshot_validation().since(&v0);
            let group = [
                (Phase::Precompute, res.precompute_ops as u64),
                (Phase::Walk, res.walk_ops as u64),
            ];
            let mut counters = BTreeMap::from([
                ("jacobian_ops".to_string(), res.group_ops as u64),
                ("precompute_ops".to_string(), res.precompute_ops as u64),
                ("walk_ops".to_string(), res.walk_ops as u64),
                ("restarts".to_string(), res.restarts as u64),
                ("fruitless_ops".to_string(), res.fruitless_ops as u64),
            ]);
            validation_counters(&checks, &mut counters);
            let detail = json!({
                "algorithm": "pollard_rho_jacobian_with",
                "variant": "DistinguishedPointsFullWidth",
                "solver_seed": seed.to_string(),
            });
            Ok(report(
                res.k,
                phases_of(&run, &group),
                counters,
                1,
                wall,
                detail,
            ))
        }
        other => Err(format!(
            "`{other}` has no hyperelliptic implementation; hec.ic_fb_walk and hec.rho_full_width do"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packing_round_trips_and_rejects_junk() {
        let i = HecInstance::build(41, 2).expect("instance");
        assert_eq!(i.r, 1321);
        for k in [1u64, 2, 447, 1320] {
            let hex = i.mul_generator_hex(k).expect("not the identity");
            let w = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
            let d = i.decode(w(&hex[0]), w(&hex[1])).expect("decodes");
            assert_eq!(d, i.d1.scalar_mul(&BigUint::from(k), &i.curve));
        }
        assert!(i.mul_generator_hex(0).is_none());
        assert!(i.decode(0, 0).is_err());
        assert!(i.decode((3u64 << 60) | 1, 0).is_err(), "degree above genus");
        assert!(
            i.decode((1u64 << 60) | 41, 0).is_err(),
            "coefficient not below p"
        );
    }

    fn method(id: &str, params: &[(&str, &str)]) -> ResolvedMethod {
        crate::cryptanalysis::ecbench::methods::resolve(
            &crate::cryptanalysis::ecbench::methods::MethodSpec {
                id: id.into(),
                params: params
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            },
        )
        .expect("resolves")
    }

    /// Both methods solve a planted genus-2 target through
    /// `methods::solve`, the answer checks as `[k]D₁ = Q`, a second solve
    /// with the same seed gives the same report, and `work` prices it
    /// completely, to the gate the counters' own pricing gives.
    #[test]
    fn both_methods_solve_replay_and_price_completely() {
        use crate::cryptanalysis::ecbench::methods::solve as ecbench_solve;
        use crate::cryptanalysis::ecbench::work::{price, Priceable};
        use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
        let spec = CurveSpec::HecThread { p: 41, genus: 2 };
        let (w, inst) = Workload::build(&spec, 5, 0, TargetKind::Planted).expect("workload");
        assert_eq!(w.curve.family, "hyperelliptic");
        assert_eq!(w.curve.r, 1321);
        let ic = method(
            "hec.ic_fb_walk",
            &[
                ("branches", "64"),
                ("d_step_in", "4"),
                ("extra_relations", "8"),
            ],
        );
        let rho = method("hec.rho_full_width", &[]);
        for m in [&ic, &rho] {
            let a = ecbench_solve(m, &inst, &w.curve, &w.target, 11).expect("solves");
            let k = a.recovered.expect("recovered");
            assert_eq!(inst.mul_generator_hex(k).as_ref(), Some(&w.target));
            assert_eq!(Some(k), w.planted);
            let b = ecbench_solve(m, &inst, &w.curve, &w.target, 11).expect("solves");
            assert_eq!(a.phases, b.phases, "{}", m.id);
            assert_eq!(a.counters, b.counters, "{}", m.id);
            assert_eq!(a.total_gae, b.total_gae);
            assert_eq!(a.unpriced, vec![GAE_GAP.to_string()]);
            let work = price(&Priceable {
                method_id: &m.id,
                method_params: &m.params,
                curve: &w.curve,
                phases: &a.phases,
                counters: &a.counters,
                solver: None,
            })
            .expect("priced");
            assert!(
                work.complete,
                "{:?} {:?}",
                work.unpriced, work.known_uncounted
            );
            // The same counts priced by the counters' own rule.
            let widths = crate::cryptanalysis::ecbench::work::widths_of(&w.curve).unwrap();
            let mut tally = ops::OpTally::default();
            for p in &a.phases {
                for k in Kind::ALL {
                    tally.counts[k as usize] += p.native.get(k.prim().name()).copied().unwrap_or(0);
                }
            }
            assert_eq!(work.bitops, tally.bitops(&widths).unwrap().to_string());
            assert!(a.counters.keys().any(|c| c.starts_with("validation_")));
        }
        // A different seed walks differently.
        let c = ecbench_solve(&rho, &inst, &w.curve, &w.target, 12).expect("solves");
        let a = ecbench_solve(&rho, &inst, &w.curve, &w.target, 11).expect("solves");
        assert_ne!(a.phases, c.phases);
        // A pinned seed overrides the harness's.
        let pinned = method("hec.rho_full_width", &[("fixed_seed", "11")]);
        let d = ecbench_solve(&pinned, &inst, &w.curve, &w.target, 999).expect("solves");
        assert_eq!(a.phases, d.phases);
    }

    #[test]
    fn the_thread_target_law_reproduces_the_studys_k() {
        use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};
        let spec = CurveSpec::HecThread { p: 41, genus: 2 };
        let (w, _) = Workload::build(&spec, 0, 0, TargetKind::PlantedHecThread).unwrap();
        assert_eq!(w.planted, Some(447));
        assert_eq!(w.kind(), TargetKind::PlantedHecThread);
        assert!(Workload::build(&spec, 0, 0, TargetKind::Public).is_err());
        let prime = CurveSpec::PrimeSearch {
            bits: 14,
            seed: 59297,
        };
        assert!(Workload::build(&prime, 0, 0, TargetKind::PlantedHecThread).is_err());
    }
}
