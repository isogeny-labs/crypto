//! Selected modular uniform sampling with the pinned rand 0.8.8 stream.
//! Algorithm: rand's UniformInt<u64>::sample_single_inclusive (MIT/Apache-2.0),
//! using high-exclusive bounds here. Cargo.lock pins the upstream package.
//! Counters describe this sampler, not the generator's internal work.
use super::{class, enabled, mark_foreign, rng_words, words, Class, Phase, OWNER, PHASE, PHASES};
use crate::cryptanalysis::bitops::{int_add, int_mul};
use rand::{rngs::StdRng, RngCore};
use serde::Serialize;
use std::cell::Cell;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct UniformCounts {
    pub calls: u64,
    pub raw_draws: u64,
    pub rejections: u64,
    pub other_modulus_calls: u64,
}
impl UniformCounts {
    fn checked_sub(self, before: Self) -> Option<Self> {
        Some(Self {
            calls: self.calls.checked_sub(before.calls)?,
            raw_draws: self.raw_draws.checked_sub(before.raw_draws)?,
            rejections: self.rejections.checked_sub(before.rejections)?,
            other_modulus_calls: self
                .other_modulus_calls
                .checked_sub(before.other_modulus_calls)?,
        })
    }
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            calls: self.calls.checked_add(rhs.calls)?,
            raw_draws: self.raw_draws.checked_add(rhs.raw_draws)?,
            rejections: self.rejections.checked_add(rhs.rejections)?,
            other_modulus_calls: self
                .other_modulus_calls
                .checked_add(rhs.other_modulus_calls)?,
        })
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UniformTally {
    pub phases: [UniformCounts; PHASES],
}
impl UniformTally {
    pub fn since(&self, before: &Self) -> Option<Self> {
        let mut out = Self::default();
        for (i, c) in self.phases.iter().enumerate() {
            out.phases[i] = c.checked_sub(before.phases[i])?;
        }
        Some(out)
    }
    pub fn sum(&self, phases: &[Phase]) -> Option<UniformCounts> {
        phases.iter().try_fold(UniformCounts::default(), |sum, p| {
            sum.checked_add(self.phases[*p as usize])
        })
    }
}
thread_local! {
    static COUNTS:[[Cell<u64>;4];PHASES]=const{[const{[const{Cell::new(0)};4]};PHASES]};
}
fn bump(event: usize) {
    if !OWNER.with(Cell::get) {
        mark_foreign();
        return;
    }
    let phase = PHASE.with(Cell::get);
    COUNTS.with(|c| c[phase][event].set(c[phase][event].get() + 1));
}
pub fn uniform_snapshot() -> UniformTally {
    COUNTS.with(|c| UniformTally {
        phases: core::array::from_fn(|i| UniformCounts {
            calls: c[i][0].get(),
            raw_draws: c[i][1].get(),
            rejections: c[i][2].get(),
            other_modulus_calls: c[i][3].get(),
        }),
    })
}

/// Uniform in [low, high), with exactly rand 0.8.8's u64 values and raw draws.
/// Enable/claim the tally before calling; changing counting during a call is
/// outside the snapshot contract. Existing StdRng APIs remain unchanged.
pub fn uniform_mod_u64(rng: &mut StdRng, low: u64, high: u64) -> u64 {
    assert!(low < high, "uniform sampler requires low < high");
    let counting = enabled();
    if counting {
        bump(0);
        if matches!(class(high), Class::Other) {
            bump(3);
        }
    }
    let range = high - low;
    let zone = (range << range.leading_zeros()).wrapping_sub(1);
    loop {
        let v = rng.next_u64();
        if counting {
            bump(1);
            rng_words(1);
            words(1);
        }
        let product = v as u128 * range as u128;
        if product as u64 <= zone {
            return low.wrapping_add((product >> 64) as u64);
        }
        if counting {
            bump(2);
        }
    }
}

/// Known native integer prices are separate from the primitive subtotal.
/// Subtraction, clz and shift events are emitted unpriced; no total is claimed.
pub fn uniform_json(c: &UniformCounts) -> Option<serde_json::Value> {
    let price = (c.raw_draws as u128)
        .checked_mul(int_mul(64))?
        .checked_add((c.calls as u128).checked_mul(int_add(64))?)?;
    Some(
        serde_json::json!({"schema":"modular-uniform-events/v1","complete":false,
        "calls":c.calls,"raw_draws":c.raw_draws,"rejections":c.rejections,
        "other_modulus_calls":c.other_modulus_calls,
        "priced_native":{"integer_mul64":c.raw_draws,"integer_add64":c.calls},
        "known_native_priced_bitops":price.to_string(),
        "unpriced_native":{"integer_sub64":c.calls.checked_mul(2)?,"integer_clz64":c.calls,"integer_shl64":c.calls},
        "generator_internal_work":"not counted; other sampling paths still require coverage"}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::jv_bitops::{self, Kind};
    use rand::{Rng, SeedableRng};
    #[test]
    fn sampler_matches_pinned_rand_values_and_final_rng_state() {
        let ranges = [
            (0, 1),
            (0, 2),
            (1, 3),
            (0, 101),
            (1, 1_000_003),
            (0, (1 << 61) - 1),
            (1, (1 << 63) - 1),
            (0, u64::MAX),
        ];
        for seed in [0, 1, 17, 277] {
            for (low, high) in ranges {
                let mut original = StdRng::seed_from_u64(seed);
                let mut observed = original.clone();
                for _ in 0..2048 {
                    assert_eq!(
                        uniform_mod_u64(&mut observed, low, high),
                        original.gen_range(low..high)
                    );
                }
                assert_eq!(observed.next_u64(), original.next_u64());
            }
        }
    }
    #[test]
    fn rejection_words_phase_and_native_prices_are_explicit() {
        jv_bitops::enable(true);
        jv_bitops::claim();
        jv_bitops::set_moduli(101, 1);
        let before = uniform_snapshot();
        let prim_before = jv_bitops::snapshot();
        let mut rng = StdRng::seed_from_u64(17);
        {
            let _phase = jv_bitops::enter(Phase::Descent);
            for _ in 0..1024 {
                assert_eq!(uniform_mod_u64(&mut rng, 0, 1), 0);
            }
        }
        let delta = uniform_snapshot().since(&before).unwrap();
        let c = delta.sum(&[Phase::Descent]).unwrap();
        assert_eq!(c.calls, 1024);
        assert_eq!(c.raw_draws, c.calls + c.rejections);
        assert!(c.rejections > 0);
        assert_eq!(c.other_modulus_calls, 0);
        let t = jv_bitops::snapshot().since(&prim_before);
        let p = t.phase(Phase::Descent);
        assert_eq!(p.get(Kind::WordOp), 2 * c.raw_draws);
        assert_eq!(p.rng_words, c.raw_draws);
        let known = uniform_json(&UniformCounts {
            calls: 1,
            raw_draws: 2,
            rejections: 1,
            other_modulus_calls: 0,
        })
        .unwrap();
        assert_eq!(known["known_native_priced_bitops"], "48832");
        assert_eq!(known["unpriced_native"]["integer_sub64"], 2);
        assert_eq!(known["complete"], false);
        assert!(UniformTally::default().since(&delta).is_none());
    }
}
