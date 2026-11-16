//! Risk-controlled empirical pruning for index-calculus relation collectors.
//!
//! This module deliberately separates three observations that are often
//! collapsed into one "yield" number:
//!
//! - a probe produced at least one independently verified relation;
//! - the verified relations raised the relation-matrix rank;
//! - independent episodes reached the frozen target rank inside the budget.
//!
//! Only capped verified-row yield and the third channel drive statistical
//! pruning.  Rank gain, factor-base coverage and hit entropy are reported as
//! diagnostics because they become dependent and non-stationary as the matrix
//! fills.  Finite, predeclared looks and a Bonferroni allocation make the
//! Hoeffding and exact binomial bounds valid under the assumptions recorded in
//! [`PruningReport`].

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::two_rank_null::clopper_pearson;

pub const INPUT_SCHEMA: &str = "ic.relation-pruning.input/v1";
pub const REPORT_SCHEMA: &str = "ic.relation-pruning.report/v1";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PruningPolicy {
    /// Familywise error budget across every candidate, look and bound.
    pub familywise_alpha: f64,
    /// Frozen number of candidate slots, including slots not eventually used.
    pub planned_candidates: usize,
    /// Frozen maximum number of independent episodes per candidate.
    pub planned_episodes: usize,
    /// Cumulative probe counts at which the analyzer may be called.
    pub checkpoint_trials: Vec<u64>,
    /// Do not issue an in-episode futility stop before this many probes.
    pub min_trials_for_futility: u64,
    /// Terminal episodes required before an across-episode prune.
    pub min_terminal_episodes: usize,
    /// Required lower limit for full-rank success.  Set to zero to disable.
    pub min_full_rank_success: f64,
    /// Absolute success-rate gap required for incumbent dominance.
    pub dominance_margin: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EpisodeObservation {
    pub candidate_id: String,
    pub episode_id: String,
    /// Cumulative probes in this episode.
    pub trials: u64,
    /// Trials producing at least one independently verified relation.
    pub relation_hit_trials: u64,
    /// Total independently verified relations.  A trial may emit more than one.
    pub verified_relations: u64,
    /// Frozen hard cap on verified relations one trial can emit.  A zero cap
    /// disables the probabilistic in-episode futility rule.
    pub relation_cap_per_trial: u64,
    /// Relations that increased rank, not merely distinct encodings.
    pub independent_relations: u64,
    pub initial_rank: u64,
    pub rank: u64,
    pub target_rank: u64,
    pub factor_base_columns: u64,
    pub covered_columns: u64,
    /// Optional deterministic upper bound from the factor-base construction.
    pub reachable_rank_upper: Option<u64>,
    /// Per-column appearances in verified relations.  Empty means unavailable.
    #[serde(default)]
    pub factor_hits: Vec<u64>,
    /// Complete charged work in the protocol's named unit.
    pub charged_cost: u64,
    pub terminal: bool,
    /// Full rank plus the protocol's independent verification, not rank alone.
    pub full_rank_verified: bool,
    pub invalid_relations: u64,
    pub inconsistent_relations: u64,
    pub verification_failures: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PruningInput {
    pub schema: String,
    pub look: usize,
    pub policy: PruningPolicy,
    pub observations: Vec<EpisodeObservation>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Interval {
    pub estimate: f64,
    pub lower: f64,
    pub upper: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeDecision {
    Continue,
    FullRank,
    BudgetExhausted,
    StopFutile,
    StopStructural,
    RejectInvalid,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CandidateDecision {
    Keep,
    InsufficientEvidence,
    PruneInvalid,
    PruneStructural,
    PruneLowSuccess,
    PruneDominated,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EpisodeReport {
    pub episode_id: String,
    pub decision: EpisodeDecision,
    pub reason: String,
    pub relation_hit_interval: Interval,
    pub optimistic_future_relations: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CandidateReport {
    pub candidate_id: String,
    pub decision: CandidateDecision,
    pub reasons: Vec<String>,
    pub episodes: usize,
    pub terminal_episodes: usize,
    pub full_rank_successes: usize,
    pub full_rank_success: Interval,
    pub trials: u64,
    pub relation_hit_trials: u64,
    pub verified_relations: u64,
    pub independent_relations: u64,
    pub relation_hit_rate: Interval,
    /// Descriptive only: verified relations may have multiplicity above one.
    pub verified_relations_per_trial: f64,
    /// Descriptive only: matrix saturation makes this non-stationary.
    pub independent_relations_per_trial: f64,
    /// Descriptive only: the share of verified rows that increased rank.
    pub rank_utility: f64,
    pub mean_factor_coverage: f64,
    pub min_factor_coverage: f64,
    pub mean_normalised_hit_entropy: Option<f64>,
    pub mean_dead_columns: Option<f64>,
    pub charged_cost: u64,
    pub charged_cost_per_trial: f64,
    pub episode_reports: Vec<EpisodeReport>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PruningReport {
    pub schema: String,
    pub input_schema: String,
    pub look: usize,
    pub checkpoint_trials: u64,
    pub familywise_alpha: f64,
    pub per_bound_alpha: f64,
    pub incumbent: Option<String>,
    pub candidates: Vec<CandidateReport>,
    pub assumptions: Vec<String>,
}

fn validate_policy(p: &PruningPolicy) -> Result<(), String> {
    if !(0.0 < p.familywise_alpha && p.familywise_alpha < 1.0) {
        return Err("familywise_alpha must lie strictly between zero and one".into());
    }
    if p.planned_candidates == 0 || p.planned_episodes == 0 {
        return Err("planned_candidates and planned_episodes must be nonzero".into());
    }
    if p.checkpoint_trials.is_empty()
        || p.checkpoint_trials[0] == 0
        || p.checkpoint_trials.windows(2).any(|w| w[0] >= w[1])
    {
        return Err("checkpoint_trials must be nonzero and strictly increasing".into());
    }
    if p.min_terminal_episodes == 0 || p.min_terminal_episodes > p.planned_episodes {
        return Err("min_terminal_episodes must be in 1..=planned_episodes".into());
    }
    if !(0.0..=1.0).contains(&p.min_full_rank_success) || !(0.0..=1.0).contains(&p.dominance_margin)
    {
        return Err("success threshold and dominance margin must lie in [0,1]".into());
    }
    Ok(())
}

fn validate_observation(
    o: &EpisodeObservation,
    checkpoint: u64,
    max_trials: u64,
) -> Result<(), String> {
    let id = format!("{}/{}", o.candidate_id, o.episode_id);
    if o.candidate_id.is_empty() || o.episode_id.is_empty() {
        return Err("candidate_id and episode_id must be nonempty".into());
    }
    if o.trials > max_trials || (!o.terminal && o.trials != checkpoint) {
        return Err(format!("{id}: active trials must equal the current checkpoint and no episode may exceed the frozen maximum"));
    }
    if !o.terminal && o.trials == max_trials {
        return Err(format!(
            "{id}: an episode at the frozen maximum must be terminal"
        ));
    }
    if o.relation_hit_trials > o.trials {
        return Err(format!("{id}: relation_hit_trials exceeds trials"));
    }
    if o.relation_cap_per_trial > 0
        && u128::from(o.verified_relations)
            > u128::from(o.trials) * u128::from(o.relation_cap_per_trial)
    {
        return Err(format!(
            "{id}: verified_relations exceeds the frozen per-trial cap"
        ));
    }
    let closed_rank = o.initial_rank.checked_add(o.independent_relations);
    if o.independent_relations > o.verified_relations || closed_rank != Some(o.rank) {
        return Err(format!(
            "{id}: rank/independent/verified relation counts do not close"
        ));
    }
    if o.target_rank == 0 || o.rank > o.target_rank {
        return Err(format!(
            "{id}: target rank must be nonzero and rank may not exceed it"
        ));
    }
    if o.covered_columns > o.factor_base_columns {
        return Err(format!("{id}: covered_columns exceeds factor_base_columns"));
    }
    if !o.factor_hits.is_empty() {
        if o.factor_hits.len() != o.factor_base_columns as usize {
            return Err(format!(
                "{id}: factor_hits length does not match factor_base_columns"
            ));
        }
        let covered = o.factor_hits.iter().filter(|&&x| x > 0).count() as u64;
        if covered != o.covered_columns {
            return Err(format!(
                "{id}: factor_hits does not reproduce covered_columns"
            ));
        }
        if o.factor_hits
            .iter()
            .try_fold(0u64, |sum, &hit| sum.checked_add(hit))
            .is_none()
        {
            return Err(format!("{id}: factor_hits total overflows u64"));
        }
    }
    if o.full_rank_verified
        && (!o.terminal
            || o.rank < o.target_rank
            || o.invalid_relations > 0
            || o.inconsistent_relations > 0
            || o.verification_failures > 0)
    {
        return Err(format!(
            "{id}: full_rank_verified contradicts status or correctness counters"
        ));
    }
    if o.terminal
        && !o.full_rank_verified
        && !observation_invalid(o)
        && !o
            .reachable_rank_upper
            .is_some_and(|upper| upper < o.target_rank)
        && o.trials < max_trials
    {
        return Err(format!(
            "{id}: a failed statistical episode must exhaust the frozen budget"
        ));
    }
    Ok(())
}

fn validate_candidate(id: &str, rows: &[&EpisodeObservation]) -> Result<(), String> {
    let first = rows.first().expect("candidate groups are nonempty");
    if rows.iter().skip(1).any(|o| {
        o.initial_rank != first.initial_rank
            || o.target_rank != first.target_rank
            || o.factor_base_columns != first.factor_base_columns
            || o.reachable_rank_upper != first.reachable_rank_upper
            || o.relation_cap_per_trial != first.relation_cap_per_trial
    }) {
        return Err(format!(
            "{id}: frozen rank, factor-base and relation-cap fields differ across episodes"
        ));
    }
    for (name, values) in [
        ("trials", rows.iter().map(|o| o.trials).collect::<Vec<_>>()),
        (
            "relation_hit_trials",
            rows.iter().map(|o| o.relation_hit_trials).collect(),
        ),
        (
            "verified_relations",
            rows.iter().map(|o| o.verified_relations).collect(),
        ),
        (
            "independent_relations",
            rows.iter().map(|o| o.independent_relations).collect(),
        ),
        (
            "charged_cost",
            rows.iter().map(|o| o.charged_cost).collect(),
        ),
    ] {
        if values
            .into_iter()
            .try_fold(0u64, u64::checked_add)
            .is_none()
        {
            return Err(format!("{id}: aggregate {name} overflows u64"));
        }
    }
    Ok(())
}

fn hoeffding_interval(successes: u64, trials: u64, alpha: f64) -> Interval {
    if trials == 0 {
        return Interval {
            estimate: 0.0,
            lower: 0.0,
            upper: 1.0,
        };
    }
    let estimate = successes as f64 / trials as f64;
    let radius = ((2.0 / alpha).ln() / (2.0 * trials as f64)).sqrt();
    Interval {
        estimate,
        lower: (estimate - radius).max(0.0),
        upper: (estimate + radius).min(1.0),
    }
}

fn full_rank_interval(successes: usize, trials: usize, alpha: f64) -> Interval {
    if trials == 0 {
        return Interval {
            estimate: 0.0,
            lower: 0.0,
            upper: 1.0,
        };
    }
    let (lower, upper) = clopper_pearson(successes, trials, alpha);
    Interval {
        estimate: successes as f64 / trials as f64,
        lower,
        upper,
    }
}

fn observation_invalid(o: &EpisodeObservation) -> bool {
    o.invalid_relations > 0 || o.inconsistent_relations > 0 || o.verification_failures > 0
}

/// A high-probability upper bound on future verified relation rows.
///
/// Each trial's row count must lie in `[0, relation_cap_per_trial]`.  One
/// one-sided Hoeffding bound covers the unknown mean and a second covers the
/// future draw around that mean; both are charged in `per_bound_alpha`.
fn optimistic_future_relations(o: &EpisodeObservation, max_trials: u64, alpha: f64) -> Option<u64> {
    let cap = o.relation_cap_per_trial;
    if cap == 0 || o.trials == 0 || o.trials >= max_trials {
        return (cap > 0).then_some(0);
    }
    let normalised_mean = o.verified_relations as f64 / (o.trials as f64 * cap as f64);
    let mean_radius = ((1.0 / alpha).ln() / (2.0 * o.trials as f64)).sqrt();
    let upper_mean = (normalised_mean + mean_radius).min(1.0);
    let remaining = max_trials - o.trials;
    let future_radius = cap as f64 * (remaining as f64 * (1.0 / alpha).ln() / 2.0).sqrt();
    let upper = remaining as f64 * cap as f64 * upper_mean + future_radius;
    Some(upper.ceil().min(u64::MAX as f64) as u64)
}

fn hit_entropy(o: &EpisodeObservation) -> Option<(f64, f64)> {
    if o.factor_hits.is_empty() {
        return None;
    }
    let total: u64 = o.factor_hits.iter().sum();
    let dead = o.factor_hits.iter().filter(|&&x| x == 0).count() as f64;
    if total == 0 {
        return Some((0.0, dead));
    }
    let entropy = o
        .factor_hits
        .iter()
        .filter(|&&x| x > 0)
        .map(|&x| {
            let p = x as f64 / total as f64;
            -p * p.ln()
        })
        .sum::<f64>();
    let normaliser = (o.factor_base_columns as f64).ln();
    let normalised = if normaliser > 0.0 {
        entropy / normaliser
    } else {
        1.0
    };
    Some((normalised, dead))
}

fn episode_report(
    o: &EpisodeObservation,
    p: &PruningPolicy,
    max_trials: u64,
    alpha: f64,
) -> EpisodeReport {
    let hit_interval = hoeffding_interval(o.relation_hit_trials, o.trials, alpha);
    if observation_invalid(o) {
        return EpisodeReport {
            episode_id: o.episode_id.clone(),
            decision: EpisodeDecision::RejectInvalid,
            reason: "a relation or recovery correctness check failed".into(),
            relation_hit_interval: hit_interval,
            optimistic_future_relations: None,
        };
    }
    if o.reachable_rank_upper
        .is_some_and(|upper| upper < o.target_rank)
    {
        return EpisodeReport {
            episode_id: o.episode_id.clone(),
            decision: EpisodeDecision::StopStructural,
            reason: "the frozen factor-base construction cannot reach target rank".into(),
            relation_hit_interval: hit_interval,
            optimistic_future_relations: None,
        };
    }
    if o.full_rank_verified {
        return EpisodeReport {
            episode_id: o.episode_id.clone(),
            decision: EpisodeDecision::FullRank,
            reason: "target rank and independent verification passed".into(),
            relation_hit_interval: hit_interval,
            optimistic_future_relations: Some(0),
        };
    }
    let optimistic = optimistic_future_relations(o, max_trials, alpha);
    if !o.terminal
        && o.trials >= p.min_trials_for_futility
        && optimistic.is_some_and(|future| o.rank.saturating_add(future) < o.target_rank)
    {
        return EpisodeReport {
            episode_id: o.episode_id.clone(),
            decision: EpisodeDecision::StopFutile,
            reason:
                "even the alpha-budgeted optimistic relation count cannot close the remaining rank"
                    .into(),
            relation_hit_interval: hit_interval,
            optimistic_future_relations: optimistic,
        };
    }
    let (decision, reason) = if o.terminal || o.trials >= max_trials {
        (
            EpisodeDecision::BudgetExhausted,
            "frozen budget ended without verified full rank",
        )
    } else {
        (EpisodeDecision::Continue, "no hard pruning gate fired")
    };
    EpisodeReport {
        episode_id: o.episode_id.clone(),
        decision,
        reason: reason.into(),
        relation_hit_interval: hit_interval,
        optimistic_future_relations: optimistic,
    }
}

fn candidate_report(
    id: &str,
    rows: &[&EpisodeObservation],
    p: &PruningPolicy,
    max_trials: u64,
    alpha: f64,
) -> CandidateReport {
    let terminal_episodes = rows.iter().filter(|o| o.terminal).count();
    let full_rank_successes = rows.iter().filter(|o| o.full_rank_verified).count();
    let trials = rows.iter().map(|o| o.trials).sum();
    let relation_hit_trials = rows.iter().map(|o| o.relation_hit_trials).sum();
    let verified_relations = rows.iter().map(|o| o.verified_relations).sum();
    let independent_relations = rows.iter().map(|o| o.independent_relations).sum();
    let charged_cost = rows.iter().map(|o| o.charged_cost).sum();
    let coverages: Vec<f64> = rows
        .iter()
        .map(|o| {
            if o.factor_base_columns == 0 {
                0.0
            } else {
                o.covered_columns as f64 / o.factor_base_columns as f64
            }
        })
        .collect();
    let entropy: Vec<(f64, f64)> = rows.iter().filter_map(|o| hit_entropy(o)).collect();
    let episode_reports = rows
        .iter()
        .map(|o| episode_report(o, p, max_trials, alpha))
        .collect();
    CandidateReport {
        candidate_id: id.into(),
        decision: CandidateDecision::InsufficientEvidence,
        reasons: Vec::new(),
        episodes: rows.len(),
        terminal_episodes,
        full_rank_successes,
        full_rank_success: full_rank_interval(full_rank_successes, terminal_episodes, alpha),
        trials,
        relation_hit_trials,
        verified_relations,
        independent_relations,
        relation_hit_rate: hoeffding_interval(relation_hit_trials, trials, alpha),
        verified_relations_per_trial: if trials == 0 {
            0.0
        } else {
            verified_relations as f64 / trials as f64
        },
        independent_relations_per_trial: if trials == 0 {
            0.0
        } else {
            independent_relations as f64 / trials as f64
        },
        rank_utility: if verified_relations == 0 {
            0.0
        } else {
            independent_relations as f64 / verified_relations as f64
        },
        mean_factor_coverage: if coverages.is_empty() {
            0.0
        } else {
            coverages.iter().sum::<f64>() / coverages.len() as f64
        },
        min_factor_coverage: coverages.iter().copied().reduce(f64::min).unwrap_or(0.0),
        mean_normalised_hit_entropy: (!entropy.is_empty())
            .then(|| entropy.iter().map(|x| x.0).sum::<f64>() / entropy.len() as f64),
        mean_dead_columns: (!entropy.is_empty())
            .then(|| entropy.iter().map(|x| x.1).sum::<f64>() / entropy.len() as f64),
        charged_cost,
        charged_cost_per_trial: if trials == 0 {
            0.0
        } else {
            charged_cost as f64 / trials as f64
        },
        episode_reports,
    }
}

pub fn analyse(input: &PruningInput) -> Result<PruningReport, String> {
    if input.schema != INPUT_SCHEMA {
        return Err(format!("schema must be {INPUT_SCHEMA}"));
    }
    validate_policy(&input.policy)?;
    if input.look == 0 || input.look > input.policy.checkpoint_trials.len() {
        return Err("look is one-based and must name a frozen checkpoint".into());
    }
    if input.observations.is_empty() {
        return Err("at least one observation is required".into());
    }
    let checkpoint = input.policy.checkpoint_trials[input.look - 1];
    let max_trials = *input
        .policy
        .checkpoint_trials
        .last()
        .expect("validated nonempty");
    let mut keys = BTreeSet::new();
    let mut candidates: BTreeMap<String, Vec<&EpisodeObservation>> = BTreeMap::new();
    for o in &input.observations {
        validate_observation(o, checkpoint, max_trials)?;
        if !keys.insert((o.candidate_id.clone(), o.episode_id.clone())) {
            return Err(format!(
                "duplicate observation {}/{}",
                o.candidate_id, o.episode_id
            ));
        }
        candidates
            .entry(o.candidate_id.clone())
            .or_default()
            .push(o);
    }
    if candidates.len() > input.policy.planned_candidates {
        return Err("observed candidates exceed planned_candidates".into());
    }
    if candidates
        .values()
        .any(|v| v.len() > input.policy.planned_episodes)
    {
        return Err("observed episodes exceed planned_episodes".into());
    }
    for (id, rows) in &candidates {
        validate_candidate(id, rows)?;
    }

    // Four bound slots cover: observed mean, future tail, full-rank interval,
    // and a spare slot for paired extensions without weakening v1's budget.
    let slots = input
        .policy
        .planned_candidates
        .checked_mul(input.policy.checkpoint_trials.len())
        .and_then(|x| x.checked_mul(input.policy.planned_episodes.checked_add(1)?))
        .and_then(|x| x.checked_mul(4))
        .ok_or("planned multiplicity budget overflows usize")?;
    let per_bound_alpha = input.policy.familywise_alpha / slots as f64;
    let mut reports: Vec<CandidateReport> = candidates
        .iter()
        .map(|(id, rows)| candidate_report(id, rows, &input.policy, max_trials, per_bound_alpha))
        .collect();

    let incumbent = reports
        .iter()
        .filter(|r| {
            r.terminal_episodes >= input.policy.min_terminal_episodes
                && !r
                    .episode_reports
                    .iter()
                    .any(|e| e.decision == EpisodeDecision::RejectInvalid)
                && !r
                    .episode_reports
                    .iter()
                    .all(|e| e.decision == EpisodeDecision::StopStructural)
        })
        .max_by(|a, b| {
            a.full_rank_success
                .lower
                .total_cmp(&b.full_rank_success.lower)
                .then_with(|| {
                    a.full_rank_success
                        .estimate
                        .total_cmp(&b.full_rank_success.estimate)
                })
                .then_with(|| b.candidate_id.cmp(&a.candidate_id))
        })
        .map(|r| (r.candidate_id.clone(), r.full_rank_success.lower));

    for r in &mut reports {
        if r.episode_reports
            .iter()
            .any(|e| e.decision == EpisodeDecision::RejectInvalid)
        {
            r.decision = CandidateDecision::PruneInvalid;
            r.reasons
                .push("at least one correctness gate failed".into());
            continue;
        }
        if r.episode_reports
            .iter()
            .all(|e| e.decision == EpisodeDecision::StopStructural)
        {
            r.decision = CandidateDecision::PruneStructural;
            r.reasons.push(
                "every observed episode has a deterministic rank ceiling below target".into(),
            );
            continue;
        }
        if r.terminal_episodes < input.policy.min_terminal_episodes {
            r.decision = CandidateDecision::InsufficientEvidence;
            r.reasons.push(format!(
                "{} terminal episodes observed; {} required",
                r.terminal_episodes, input.policy.min_terminal_episodes
            ));
            continue;
        }
        if r.full_rank_success.upper < input.policy.min_full_rank_success {
            r.decision = CandidateDecision::PruneLowSuccess;
            r.reasons.push(format!(
                "full-rank success upper bound {:.6} is below the frozen threshold {:.6}",
                r.full_rank_success.upper, input.policy.min_full_rank_success
            ));
            continue;
        }
        if let Some((incumbent_id, incumbent_lower)) = &incumbent {
            if &r.candidate_id != incumbent_id
                && r.full_rank_success.upper + input.policy.dominance_margin < *incumbent_lower
            {
                r.decision = CandidateDecision::PruneDominated;
                r.reasons.push(format!(
                    "success upper bound plus margin is below incumbent {incumbent_id}'s lower bound"
                ));
                continue;
            }
        }
        r.decision = CandidateDecision::Keep;
        r.reasons
            .push("no hard candidate-level pruning gate fired".into());
    }

    Ok(PruningReport {
        schema: REPORT_SCHEMA.into(),
        input_schema: INPUT_SCHEMA.into(),
        look: input.look,
        checkpoint_trials: checkpoint,
        familywise_alpha: input.policy.familywise_alpha,
        per_bound_alpha,
        incumbent: incumbent.map(|x| x.0),
        candidates: reports,
        assumptions: vec![
            "checkpoint schedule, candidate slots, episode slots and thresholds were frozen before observing this panel".into(),
            "within an episode, capped relation counts are independent bounded draws with a stationary mean for the futility projection".into(),
            "terminal full-rank outcomes are independent across episode seeds; pairing across candidates is allowed".into(),
            "every counted relation passed independent group verification and every rank was computed over the subgroup scalar field".into(),
            "rank utility, factor coverage, entropy and cost are diagnostics and never trigger a v1 hard prune".into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> PruningPolicy {
        PruningPolicy {
            familywise_alpha: 0.2,
            planned_candidates: 2,
            planned_episodes: 40,
            checkpoint_trials: vec![1_000, 2_000],
            min_trials_for_futility: 1_000,
            min_terminal_episodes: 40,
            min_full_rank_success: 0.0,
            dominance_margin: 0.05,
        }
    }

    fn observation(candidate: &str, episode: usize, success: bool) -> EpisodeObservation {
        let relations = if success { 120 } else { 20 };
        let rank = if success { 100 } else { 20 };
        EpisodeObservation {
            candidate_id: candidate.into(),
            episode_id: format!("seed-{episode:02}"),
            trials: 2_000,
            relation_hit_trials: relations,
            verified_relations: relations,
            relation_cap_per_trial: 1,
            independent_relations: rank,
            initial_rank: 0,
            rank,
            target_rank: 100,
            factor_base_columns: 100,
            covered_columns: if success { 100 } else { 20 },
            reachable_rank_upper: Some(100),
            factor_hits: (0..100).map(|i| u64::from(i < rank as usize)).collect(),
            charged_cost: 20_000,
            terminal: true,
            full_rank_verified: success,
            invalid_relations: 0,
            inconsistent_relations: 0,
            verification_failures: 0,
        }
    }

    #[test]
    fn full_rank_episodes_prune_a_dominated_candidate() {
        let mut observations = Vec::new();
        for i in 0..40 {
            observations.push(observation("winner", i, true));
            observations.push(observation("loser", i, false));
        }
        let report = analyse(&PruningInput {
            schema: INPUT_SCHEMA.into(),
            look: 2,
            policy: policy(),
            observations,
        })
        .unwrap();
        assert_eq!(report.incumbent.as_deref(), Some("winner"));
        assert_eq!(report.candidates[0].candidate_id, "loser");
        assert_eq!(
            report.candidates[0].decision,
            CandidateDecision::PruneDominated
        );
        assert_eq!(report.candidates[1].decision, CandidateDecision::Keep);
    }

    #[test]
    fn in_episode_futility_uses_cap_and_remaining_budget() {
        let mut p = policy();
        p.min_terminal_episodes = 1;
        let mut o = observation("slow", 0, false);
        o.trials = 1_000;
        o.relation_hit_trials = 0;
        o.verified_relations = 0;
        o.independent_relations = 0;
        o.rank = 0;
        o.target_rank = 200;
        o.factor_base_columns = 200;
        o.covered_columns = 0;
        o.reachable_rank_upper = Some(200);
        o.factor_hits = vec![0; 200];
        o.terminal = false;
        let report = analyse(&PruningInput {
            schema: INPUT_SCHEMA.into(),
            look: 1,
            policy: p,
            observations: vec![o],
        })
        .unwrap();
        assert_eq!(
            report.candidates[0].episode_reports[0].decision,
            EpisodeDecision::StopFutile
        );
    }

    #[test]
    fn zero_relation_cap_disables_statistical_futility() {
        let mut p = policy();
        p.min_terminal_episodes = 1;
        let mut o = observation("uncapped", 0, false);
        o.trials = 1_000;
        o.relation_hit_trials = 0;
        o.verified_relations = 0;
        o.relation_cap_per_trial = 0;
        o.independent_relations = 0;
        o.rank = 0;
        o.covered_columns = 0;
        o.factor_hits.fill(0);
        o.terminal = false;
        let report = analyse(&PruningInput {
            schema: INPUT_SCHEMA.into(),
            look: 1,
            policy: p,
            observations: vec![o],
        })
        .unwrap();
        assert_eq!(
            report.candidates[0].episode_reports[0].decision,
            EpisodeDecision::Continue
        );
    }

    #[test]
    fn invalid_relation_fails_closed() {
        let mut p = policy();
        p.min_terminal_episodes = 1;
        let mut o = observation("bad", 0, false);
        o.invalid_relations = 1;
        let report = analyse(&PruningInput {
            schema: INPUT_SCHEMA.into(),
            look: 2,
            policy: p,
            observations: vec![o],
        })
        .unwrap();
        assert_eq!(
            report.candidates[0].decision,
            CandidateDecision::PruneInvalid
        );
    }

    #[test]
    fn factor_hit_vector_must_reproduce_coverage() {
        let mut o = observation("bad-coverage", 0, true);
        o.covered_columns -= 1;
        let error = analyse(&PruningInput {
            schema: INPUT_SCHEMA.into(),
            look: 2,
            policy: policy(),
            observations: vec![o],
        })
        .unwrap_err();
        assert!(error.contains("does not reproduce covered_columns"));
    }

    #[test]
    fn candidate_protocol_fields_must_stay_frozen() {
        let mut a = observation("drifted", 0, true);
        let mut b = observation("drifted", 1, true);
        b.target_rank += 1;
        b.rank += 1;
        b.independent_relations += 1;
        b.verified_relations += 1;
        b.relation_hit_trials += 1;
        b.factor_base_columns += 1;
        b.covered_columns += 1;
        b.factor_hits.push(1);
        a.terminal = true;
        let error = analyse(&PruningInput {
            schema: INPUT_SCHEMA.into(),
            look: 2,
            policy: policy(),
            observations: vec![a, b],
        })
        .unwrap_err();
        assert!(error.contains("differ across episodes"));
    }
}
