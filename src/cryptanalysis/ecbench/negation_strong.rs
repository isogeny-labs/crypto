//! `rho.negation_strong`: the strong negation-map Pollard rho on a curve
//! over the tower `F_{q⁴}`, `q = 2^m` (`Instance::F2m4`, the
//! `f2m4_torsion` workload that `ic.pipeline` with `gaudry-d4-bin`
//! attacks), charged in the unit that pipeline is charged in: counted
//! `F_q` multiplications.
//!
//! `rho.negation` reaches these curves through the generic walk on
//! [`CountedGroup`], which charges one group-addition equivalent per affine
//! addition (an addition with its own `F_{q⁴}` inversion) and leaves the
//! canonicalisations uncharged.  The red team of 2026-10-26
//! (`research/gaudry_k4_char2_redteam_20261026/REPORT.md` §§1, 3) found
//! both understate rho.  This walk is the reference its replication plan
//! asks for:
//!
//! * **negation map**: the state is the class `{P, −P}`, represented by the
//!   point whose packed ordinate is the smaller of `y` and `x + y`;
//! * **additive walk** with [`JUMPS`] precomputed jumps `[c_j]G`, the
//!   partition a hash of the canonical abscissa;
//! * **fruitless cycles**: a step whose result equals the current point or
//!   one of the four before it escapes to the canonical form of `[2]P_next`
//!   (a function of the state, so two merged walks escape alike); longer
//!   cycles end at the walk cap;
//! * **distinguished points**: only states whose abscissa hash has
//!   `dp_bits` low zero bits are stored;
//! * **lockstep lanes**: `lanes` walks step together and their slope
//!   denominators are inverted with one `F_{q⁴}` inversion (Montgomery's
//!   trick: `3(n − 1)` products and one inversion for `n` denominators).
//!
//! **What is charged.**  Every `F_q` multiplication of the solve, read from
//! the tower's own counter (`Gf2q::muls`, the counter the IC pipeline's
//! oracle and `fq_muls_per_add` read): the jump table and the start points
//! (`setup`), every walk step with its share of the batched inversion, the
//! start additions, the escapes and the unbatched additions where a
//! denominator vanishes (`search`), and the scalar multiplications that
//! check a candidate (`check`).  The canonicalisation computes `x + y`
//! (four `F_q` additions) and compares two `4m`-bit words: no `F_q`
//! multiplication.  It is charged [`CANON_FQ_MULS`] `F_q` multiplication
//! each (`canonicalisation`), a declared convention that bounds its work
//! from above (four word XORs and one comparison against a table
//! multiplication's two logarithm lookups, an addition and an exponential
//! lookup).  Each phase's group-addition equivalents are its `F_q`
//! multiplications divided by the curve's [`fq_muls_per_add`], the
//! divisor `solve_ic_f2m4` prices IC with, so the IC/rho ratio is one
//! unit.  The partition hashes, distinguished-point tests and table
//! operations are counted under `*_uncharged` names, as the Koblitz strong
//! reference counts them.

use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cryptanalysis::gaudry_quartic_bin::{fq_muls_per_add, BinCurve, CurveB, Eb, PtB};

/// Precomputed jumps of the additive walk.
pub const JUMPS: usize = 32;

/// `F_q` multiplications charged for one canonicalisation (see the module
/// documentation).
pub const CANON_FQ_MULS: u64 = 1;

/// Walk shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NegStrongParams {
    /// Walks stepped in lockstep: the batch-inversion width.
    pub lanes: usize,
    pub dp_bits: u32,
    /// Give up after this multiple of the ideal step count.
    pub step_cap_factor: u64,
}

/// The `auto` distinguished-point rule: `round(log₂(ideal) / 2)` from 1
/// to 20, a walk of about `√ideal` steps (as `strong_v2_shape`).
pub fn auto_dp_bits(ideal: f64) -> u32 {
    ((ideal.max(2.0).log2() / 2.0).round() as u32).clamp(1, 20)
}

/// `√(πr/4)`: the expected steps of a negation-map walk.
pub fn ideal_steps(r: u64) -> f64 {
    (std::f64::consts::PI * r as f64 / 4.0).sqrt()
}

/// Everything a solve counted, whether or not it recovered the logarithm.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NegStrongOutcome {
    /// The candidate logarithm, checked here by `[d]G = Q` (the runner
    /// checks it again in its own process).
    pub scalar: Option<u64>,
    /// `F_q` multiplications of the jump table and the start points.
    pub setup_fq_muls: u64,
    /// `F_q` multiplications of the walk.
    pub search_fq_muls: u64,
    /// `F_q` multiplications of the candidate checks.
    pub check_fq_muls: u64,
    pub canonicalisations: u64,
    /// `F_{q⁴}` inversions: one per batch, one per unbatched affine
    /// addition or doubling of the search.
    pub inversions: u64,
    pub batch_inversions: u64,
    /// Walk steps (one addition each), escapes excluded.
    pub walk_steps: u64,
    /// Steps whose denominator vanished, added without the batch.
    pub unbatched_steps: u64,
    pub walks: u64,
    pub start_additions: u64,
    pub escapes: u64,
    pub capped_walks: u64,
    pub wasted_merges: u64,
    pub failed_collisions: u64,
    pub table_entries: u64,
    pub partition_hashes: u64,
    pub dp_tests: u64,
    pub table_queries: u64,
    pub table_inserts: u64,
    pub scalar_multiplications: u64,
    pub ideal_steps: f64,
}

impl NegStrongOutcome {
    /// Every `F_q` multiplication charged, the canonicalisations included.
    pub fn charged_fq_muls(&self) -> u64 {
        self.setup_fq_muls
            + self.search_fq_muls
            + self.check_fq_muls
            + self.canonicalisations * CANON_FQ_MULS
    }
}

#[derive(Clone, Copy)]
struct State {
    p: PtB,
    a: u64,
    b: u64,
}

#[derive(Clone, Copy)]
struct Lane {
    s: State,
    previous: [PtB; 4],
    length: u64,
    live: bool,
}

fn add_mod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 + b as u128) % r as u128) as u64
}

fn sub_mod(a: u64, b: u64, r: u64) -> u64 {
    add_mod(a, r - b % r, r)
}

fn mul_mod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn inv_mod(v: u64, r: u64) -> Option<u64> {
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut rr, mut nr) = (r as i128, (v % r) as i128);
    while nr != 0 {
        let q = rr / nr;
        (t, nt) = (nt, t - q * nt);
        (rr, nr) = (nr, rr - q * nr);
    }
    (rr == 1).then(|| t.rem_euclid(r as i128) as u64)
}

fn mix(v: u64, k: u64) -> u64 {
    let mut z = v ^ k;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// Whether `CurveB::add(p, q)` inverts (both affine, not `P = −Q`, not a
/// doubling of a point of order 2).
fn needs_inversion(p: &PtB, q: &PtB) -> bool {
    !(p.inf || q.inf || (p.x == q.x && (p.y != q.y || p.x.is_zero())))
}

/// The walk on one curve.
pub struct NegStrongRho<'a> {
    pub inst: &'a BinCurve,
}

impl<'a> NegStrongRho<'a> {
    pub fn new(inst: &'a BinCurve) -> Self {
        NegStrongRho { inst }
    }

    fn c(&self) -> &CurveB {
        &self.inst.curve
    }

    fn muls(&self) -> u64 {
        self.c().f.f.muls()
    }

    /// The representative of `{P, −P}` with the smaller packed ordinate,
    /// with `(a, b)` negated when the point is.
    fn canonical(&self, s: State, out: &mut NegStrongOutcome) -> State {
        out.canonicalisations += 1;
        if s.p.inf {
            return s;
        }
        let f = &self.c().f;
        let ny = s.p.x.add(&s.p.y);
        if f.pack(&ny) < f.pack(&s.p.y) {
            let r = u64::try_from(self.inst.r).expect("one-word negation order");
            State {
                p: PtB {
                    x: s.p.x,
                    y: ny,
                    inf: false,
                },
                a: sub_mod(0, s.a, r),
                b: sub_mod(0, s.b, r),
            }
        } else {
            s
        }
    }

    fn class_key(&self, p: &PtB) -> u64 {
        if p.inf {
            u64::MAX
        } else {
            u64::try_from(self.c().f.pack(&p.x)).expect("one-word negation point key")
        }
    }

    fn partition(&self, p: &PtB, out: &mut NegStrongOutcome) -> usize {
        out.partition_hashes += 1;
        (mix(self.class_key(p), 0x9e37_79b9_7f4a_7c15) >> 59) as usize % JUMPS
    }

    fn distinguished(&self, p: &PtB, mask: u64, out: &mut NegStrongOutcome) -> bool {
        out.dp_tests += 1;
        !p.inf && mix(self.class_key(p), 0x2545_f491_4f6c_dd1d) & mask == 0
    }

    /// An unbatched addition, counted as one inversion when it inverts.
    fn add_unbatched(&self, p: &PtB, q: &PtB, out: &mut NegStrongOutcome) -> PtB {
        if needs_inversion(p, q) {
            out.inversions += 1;
        }
        self.c().add(p, q)
    }

    /// Montgomery's simultaneous inversion of `d` in place: `3(n − 1)`
    /// products and one inversion, every product counted.
    pub fn batch_inverse(&self, d: &mut [Eb], prefix: &mut Vec<Eb>) {
        let f = &self.c().f;
        let n = d.len();
        if n == 0 {
            return;
        }
        prefix.clear();
        prefix.push(d[0]);
        for i in 1..n {
            let next = f.mul(&prefix[i - 1], &d[i]);
            prefix.push(next);
        }
        let mut inv = f.inv(&prefix[n - 1]);
        for i in (1..n).rev() {
            let di = f.mul(&inv, &prefix[i - 1]);
            inv = f.mul(&inv, &d[i]);
            d[i] = di;
        }
        d[0] = inv;
    }

    /// `P + J` from the inverted denominator `(x_P + x_J)⁻¹`:
    /// `λ = (y_P + y_J)·inv`, `x₃ = λ² + λ + x_P + x_J + a`,
    /// `y₃ = λ(x_P + x₃) + x₃ + y_P`: two products and one square.
    pub fn add_with_inverse(&self, p: &PtB, j: &PtB, inv: &Eb) -> PtB {
        let f = &self.c().f;
        let l = f.mul(&p.y.add(&j.y), inv);
        let x3 = f.sq(&l).add(&l).add(&p.x).add(&j.x).add(&self.c().a);
        let y3 = f.mul(&l, &p.x.add(&x3)).add(&x3).add(&p.y);
        PtB {
            x: x3,
            y: y3,
            inf: false,
        }
    }

    /// The jump table `[c_j]G`, `c_j` uniform in `[1, r)` from `jump_seed`.
    fn jumps(&self, jump_seed: u64, out: &mut NegStrongOutcome) -> Vec<(PtB, u64)> {
        let mut rng = StdRng::seed_from_u64(jump_seed);
        let g = self.inst.generator;
        let r = u64::try_from(self.inst.r).expect("one-word negation order");
        (0..JUMPS)
            .map(|_| {
                let c = rng.gen_range(1..r);
                out.scalar_multiplications += 1;
                (self.c().mul(&g, u128::from(c)), c)
            })
            .collect()
    }

    /// Solve `Q = [d]G`.  Deterministic in `(curve, q, seed, params)`.
    pub fn solve(&self, q: PtB, seed: u64, params: &NegStrongParams) -> NegStrongOutcome {
        assert!(
            self.inst.curve.f.f.m <= 16,
            "rho.negation_strong requires a one-word packed point key"
        );
        let c = self.c();
        let r = u64::try_from(self.inst.r).expect("one-word negation order");
        let g = self.inst.generator;
        let mut out = NegStrongOutcome {
            ideal_steps: ideal_steps(r),
            ..Default::default()
        };
        assert!(params.dp_bits < 40, "dp_bits below 40");
        let lanes_n = params.lanes.max(1);
        let m0 = self.muls();
        let jump_seed = seed ^ 0x6a75_6d70_7321_0001;
        let jumps = self.jumps(jump_seed, &mut out);
        let mut rng = StdRng::seed_from_u64(seed);
        let stride_a = rng.gen_range(1..r);
        let stride = c.mul(&g, u128::from(stride_a));
        let mut cursor_a = rng.gen_range(0..r);
        let mut cursor = c.add(&c.mul(&g, u128::from(cursor_a)), &q);
        out.scalar_multiplications += 2;
        let m1 = self.muls();
        out.setup_fq_muls = m1 - m0;

        let mask = (1u64 << params.dp_bits) - 1;
        let walk_cap = 8u64 << params.dp_bits;
        let step_cap = (out.ideal_steps.ceil() as u64)
            .saturating_mul(params.step_cap_factor)
            .max(1_000_000);
        let mut table: HashMap<u64, (u64, u64)> = HashMap::new();
        let mut lanes = vec![
            Lane {
                s: State {
                    p: PtB::INF,
                    a: 0,
                    b: 0,
                },
                previous: [PtB::INF; 4],
                length: 0,
                live: false,
            };
            lanes_n
        ];
        let mut den: Vec<Eb> = Vec::with_capacity(lanes_n);
        let mut batched: Vec<(usize, usize)> = Vec::with_capacity(lanes_n);
        let mut direct: Vec<(usize, usize)> = Vec::with_capacity(lanes_n);
        let mut prefix: Vec<Eb> = Vec::with_capacity(lanes_n);
        let mut steps: Vec<(usize, usize, Option<Eb>)> = Vec::with_capacity(lanes_n);
        let mut check_muls = 0u64;
        let mut found = None;

        'walk: while out.walk_steps < step_cap {
            for lane in lanes.iter_mut().filter(|l| !l.live) {
                loop {
                    out.walks += 1;
                    let start = State {
                        p: cursor,
                        a: cursor_a,
                        b: 1 % r,
                    };
                    cursor = self.add_unbatched(&cursor, &stride, &mut out);
                    cursor_a = add_mod(cursor_a, stride_a, r);
                    out.start_additions += 1;
                    if start.p.inf {
                        continue;
                    }
                    lane.s = self.canonical(start, &mut out);
                    lane.previous = [PtB::INF; 4];
                    lane.length = 0;
                    lane.live = true;
                    break;
                }
            }
            for lane in lanes.iter_mut() {
                if !lane.live || !self.distinguished(&lane.s.p, mask, &mut out) {
                    continue;
                }
                lane.live = false;
                let key = self.class_key(&lane.s.p);
                out.table_queries += 1;
                let Some(&(ha, hb)) = table.get(&key) else {
                    table.insert(key, (lane.s.a, lane.s.b));
                    out.table_inserts += 1;
                    continue;
                };
                // Both states hold the canonical point of one class:
                // a + b·d = a' + b'·d.
                let Some(inv) = inv_mod(sub_mod(lane.s.b, hb, r), r) else {
                    out.wasted_merges += 1;
                    continue;
                };
                let d = mul_mod(sub_mod(ha, lane.s.a, r), inv, r);
                let before = self.muls();
                out.scalar_multiplications += 1;
                let hit = c.mul(&g, u128::from(d)) == q;
                check_muls += self.muls() - before;
                if hit {
                    found = Some(d);
                    break 'walk;
                }
                out.failed_collisions += 1;
            }
            // One step for every live lane, denominators inverted together.
            den.clear();
            batched.clear();
            direct.clear();
            for (li, lane) in lanes.iter().enumerate() {
                if !lane.live {
                    continue;
                }
                let ji = self.partition(&lane.s.p, &mut out);
                let jp = &jumps[ji].0;
                let d = lane.s.p.x.add(&jp.x);
                if lane.s.p.inf || jp.inf || d.is_zero() {
                    direct.push((li, ji));
                } else {
                    den.push(d);
                    batched.push((li, ji));
                }
            }
            if !den.is_empty() {
                self.batch_inverse(&mut den, &mut prefix);
                out.batch_inversions += 1;
                out.inversions += 1;
            }
            steps.clear();
            steps.extend(
                batched
                    .iter()
                    .zip(den.iter())
                    .map(|(&(li, ji), inv)| (li, ji, Some(*inv)))
                    .chain(direct.iter().map(|&(li, ji)| (li, ji, None))),
            );
            for &(li, ji, inv) in &steps {
                let (jp, jc) = jumps[ji];
                let lane = &mut lanes[li];
                let sum = match inv {
                    Some(inv) => self.add_with_inverse(&lane.s.p, &jp, &inv),
                    None => {
                        out.unbatched_steps += 1;
                        self.add_unbatched(&lane.s.p, &jp, &mut out)
                    }
                };
                let next = self.canonical(
                    State {
                        p: sum,
                        a: add_mod(lane.s.a, jc, r),
                        b: lane.s.b,
                    },
                    &mut out,
                );
                out.walk_steps += 1;
                lane.length += 1;
                if next.p == lane.s.p || lane.previous.contains(&next.p) {
                    // [2]P_next: the history restarts so the escaped point
                    // is not taken for a cycle.
                    out.escapes += 1;
                    let dbl = self.add_unbatched(&next.p, &next.p, &mut out);
                    lane.s = self.canonical(
                        State {
                            p: dbl,
                            a: add_mod(next.a, next.a, r),
                            b: add_mod(next.b, next.b, r),
                        },
                        &mut out,
                    );
                    lane.previous = [PtB::INF; 4];
                    if lane.s.p.inf {
                        lane.live = false;
                    }
                    continue;
                }
                if lane.length > walk_cap {
                    out.capped_walks += 1;
                    lane.live = false;
                    continue;
                }
                lane.previous = [
                    lane.s.p,
                    lane.previous[0],
                    lane.previous[1],
                    lane.previous[2],
                ];
                lane.s = next;
                if lane.s.p.inf {
                    lane.live = false;
                }
            }
        }
        let end = self.muls();
        out.check_fq_muls = check_muls;
        out.search_fq_muls = end - m1 - check_muls;
        out.table_entries = table.len() as u64;
        out.scalar = found;
        out
    }

    /// `F_q` multiplications of one affine addition of distinct points on
    /// this curve (`fq_muls_per_add`, the IC pipeline's divisor).
    pub fn fq_muls_per_add(&self) -> u64 {
        fq_muls_per_add(self.c())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::gaudry_quartic_bin::find_bin_curve;

    fn params(lanes: usize, dp_bits: u32) -> NegStrongParams {
        NegStrongParams {
            lanes,
            dp_bits,
            step_cap_factor: 2000,
        }
    }

    #[test]
    fn solves_planted_targets_on_small_towers() {
        for m in 4..=6u32 {
            let inst = find_bin_curve(m, 1, 16).unwrap();
            let rho = NegStrongRho::new(&inst);
            let r = u64::try_from(inst.r).expect("small test subgroup order");
            for seed in 0..4u64 {
                let k = (1 + seed * 7919 + u64::from(m) * 104_729) % r;
                let q = inst.curve.mul(&inst.generator, u128::from(k));
                let dp = auto_dp_bits(ideal_steps(r));
                let o = rho.solve(q, 0x5eed + seed, &params(4, dp));
                assert_eq!(o.scalar, Some(k), "m = {m}, seed {seed}");
                assert!(o.walk_steps > 0 && o.search_fq_muls > 0);
                assert!(o.canonicalisations >= o.walk_steps);
            }
        }
    }

    #[test]
    fn deterministic_in_seed() {
        let inst = find_bin_curve(5, 3, 16).unwrap();
        let rho = NegStrongRho::new(&inst);
        let q = inst.curve.mul(&inst.generator, 12345 % inst.r);
        let a = rho.solve(q, 9, &params(8, 3));
        let b = rho.solve(q, 9, &params(8, 3));
        assert_eq!(a, b);
    }

    #[test]
    fn batch_inverse_inverts_and_costs_three_products_a_lane_plus_one_inversion() {
        let inst = find_bin_curve(5, 2, 16).unwrap();
        let rho = NegStrongRho::new(&inst);
        let f = &inst.curve.f;
        let mut rng = StdRng::seed_from_u64(4);
        let xs: Vec<Eb> = (0..7)
            .map(|_| loop {
                let x = f.random(&mut rng);
                if !x.is_zero() {
                    break x;
                }
            })
            .collect();
        // The hand count: the same products and inversion, one at a time.
        let before = f.f.muls();
        let mut prefix = vec![xs[0]];
        for i in 1..xs.len() {
            let p = f.mul(&prefix[i - 1], &xs[i]);
            prefix.push(p);
        }
        let mut inv = f.inv(&prefix[6]);
        for i in (1..7).rev() {
            let _ = f.mul(&inv, &prefix[i - 1]);
            inv = f.mul(&inv, &xs[i]);
        }
        let hand = f.f.muls() - before;
        let before = f.f.muls();
        let mut d = xs.clone();
        let mut scratch = Vec::new();
        rho.batch_inverse(&mut d, &mut scratch);
        assert_eq!(f.f.muls() - before, hand);
        for (x, y) in xs.iter().zip(&d) {
            assert_eq!(f.mul(x, y), Eb::ONE);
        }
    }

    /// One lane, one step: the counted multiplications are exactly an
    /// `F_{q⁴}` inversion of the denominator, two products and a square.
    #[test]
    fn a_step_costs_what_a_hand_count_says() {
        let inst = find_bin_curve(4, 1, 16).unwrap();
        let rho = NegStrongRho::new(&inst);
        let c = &inst.curve;
        let f = &c.f;
        let p = c.mul(&inst.generator, 3);
        let j = c.mul(&inst.generator, 11);
        let d = p.x.add(&j.x);
        assert!(!d.is_zero());
        // Hand count, operation by operation.
        let before = f.f.muls();
        let inv = f.inv(&d);
        let l = f.mul(&p.y.add(&j.y), &inv);
        let x3 = f.sq(&l).add(&l).add(&p.x).add(&j.x).add(&c.a);
        let _y3 = f.mul(&l, &p.x.add(&x3));
        let hand = f.f.muls() - before;
        // The walk's batched path with one lane.
        let before = f.f.muls();
        let mut den = vec![d];
        rho.batch_inverse(&mut den, &mut Vec::new());
        let sum = rho.add_with_inverse(&p, &j, &den[0]);
        let walk = f.f.muls() - before;
        assert_eq!(walk, hand);
        assert_eq!(sum, c.add(&p, &j), "the batched addition is the group law");
        // And the full unbatched addition costs the same here.
        let before = f.f.muls();
        let _ = c.add(&p, &j);
        assert_eq!(f.f.muls() - before, hand);
    }

    /// Per-step cost on a short walk at a given `m`, for the record of the
    /// replication's prerequisite (development diagnostic, not a solve):
    /// `cargo test --release --lib negation_strong::tests::step_cost_probe -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn step_cost_probe() {
        for m in [14u32, 15] {
            let inst = find_bin_curve(m, 1, 16).unwrap();
            let rho = NegStrongRho::new(&inst);
            let q = inst.curve.mul(&inst.generator, 987_654_321 % inst.r);
            let p = NegStrongParams {
                lanes: 64,
                dp_bits: 30,
                step_cap_factor: 0,
            };
            // step_cap = max(ideal·0, 10⁶) = 10⁶ steps, no solve expected.
            let o = rho.solve(q, 1, &p);
            let per_step = (o.search_fq_muls + o.canonicalisations * CANON_FQ_MULS) as f64
                / o.walk_steps as f64;
            println!(
                "m={m} r={} fq_muls_per_add={} steps={} search_fq_muls={} canon={} escapes={} unbatched={} inversions={} per_step_incl_canon={per_step:.3}",
                inst.r,
                rho.fq_muls_per_add(),
                o.walk_steps,
                o.search_fq_muls,
                o.canonicalisations,
                o.escapes,
                o.unbatched_steps,
                o.inversions
            );
        }
    }
}
