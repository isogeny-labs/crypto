//! # Forecasting the degree of regularity while a sweep collects relations.
//!
//! A factor-base parameter sweep (`factor-base-sweep`) decides one Semaev
//! decomposition system per probe target.  Every decided system reports the
//! degree at which degree-bounded F4 resolved it (the *observed solving
//! degree*); every exhausted one reports only the highest Macaulay degree it
//! built without resolving.  The degree of regularity of the cell's systems
//! is what the relation phase is really paying for, and a sweep only learns
//! it for certain once that phase has run to the end.  This module forecasts
//! it from the solves made so far, so the sweep can say at every trial where
//! a cell is heading and a reader can discard a hopeless cell earlier.
//!
//! **It never stops a run.**  The forecast is advisory: it is written next to
//! the measurement, labelled a forecast, and the sweep keeps every candidate
//! on its frozen budget.  `certified_degree_of_regularity` stays `null`
//! wherever it already is; nothing here certifies anything.
//!
//! ## Estimator
//!
//! Each solve is one draw of the random variable `D`, the solving degree of
//! a random target's system in this cell.  A decided solve observes `D`
//! exactly.  An exhausted solve observes only `D > c`, where `c` is the
//! highest degree F4 completed: a right-censored observation.  The
//! Kaplan–Meier product-limit estimator is the nonparametric maximum
//! likelihood estimate of the survival function `S(d) = P(D > d)` under
//! right censoring, and Greenwood's formula gives its standard error.  The
//! **predicted degree of regularity** is the median of that distribution
//! (the smallest `d` with `S(d) ≤ 1/2`), with a band from the `±1.96·se`
//! envelope of `S`.  The same estimator restricted to the decomposable
//! targets forecasts the degree the *relations* cost, which is what the
//! relation phase pays for.
//!
//! A weak prior of [`Forecaster::prior_weight`] pseudo-observations keeps the
//! forecast defined before the first decided solve and vanishes as data
//! accumulate: half its weight sits at the semi-regular degree of the system
//! shape (Bardet–Faugère–Salvy, the degree a system with no exploitable
//! structure reaches), half is spread uniformly over the degree range.  The
//! semi-regular degree is also the yardstick: a cell whose forecast band
//! lies below it is extracting structure; one whose band sits on it is not.
//!
//! ## Cost accounting
//!
//! Per observed solving degree the mean solver work (field multiplications
//! in row reduction, as the sweep counts them) is recorded; unobserved
//! degrees are filled by a log-linear fit when two or more degrees were
//! seen, else by the dense-Macaulay column-count scaling from the nearest
//! observed degree, and the fill is named in the record.  The expected work
//! of one more solve is `Σ_d P(D = d)·w(d) + P(D > bound)·w(exhausted)`.
//! With the relation yield (Jeffreys posterior mean of the fraction of
//! relation-phase solves that added an independent row) and the rows still
//! missing, that gives the projected work of the remaining relation phase
//! and of the whole one (`columns + 1` relations).  The projection charges
//! the data-only distribution once a solve has been observed: the prior's
//! tail mass above the cap, multiplied by an extrapolated exhausted-solve
//! cost, would otherwise dominate the expectation.  Against the generic
//! (rho) floor in group additions, the ratio is reported at 1, 10 and 100
//! field multiplications per group addition, and the advice ranks only
//! where the order holds across that band (AGENTS.md §5a).
//!
//! Linear algebra is charged beside the solver work when the caller reports
//! it ([`Forecaster::observe_linear_algebra`] after every admitted row, or
//! [`Forecaster::set_linear_algebra_total`] once): the work of the `k`-th
//! row is fitted as `a + b·k` (incremental elimination costs more as the
//! rank grows) and summed over the rows still missing and over the whole
//! phase.  Its unit differs from the solver's, so the combined projection
//! prices one row operation at [`Forecaster::set_linear_algebra_price`]
//! solver units (default one), names that assumption, and the advice reads
//! the combined band when linear algebra was observed.
//!
//! ## Advice
//!
//! [`Advice`] is a triage label with its reasons, computed only after
//! [`MIN_SOLVES`] real solves.  `HopelessAtBound` means even the optimistic
//! edge of the band puts the median degree above the sweep's F4 cap: the
//! typical target of this cell cannot be solved at this bound at all.
//! `Doubtful` means the relation phase costs more than the rho floor even
//! with group additions priced at 100 multiplications, or the band shows no
//! structural gain over the semi-regular degree.  `Promising` means the
//! relation phase undercuts the rho floor even with group additions priced
//! at one multiplication.  Everything else is `Undecided`.  None of these
//! labels changes what the sweep does.

use serde::{Deserialize, Serialize};

/// Wire schema of [`Forecast`].
pub const SCHEMA: &str = "dreg_forecast/v1";
/// Real solves (decided or exhausted) before advice is given.
pub const MIN_SOLVES: usize = 4;
/// Two-sided normal quantile for the 95 % band.
pub const Z95: f64 = 1.96;
/// Field multiplications one group addition is priced at, low to high.
pub const PRICE_BAND: [f64; 3] = [1.0, 10.0, 100.0];

/// What one solve of one target's system decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    /// A verified decomposition (a relation candidate).
    Decomposed,
    /// The system was proved inconsistent: no decomposition over the base.
    Refuted,
    /// Degree cap, deadline or resource cap before a verdict.
    Exhausted,
}

/// One solve, as the forecaster sees it.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SolveObservation {
    pub outcome: Outcome,
    /// Observed solving degree of a decided solve.  Ignored when exhausted.
    pub solving_degree: Option<u32>,
    /// For an exhausted solve: the true solving degree is strictly above
    /// this (the highest degree F4 completed).  Ignored when decided.
    pub censored_above: u32,
    /// Solver work of this solve, in the sweep's unit.
    pub work: u64,
    /// Whether this solve belongs to the relation-collecting phase.
    pub relation_phase: bool,
    /// Whether it added an independent relation row (`rank_gain > 0`).
    pub relation: bool,
}

impl SolveObservation {
    /// The censoring point of an exhausted solve: a solve stopped by the
    /// degree cap completed `degree_reached`; one stopped by a deadline may
    /// have been inside degree `degree_reached`, so it is censored one
    /// degree lower.
    pub fn censor_point(degree_reached: u32, timed_out: bool) -> u32 {
        if timed_out {
            degree_reached.saturating_sub(1)
        } else {
            degree_reached
        }
    }
}

/// The shape of the cell's systems, for the semi-regular yardstick.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemShape {
    pub variables: usize,
    /// Total degrees of the equations, field equations excluded.
    pub equation_degrees: Vec<u32>,
    /// Boolean variables with the field equations `x² = x` implied.
    pub boolean: bool,
}

impl SystemShape {
    /// Semi-regular degree of regularity of this shape: the index of the
    /// first non-positive coefficient of `(1 + t)^v / Π (1 + t^{d_i})`
    /// (boolean) or `Π (1 − t^{d_i}) / (1 − t)^v` (a field without field
    /// equations), Bardet–Faugère–Salvy.  `None` when the series stays
    /// positive to the ring's own ceiling.
    pub fn semi_regular_degree(&self) -> Option<u32> {
        if self.boolean {
            return super::ic_descent_degrees::semi_regular_degree(
                self.variables,
                &self.equation_degrees,
            );
        }
        semi_regular_degree_field(self.variables, &self.equation_degrees)
    }
}

/// First non-positive coefficient of `Π (1 − t^{d_i}) / (1 − t)^v`, the
/// semi-regular series over a field whose equations carry no field
/// equations.  The Macaulay bound `Σ (d_i − 1) + 1` caps the search.
pub fn semi_regular_degree_field(n_vars: usize, degrees: &[u32]) -> Option<u32> {
    if n_vars == 0 || n_vars > 512 || degrees.is_empty() {
        return None;
    }
    let top: usize = degrees
        .iter()
        .map(|d| d.saturating_sub(1) as usize)
        .sum::<usize>()
        .saturating_add(2)
        .min(4096);
    // Numerator Π (1 − t^{d_i}), truncated at t^top.
    let mut num = vec![0i128; top + 1];
    num[0] = 1;
    for &d in degrees {
        let d = d as usize;
        if d == 0 || d > top {
            continue;
        }
        for k in (d..=top).rev() {
            num[k] -= num[k - d];
        }
    }
    // Divide by (1 − t)^v: v running prefix sums.
    for _ in 0..n_vars {
        for k in 1..=top {
            num[k] = num[k].saturating_add(num[k - 1]);
        }
    }
    (1..=top).find(|&k| num[k] <= 0).map(|k| k as u32)
}

/// The estimated distribution of the solving degree over one population
/// of targets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DegreeForecast {
    /// Decided solves (exact degrees) in this population.
    pub decided: usize,
    /// Exhausted solves (right-censored) in this population.
    pub censored: usize,
    /// Prior pseudo-observations folded into the estimate.
    pub prior_weight: f64,
    /// `[degree, S(degree) = P(D > degree), Greenwood standard error]` for
    /// every degree up to the bound.
    pub survival: Vec<[f64; 3]>,
    /// Median of the estimated distribution; `None` when it lies above the
    /// bound (then `predicted_degree_label` reads `>bound`).
    pub predicted_degree: Option<u32>,
    pub predicted_degree_label: String,
    /// 95 % band on the median: `[lower, upper]` as labels.
    pub band: [String; 2],
    pub band_lower: Option<u32>,
    pub band_upper: Option<u32>,
    /// 90th percentile of the distribution, same convention.
    pub predicted_degree_p90: Option<u32>,
    /// `S(bound)`: the estimated fraction of targets that need a degree
    /// above the sweep's F4 cap, with its 95 % band.
    pub p_exceeds_bound: f64,
    pub p_exceeds_bound_band: [f64; 2],
    /// Mean of `min(D, bound + 1)` under the estimate.
    pub mean_degree_truncated: Option<f64>,
}

/// Mean solver work per solving degree, observed or filled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkAtDegree {
    pub degree_label: String,
    pub mean_work: f64,
    pub solves: usize,
    /// `observed`, `log-linear fit` or `dense Macaulay extrapolation`.
    pub source: String,
}

/// The relation-phase projection and its price band.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CostProjection {
    pub unit: String,
    pub work_by_degree: Vec<WorkAtDegree>,
    /// Expected work of one more solve under the charged distribution.
    pub expected_work_per_solve: Option<f64>,
    /// Which distribution the expectation charges: the data-only
    /// Kaplan–Meier estimate once a solve was observed, else the prior.
    pub charged_distribution: String,
    pub relation_phase_solves: usize,
    pub relations: u64,
    /// Jeffreys posterior mean `(relations + 1/2) / (solves + 1)`.
    pub relation_yield: f64,
    /// Counting heuristic `N(N+1) / (2·h·r)` for the inventory, or `None`.
    pub heuristic_yield: Option<f64>,
    pub columns: usize,
    pub rank: usize,
    pub remaining_relations: usize,
    pub projected_remaining_solves: Option<f64>,
    pub projected_remaining_work: Option<f64>,
    /// `(columns + 1) / yield` solves at the expected work each.
    pub projected_relation_phase_work: Option<f64>,
    /// The generic floor `√(π r / 2A)` in group additions, if supplied.
    pub rho_floor_group_additions: Option<f64>,
    /// `rho_floor · price / projected_relation_phase_work` at each price of
    /// [`PRICE_BAND`]; above one means the relation phase undercuts rho at
    /// that price.
    pub hope_ratio_band: Vec<[f64; 2]>,
    /// The relation matrix's cost, when the caller reported it.
    pub linear_algebra: Option<LinearAlgebraProjection>,
    /// Solver work plus linear algebra priced into the solver's unit, for
    /// the remaining rows and for the whole phase; `None` without linear
    /// algebra observations.
    pub projected_remaining_work_with_linear_algebra: Option<f64>,
    pub projected_relation_phase_work_with_linear_algebra: Option<f64>,
    /// The hope band on the combined work; empty without linear algebra.
    pub hope_ratio_band_with_linear_algebra: Vec<[f64; 2]>,
    pub scope: String,
}

/// The relation matrix's work: observed so far and projected to the end of
/// the relation phase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinearAlgebraProjection {
    pub unit: String,
    /// Rows admitted so far and the work they cost, cumulative.
    pub observed_rows: usize,
    pub observed_work: u64,
    pub mean_work_per_row: Option<f64>,
    /// Work of the `k`-th row modelled as `intercept + slope·k`.
    pub work_per_row_intercept: f64,
    pub work_per_row_slope: f64,
    /// `least squares over N increments`, `mean over N increments` or
    /// `linear growth through the origin from one total`.
    pub model: String,
    pub projected_remaining_work: Option<f64>,
    /// Work of `columns + 1` rows under the model.
    pub projected_phase_work: Option<f64>,
    /// Solver units one row operation is priced at in the combined totals.
    pub price_in_solver_units: f64,
    pub scope: String,
}

/// Does the solving degree of successive relations drift?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationTrend {
    /// `[relation index from 1, solving degree]` for every relation.
    pub trace: Vec<[u32; 2]>,
    /// Pearson correlation of index and degree, from three relations on.
    pub pearson_r: Option<f64>,
    pub mean_degree_first_half: Option<f64>,
    pub mean_degree_second_half: Option<f64>,
}

/// Advisory triage label.  Never changes what a sweep does.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Advice {
    Promising,
    Undecided,
    Doubtful,
    HopelessAtBound,
}

/// The forecast after some number of solves.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Forecast {
    pub schema: String,
    pub status: String,
    pub solves: usize,
    pub decided: usize,
    pub censored: usize,
    pub relations: u64,
    pub degree_bound: u32,
    pub shape: Option<SystemShape>,
    pub semi_regular_degree: Option<u32>,
    pub all_targets: DegreeForecast,
    pub decomposable_targets: DegreeForecast,
    /// Predicted median minus the semi-regular degree (all targets).
    pub excess_over_semi_regular: Option<i64>,
    /// `Some(true)` when the whole band lies below the semi-regular degree,
    /// `Some(false)` when it lies on or above it, `None` when undecided.
    pub structural_gain: Option<bool>,
    pub cost: CostProjection,
    pub relation_trend: RelationTrend,
    pub advice: Advice,
    pub advice_reasons: Vec<String>,
    pub advisory: String,
    pub certified_degree_of_regularity: Option<u32>,
}

/// Sequential forecaster for one cell of a sweep.
#[derive(Clone, Debug)]
pub struct Forecaster {
    bound: u32,
    shape: Option<SystemShape>,
    /// Pseudo-observations of prior; see the module documentation.
    pub prior_weight: f64,
    observations: Vec<SolveObservation>,
    columns: usize,
    rank: usize,
    rho_floor: Option<f64>,
    heuristic_yield: Option<f64>,
    work_unit: String,
    /// `(rows admitted, cumulative work)` after each admitted row.
    linear_algebra: Vec<(usize, u64)>,
    linear_algebra_unit: String,
    linear_algebra_price: f64,
}

fn label(d: Option<u32>, bound: u32) -> String {
    d.map_or_else(|| format!(">{bound}"), |d| d.to_string())
}

/// Weighted Kaplan–Meier over integer degrees `1..=bound`, with event
/// weights `events[d]` and censor weights `censored[c]` meaning `D > c`.
/// Returns `(S(d), se(d))` for `d = 1..=bound`.
fn kaplan_meier(events: &[f64], censored: &[f64], bound: u32) -> Vec<(f64, f64)> {
    let b = bound as usize;
    let mut out = Vec::with_capacity(b);
    let mut survival = 1.0f64;
    let mut greenwood = 0.0f64;
    for d in 1..=b {
        let at_risk: f64 = events[d..].iter().sum::<f64>() + censored[d..].iter().sum::<f64>();
        let e = events[d];
        if at_risk > 0.0 && e > 0.0 {
            let hazard = (e / at_risk).min(1.0);
            survival *= 1.0 - hazard;
            if at_risk > e {
                greenwood += e / (at_risk * (at_risk - e));
            } else {
                greenwood = f64::INFINITY;
            }
        }
        let se = if survival > 0.0 && greenwood.is_finite() {
            survival * greenwood.sqrt()
        } else {
            0.0
        };
        out.push((survival.clamp(0.0, 1.0), se));
    }
    out
}

/// Smallest degree whose survival (shifted by `shift·se`) is at most `q`.
fn quantile(curve: &[(f64, f64)], q: f64, shift: f64) -> Option<u32> {
    curve
        .iter()
        .enumerate()
        .find(|(_, (s, se))| s + shift * se <= q)
        .map(|(i, _)| i as u32 + 1)
}

fn degree_forecast(
    observations: impl Iterator<Item = SolveObservation>,
    bound: u32,
    prior_weight: f64,
    prior_degree: Option<u32>,
    lowest: u32,
) -> DegreeForecast {
    // Index B+1 is the "> bound" bucket for events; censor weights at c mean D > c.
    let b = bound as usize;
    let mut events = vec![0.0f64; b + 2];
    let mut censored = vec![0.0f64; b + 2];
    let (mut decided, mut n_censored) = (0usize, 0usize);
    for o in observations {
        match o.outcome {
            Outcome::Exhausted => {
                n_censored += 1;
                censored[(o.censored_above as usize).min(b)] += 1.0;
            }
            _ => {
                decided += 1;
                let d = o.solving_degree.unwrap_or(bound + 1) as usize;
                events[d.clamp(1, b + 1)] += 1.0;
            }
        }
    }
    let lo = (lowest.max(1) as usize).min(b + 1);
    if prior_weight > 0.0 {
        let point = prior_degree.map_or(b + 1, |d| (d as usize).clamp(lo, b + 1));
        events[point] += prior_weight / 2.0;
        let span = (b + 1 - lo + 1) as f64;
        for d in lo..=b + 1 {
            events[d] += prior_weight / 2.0 / span;
        }
    }
    let curve = kaplan_meier(&events, &censored, bound + 1);
    let visible: Vec<(f64, f64)> = curve[..b].to_vec();
    let median = quantile(&visible, 0.5, 0.0);
    let lower = quantile(&visible, 0.5, -Z95);
    let upper = quantile(&visible, 0.5, Z95);
    let p90 = quantile(&visible, 0.1, 0.0);
    let (s_bound, se_bound) = if b >= 1 { visible[b - 1] } else { (1.0, 0.0) };
    let mut mean = 0.0;
    let mut previous = 1.0;
    for (i, (s, _)) in visible.iter().enumerate() {
        mean += (previous - s) * (i as f64 + 1.0);
        previous = *s;
    }
    mean += previous * (b as f64 + 1.0);
    DegreeForecast {
        decided,
        censored: n_censored,
        prior_weight,
        survival: visible
            .iter()
            .enumerate()
            .map(|(i, (s, se))| [i as f64 + 1.0, *s, *se])
            .collect(),
        predicted_degree: median,
        predicted_degree_label: label(median, bound),
        band: [label(lower, bound), label(upper, bound)],
        band_lower: lower,
        band_upper: upper,
        predicted_degree_p90: p90,
        p_exceeds_bound: s_bound,
        p_exceeds_bound_band: [
            (s_bound - Z95 * se_bound).clamp(0.0, 1.0),
            (s_bound + Z95 * se_bound).clamp(0.0, 1.0),
        ],
        mean_degree_truncated: (decided + n_censored > 0 || prior_weight > 0.0).then_some(mean),
    }
}

fn binomial_f64(n: f64, k: u32) -> f64 {
    (0..k).fold(1.0f64, |acc, i| acc * (n - i as f64) / (i as f64 + 1.0))
}

impl Forecaster {
    /// A forecaster for a cell whose F4 is capped at `degree_bound`.
    pub fn new(degree_bound: u32) -> Self {
        Self {
            bound: degree_bound.max(1),
            shape: None,
            prior_weight: 1.0,
            observations: Vec::new(),
            columns: 0,
            rank: 0,
            rho_floor: None,
            heuristic_yield: None,
            work_unit: "solver field multiplications".into(),
            linear_algebra: Vec::new(),
            linear_algebra_unit: "relation-matrix row operations".into(),
            linear_algebra_price: 1.0,
        }
    }
    /// Name the unit the relation matrix's work is counted in.
    pub fn set_linear_algebra_unit(&mut self, unit: &str) {
        self.linear_algebra_unit = unit.into();
    }
    /// Solver units one row operation is priced at in the combined totals
    /// (default one).  An assumption, recorded on the projection.
    pub fn set_linear_algebra_price(&mut self, solver_units_per_row_operation: f64) {
        if solver_units_per_row_operation.is_finite() && solver_units_per_row_operation >= 0.0 {
            self.linear_algebra_price = solver_units_per_row_operation;
        }
    }
    /// Record the matrix's cumulative work after its `rows`-th admitted
    /// row.  Call after every row; the per-row fit comes from the increments.
    pub fn observe_linear_algebra(&mut self, rows: usize, cumulative_work: u64) {
        if rows == 0 {
            return;
        }
        if let Some(&(last_rows, last_work)) = self.linear_algebra.last() {
            if rows <= last_rows || cumulative_work < last_work {
                return;
            }
        }
        self.linear_algebra.push((rows, cumulative_work));
    }
    /// Record the matrix's total work for `rows` admitted rows, when the
    /// caller has only the total (a batch elimination after collection).
    pub fn set_linear_algebra_total(&mut self, rows: usize, work: u64, unit: &str) {
        self.linear_algebra.clear();
        self.linear_algebra_unit = unit.into();
        if rows > 0 {
            self.linear_algebra.push((rows, work));
        }
    }
    /// Name the unit `work` is counted in.
    pub fn with_work_unit(mut self, unit: &str) -> Self {
        self.work_unit = unit.into();
        self
    }
    /// Record the shape of the cell's systems.  Later shapes with more
    /// equations or variables replace an earlier one.
    pub fn set_shape(&mut self, shape: SystemShape) {
        let replace = match &self.shape {
            None => true,
            Some(old) => {
                shape.variables > old.variables
                    || shape.equation_degrees.len() > old.equation_degrees.len()
            }
        };
        if replace {
            self.shape = Some(shape);
        }
    }
    pub fn shape(&self) -> Option<&SystemShape> {
        self.shape.as_ref()
    }
    /// Columns of the relation matrix and its current rank.
    pub fn set_relation_state(&mut self, columns: usize, rank: usize) {
        self.columns = columns;
        self.rank = rank.min(columns);
    }
    /// The generic floor `√(π r / 2A)` in group additions, for the hope band.
    pub fn set_rho_floor_group_additions(&mut self, floor: f64) {
        self.rho_floor = floor.is_finite().then_some(floor);
    }
    /// Counting heuristic for the decomposable fraction of two-summand
    /// targets: `N(N+1)/2` sums of `N` inventory points over a group of
    /// `h·r` elements.
    pub fn set_inventory(&mut self, points: usize, r: u64, cofactor: u64) {
        let order = r as f64 * cofactor.max(1) as f64;
        if order > 0.0 && points > 0 {
            let n = points as f64;
            self.heuristic_yield = Some((n * (n + 1.0) / 2.0 / order).min(1.0));
        }
    }
    /// Feed one solve.
    pub fn observe(&mut self, observation: SolveObservation) {
        self.observations.push(observation);
    }
    pub fn observations(&self) -> &[SolveObservation] {
        &self.observations
    }

    fn work_by_degree(&self, all: &DegreeForecast) -> (Vec<WorkAtDegree>, Option<f64>) {
        let b = self.bound as usize;
        let mut sum = vec![0.0f64; b + 2];
        let mut count = vec![0usize; b + 2];
        for o in &self.observations {
            let d = match o.outcome {
                Outcome::Exhausted => b + 1,
                _ => (o.solving_degree.unwrap_or(self.bound + 1) as usize).clamp(1, b + 1),
            };
            sum[d] += o.work as f64;
            count[d] += 1;
        }
        let observed: Vec<(usize, f64)> = (1..=b + 1)
            .filter(|&d| count[d] > 0 && sum[d] > 0.0)
            .map(|d| (d, sum[d] / count[d] as f64))
            .collect();
        if observed.is_empty() {
            return (Vec::new(), None);
        }
        let v = self.shape.as_ref().map_or(1.0, |s| s.variables as f64);
        let distinct_within_bound = observed.iter().filter(|(d, _)| *d <= b).count();
        let fit = (distinct_within_bound >= 2).then(|| {
            let pts: Vec<(f64, f64, f64)> = observed
                .iter()
                .filter(|(d, _)| *d <= b)
                .map(|&(d, m)| (d as f64, m.ln(), count[d] as f64))
                .collect();
            let w: f64 = pts.iter().map(|p| p.2).sum();
            let mx = pts.iter().map(|p| p.0 * p.2).sum::<f64>() / w;
            let my = pts.iter().map(|p| p.1 * p.2).sum::<f64>() / w;
            let sxx: f64 = pts.iter().map(|p| p.2 * (p.0 - mx).powi(2)).sum();
            let sxy: f64 = pts.iter().map(|p| p.2 * (p.0 - mx) * (p.1 - my)).sum();
            let slope = if sxx > 0.0 { (sxy / sxx).max(0.0) } else { 0.0 };
            (my - slope * mx, slope)
        });
        let mut table = Vec::with_capacity(b + 1);
        let mut per_degree = vec![0.0f64; b + 2];
        for d in 1..=b + 1 {
            let entry = if count[d] > 0 && sum[d] > 0.0 {
                WorkAtDegree {
                    degree_label: label((d <= b).then_some(d as u32), self.bound),
                    mean_work: sum[d] / count[d] as f64,
                    solves: count[d],
                    source: "observed".into(),
                }
            } else if let Some((a, slope)) = fit.filter(|_| d <= b) {
                WorkAtDegree {
                    degree_label: label(Some(d as u32), self.bound),
                    mean_work: (a + slope * d as f64).exp(),
                    solves: 0,
                    source: "log-linear fit".into(),
                }
            } else {
                // Nearest observed degree, scaled by the square of the dense
                // Macaulay column count C(v + d, d).
                let (d0, m0) = observed
                    .iter()
                    .min_by_key(|(x, _)| (*x as i64 - d as i64).abs())
                    .copied()
                    .expect("observed is non-empty");
                let target = d.min(b + 1);
                let cols = |k: usize| binomial_f64(v + k as f64, k as u32).max(1.0);
                let ratio = (cols(target) / cols(d0)).powi(2);
                WorkAtDegree {
                    degree_label: label((d <= b).then_some(d as u32), self.bound),
                    mean_work: m0 * ratio,
                    solves: 0,
                    source: "dense Macaulay extrapolation".into(),
                }
            };
            per_degree[d] = entry.mean_work;
            table.push(entry);
        }
        let mut expected = 0.0;
        let mut previous = 1.0;
        for (i, row) in all.survival.iter().enumerate() {
            expected += (previous - row[1]) * per_degree[i + 1];
            previous = row[1];
        }
        expected += previous * per_degree[b + 1];
        (table, Some(expected))
    }

    /// The relation matrix's work so far and its projection to the end of
    /// the phase, from the increments between successive observations.
    fn linear_algebra_projection(&self) -> Option<LinearAlgebraProjection> {
        let points = &self.linear_algebra;
        let &(rows, work) = points.last()?;
        // Increments: the work of each row index between two observations,
        // spread evenly when an observation covers several rows.
        let mut increments: Vec<(f64, f64)> = Vec::new();
        let mut previous = (0usize, 0u64);
        for &(r, w) in points {
            let span = r.saturating_sub(previous.0);
            if span > 0 {
                let each = w.saturating_sub(previous.1) as f64 / span as f64;
                for k in previous.0 + 1..=r {
                    increments.push((k as f64, each));
                }
            }
            previous = (r, w);
        }
        let (intercept, slope, model) = if points.len() == 1 {
            // One total: incremental elimination grows with the rank, so a
            // single total is read as linear growth through the origin.
            let n = rows as f64;
            (
                0.0,
                2.0 * work as f64 / (n * (n + 1.0)),
                "linear growth through the origin from one total".to_string(),
            )
        } else if increments.len() >= 3 {
            let n = increments.len() as f64;
            let mx = increments.iter().map(|p| p.0).sum::<f64>() / n;
            let my = increments.iter().map(|p| p.1).sum::<f64>() / n;
            let sxx: f64 = increments.iter().map(|p| (p.0 - mx).powi(2)).sum();
            let sxy: f64 = increments.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
            let slope = if sxx > 0.0 { (sxy / sxx).max(0.0) } else { 0.0 };
            (
                (my - slope * mx).max(0.0),
                slope,
                format!("least squares over {} increments", increments.len()),
            )
        } else {
            let my = increments.iter().map(|p| p.1).sum::<f64>() / increments.len().max(1) as f64;
            (
                my,
                0.0,
                format!("mean over {} increments", increments.len()),
            )
        };
        let row_work = |k: usize| (intercept + slope * k as f64).max(0.0);
        let sum = |from: usize, to: usize| -> f64 { (from..=to).map(row_work).sum() };
        let remaining = (self.columns > rows).then(|| sum(rows + 1, self.columns));
        Some(LinearAlgebraProjection {
            unit: self.linear_algebra_unit.clone(),
            observed_rows: rows,
            observed_work: work,
            mean_work_per_row: (rows > 0).then(|| work as f64 / rows as f64),
            work_per_row_intercept: intercept,
            work_per_row_slope: slope,
            model,
            projected_remaining_work: remaining.or(Some(0.0)),
            projected_phase_work: Some(sum(1, self.columns + 1)),
            price_in_solver_units: self.linear_algebra_price,
            scope: "work of the k-th admitted row modelled as intercept + slope·k from the observed increments, summed over the rows still missing and over columns + 1 rows; priced into the solver's unit at the recorded price, an assumption, not a measurement".into(),
        })
    }

    /// The forecast from everything observed so far.
    pub fn forecast(&self) -> Forecast {
        let semi_regular = self.shape.as_ref().and_then(|s| s.semi_regular_degree());
        let lowest = self
            .shape
            .as_ref()
            .and_then(|s| s.equation_degrees.iter().copied().min())
            .unwrap_or(2)
            .max(1);
        let all = degree_forecast(
            self.observations.iter().copied(),
            self.bound,
            self.prior_weight,
            semi_regular,
            lowest,
        );
        let decomposable = degree_forecast(
            self.observations
                .iter()
                .copied()
                .filter(|o| o.outcome == Outcome::Decomposed),
            self.bound,
            self.prior_weight,
            semi_regular,
            lowest,
        );
        // The prior shapes the degree forecast; the cost projection charges
        // the observed distribution, so a pseudo-observation's tail above the
        // cap is not priced at an extrapolated exhausted-solve cost.
        let data_only = degree_forecast(
            self.observations.iter().copied(),
            self.bound,
            0.0,
            semi_regular,
            lowest,
        );
        let (charged, charged_distribution) = if self.observations.is_empty() {
            (&all, "prior (no solve observed yet)")
        } else {
            (
                &data_only,
                "data-only Kaplan–Meier estimate (prior excluded)",
            )
        };
        let (work_by_degree, expected_work) = self.work_by_degree(charged);
        let relation_phase_solves = self
            .observations
            .iter()
            .filter(|o| o.relation_phase)
            .count();
        let relations = self
            .observations
            .iter()
            .filter(|o| o.relation_phase && o.relation)
            .count() as u64;
        let relation_yield = (relations as f64 + 0.5) / (relation_phase_solves as f64 + 1.0);
        let remaining = self.columns.saturating_sub(self.rank);
        let remaining_solves = (relation_yield > 0.0).then(|| remaining as f64 / relation_yield);
        let remaining_work = remaining_solves.zip(expected_work).map(|(s, w)| s * w);
        let phase_work = expected_work
            .map(|w| (self.columns as f64 + 1.0) / relation_yield * w)
            .filter(|w| w.is_finite());
        let band = |work: Option<f64>| -> Vec<[f64; 2]> {
            match (self.rho_floor, work) {
                (Some(rho), Some(work)) if work > 0.0 => PRICE_BAND
                    .iter()
                    .map(|&price| [price, rho * price / work])
                    .collect(),
                _ => Vec::new(),
            }
        };
        let hope = band(phase_work);
        let linear_algebra = self.linear_algebra_projection();
        let la_price = self.linear_algebra_price;
        let remaining_with_la = linear_algebra
            .as_ref()
            .and_then(|la| Some(remaining_work? + la_price * la.projected_remaining_work?));
        let phase_with_la = linear_algebra
            .as_ref()
            .and_then(|la| Some(phase_work? + la_price * la.projected_phase_work?));
        let hope_with_la = band(phase_with_la);
        let cost = CostProjection {
            unit: self.work_unit.clone(),
            work_by_degree,
            expected_work_per_solve: expected_work,
            charged_distribution: charged_distribution.into(),
            relation_phase_solves,
            relations,
            relation_yield,
            heuristic_yield: self.heuristic_yield,
            columns: self.columns,
            rank: self.rank,
            remaining_relations: remaining,
            projected_remaining_solves: remaining_solves,
            projected_remaining_work: remaining_work,
            projected_relation_phase_work: phase_work,
            rho_floor_group_additions: self.rho_floor,
            hope_ratio_band: hope.clone(),
            linear_algebra,
            projected_remaining_work_with_linear_algebra: remaining_with_la,
            projected_relation_phase_work_with_linear_algebra: phase_with_la,
            hope_ratio_band_with_linear_algebra: hope_with_la.clone(),
            scope: "projection from observed solver work and relation yield at the forecast degree distribution; linear algebra charged from its own observations at the recorded price when reported, else excluded; system construction, inventory and verification excluded; rho floor in group additions priced at 1, 10 and 100 multiplications; provisional, not a §5a comparison or a speedup claim".into(),
        };
        // Advice reads the combined band when linear algebra was observed.
        let (hope, hope_basis) = if hope_with_la.is_empty() {
            (hope, "solver work only")
        } else {
            (hope_with_la, "solver work plus priced linear algebra")
        };
        let mut trace = Vec::new();
        for o in self
            .observations
            .iter()
            .filter(|o| o.relation_phase && o.relation)
        {
            if let Some(d) = o.solving_degree {
                trace.push([trace.len() as u32 + 1, d]);
            }
        }
        let relation_trend = trend(trace);
        let excess = all
            .predicted_degree
            .zip(semi_regular)
            .map(|(d, s)| d as i64 - s as i64);
        let structural_gain = semi_regular.and_then(|s| match (all.band_lower, all.band_upper) {
            (_, Some(upper)) if upper < s => Some(true),
            (Some(lower), _) if lower >= s => Some(false),
            (None, _) => Some(false),
            _ => None,
        });
        let real = self.observations.len();
        let mut reasons = Vec::new();
        let advice = if real < MIN_SOLVES {
            reasons.push(format!("{real} solves; advice needs {MIN_SOLVES}"));
            Advice::Undecided
        } else if all.band_lower.is_none() {
            reasons.push(format!(
                "even the optimistic edge of the band puts the median solving degree above the F4 cap {}",
                self.bound
            ));
            Advice::HopelessAtBound
        } else {
            let low = hope.first().map(|h| h[1]);
            let high = hope.last().map(|h| h[1]);
            if high.is_some_and(|h| h < 1.0) {
                reasons.push(format!("projected relation-phase work ({hope_basis}) exceeds the rho floor even with group additions priced at 100 multiplications"));
                Advice::Doubtful
            } else if structural_gain == Some(false) {
                reasons.push(format!(
                    "forecast band [{}, {}] shows no gain over the semi-regular degree {}",
                    all.band[0],
                    all.band[1],
                    semi_regular.map_or_else(|| "?".into(), |s| s.to_string())
                ));
                Advice::Doubtful
            } else if low.is_some_and(|h| h > 1.0) {
                reasons.push(format!("projected relation-phase work ({hope_basis}) undercuts the rho floor even with group additions priced at one multiplication"));
                Advice::Promising
            } else {
                reasons.push("order against the rho floor changes across the price band, or no floor was supplied".into());
                Advice::Undecided
            }
        };
        Forecast {
            schema: SCHEMA.into(),
            status: "forecast of the observed solving-degree distribution; not a certified degree of regularity".into(),
            solves: real,
            decided: all.decided,
            censored: all.censored,
            relations,
            degree_bound: self.bound,
            shape: self.shape.clone(),
            semi_regular_degree: semi_regular,
            all_targets: all,
            decomposable_targets: decomposable,
            excess_over_semi_regular: excess,
            structural_gain,
            cost,
            relation_trend,
            advice,
            advice_reasons: reasons,
            advisory: "advisory triage only; the sweep never stops, skips or shortens a candidate on this forecast".into(),
            certified_degree_of_regularity: None,
        }
    }
}

/// Structural equality of two JSON values with numbers compared to a
/// relative tolerance.  A forecast written to a file and parsed back can
/// differ from the original in the last bit of a float (serde_json parses
/// floats without the `float_roundtrip` feature), so a replay is checked
/// with this rather than `==`.
pub fn json_approx_eq(a: &serde_json::Value, b: &serde_json::Value, relative: f64) -> bool {
    use serde_json::Value;
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => match (x.as_f64(), y.as_f64()) {
            (Some(x), Some(y)) => x == y || (x - y).abs() <= relative * x.abs().max(y.abs()),
            _ => x == y,
        },
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| json_approx_eq(p, q, relative))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, p)| y.get(k).is_some_and(|q| json_approx_eq(p, q, relative)))
        }
        _ => a == b,
    }
}

fn trend(trace: Vec<[u32; 2]>) -> RelationTrend {
    let n = trace.len();
    let mean = |xs: &[[u32; 2]]| {
        (!xs.is_empty()).then(|| xs.iter().map(|p| p[1] as f64).sum::<f64>() / xs.len() as f64)
    };
    let pearson = (n >= 3).then(|| {
        let mx = trace.iter().map(|p| p[0] as f64).sum::<f64>() / n as f64;
        let my = trace.iter().map(|p| p[1] as f64).sum::<f64>() / n as f64;
        let sxx: f64 = trace.iter().map(|p| (p[0] as f64 - mx).powi(2)).sum();
        let syy: f64 = trace.iter().map(|p| (p[1] as f64 - my).powi(2)).sum();
        let sxy: f64 = trace
            .iter()
            .map(|p| (p[0] as f64 - mx) * (p[1] as f64 - my))
            .sum();
        (sxx > 0.0 && syy > 0.0).then(|| sxy / (sxx * syy).sqrt())
    });
    RelationTrend {
        mean_degree_first_half: mean(&trace[..n / 2]),
        mean_degree_second_half: mean(&trace[n / 2..]),
        pearson_r: pearson.flatten(),
        trace,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decided(outcome: Outcome, degree: u32, work: u64, relation: bool) -> SolveObservation {
        SolveObservation {
            outcome,
            solving_degree: Some(degree),
            censored_above: 0,
            work,
            relation_phase: true,
            relation,
        }
    }
    fn exhausted(censored_above: u32, work: u64) -> SolveObservation {
        SolveObservation {
            outcome: Outcome::Exhausted,
            solving_degree: None,
            censored_above,
            work,
            relation_phase: true,
            relation: false,
        }
    }
    fn quadratic_shape(vars: usize, boolean: bool) -> SystemShape {
        SystemShape {
            variables: vars,
            equation_degrees: vec![2; vars],
            boolean,
        }
    }

    #[test]
    fn semi_regular_field_series_matches_the_macaulay_bound_for_regular_systems() {
        // n generic quadrics in n variables: D_reg = n + 1.
        for n in 1..8usize {
            assert_eq!(
                semi_regular_degree_field(n, &vec![2; n]),
                Some(n as u32 + 1)
            );
        }
        // Two quadrics and a cubic in three variables: Macaulay bound 1+1+2+1 = 5.
        assert_eq!(semi_regular_degree_field(3, &[2, 2, 3]), Some(5));
        // Overdetermined: more quadrics than variables fall earlier.
        assert!(semi_regular_degree_field(4, &[2; 8]).unwrap() < 5);
        assert_eq!(semi_regular_degree_field(0, &[2]), None);
        assert_eq!(semi_regular_degree_field(3, &[]), None);
    }

    #[test]
    fn boolean_shape_uses_the_boolean_series() {
        let s = quadratic_shape(6, true);
        assert_eq!(
            s.semi_regular_degree(),
            super::super::ic_descent_degrees::semi_regular_degree(6, &[2; 6])
        );
        assert!(s.semi_regular_degree().is_some());
    }

    #[test]
    fn without_censoring_or_prior_the_estimate_is_the_empirical_distribution() {
        let mut f = Forecaster::new(8);
        f.prior_weight = 0.0;
        for d in [3, 3, 4, 4, 4, 5, 6, 6] {
            f.observe(decided(Outcome::Decomposed, d, 100, true));
        }
        let fc = f.forecast();
        let s = &fc.all_targets.survival;
        assert_eq!(s.len(), 8);
        assert!((s[2][1] - 6.0 / 8.0).abs() < 1e-12, "S(3) = {}", s[2][1]);
        assert!((s[3][1] - 3.0 / 8.0).abs() < 1e-12);
        assert!((s[4][1] - 2.0 / 8.0).abs() < 1e-12);
        assert!(s[5][1].abs() < 1e-12);
        assert_eq!(fc.all_targets.predicted_degree, Some(4));
        assert_eq!(fc.all_targets.predicted_degree_p90, Some(6));
        assert_eq!(fc.all_targets.p_exceeds_bound, 0.0);
        assert!((fc.all_targets.mean_degree_truncated.unwrap() - 35.0 / 8.0).abs() < 1e-12);
        assert_eq!(fc.relations, 8);
    }

    #[test]
    fn censored_solves_raise_the_survival_above_the_cap() {
        let mut f = Forecaster::new(6);
        f.prior_weight = 0.0;
        for d in [3, 4] {
            f.observe(decided(Outcome::Refuted, d, 50, false));
        }
        for _ in 0..14 {
            f.observe(exhausted(6, 5000));
        }
        let fc = f.forecast();
        // Two of sixteen decided below the cap; fourteen censored at the cap.
        assert!((fc.all_targets.p_exceeds_bound - 0.875).abs() < 1e-12);
        assert_eq!(fc.all_targets.predicted_degree, None);
        assert_eq!(fc.all_targets.predicted_degree_label, ">6");
        assert_eq!(
            fc.all_targets.band_lower, None,
            "{:?}",
            fc.all_targets.survival
        );
        assert_eq!(fc.all_targets.censored, 14);
        assert_eq!(fc.advice, Advice::HopelessAtBound);
        // A deadline inside degree 6 censors one lower than a completed cap.
        assert_eq!(SolveObservation::censor_point(6, true), 5);
        assert_eq!(SolveObservation::censor_point(6, false), 6);
    }

    #[test]
    fn censoring_below_the_cap_keeps_the_mass_at_risk_for_later_degrees() {
        // Kaplan–Meier: a solve censored above 3 is at risk at degrees ≤ 3
        // and leaves afterwards, so S(4) is estimated from the others only.
        let mut f = Forecaster::new(8);
        f.prior_weight = 0.0;
        f.observe(decided(Outcome::Decomposed, 4, 1, true));
        f.observe(decided(Outcome::Decomposed, 5, 1, true));
        f.observe(exhausted(3, 1));
        let s = f.forecast().all_targets.survival;
        assert!((s[2][1] - 1.0).abs() < 1e-12);
        assert!((s[3][1] - 0.5).abs() < 1e-12);
        assert!(s[4][1].abs() < 1e-12);
    }

    #[test]
    fn the_prior_sits_at_the_semi_regular_degree_and_washes_out() {
        let mut f = Forecaster::new(8);
        f.set_shape(quadratic_shape(2, false)); // semi-regular degree 3
        let empty = f.forecast();
        assert_eq!(empty.semi_regular_degree, Some(3));
        assert_eq!(empty.all_targets.predicted_degree, Some(3));
        assert_eq!(empty.advice, Advice::Undecided);
        assert_eq!(empty.solves, 0);
        for _ in 0..40 {
            f.observe(decided(Outcome::Decomposed, 6, 10, true));
        }
        let full = f.forecast();
        assert_eq!(full.all_targets.predicted_degree, Some(6));
        assert_eq!(full.band_of_all(), (Some(6), Some(6)));
        assert_eq!(full.excess_over_semi_regular, Some(3));
        assert_eq!(full.structural_gain, Some(false));
    }

    impl Forecast {
        fn band_of_all(&self) -> (Option<u32>, Option<u32>) {
            (self.all_targets.band_lower, self.all_targets.band_upper)
        }
    }

    #[test]
    fn the_band_narrows_as_relations_accumulate() {
        let mut f = Forecaster::new(8);
        f.set_shape(quadratic_shape(4, true));
        let mut widths = Vec::new();
        for k in 0..64 {
            f.observe(decided(Outcome::Decomposed, 3 + (k % 2), 10, true));
            if [3, 7, 15, 31, 63].contains(&k) {
                let fc = f.forecast();
                let w = fc.all_targets.band_upper.map_or(9, |u| u as i64)
                    - fc.all_targets.band_lower.map_or(0, |l| l as i64);
                widths.push(w);
            }
        }
        assert!(widths.windows(2).all(|w| w[1] <= w[0]), "{widths:?}");
        assert_eq!(*widths.last().unwrap(), 1);
    }

    #[test]
    fn cost_projection_charges_the_forecast_degrees_and_prices_the_band() {
        let mut f = Forecaster::new(6);
        f.prior_weight = 0.0;
        f.set_shape(quadratic_shape(4, false)); // semi-regular degree 5
        f.set_relation_state(10, 2);
        f.set_rho_floor_group_additions(1.0e6);
        f.set_inventory(8, 1009, 2);
        // Half the solves resolve at degree 3 for 100 units, half at degree 4
        // for 1000 units; two of four add a row.
        f.observe(decided(Outcome::Decomposed, 3, 100, true));
        f.observe(decided(Outcome::Refuted, 3, 100, false));
        f.observe(decided(Outcome::Decomposed, 4, 1000, true));
        f.observe(decided(Outcome::Refuted, 4, 1000, false));
        let fc = f.forecast();
        let c = &fc.cost;
        assert!((c.expected_work_per_solve.unwrap() - 550.0).abs() < 1e-9);
        assert!((c.relation_yield - 2.5 / 5.0).abs() < 1e-12);
        assert_eq!(c.remaining_relations, 8);
        assert!((c.projected_remaining_solves.unwrap() - 16.0).abs() < 1e-9);
        assert!((c.projected_remaining_work.unwrap() - 8800.0).abs() < 1e-6);
        assert!((c.projected_relation_phase_work.unwrap() - 22.0 * 550.0).abs() < 1e-6);
        assert_eq!(c.hope_ratio_band.len(), 3);
        assert!((c.hope_ratio_band[0][1] - 1.0e6 / 12100.0).abs() < 1e-9);
        assert!(c.hope_ratio_band[2][1] > c.hope_ratio_band[1][1]);
        assert!((c.heuristic_yield.unwrap() - 36.0 / 2018.0).abs() < 1e-12);
        let observed: Vec<_> = c
            .work_by_degree
            .iter()
            .filter(|w| w.source == "observed")
            .map(|w| w.degree_label.clone())
            .collect();
        assert_eq!(observed, ["3", "4"]);
        let fitted = c
            .work_by_degree
            .iter()
            .find(|w| w.degree_label == "5")
            .unwrap();
        assert_eq!(fitted.source, "log-linear fit");
        assert!(
            (fitted.mean_work - 10_000.0).abs() < 1e-6,
            "{}",
            fitted.mean_work
        );
        assert!(c.work_by_degree.iter().any(|w| w.degree_label == ">6"));
        assert_eq!(fc.semi_regular_degree, Some(5));
        assert_eq!(fc.structural_gain, Some(true));
        assert_eq!(fc.advice, Advice::Promising);
        // The same cell at the semi-regular degree is doubtful, rho or not.
        let mut flat = Forecaster::new(6);
        flat.prior_weight = 0.0;
        flat.set_shape(quadratic_shape(4, true)); // semi-regular degree 3
        flat.set_relation_state(10, 2);
        flat.set_rho_floor_group_additions(1.0e6);
        for o in f.observations() {
            flat.observe(*o);
        }
        let flat = flat.forecast();
        assert_eq!(flat.semi_regular_degree, Some(3));
        assert_eq!(flat.structural_gain, Some(false));
        assert_eq!(flat.advice, Advice::Doubtful);
    }

    #[test]
    fn the_prior_tail_is_not_charged_once_a_solve_was_observed() {
        let mut f = Forecaster::new(8);
        f.set_shape(quadratic_shape(4, true));
        assert_eq!(
            f.forecast().cost.charged_distribution,
            "prior (no solve observed yet)"
        );
        for _ in 0..16 {
            f.observe(decided(Outcome::Decomposed, 2, 400, true));
        }
        let fc = f.forecast();
        // The prior still leaves mass above the cap in the degree forecast …
        assert!(fc.all_targets.p_exceeds_bound > 0.0);
        // … but the expected work is the observed mean, not an extrapolated tail.
        assert!((fc.cost.expected_work_per_solve.unwrap() - 400.0).abs() < 1e-9);
        assert!(fc.cost.charged_distribution.contains("prior excluded"));
    }

    #[test]
    fn linear_algebra_is_charged_from_its_increments_and_enters_the_combined_band() {
        let mut f = Forecaster::new(6);
        f.prior_weight = 0.0;
        f.set_shape(quadratic_shape(4, false));
        f.set_relation_state(10, 4);
        f.set_rho_floor_group_additions(1.0e6);
        for k in 0..4usize {
            f.observe(decided(Outcome::Decomposed, 3, 100, true));
            // Row k costs 10·k units: cumulative 10, 30, 60, 100.
            f.observe_linear_algebra(k + 1, (10 * (k + 1) * (k + 2) / 2) as u64);
        }
        let fc = f.forecast();
        let la = fc
            .cost
            .linear_algebra
            .as_ref()
            .expect("linear algebra observed");
        assert_eq!(la.observed_rows, 4);
        assert_eq!(la.observed_work, 100);
        assert!(la.model.starts_with("least squares"), "{}", la.model);
        assert!(
            (la.work_per_row_slope - 10.0).abs() < 1e-9,
            "{}",
            la.work_per_row_slope
        );
        assert!(la.work_per_row_intercept.abs() < 1e-9);
        // Rows 5..=10 at 10·k: 10·(5+…+10) = 450; rows 1..=11: 10·66 = 660.
        assert!((la.projected_remaining_work.unwrap() - 450.0).abs() < 1e-6);
        assert!((la.projected_phase_work.unwrap() - 660.0).abs() < 1e-6);
        // Solver: yield 4.5/5 = 0.9, 100 per solve: phase 11/0.9·100; the
        // combined totals add the priced rows.
        let solver_phase = fc.cost.projected_relation_phase_work.unwrap();
        assert!((solver_phase - 11.0 / 0.9 * 100.0).abs() < 1e-6);
        assert!(
            (fc.cost
                .projected_relation_phase_work_with_linear_algebra
                .unwrap()
                - (solver_phase + 660.0))
                .abs()
                < 1e-6
        );
        assert!(
            (fc.cost
                .projected_remaining_work_with_linear_algebra
                .unwrap()
                - (fc.cost.projected_remaining_work.unwrap() + 450.0))
                .abs()
                < 1e-6
        );
        assert_eq!(fc.cost.hope_ratio_band_with_linear_algebra.len(), 3);
        assert!(fc.cost.hope_ratio_band_with_linear_algebra[0][1] < fc.cost.hope_ratio_band[0][1]);
        assert!(fc.advice_reasons[0].contains("priced linear algebra"));
        // A price changes only the combined totals.
        f.set_linear_algebra_price(0.5);
        let half = f.forecast();
        assert!(
            (half
                .cost
                .projected_relation_phase_work_with_linear_algebra
                .unwrap()
                - (solver_phase + 330.0))
                .abs()
                < 1e-6
        );
        assert_eq!(
            half.cost.projected_relation_phase_work,
            fc.cost.projected_relation_phase_work
        );
        // Out-of-order or regressing observations are ignored.
        f.observe_linear_algebra(3, 1);
        f.observe_linear_algebra(4, 50);
        assert_eq!(f.forecast().cost.linear_algebra.unwrap().observed_work, 100);
    }

    #[test]
    fn one_linear_algebra_total_reads_as_linear_growth_and_none_leaves_the_band_alone() {
        let mut f = Forecaster::new(6);
        f.prior_weight = 0.0;
        f.set_relation_state(10, 5);
        f.set_rho_floor_group_additions(1.0e6);
        for _ in 0..5 {
            f.observe(decided(Outcome::Decomposed, 3, 100, true));
        }
        let bare = f.forecast();
        assert!(bare.cost.linear_algebra.is_none());
        assert!(bare.cost.hope_ratio_band_with_linear_algebra.is_empty());
        assert!(bare
            .cost
            .projected_relation_phase_work_with_linear_algebra
            .is_none());
        assert!(bare.advice_reasons[0].contains("solver work only"));
        // 5 rows cost 150 in total: slope 2·150/(5·6) = 10, through the origin.
        f.set_linear_algebra_total(5, 150, "multiply-subtracts mod r");
        let fc = f.forecast();
        let la = fc.cost.linear_algebra.as_ref().unwrap();
        assert_eq!(la.unit, "multiply-subtracts mod r");
        assert!(la.model.contains("one total"));
        assert!((la.work_per_row_slope - 10.0).abs() < 1e-9);
        assert!(
            (la.projected_remaining_work.unwrap() - 10.0 * (6 + 7 + 8 + 9 + 10) as f64).abs()
                < 1e-6
        );
        assert!((la.projected_phase_work.unwrap() - 660.0).abs() < 1e-6);
    }

    #[test]
    fn a_single_observed_degree_extrapolates_by_the_dense_macaulay_scaling() {
        let mut f = Forecaster::new(5);
        f.prior_weight = 0.0;
        f.set_shape(quadratic_shape(3, false));
        for _ in 0..4 {
            f.observe(decided(Outcome::Decomposed, 3, 100, true));
        }
        let fc = f.forecast();
        let at = |d: &str| {
            fc.cost
                .work_by_degree
                .iter()
                .find(|w| w.degree_label == d)
                .unwrap()
                .clone()
        };
        assert_eq!(at("4").source, "dense Macaulay extrapolation");
        // C(3+4,4)/C(3+3,3) = 35/20 → squared 3.0625.
        assert!((at("4").mean_work - 306.25).abs() < 1e-9);
        assert!(at("2").mean_work < 100.0);
        assert!(at(">5").mean_work > at("5").mean_work);
    }

    #[test]
    fn doubtful_when_rho_wins_across_the_band_and_undecided_in_between() {
        let mut f = Forecaster::new(8);
        f.prior_weight = 0.0;
        f.set_relation_state(100, 0);
        f.set_rho_floor_group_additions(10.0);
        for _ in 0..4 {
            f.observe(decided(Outcome::Decomposed, 3, 1_000_000, true));
        }
        let fc = f.forecast();
        assert_eq!(fc.advice, Advice::Doubtful);
        assert!(fc.advice_reasons[0].contains("100 multiplications"));
        // Floor between the band edges: undecided.
        f.set_rho_floor_group_additions(fc.cost.projected_relation_phase_work.unwrap() / 10.0);
        assert_eq!(f.forecast().advice, Advice::Undecided);
        // No floor supplied and no shape: undecided too.
        let mut bare = Forecaster::new(8);
        for _ in 0..4 {
            bare.observe(decided(Outcome::Decomposed, 3, 1, true));
        }
        assert_eq!(bare.forecast().advice, Advice::Undecided);
    }

    #[test]
    fn relation_trend_tracks_the_degree_of_successive_relations() {
        let mut f = Forecaster::new(8);
        for (k, d) in [2u32, 2, 3, 3, 4, 4].iter().enumerate() {
            f.observe(decided(Outcome::Decomposed, *d, 1, true));
            if k == 2 {
                f.observe(decided(Outcome::Refuted, 2, 1, false));
                f.observe(exhausted(8, 1));
            }
        }
        let t = f.forecast().relation_trend;
        assert_eq!(t.trace.len(), 6);
        assert_eq!(t.trace[0], [1, 2]);
        assert_eq!(t.trace[5], [6, 4]);
        assert!(t.pearson_r.unwrap() > 0.9);
        assert!(t.mean_degree_second_half.unwrap() > t.mean_degree_first_half.unwrap());
        let decomposable = f.forecast().decomposable_targets;
        assert_eq!(decomposable.decided, 6);
        assert_eq!(decomposable.censored, 0);
    }

    #[test]
    fn shapes_only_grow_and_the_forecast_round_trips_through_json() {
        let mut f = Forecaster::new(8);
        f.set_shape(quadratic_shape(4, true));
        f.set_shape(quadratic_shape(2, true));
        assert_eq!(f.shape().unwrap().variables, 4);
        f.observe(exhausted(8, 7));
        let fc = f.forecast();
        let text = serde_json::to_string(&fc).unwrap();
        let back: Forecast = serde_json::from_str(&text).unwrap();
        assert_eq!(back.schema, SCHEMA);
        assert_eq!(back.all_targets.band, fc.all_targets.band);
        assert_eq!(back.cost.work_by_degree.len(), fc.cost.work_by_degree.len());
        assert!(back.certified_degree_of_regularity.is_none());
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert!(json_approx_eq(
            &serde_json::to_value(&fc).unwrap(),
            &parsed,
            1e-12
        ));
        let mut other = parsed.clone();
        other["degree_bound"] = serde_json::json!(9);
        assert!(!json_approx_eq(&parsed, &other, 1e-12));
        other = parsed.clone();
        other["all_targets"]["p_exceeds_bound"] = serde_json::json!(0.25);
        assert!(!json_approx_eq(&parsed, &other, 1e-12));
    }
}
