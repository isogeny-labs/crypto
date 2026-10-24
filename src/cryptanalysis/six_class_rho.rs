//! Experimental six-class GLV distinguished-point walk (Curve Smash v2).
//!
//! This is a separate protocol from `pollard_collab` v1. Its class key is
//! `min(y, p-y)`, its jump is twisted by the current automorphism, and its
//! distinguished predicate reads bits above the branch selector. Records from
//! the two protocols must never be merged. This BigUint implementation is a
//! correctness and small-curve research worker, not a secp256k1 throughput
//! claim. The production cost and success estimate remain heuristic.

use std::collections::{HashMap, VecDeque};

use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize};

use crate::cryptanalysis::pollard_collab::job::{
    derive_scalar, hex_of, parse_hex, JobContext, JobSpec,
};
use crate::ecc::field::FieldElement;
use crate::ecc::point::Point;
use crate::hash::sha256;
use crate::utils::mod_inverse;

pub const WALK_VERSION: &str = "six-class-rho-secp-tight-v2/rust1";

/// The old job stays embedded only for curve validation and deterministic
/// initial branch generation. `id()` binds the new automorphism parameters.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SixClassJob {
    pub curve: JobSpec,
    pub beta: String,
    pub lambda: String,
}

#[derive(Clone, Debug)]
pub struct ClassState {
    pub point: Point,
    pub a: BigUint,
    pub b: BigUint,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassDp {
    pub job_id: String,
    pub walker: u64,
    pub steps: u64,
    pub x: String,
    pub y: String,
    pub a: String,
    pub b: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CampaignResult {
    pub job_id: String,
    pub walkers: u64,
    pub class_visits: u64,
    pub distinguished: u64,
    pub short_cycle_escapes: u64,
    pub log: Option<String>,
    pub first_dp: Option<ClassDp>,
    #[serde(default)]
    pub step_additions: u64,
    #[serde(default)]
    pub dead_trails: u64,
    #[serde(default)]
    pub exceptional_steps: u64,
    #[serde(default)]
    pub collision_attempts: u64,
    #[serde(default)]
    pub unique_dp_entries: usize,
    #[serde(default)]
    pub dp_records_verified: u64,
}

#[derive(Clone)]
struct Jump {
    point: Point,
    a: BigUint,
    b: BigUint,
}

pub struct SixClassContext {
    pub job: SixClassJob,
    pub base: JobContext,
    pub job_id: String,
    beta: BigUint,
    units: [BigUint; 6],
    jumps: Vec<[Jump; 6]>,
    branch_mask: BigUint,
    dp_mask: BigUint,
    branch_bits: usize,
}

fn sub_mod(a: &BigUint, b: &BigUint, n: &BigUint) -> BigUint {
    if a >= b {
        (a - b) % n
    } else {
        (n - ((b - a) % n)) % n
    }
}

impl SixClassJob {
    pub fn id(&self) -> String {
        let bytes = format!(
            "{WALK_VERSION}\n{}\n{}\n{}\n",
            self.curve.job_id(),
            self.beta.to_lowercase(),
            self.lambda.to_lowercase()
        );
        hex::encode(sha256(bytes.as_bytes()))
    }

    pub fn build(&self) -> Result<SixClassContext, String> {
        if self.curve.negation_map || self.curve.a != "0" {
            return Err("six-class walk requires a=0 and negation_map=false".into());
        }
        let r = self.curve.num_branches;
        if r < 2 || !r.is_power_of_two() {
            return Err("num_branches must be a power of two >= 2".into());
        }
        let base = self.curve.build()?;
        let beta = parse_hex(&self.beta)?;
        let lambda = parse_hex(&self.lambda)?;
        if beta <= BigUint::one()
            || beta >= base.p
            || beta.modpow(&BigUint::from(3u8), &base.p) != BigUint::one()
        {
            return Err("beta must have order three modulo p".into());
        }
        if lambda <= BigUint::one()
            || lambda >= base.n
            || lambda.modpow(&BigUint::from(3u8), &base.n) != BigUint::one()
        {
            return Err("lambda must have order three modulo n".into());
        }
        let phi_g = Self::phi(&base.g, &beta, &base.p, 1, false);
        if base.g.scalar_mul_vartime(&lambda, &base.a) != phi_g {
            return Err("lambda and beta do not act identically on the generator".into());
        }
        let lam2 = (&lambda * &lambda) % &base.n;
        let units = [
            BigUint::one(),
            lambda.clone(),
            lam2.clone(),
            &base.n - BigUint::one(),
            &base.n - &lambda,
            &base.n - &lam2,
        ];
        let inv = [
            BigUint::one(),
            lam2,
            lambda,
            units[3].clone(),
            &base.n - &units[2],
            &base.n - &units[1],
        ];
        let jumps = base
            .branches
            .iter()
            .map(|branch| {
                std::array::from_fn(|t| {
                    let point = Self::phi(&branch.point, &beta, &base.p, (3 - t % 3) % 3, t >= 3);
                    Jump {
                        point,
                        a: (&branch.u * &inv[t]) % &base.n,
                        b: (&branch.v * &inv[t]) % &base.n,
                    }
                })
            })
            .collect();
        let branch_bits = r.trailing_zeros() as usize;
        let branch_mask = BigUint::from(r - 1);
        let dp_mask = (BigUint::one() << self.curve.dp_bits) - BigUint::one();
        Ok(SixClassContext {
            job: self.clone(),
            base,
            job_id: self.id(),
            beta,
            units,
            jumps,
            branch_mask,
            dp_mask,
            branch_bits,
        })
    }

    fn phi(pt: &Point, beta: &BigUint, p: &BigUint, power: usize, negate: bool) -> Point {
        match pt {
            Point::Infinity => Point::Infinity,
            Point::Affine { x, y } => {
                let factor = beta.modpow(&BigUint::from(power), p);
                let xx = (&x.value * factor) % p;
                let yy = if negate && !y.value.is_zero() {
                    p - &y.value
                } else {
                    y.value.clone()
                };
                Point::Affine {
                    x: FieldElement::new(xx, p.clone()),
                    y: FieldElement::new(yy, p.clone()),
                }
            }
        }
    }
}

impl SixClassContext {
    /// Returns the class key and the automorphism that sends `point` to the
    /// canonical representative. Ties use the first x orientation.
    pub fn canonical(&self, point: &Point) -> Result<(BigUint, usize, Point), String> {
        let Point::Affine { x, y } = point else {
            return Err("identity has no class key".into());
        };
        let xs = [
            x.value.clone(),
            (&x.value * &self.beta) % &self.base.p,
            (&x.value * &self.beta * &self.beta) % &self.base.p,
        ];
        let mut j = 0;
        for i in 1..3 {
            if xs[i] < xs[j] {
                j = i;
            }
        }
        let ny = if y.value.is_zero() {
            BigUint::zero()
        } else {
            &self.base.p - &y.value
        };
        let flip = ny < y.value;
        let key = if flip { ny } else { y.value.clone() };
        let t = j + if flip { 3 } else { 0 };
        let canonical = Point::Affine {
            x: FieldElement::new(xs[j].clone(), self.base.p.clone()),
            y: FieldElement::new(key.clone(), self.base.p.clone()),
        };
        Ok((key, t, canonical))
    }

    pub fn start(&self, walker: u64) -> ClassState {
        let a = derive_scalar(&self.job_id, "start-a", walker, &self.base.n);
        let b = derive_scalar(&self.job_id, "start-b", walker, &self.base.n);
        let point = self.base.combine(&a, &b);
        ClassState { point, a, b }
    }

    pub fn step(&self, state: &ClassState) -> Result<ClassState, String> {
        let (key, t, _) = self.canonical(&state.point)?;
        let h = (&key & &self.branch_mask)
            .iter_u64_digits()
            .next()
            .unwrap_or(0) as usize;
        let jump = &self.jumps[h][t];
        if let (Point::Affine { x, .. }, Point::Affine { x: jx, .. }) = (&state.point, &jump.point)
        {
            if x.value == jx.value {
                return Err("exceptional affine addition".into());
            }
        }
        Ok(ClassState {
            point: state.point.add_vartime(&jump.point, &self.base.a),
            a: (&state.a + &jump.a) % &self.base.n,
            b: (&state.b + &jump.b) % &self.base.n,
        })
    }

    pub fn is_dp(&self, key: &BigUint) -> bool {
        ((key >> self.branch_bits) & &self.dp_mask).is_zero()
    }

    pub fn record(&self, walker: u64, steps: u64, state: &ClassState) -> Result<ClassDp, String> {
        let (key, t, point) = self.canonical(&state.point)?;
        if !self.is_dp(&key) {
            return Err("class is not distinguished".into());
        }
        let Point::Affine { x, y } = point else {
            unreachable!()
        };
        Ok(ClassDp {
            job_id: self.job_id.clone(),
            walker,
            steps,
            x: hex_of(&x.value),
            y: hex_of(&y.value),
            a: hex_of(&((&state.a * &self.units[t]) % &self.base.n)),
            b: hex_of(&((&state.b * &self.units[t]) % &self.base.n)),
        })
    }

    pub fn verify_record(&self, dp: &ClassDp) -> Result<(), String> {
        if dp.job_id != self.job_id {
            return Err("wrong six-class job id".into());
        }
        let x = parse_hex(&dp.x)?;
        let y = parse_hex(&dp.y)?;
        let a = parse_hex(&dp.a)?;
        let b = parse_hex(&dp.b)?;
        if x >= self.base.p || y >= self.base.p || a >= self.base.n || b >= self.base.n {
            return Err("unreduced record field".into());
        }
        let point = Point::Affine {
            x: FieldElement::new(x, self.base.p.clone()),
            y: FieldElement::new(y, self.base.p.clone()),
        };
        let (key, _, canonical) = self.canonical(&point)?;
        if canonical != point
            || !self.is_dp(&key)
            || !self.base.on_curve(&point)
            || self.base.combine(&a, &b) != point
        {
            return Err("invalid six-class record".into());
        }
        Ok(())
    }

    pub fn recover(&self, first: &ClassDp, second: &ClassDp) -> Option<BigUint> {
        self.verify_record(first).ok()?;
        self.verify_record(second).ok()?;
        if first.x != second.x || first.y != second.y {
            return None;
        }
        let a1 = parse_hex(&first.a).ok()?;
        let a2 = parse_hex(&second.a).ok()?;
        let b1 = parse_hex(&first.b).ok()?;
        let b2 = parse_hex(&second.b).ok()?;
        let numerator = sub_mod(&a1, &a2, &self.base.n);
        let denominator = sub_mod(&b2, &b1, &self.base.n);
        let inv = mod_inverse(&denominator, &self.base.n)?;
        let log = (numerator * inv) % &self.base.n;
        if self.base.g.scalar_mul_vartime(&log, &self.base.a) == self.base.q {
            Some(log)
        } else {
            None
        }
    }

    /// Bounded native small-curve campaign. Restarts after every DP and cap.
    /// A repeated key within 64 steps takes one deterministic doubling escape.
    pub fn campaign(&self, max_walkers: u64) -> CampaignResult {
        let mut result = CampaignResult {
            job_id: self.job_id.clone(),
            walkers: 0,
            class_visits: 0,
            distinguished: 0,
            short_cycle_escapes: 0,
            log: None,
            first_dp: None,
            step_additions: 0,
            dead_trails: 0,
            exceptional_steps: 0,
            collision_attempts: 0,
            unique_dp_entries: 0,
            dp_records_verified: 0,
        };
        let mut seen: HashMap<String, ClassDp> = HashMap::new();
        for walker in 0..max_walkers {
            result.walkers += 1;
            let mut state = self.start(walker);
            let mut recent: VecDeque<(BigUint, ClassState)> = VecDeque::new();
            let cap = self.base.step_cap.min(1_000_000);
            let mut reached_dp = false;
            for step in 0..cap {
                let Ok((key, _, _)) = self.canonical(&state.point) else {
                    break;
                };
                result.class_visits += 1;
                if self.is_dp(&key) {
                    if let Ok(record) = self.record(walker, step, &state) {
                        if result.first_dp.is_none() {
                            result.first_dp = Some(record.clone());
                        }
                        result.distinguished += 1;
                        reached_dp = true;
                        let entry = seen.insert(hex_of(&key), record.clone());
                        result.unique_dp_entries = seen.len();
                        if let Some(previous) = entry {
                            result.collision_attempts += 1;
                            result.dp_records_verified += 2;
                            if let Some(log) = self.recover(&previous, &record) {
                                result.log = Some(hex_of(&log));
                                return result;
                            }
                        }
                    }
                    break;
                }
                if let Some(prior) = recent.iter().position(|(seen, _)| seen == &key) {
                    // Every entry into this cycle chooses its least class
                    // before doubling, so merged trails remain merged.
                    let mut best_key = key.clone();
                    let mut best = state.clone();
                    for (candidate_key, candidate) in recent.iter().skip(prior) {
                        if candidate_key < &best_key {
                            best_key = candidate_key.clone();
                            best = candidate.clone();
                        }
                    }
                    state = ClassState {
                        point: best.point.double_vartime(&self.base.a),
                        a: (&best.a << 1) % &self.base.n,
                        b: (&best.b << 1) % &self.base.n,
                    };
                    result.short_cycle_escapes += 1;
                    recent.clear();
                    continue;
                }
                recent.push_back((key, state.clone()));
                if recent.len() > 64 {
                    recent.pop_front();
                }
                match self.step(&state) {
                    Ok(next) => {
                        result.step_additions += 1;
                        state = next;
                    }
                    Err(_) => {
                        result.exceptional_steps += 1;
                        break;
                    }
                }
            }
            if !reached_dp {
                result.dead_trails += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::pollard_collab::job::HexPoint;

    fn fixture() -> SixClassContext {
        let p = BigUint::from(33_756_013u64);
        let a = FieldElement::new(BigUint::zero(), p.clone());
        let g = Point::Affine {
            x: FieldElement::new(BigUint::from(8_339_409u64), p.clone()),
            y: FieldElement::new(BigUint::from(9_189_288u64), p.clone()),
        };
        let q = g.scalar_mul_vartime(&BigUint::from(12345u64), &a);
        let spec = JobSpec {
            version: 1,
            name: "six-class-p25".into(),
            p: hex_of(&p),
            a: "0".into(),
            b: "2".into(),
            generator: HexPoint::from_point(&g).unwrap(),
            target: HexPoint::from_point(&q).unwrap(),
            order: hex_of(&BigUint::from(33_750_391u64)),
            dp_bits: 4,
            num_branches: 32,
            negation_map: false,
            unit_size: 64,
            max_steps_per_walker: 320,
            seed: 7,
        };
        SixClassJob {
            curve: spec,
            beta: hex_of(&BigUint::from(13_339_116u64)),
            lambda: hex_of(&BigUint::from(25_380_340u64)),
        }
        .build()
        .unwrap()
    }

    #[test]
    fn twisted_step_is_class_equivariant() {
        let ctx = fixture();
        let state = ctx.start(3);
        for t in 0..6 {
            let transformed = ClassState {
                point: SixClassJob::phi(&state.point, &ctx.beta, &ctx.base.p, t % 3, t >= 3),
                a: (&state.a * &ctx.units[t]) % &ctx.base.n,
                b: (&state.b * &ctx.units[t]) % &ctx.base.n,
            };
            let next = ctx.step(&state).unwrap();
            let transformed_next = ctx.step(&transformed).unwrap();
            assert_eq!(
                ctx.canonical(&next.point).unwrap().0,
                ctx.canonical(&transformed_next.point).unwrap().0
            );
            assert_eq!(
                ctx.base.combine(&transformed_next.a, &transformed_next.b),
                transformed_next.point
            );
        }
    }

    #[test]
    fn native_campaign_recovers_planted_log() {
        let ctx = fixture();
        let result = ctx.campaign(1000);
        assert_eq!(result.log.as_deref(), Some("3039"));
        assert!(result.distinguished > 0);
    }
}
