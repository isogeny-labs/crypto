//! # `E(F_{p³})` in ecbench: the curve, its counted group, and Gaudry's
//! `k = 3` index calculus with the eliminator or the cycle harvester
//!
//! The curve is `gaudry_cubic::generate_instance3(p, seed)`: a random
//! `y² = x³ + ax + b` over `F_{p³} = F_p[t]/(t³ − c)` whose generator has
//! prime order `r` equal to the unique multiple in the Hasse window
//! (cofactor one), `p ≡ 1 (mod 3)`, `67 ≤ p < 2^20`.  The constructor's
//! planted target and logarithm are dropped: methods see `G`, the target and
//! a seed only.
//!
//! A point is two packed words, `x₀ + 2²⁰x₁ + 2⁴⁰x₂` (and the same for `y`),
//! so the harness's hex targets, verification and replays carry an
//! `E(F_{p³})` point unchanged.
//!
//! [`Fp3Group`] implements [`CountedGroup`]: one addition or doubling is one
//! group operation, so `rho.plain`, `rho.negation`, BSGS and the kangaroo run
//! on the same curve under their own registered accounting.
//!
//! [`solve_k3`] is `ic.k3_gaudry`: the repository's own
//! [`gaudry_cubic`](crate::cryptanalysis::gaudry_cubic) relation loop, run as
//! is (subspace base `{P : x(P) ∈ F_p}`, symmetrised-`S₄` solve or
//! meet-in-the-middle oracle, Gaudry's double large primes with
//! `|F'| = ⌈|F|^{lp_exponent}⌉`, filtering and Wiedemann), with the large
//! primes handled by
//!
//! - `harvester=eliminator`: [`LargePrimeEliminator`] pivots with the target
//!   column protected (the module's `LargePrimeBackend::Pivot`), at most two
//!   large primes per relation; or
//! - `harvester=cycles`: the gain-forest cycle harvester
//!   ([`GainForestHarvester`]), at most three large primes per relation.
//!
//! **The charge.**  Unlike `ic.k4_gaudry`, whose algebraic solve is out of
//! reach and is replaced by an enumeration oracle charged a measured
//! per-trial constant, the `k = 3` oracle runs for real and counts its own
//! `F_p` multiplications.  Everything is charged, in group-addition
//! equivalents (GAE), with the curve's measured `F_p` multiplications per
//! affine addition `c_E` as the exchange rate (this is `GaudryReport::s`'s
//! accounting, `total_ops = group_ops + F_p muls / c_E + la_ops · 9 / c_E`):
//!
//! - `factor_base`: lifting `x ∈ F_p` to points (square roots) and the
//!   once-per-curve `S₄` symmetrisation, `F_p` multiplications ÷ `c_E`;
//! - `relations`: every group addition (`aG + bQ`, decomposition checks, and
//!   the `[d]G = Q` checks of the linear-algebra attempts) plus the oracle's
//!   `F_p` multiplications ÷ `c_E`;
//! - `linear_algebra`: eliminator or harvester merges and Wiedemann, each
//!   multiplication modulo `r` charged 9 `F_p` multiplications (`r ≈ p³` has
//!   three limbs; `ic.k4_gaudry` charges 16 for four) ÷ `c_E`.
//!
//! The only counted-but-uncharged work is the BLAKE3 digest of the admitted
//! relations that the protected path computes (`relation_hash_bytes_uncharged`)
//! and bookkeeping that is not arithmetic.

use std::collections::BTreeMap;

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

use crate::cryptanalysis::gaudry_cubic::{
    generate_instance3, run_gaudry_with_eliminator, Curve3, Fp3, GaudryOptions, Harvester,
    Instance3, LargePrimeBackend, Pt3, Solver, SubspaceBase, E3,
};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::residual_walk::{is_prime_u64, mix64};
use crate::hash::sha256::sha256;

/// Smallest `p` the constructor's order search terminates on quickly.
pub const P_MIN: u64 = 67;
/// `generate_instance3` bound; a coordinate limb is 20 bits.
pub const P_MAX: u64 = 1 << 20;
/// `F_p` multiplications charged per multiplication modulo `r`.
pub const C_MODN: f64 = 9.0;

/// A prime-order curve over `F_{p³}`.
#[derive(Clone, Debug)]
pub struct Fp3Instance {
    pub p: u64,
    pub seed: u64,
    /// The field is `F_p[t]/(t³ − c)`.
    pub c: u64,
    pub a: E3,
    pub b: E3,
    pub group_order: u64,
    pub r: u64,
    pub cofactor: u64,
    pub g: Pt3,
}

fn pack(e: &E3) -> u64 {
    e.0[0] | e.0[1] << 20 | e.0[2] << 40
}

fn unpack(v: u64) -> E3 {
    E3(core::array::from_fn(|i| (v >> (20 * i)) & 0xf_ffff))
}

impl Fp3Instance {
    pub fn build(p: u64, seed: u64) -> Result<Fp3Instance, String> {
        if p % 3 != 1 || !is_prime_u64(p) || !(P_MIN..P_MAX).contains(&p) {
            return Err(format!(
                "fp3_random takes a prime p ≡ 1 (mod 3) from {P_MIN} to 2^20, not {p}"
            ));
        }
        let inst = generate_instance3(p, seed);
        let c = &inst.curve;
        Ok(Fp3Instance {
            p,
            seed,
            c: c.field.c,
            a: c.a,
            b: c.b,
            group_order: c.n,
            r: c.n,
            cofactor: 1,
            g: c.g,
        })
    }

    /// A fresh curve (its operation counters at zero).
    pub fn curve(&self) -> Curve3 {
        let field = Fp3::with_cube_nonresidue(self.p, self.c).expect("a non-cube by construction");
        Curve3::new(field, self.a, self.b, self.r, self.g)
    }

    pub fn encode(pt: &Pt3) -> Option<[String; 2]> {
        (!pt.inf).then(|| {
            [
                format!("0x{:x}", pack(&pt.x)),
                format!("0x{:x}", pack(&pt.y)),
            ]
        })
    }

    /// The point a hex pair names, checked to lie on the curve.
    pub fn decode(&self, x: u64, y: u64) -> Result<Pt3, String> {
        let (x, y) = (unpack(x), unpack(y));
        if x.0.iter().chain(y.0.iter()).any(|&v| v >= self.p) {
            return Err("a limb is not reduced mod p".into());
        }
        let pt = Pt3::affine(x, y);
        if !self.curve().is_on_curve(&pt) {
            return Err("the target is not on the curve".into());
        }
        Ok(pt)
    }

    pub fn mul_generator_hex(&self, k: u64) -> Option<[String; 2]> {
        Self::encode(&self.curve().mul(&self.g, k % self.r))
    }

    /// The canonical model JSON and its SHA-256, the last field of the name.
    pub fn model(&self) -> (String, String) {
        let limbs = |e: &E3| format!("[{},{},{}]", e.0[0], e.0[1], e.0[2]);
        let json = format!(
            "{{\"field\":\"F_p[t]/(t^3-{})\",\"p\":{},\"a\":{},\"b\":{},\"order\":{}}}",
            self.c,
            self.p,
            limbs(&self.a),
            limbs(&self.b),
            self.group_order
        );
        let h: String = sha256(json.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        (json, h)
    }

    /// `(icv1, slug)`.  ICV1 has no extension-field form, so the name keeps
    /// ICV1's shape with a `fp3` field tag; it is never registered.
    pub fn names(&self) -> (String, String) {
        let (_, h) = self.model();
        let q = (self.p as u128).pow(3);
        let t = q as i128 + 1 - self.group_order as i128;
        let tt = if t < 0 {
            format!("tm{}", -t)
        } else {
            format!("t{t}")
        };
        (
            format!(
                "ICV1:fp3-p{}:{t}:{}:-:-:-:-:{}",
                self.p,
                self.group_order,
                &h[..12]
            ),
            format!("icv1-fp3p{}-{tt}-{}", self.p, &h[..8]),
        )
    }
}

/// `E(F_{p³})` as a [`CountedGroup`]: every addition and doubling counted.
pub struct Fp3Group(pub Curve3);

fn fp(e: &E3) -> u64 {
    mix64(e.0[0] ^ mix64(e.0[1] ^ mix64(e.0[2] ^ 0x5eed)))
}

impl CountedGroup for Fp3Group {
    type Elt = Pt3;
    fn identity(&self) -> Pt3 {
        Pt3::INFINITY
    }
    fn is_identity(&self, p: &Pt3) -> bool {
        p.inf
    }
    fn add(&self, ops: &mut GroupOps, p: Pt3, q: Pt3) -> Pt3 {
        if p == q {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.0.add(&p, &q)
    }
    fn double(&self, ops: &mut GroupOps, p: Pt3) -> Pt3 {
        ops.doubles += 1;
        self.0.double(&p)
    }
    fn neg(&self, p: Pt3) -> Pt3 {
        self.0.neg(&p)
    }
    fn key(&self, p: &Pt3) -> u64 {
        if p.inf {
            return 0;
        }
        mix64(fp(&p.x) ^ fp(&p.y).rotate_left(23))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct K3Params {
    /// How partial relations become full rows.
    pub harvester: Harvester,
    /// Large primes accepted per relation (2 for the eliminator, 3 for the
    /// harvester in the pre-registered pairing).
    pub max_lp: u8,
    /// `|F'| = ⌈|F|^{lp_exponent}⌉`.
    pub lp_exponent: f64,
    /// The decomposition oracle.
    pub solver: Solver,
    /// Residuals per base column before the run counts as exhausted.
    pub trial_cap_per_column: u64,
}

/// What [`solve_k3`] did, phase by phase, in GAE.
#[derive(Clone, Debug)]
pub struct K3Outcome {
    pub recovered: Option<u64>,
    pub exhausted: bool,
    /// `(phase, adds, gae)`.
    pub phases: Vec<(&'static str, u64, f64)>,
    pub counters: BTreeMap<String, u64>,
    pub detail: Value,
}

/// `ic.k3_gaudry` on `inst` for `target`.  Never sees the logarithm: the
/// instance handed to the relation loop carries `d = 0`, and the recovered
/// value is accepted by the loop only when `[d]G = Q` (the runner checks it
/// again).
pub fn solve_k3(inst: &Fp3Instance, target: Pt3, seed: u64, prm: &K3Params) -> K3Outcome {
    let i3 = Instance3 {
        curve: inst.curve(),
        q: target,
        d: 0,
    };
    let mut rng = StdRng::seed_from_u64(seed ^ 0x01F3_6A0D);
    i3.curve.reset();
    let base = SubspaceBase::build(&i3, &mut rng);
    let fb_muls = i3.curve.field.muls();
    let fb_adds = i3.curve.ops();
    let m = base.len();
    let small = ((m as f64).powf(prm.lp_exponent)).ceil() as usize;
    let opts = GaudryOptions {
        max_residuals: prm.trial_cap_per_column.saturating_mul(m as u64),
        solver: prm.solver,
        sparse_la: true,
        small_base: small,
        max_large_primes: prm.max_lp,
        harvester: prm.harvester,
        ..GaudryOptions::default()
    };
    let rep = run_gaudry_with_eliminator(&i3, &base, seed, &opts, LargePrimeBackend::Pivot);
    let cpa = rep.fp_muls_per_add;
    let fb_gae = (fb_muls + rep.precompute_fp_muls) as f64 / cpa + fb_adds as f64;
    let rel_gae =
        rep.group_ops as f64 + (rep.oracle_fp_muls + rep.torsion_lift_fp_muls) as f64 / cpa;
    let la_gae = rep.la_ops as f64 * C_MODN / cpa;
    let phases = vec![
        ("factor_base", fb_adds, fb_gae),
        ("relations", rep.group_ops, rel_gae),
        ("linear_algebra", 0, la_gae),
    ];
    let mut counters = BTreeMap::new();
    counters.insert("base_points".into(), m as u64);
    counters.insert("small_columns".into(), rep.small_base as u64);
    counters.insert("residuals".into(), rep.residuals);
    counters.insert("decompositions".into(), rep.decompositions);
    counters.insert("relations_verified".into(), rep.relations_verified);
    counters.insert("lp_full_relations".into(), rep.lp_full_relations);
    counters.insert("la_attempts".into(), rep.la_attempts);
    counters.insert("la_rows".into(), rep.la_rows as u64);
    counters.insert("la_modr_mults".into(), rep.la_ops);
    counters.insert("merge_modr_mults".into(), rep.lp_merge_ops);
    counters.insert("group_ops".into(), rep.group_ops);
    counters.insert("oracle_fp_muls".into(), rep.oracle_fp_muls);
    counters.insert("factor_base_fp_muls".into(), fb_muls);
    counters.insert("precompute_fp_muls".into(), rep.precompute_fp_muls);
    for (i, v) in rep.lp_histogram.iter().enumerate() {
        counters.insert(format!("decompositions_with_{i}_large_primes"), *v);
    }
    counters.insert(
        "relation_hash_bytes_uncharged".into(),
        rep.lp_input_hash_bytes.unwrap_or(0),
    );
    K3Outcome {
        recovered: rep.recovered_scalar,
        exhausted: rep.recovered_scalar.is_none(),
        phases,
        counters,
        detail: json!({
            "harvester": format!("{:?}", prm.harvester),
            "max_large_primes": prm.max_lp,
            "lp_exponent": prm.lp_exponent,
            "solver": format!("{:?}", prm.solver),
            "fp_muls_per_add": cpa,
            "c_modn": C_MODN,
            "relations_gae": rel_gae,
            "oracle_gae": rep.oracle_fp_muls as f64 / cpa,
            "la_gae": la_gae,
            "harvest": serde_json::to_value(&rep.harvest).unwrap_or(Value::Null),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(h: Harvester) -> K3Params {
        K3Params {
            harvester: h,
            max_lp: if h == Harvester::Cycles { 3 } else { 2 },
            lp_exponent: 2.0 / 3.0,
            solver: Solver::Groebner,
            trial_cap_per_column: 400,
        }
    }

    #[test]
    fn the_instance_round_trips_and_the_group_counts() {
        let inst = Fp3Instance::build(67, 3).unwrap();
        assert!(Fp3Instance::build(68, 3).is_err());
        assert!(Fp3Instance::build(61, 3).is_err());
        let hex = inst.mul_generator_hex(5).unwrap();
        let x = u64::from_str_radix(hex[0].trim_start_matches("0x"), 16).unwrap();
        let y = u64::from_str_radix(hex[1].trim_start_matches("0x"), 16).unwrap();
        let pt = inst.decode(x, y).unwrap();
        let g = Fp3Group(inst.curve());
        let mut ops = GroupOps::default();
        let mut acc = inst.g;
        for _ in 0..4 {
            acc = g.add(&mut ops, acc, inst.g);
        }
        assert_eq!(acc, pt);
        assert_eq!(ops.adds + ops.doubles, 4);
        assert!(inst.mul_generator_hex(inst.r).is_none());
    }

    #[test]
    fn both_harvesters_recover_the_logarithm_and_the_phases_sum() {
        let inst = Fp3Instance::build(67, 3).unwrap();
        let curve = inst.curve();
        for h in [Harvester::Eliminator, Harvester::Cycles] {
            let d = 12345 % inst.r;
            let target = curve.mul(&inst.g, d);
            let a = solve_k3(&inst, target, 1, &params(h));
            let b = solve_k3(&inst, target, 1, &params(h));
            let got = a.recovered.expect("recovered");
            assert_eq!(curve.mul(&inst.g, got), target, "{h:?}");
            assert_eq!(a.recovered, b.recovered, "replay {h:?}");
            assert_eq!(a.phases, b.phases, "replay {h:?}");
            assert_eq!(a.counters, b.counters, "replay {h:?}");
            assert!(a.phases.iter().all(|p| p.2 > 0.0));
            assert_eq!(a.detail["harvest"].is_null(), h == Harvester::Eliminator);
        }
    }
}
