//! Opt-in template basis to the existing Nagao root stage, with explicit F4
//! fallback and attempted-work accounting. The root stage's unresolved flags
//! remain authoritative; a complete basis does not certify complete roots.
use super::jv_nagao_template::{Template, Work};
use crate::cryptanalysis::{
    f4_fp::{F4Options, Ordering, Poly},
    jv_bitops as native,
    jv_cover::{self as cover, LinMode, ZeroDimStats},
};
use certified_relations as algebra;
use rand::rngs::StdRng;
use serde::Serialize;
use std::time::Instant;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Fallback {
    Plain,
    Traced,
}

#[derive(Clone, Debug, Serialize)]
pub struct Audit {
    pub basis_accepted: bool,
    pub attempted_elimination: Work,
    /// None when this root feature is absent, even if another dependency build
    /// happens to collect counters. Callers must not promote None to zero.
    pub checker: Option<Work>,
    pub checker_counter_feature: bool,
    pub refusal: Option<String>,
    pub fallback_used: bool,
    pub fallback: Fallback,
    pub compiled_rows: usize,
    pub compiled_columns: usize,
    /// Field arithmetic is recorded; map/allocation/word bookkeeping is not
    /// completely instrumented. Do not promote this prototype's native total.
    pub full_native_cost_available: bool,
}
#[derive(Serialize)]
pub struct Solved {
    pub roots: Vec<Vec<u64>>,
    pub stats: ZeroDimStats,
    pub audit: Audit,
}

fn expired(options: &F4Options) -> bool {
    options.deadline.is_some_and(|d| Instant::now() >= d)
}

fn checker_delta(
    before: algebra::Counters,
    after: algebra::Counters,
) -> Result<Option<Work>, &'static str> {
    let delta = Work {
        modular_products: after
            .modular_products
            .checked_sub(before.modular_products)
            .ok_or("checker counter reset/overflow")?,
        nontrivial_inversions: after
            .nontrivial_inversions
            .checked_sub(before.nontrivial_inversions)
            .ok_or("checker counter reset/overflow")?,
        coefficient_updates: after
            .coefficient_updates
            .checked_sub(before.coefficient_updates)
            .ok_or("checker counter reset/overflow")?,
    };
    Ok(cfg!(feature = "nagao-template-counters").then_some(delta))
}

/// Suitable for a `jv_nagao_unit::decompose_using` closure. Capture Audit beside
/// its legacy optional unit-elimination audit; do not discard failed work.
/// This adapter never clears a trace pool or refreshes the caller's deadline.
pub fn solve(
    template: &Template,
    input: &[Poly],
    p: u64,
    options: &F4Options,
    rng: &mut StdRng,
    fallback: Fallback,
) -> Result<Solved, &'static str> {
    let n = template.variables();
    if template.modulus() != p
        || p >= (1u64 << 31)
        || options.order != Ordering::Grevlex
        || input.is_empty()
        || input
            .iter()
            .flat_map(|f| f.iter())
            .any(|(m, c)| m.len() != n || *c >= p)
    {
        return Err("template adapter domain/arity/order");
    }
    let started = Instant::now();
    let before = algebra::counters();
    let attempted = template.apply_until(input, options.deadline);
    let checker = checker_delta(before, algebra::counters())?;
    let (basis, work, refusal) = match attempted {
        Ok(value) => (Some(value.basis), value.elimination, None),
        Err(error) => (None, error.attempted, Some(error.reason)),
    };
    let known_products = work
        .modular_products
        .checked_add(checker.as_ref().map_or(0, |w| w.modular_products))
        .ok_or("template product counter overflow")?;
    let known_updates = work
        .coefficient_updates
        .checked_add(checker.as_ref().map_or(0, |w| w.coefficient_updates))
        .ok_or("template update counter overflow")?;
    // Both kernels maintain their own counters and do not call native hooks.
    // Inversion exponentiation products are already included: do not price a
    // second opaque inversion beside them. Bookkeeping coverage stays partial.
    native::hook_muls(p, known_products);
    native::hook_adds(p, known_updates);
    let attempt_ms = started.elapsed().as_secs_f64() * 1000.0;
    let mut audit = Audit {
        basis_accepted: basis.is_some(),
        attempted_elimination: work,
        checker,
        checker_counter_feature: cfg!(feature = "nagao-template-counters"),
        refusal,
        fallback_used: false,
        fallback,
        compiled_rows: template.rows(),
        compiled_columns: template.columns(),
        full_native_cost_available: false,
    };
    if expired(options) {
        return Ok(Solved {
            roots: Vec::new(),
            stats: ZeroDimStats {
                timed_out: true,
                f4_muls: known_products,
                f4_ms: attempt_ms,
                ..Default::default()
            },
            audit,
        });
    }
    let (roots, mut stats) = match basis {
        Some(basis) => {
            let mut stats = ZeroDimStats {
                f4_muls: known_products,
                f4_ms: attempt_ms,
                degree_reached: template.matrix_degree(),
                max_rows: template.rows(),
                max_cols: template.columns(),
                ..Default::default()
            };
            let roots = if basis
                .iter()
                .any(|f| f.len() == 1 && f[0].0.iter().all(|e| *e == 0))
            {
                stats.inconsistent = true;
                Vec::new()
            } else {
                cover::linear_stage_from_basis(
                    &basis,
                    input,
                    n,
                    p,
                    rng,
                    LinMode::Fast,
                    &mut stats,
                    Instant::now(),
                )
            };
            (roots, stats)
        }
        None => {
            audit.fallback_used = true;
            let (roots, mut stats) = cover::solve_zero_dim_traced(
                input,
                n,
                p,
                options,
                rng,
                matches!(fallback, Fallback::Traced),
            );
            stats.f4_muls = stats
                .f4_muls
                .checked_add(known_products)
                .ok_or("combined solver counter overflow")?;
            stats.f4_ms += attempt_ms;
            (roots, stats)
        }
    };
    if expired(options) {
        stats.timed_out = true;
    }
    Ok(Solved {
        roots,
        stats,
        audit,
    })
}
