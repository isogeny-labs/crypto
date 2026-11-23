//! Opt-in action-coordinate sparse FGLM with the existing dense fallback.
//!
//! This adapter does not select a coordinate from observed target results.
//! The caller supplies a zero-based coordinate and retains the original
//! basis/input order. Both attempted fast work and dense fallback products
//! are included in the existing, partial `ZeroDimStats::lin_muls` counter.

use super::{
    f4_fp::Poly,
    jv_cover::{self as cover, LinMode, ZeroDimStats},
    zero_dim_la::{self, FastStats},
};
use rand::rngs::StdRng;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct ActionStageResult {
    pub action_variable: usize,
    pub roots: Vec<Vec<u64>>,
    pub fast_stats: FastStats,
    pub used_dense_fallback: bool,
}

/// Extract rational roots using a supplied action coordinate, falling back
/// to the incumbent dense stage when the fast stage refuses the instance.
///
/// Preconditions on the basis, input, field and clean statistics are the
/// same as `jv_cover::linear_stage_from_basis`. This adds no completeness
/// guarantee for a truncated or uncertified input basis. An out-of-range
/// coordinate is a configuration error, not an empty solution set.
#[allow(clippy::too_many_arguments)]
pub fn linear_stage_from_basis_with_action(
    basis: &[Poly],
    input: &[Poly],
    n: usize,
    p: u64,
    action_variable: usize,
    rng: &mut StdRng,
    st: &mut ZeroDimStats,
    t_lin: Instant,
) -> Result<ActionStageResult, &'static str> {
    if action_variable >= n {
        return Err("action variable out of range");
    }
    let mut fast_stats = FastStats::default();
    let (roots, used_dense_fallback) = match zero_dim_la::fast_stage_with_action(
        basis,
        input,
        n,
        p,
        action_variable,
        rng,
        &mut fast_stats,
    ) {
        Some(roots) => {
            st.delta = fast_stats.delta;
            st.lin_muls = fast_stats.muls();
            st.lin_ms = t_lin.elapsed().as_secs_f64() * 1e3;
            (roots, false)
        }
        None => {
            st.lin_fallback = st
                .lin_fallback
                .checked_add(1)
                .ok_or("linear fallback counter overflow")?;
            let roots =
                cover::linear_stage_from_basis(basis, input, n, p, rng, LinMode::Dense, st, t_lin);
            st.lin_muls = st
                .lin_muls
                .checked_add(fast_stats.muls())
                .ok_or("linear multiplication counter overflow")?;
            (roots, true)
        }
    };
    Ok(ActionStageResult {
        action_variable,
        roots,
        fast_stats,
        used_dense_fallback,
    })
}
