//! A Tree-structured Parzen Estimator over a conditional search space.
//!
//! The same idea as Optuna's default sampler, small enough to own: split the
//! finished trials into a "good" fraction `l` and the rest `g`, model each
//! parameter independently in each group, draw candidates from `l` and keep
//! the one with the best `l(x) / g(x)`.  Conditional parameters are modelled
//! only from the trials in which they existed, which is how a categorical
//! choice (the solver, say) gets its own parameters (its degree cap) without
//! the others polluting them.
//!
//! No external dependency, and no clock: the same seed and the same history
//! give the same suggestion, which is what lets a study be resumed and
//! replayed.

use std::cmp::Ordering;

use serde_json::Value;

use super::space::{Assignment, Domain, Param, Space};

/// SplitMix64: tiny, fast, and the same on every host.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed ^ 0x9E37_79B9_7F4A_7C15)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `[0, 1)`.
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Uniform in `0..n`.
    pub fn below(&mut self, n: usize) -> usize {
        ((self.f64() * n as f64) as usize).min(n.saturating_sub(1))
    }
    /// Standard normal (Box–Muller).
    pub fn normal(&mut self) -> f64 {
        let u1 = self.f64().max(1e-300);
        let u2 = self.f64();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Complete,
    /// Stopped early because it was worse than the median at its rung.
    Pruned,
    /// Never produced a score: refused, exhausted, crashed, or still running
    /// (a pending trial is lied about as failed, so a parallel study does not
    /// propose the same point twice).
    Failed,
}

#[derive(Clone, Debug)]
pub struct Obs {
    pub assignment: Assignment,
    pub state: State,
    /// Lower is better.  For `Pruned` the value at the rung it stopped on.
    pub value: f64,
    /// Rungs a trial finished before it ended.
    pub rungs_done: usize,
}

/// The order TPE ranks observations in: completed trials by value, then
/// pruned trials (the further they got the better), then failures.
fn rank(a: &Obs, b: &Obs) -> Ordering {
    let class = |o: &Obs| match o.state {
        State::Complete => 0,
        State::Pruned => 1,
        State::Failed => 2,
    };
    class(a)
        .cmp(&class(b))
        .then_with(|| match a.state {
            State::Pruned => b.rungs_done.cmp(&a.rungs_done),
            _ => Ordering::Equal,
        })
        .then_with(|| match a.state {
            State::Failed => Ordering::Equal,
            _ => a.value.total_cmp(&b.value),
        })
}

#[derive(Clone, Debug)]
pub struct TpeOptions {
    /// Trials chosen from the prior before the model is used.
    pub startup: usize,
    /// Candidates drawn from `l` per parameter.
    pub candidates: usize,
    /// The good fraction.  The defaults here (`gamma` 0.10, `explore` 0.15)
    /// were chosen from the `tune_defaults` benchmark: a wider good fraction
    /// was worse on every problem in it.
    pub gamma: f64,
    pub gamma_cap: usize,
    /// Weight of the prior component (and of the categorical prior).
    pub prior_weight: f64,
    /// Probability that a suggestion ignores the model and is drawn whole
    /// from the prior.  Independent per-parameter models judge a categorical
    /// choice by how often it appears among the best few trials, so a branch
    /// whose good region is narrow looks bad on average and is starved of
    /// the trials that would show it is the best one.  A steady trickle of
    /// random points keeps every branch sampled.
    pub explore: f64,
}

impl Default for TpeOptions {
    fn default() -> Self {
        TpeOptions {
            startup: 10,
            candidates: 24,
            gamma: 0.10,
            gamma_cap: 25,
            prior_weight: 1.0,
            explore: 0.15,
        }
    }
}

// ── The unit interval ───────────────────────────────────────────────

fn to_unit(domain: &Domain, v: f64) -> f64 {
    let (lo, hi, log) = match domain {
        Domain::Int { lo, hi, log } => (*lo as f64, *hi as f64, *log),
        Domain::Float { lo, hi, log } => (*lo, *hi, *log),
        Domain::Cat(_) => return 0.0,
    };
    if hi <= lo {
        return 0.5;
    }
    let u = if log {
        (v.max(lo).ln() - lo.ln()) / (hi.ln() - lo.ln())
    } else {
        (v - lo) / (hi - lo)
    };
    u.clamp(0.0, 1.0)
}

fn from_unit(domain: &Domain, u: f64) -> Value {
    let u = u.clamp(0.0, 1.0);
    match domain {
        Domain::Int { lo, hi, log } => {
            let (l, h) = (*lo as f64, *hi as f64);
            let v = if *log {
                (l.ln() + u * (h.ln() - l.ln())).exp()
            } else {
                l + u * (h - l)
            };
            Value::from((v.round() as i64).clamp(*lo, *hi))
        }
        Domain::Float { lo, hi, log } => {
            let v = if *log {
                (lo.ln() + u * (hi.ln() - lo.ln())).exp()
            } else {
                lo + u * (hi - lo)
            };
            Value::from(v.clamp(*lo, *hi))
        }
        Domain::Cat(values) => Value::from(values[0].clone()),
    }
}

// ── Parzen mixtures on [0, 1] ───────────────────────────────────────

fn phi(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Abramowitz & Stegun 7.1.26, error below 1.5e-7.
fn erf(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    sign * (1.0 - poly * (-x * x).exp())
}

struct Mixture {
    mu: Vec<f64>,
    sigma: Vec<f64>,
    weight: Vec<f64>,
    /// Truncation to [0, 1], per component.
    norm: Vec<f64>,
}

impl Mixture {
    /// One Gaussian per observation plus a broad prior component.  Each
    /// bandwidth is the larger gap to a neighbour, clipped, as in Hyperopt.
    fn new(points: &[f64], prior_weight: f64) -> Mixture {
        let mut mus: Vec<(f64, bool)> = points.iter().map(|&p| (p, false)).collect();
        mus.push((0.5, true));
        mus.sort_by(|a, b| a.0.total_cmp(&b.0));
        let n = mus.len();
        let min_sigma = 1.0 / (1 + n).min(100) as f64;
        let mut mu = Vec::with_capacity(n);
        let mut sigma = Vec::with_capacity(n);
        let mut weight = Vec::with_capacity(n);
        for k in 0..n {
            let left = if k == 0 {
                mus[k].0
            } else {
                mus[k].0 - mus[k - 1].0
            };
            let right = if k + 1 == n {
                1.0 - mus[k].0
            } else {
                mus[k + 1].0 - mus[k].0
            };
            let (s, w) = if mus[k].1 {
                (1.0, prior_weight)
            } else {
                (left.max(right).clamp(min_sigma, 1.0), 1.0)
            };
            mu.push(mus[k].0);
            sigma.push(s);
            weight.push(w);
        }
        let total: f64 = weight.iter().sum();
        weight.iter_mut().for_each(|w| *w /= total);
        let norm = mu
            .iter()
            .zip(&sigma)
            .map(|(&m, &s)| (phi((1.0 - m) / s) - phi((0.0 - m) / s)).max(1e-12))
            .collect();
        Mixture {
            mu,
            sigma,
            weight,
            norm,
        }
    }

    fn log_pdf(&self, x: f64) -> f64 {
        let mut total = 0.0;
        for k in 0..self.mu.len() {
            let z = (x - self.mu[k]) / self.sigma[k];
            total += self.weight[k] * (-0.5 * z * z).exp()
                / (self.sigma[k] * (2.0 * std::f64::consts::PI).sqrt())
                / self.norm[k];
        }
        total.max(1e-300).ln()
    }

    fn sample(&self, rng: &mut Rng) -> f64 {
        let mut pick = rng.f64();
        let mut k = self.mu.len() - 1;
        for (i, w) in self.weight.iter().enumerate() {
            if pick < *w {
                k = i;
                break;
            }
            pick -= w;
        }
        for _ in 0..64 {
            let x = self.mu[k] + self.sigma[k] * rng.normal();
            if (0.0..=1.0).contains(&x) {
                return x;
            }
        }
        self.mu[k].clamp(0.0, 1.0)
    }
}

// ── Suggestion ──────────────────────────────────────────────────────

fn prior_sample(p: &Param, rng: &mut Rng) -> Value {
    match &p.domain {
        Domain::Cat(values) => Value::from(values[rng.below(values.len())].clone()),
        d => from_unit(d, rng.f64()),
    }
}

fn choose_categorical(
    values: &[String],
    below: &[&Value],
    above: &[&Value],
    rng: &mut Rng,
    opt: &TpeOptions,
) -> Value {
    let k = values.len() as f64;
    let weights = |group: &[&Value]| -> Vec<f64> {
        let n = group.len() as f64;
        values
            .iter()
            .map(|v| {
                let count = group.iter().filter(|g| g.as_str() == Some(v)).count() as f64;
                (count + opt.prior_weight / k) / (n + opt.prior_weight)
            })
            .collect()
    };
    let (l, g) = (weights(below), weights(above));
    let mut best = (f64::NEG_INFINITY, 0usize);
    for _ in 0..opt.candidates {
        let mut pick = rng.f64();
        let mut idx = l.len() - 1;
        for (i, w) in l.iter().enumerate() {
            if pick < *w {
                idx = i;
                break;
            }
            pick -= w;
        }
        let score = (l[idx] / g[idx]).ln();
        if score > best.0 {
            best = (score, idx);
        }
    }
    Value::from(values[best.1].clone())
}

fn choose_numeric(
    p: &Param,
    below: &[&Value],
    above: &[&Value],
    rng: &mut Rng,
    opt: &TpeOptions,
) -> Value {
    let unit = |group: &[&Value]| -> Vec<f64> {
        group
            .iter()
            .filter_map(|v| v.as_f64())
            .map(|v| to_unit(&p.domain, v))
            .collect()
    };
    let l = Mixture::new(&unit(below), opt.prior_weight);
    let g = Mixture::new(&unit(above), opt.prior_weight);
    let mut best = (f64::NEG_INFINITY, 0.5);
    for _ in 0..opt.candidates {
        let x = l.sample(rng);
        let score = l.log_pdf(x) - g.log_pdf(x);
        if score > best.0 {
            best = (score, x);
        }
    }
    from_unit(&p.domain, best.1)
}

/// The next assignment to try.  `history` holds every observation so far,
/// pending trials included as `Failed`.
pub fn suggest(space: &Space, history: &[Obs], rng: &mut Rng, opt: &TpeOptions) -> Assignment {
    let mut order: Vec<&Obs> = history.iter().collect();
    order.sort_by(|a, b| rank(a, b));
    let model = history.len() >= opt.startup && rng.f64() >= opt.explore;
    let mut chosen = Assignment::new();
    for p in &space.params {
        if !p.active(&chosen) {
            continue;
        }
        let seen: Vec<&Value> = order
            .iter()
            .filter_map(|o| o.assignment.get(&p.name))
            .collect();
        if !model || seen.len() < 2 {
            chosen.insert(p.name.clone(), prior_sample(p, rng));
            continue;
        }
        let n = seen.len();
        let below_n = ((opt.gamma * n as f64).ceil() as usize)
            .clamp(1, opt.gamma_cap)
            .min(n - 1);
        let (below, above) = seen.split_at(below_n);
        let value = match &p.domain {
            Domain::Cat(values) => choose_categorical(values, below, above, rng, opt),
            _ => choose_numeric(p, below, above, rng, opt),
        };
        chosen.insert(p.name.clone(), value);
    }
    chosen
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sweep::space::Track;
    use serde_json::json;

    /// A problem with a known optimum, to test the sampler against random
    /// search without running any cryptography.
    struct Problem {
        name: &'static str,
        space: Space,
        loss: fn(&Assignment) -> f64,
        /// The best possible loss.
        optimum: f64,
    }

    fn f(a: &Assignment, k: &str) -> f64 {
        a[k].as_f64().unwrap()
    }
    fn c(a: &Assignment) -> &str {
        a["c"].as_str().unwrap()
    }

    /// Smooth, with a categorical whose levels differ by a constant.
    fn smooth() -> Problem {
        Problem {
            name: "smooth",
            space: Space::custom(
                "smooth",
                Track::Algebraic,
                vec![
                    Param::cat("c", &["a", "b", "c"], ""),
                    Param::float("x", 1e-3, 1e3, true, ""),
                    Param::float("z1", 0.0, 1.0, false, ""),
                    Param::float("z2", 0.0, 1.0, false, ""),
                    Param::int("k", 1, 64, true, ""),
                ],
            ),
            loss: |a| {
                let pen = match c(a) {
                    "a" => 0.0,
                    "b" => 0.5,
                    _ => 1.0,
                };
                pen + ((f(a, "x").log10() - 1.0) / 2.0).powi(2)
                    + 4.0 * (f(a, "z1") - 0.3).powi(2)
                    + 4.0 * (f(a, "z2") - 0.8).powi(2)
                    + ((f(a, "k").ln() - 8f64.ln()) / 2.0).powi(2)
            },
            optimum: 0.0,
        }
    }

    /// Conditional: each branch has its own parameter, with a good region of
    /// moderate width; every branch shares `z`.
    fn conditional() -> Problem {
        Problem {
            name: "conditional",
            space: Space::custom(
                "conditional",
                Track::Algebraic,
                vec![
                    Param::cat("c", &["a", "b", "c"], ""),
                    Param::float("x", 1e-1, 1e3, true, "").when("c", &["b"]),
                    Param::int("y", 0, 100, false, "").when("c", &["a"]),
                    Param::float("z", 0.0, 1.0, false, ""),
                ],
            ),
            loss: |a| {
                let base = match c(a) {
                    "b" => ((f(a, "x").log10() - 1.0) / 2.0).powi(2) + 0.1,
                    "a" => ((f(a, "y") - 30.0) / 50.0).powi(2) + 0.3,
                    _ => 0.8,
                };
                base + 2.0 * (f(a, "z") - 0.7).powi(2)
            },
            optimum: 0.1,
        }
    }

    /// Deceptive: the best branch pays off only in a narrow window, so on
    /// average it looks worse than a flat one.  An independent per-parameter
    /// model has no way to see that; a known limit, kept as a floor.
    fn deceptive() -> Problem {
        Problem {
            name: "deceptive",
            space: Space::custom(
                "deceptive",
                Track::Algebraic,
                vec![
                    Param::cat("c", &["a", "b", "c"], ""),
                    Param::float("x", 1e-3, 1e3, true, "").when("c", &["b"]),
                    Param::int("y", 0, 100, false, "").when("c", &["a"]),
                    Param::float("z", 0.0, 1.0, false, ""),
                ],
            ),
            loss: |a| {
                let base = match c(a) {
                    "b" => (f(a, "x").log10() - 1.0).powi(2) + 0.1,
                    "a" => ((f(a, "y") - 30.0) / 30.0).powi(2) + 0.5,
                    _ => 1.0,
                };
                base + (f(a, "z") - 0.7).powi(2)
            },
            optimum: 0.1,
        }
    }

    /// Mean best loss over `seeds` studies of `n` trials, and the fraction of
    /// trials in the second half that took the best branch.
    fn mean_best(p: &Problem, opt: Option<&TpeOptions>, n: usize, seeds: u64) -> f64 {
        let mut total = 0.0;
        for seed in 0..seeds {
            let mut rng = Rng::new(7000 + seed);
            let mut history: Vec<Obs> = Vec::new();
            let mut best = f64::INFINITY;
            for _ in 0..n {
                let a = match opt {
                    Some(o) => suggest(&p.space, &history, &mut rng, o),
                    None => suggest(&p.space, &[], &mut rng, &TpeOptions::default()),
                };
                let loss = (p.loss)(&a);
                best = best.min(loss);
                history.push(Obs {
                    assignment: a,
                    state: State::Complete,
                    value: loss,
                    rungs_done: 1,
                });
            }
            total += best - p.optimum;
        }
        total / seeds as f64
    }

    /// How the defaults were chosen: `cargo test --bin ic tune_defaults --
    /// --ignored --nocapture`.  Mean regret (best loss minus the optimum)
    /// after 60 trials, 40 studies per cell; lower is better.
    #[test]
    #[ignore]
    fn tune_defaults() {
        let problems = [smooth(), conditional(), deceptive()];
        let variants: [(f64, f64); 6] = [
            (0.10, 0.0),
            (0.25, 0.0),
            (0.10, 0.15),
            (0.25, 0.15),
            (0.25, 0.30),
            (0.15, 0.10),
        ];
        for p in &problems {
            let rnd = mean_best(p, None, 60, 40);
            eprintln!("TUNE {:<12} random            regret {rnd:.4}", p.name);
            for (gamma, explore) in variants {
                let opt = TpeOptions {
                    gamma,
                    explore,
                    ..TpeOptions::default()
                };
                eprintln!(
                    "TUNE {:<12} gamma {gamma:.2} explore {explore:.2} regret {:.4}",
                    p.name,
                    mean_best(p, Some(&opt), 60, 40)
                );
            }
        }
    }

    #[test]
    fn tpe_beats_random_search_on_a_smooth_problem() {
        let p = smooth();
        let tpe = mean_best(&p, Some(&TpeOptions::default()), 60, 40);
        let rnd = mean_best(&p, None, 60, 40);
        assert!(
            tpe < 0.6 * rnd,
            "TPE regret {tpe:.4} should be well under random's {rnd:.4}"
        );
    }

    #[test]
    fn tpe_beats_random_search_on_a_conditional_problem() {
        let p = conditional();
        let tpe = mean_best(&p, Some(&TpeOptions::default()), 60, 40);
        let rnd = mean_best(&p, None, 60, 40);
        assert!(
            tpe < 0.8 * rnd,
            "TPE regret {tpe:.4} should be under random's {rnd:.4}"
        );
    }

    /// The floor on a problem built to defeat independent models: not
    /// better than random, but not catastrophically worse either.
    #[test]
    fn a_deceptive_conditional_problem_costs_at_most_a_constant_factor() {
        let p = deceptive();
        let tpe = mean_best(&p, Some(&TpeOptions::default()), 60, 40);
        let rnd = mean_best(&p, None, 60, 40);
        assert!(
            tpe < 3.0 * rnd + 0.05,
            "TPE regret {tpe:.4} vs random {rnd:.4}"
        );
    }

    #[test]
    fn a_suggestion_contains_exactly_the_parameters_that_are_active() {
        let p = conditional();
        let mut rng = Rng::new(7);
        let mut history = Vec::new();
        for _ in 0..60 {
            let a = suggest(&p.space, &history, &mut rng, &TpeOptions::default());
            assert_eq!(a.contains_key("x"), a["c"] == json!("b"), "{a:?}");
            assert_eq!(a.contains_key("y"), a["c"] == json!("a"), "{a:?}");
            assert!(a.contains_key("z"));
            history.push(Obs {
                assignment: a.clone(),
                state: State::Complete,
                value: (p.loss)(&a),
                rungs_done: 1,
            });
        }
    }

    #[test]
    fn values_stay_inside_their_domains() {
        let p = smooth();
        let mut rng = Rng::new(11);
        let mut history = Vec::new();
        for _ in 0..120 {
            let a = suggest(&p.space, &history, &mut rng, &TpeOptions::default());
            assert!((1e-3..=1e3).contains(&f(&a, "x")));
            assert!((0.0..=1.0).contains(&f(&a, "z1")) && (0.0..=1.0).contains(&f(&a, "z2")));
            assert!((1..=64).contains(&a["k"].as_i64().unwrap()));
            history.push(Obs {
                assignment: a.clone(),
                state: State::Complete,
                value: (p.loss)(&a),
                rungs_done: 1,
            });
        }
    }

    fn sequence(seed: u64) -> Vec<Assignment> {
        let p = conditional();
        let mut rng = Rng::new(seed);
        let mut history = Vec::new();
        let mut out = Vec::new();
        for _ in 0..30 {
            let a = suggest(&p.space, &history, &mut rng, &TpeOptions::default());
            history.push(Obs {
                assignment: a.clone(),
                state: State::Complete,
                value: (p.loss)(&a),
                rungs_done: 1,
            });
            out.push(a);
        }
        out
    }

    #[test]
    fn the_same_seed_and_history_give_the_same_suggestion() {
        assert_eq!(sequence(5), sequence(5));
        assert_ne!(sequence(5), sequence(6));
    }

    #[test]
    fn the_mixture_is_a_density_on_the_unit_interval() {
        let mix = Mixture::new(&[0.1, 0.12, 0.8], 1.0);
        let steps = 20_000;
        let area: f64 = (0..steps)
            .map(|i| mix.log_pdf((i as f64 + 0.5) / steps as f64).exp() / steps as f64)
            .sum();
        assert!(
            (area - 1.0).abs() < 0.02,
            "the truncated mixture integrates to {area}"
        );
        let mut rng = Rng::new(3);
        assert!((0..2000)
            .map(|_| mix.sample(&mut rng))
            .all(|x| (0.0..=1.0).contains(&x)));
    }

    #[test]
    fn observations_rank_complete_then_pruned_then_failed() {
        let mk = |state, value, rungs| Obs {
            assignment: Assignment::new(),
            state,
            value,
            rungs_done: rungs,
        };
        let mut v = vec![
            mk(State::Failed, 0.0, 0),
            mk(State::Pruned, 1.0, 1),
            mk(State::Complete, 9.0, 3),
            mk(State::Pruned, 5.0, 2),
            mk(State::Complete, 2.0, 3),
        ];
        v.sort_by(rank);
        let order: Vec<_> = v.iter().map(|o| (o.state, o.value)).collect();
        assert_eq!(
            order,
            vec![
                (State::Complete, 2.0),
                (State::Complete, 9.0),
                (State::Pruned, 5.0), // got further, so ranks above the earlier prune
                (State::Pruned, 1.0),
                (State::Failed, 0.0),
            ]
        );
    }

    #[test]
    fn a_study_of_only_failures_still_suggests() {
        let p = conditional();
        let history: Vec<Obs> = (0..20)
            .map(|_| Obs {
                assignment: Assignment::new(),
                state: State::Failed,
                value: f64::INFINITY,
                rungs_done: 0,
            })
            .collect();
        let a = suggest(&p.space, &history, &mut Rng::new(1), &TpeOptions::default());
        assert!(a.contains_key("c") && a.contains_key("z"));
    }
}
