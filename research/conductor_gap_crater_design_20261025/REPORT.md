# Crater comparison, repetition planning, and subgroup sizes

Date: 2026-10-25. Classification: reanalysis and experimental design, not a
new solver measurement or speed result. Source measurements are frozen at
`20af12f8641c19710a8135655b012b5359ffe02d` in the
[replication study](../conductor_gap_replication_20261025/README.md).

## Requested work and actual status

| Requirement | Status and evidence |
|:--|:--|
| Recompute the corrected crater comparison | Complete: native C++ enumeration below; equivalence remains inconclusive. |
| Decide whether 100 or 1,000 repetitions are appropriate | A provisional design is sized from existing data; assumptions and uncertainty are explicit. It is not a guaranteed minimum. |
| Establish the key/subgroup size | Complete: replication subgroups are 25–32 bits, which are toy sizes. The original 52-bit-field study has a 41-bit subgroup. |
| Run a new, larger-size or higher-repetition campaign | Not performed by this reanalysis. The size ladder, execution envelope, exact fixtures, and protocol must be fixed before such measurements. |

Existing-data reanalysis is complete. A new measured size and execution design
remain unselected; the calculations here do not silently select them.
No new paid resources, scientific solver measurements, or edits to frozen
records are involved. Functional test executions are validation only.

## Corrected crater comparison

The estimand is unchanged: within each of eight classes, average the three S2
crater means and divide by the S1 crater mean; take the geometric mean of these
eight ratios. There are 192 primary targets on each of the 32 crater curves,
or 6,144 primary crater observations. Shared scalar indices remain paired.

| Calculation | S2/S1 ratio | 98% whole-class bootstrap interval | Within [0.95, 1.05]? |
|:--|--:|:--|:--|
| Frozen 10,000-draw Rust calculation | 1.01112520 | [0.97429346, 1.05659648] | Inconclusive |
| Full finite resampling distribution, C++ | 1.01112520 | [0.97323326, 1.05705769] | Inconclusive |

The new calculation enumerates all 6,435 class-count compositions with their
multinomial multiplicities, representing all 16,777,216 ordered resamples of
eight classes. Quantiles use the nearest rank `round((N-1)*q)`, with zero-based
ranks at `q = 0.01, 0.99`. It removes Monte Carlo error in the resampling
calculation; it adds no targets, walk seeds, classes, or statistical information.
"Full enumeration" does not mean exact frequentist coverage or representative
sampling of all curves. The eight selected classes remain a fixed cohort.

The 98% rule retains the original simultaneous five-level correction. A fresh
crater-only analysis does not retroactively replace it with a 90% interval.
Failure to establish equivalence does not establish a difference: this remains
an inconclusive comparison, and individual class ratios must remain visible.
The original result and interval are preserved as historical evidence.

![The same crater estimate with frozen Monte Carlo and fully enumerated intervals. Shading is the predeclared equivalence band; neither interval fits inside it.](figures/crater-ci.pdf)

## What should be repeated?

The repetition unit needs three indices: independent class, sampled logarithm,
and solver seed. Transport generator and target together so each endpoint uses
the same logarithm in the chosen prime subgroup. Each solve owns fresh state;
there is no cross-target distinguished-point sharing or throughput division.

Repeating exactly the same target and seed is a correctness replay, not another
statistical sample. Repeating a seed on one target does not increase target or
class coverage. Taking more bootstrap draws does not increase measured coverage.

The existing primary experiment has one seed per geometric workload. The
crater A/A controls provide a second seed for 32 matched logarithms per class on
each of its four craters. These 1,024 control observations overlap 1,024 primary
geometric workloads. The 1,280-pair total in the full study also includes 256
selected floor pairs; those floor observations are excluded from this design
calculation.

## Provisional sample-size model

This is a planning calculation, not a new equivalence test or measured power.
Let `A` be S1 cost, `B` the mean of three S2 costs, and `muA`, `muB` their
primary class means. The delta-method influence is

`z = B/muB - A/muA`.

For each class, the 32 targets with two seeds give a seed component
`Var(z0-z1)/2` and a target component `Cov(z0,z1)`. Negative target covariance
estimates are truncated to zero; six of eight raw covariances are negative.
That is evidence of noisy variance-component estimates, not a proved zero
target effect. All raw and truncated components are retained in
[design.json](design.json).

Across the current classes, the log-ratio variance is 0.00295884. Estimated
within-class sampling variance at the existing 192 targets is 0.00177905;
their truncated difference, 0.00117979, is used only as a hypothetical
between-class variance in the scenarios below. The average seed and truncated
target components are 0.39528768 and 0.00465244, respectively. These quantities
are estimated from a small, deterministically selected cohort; they do not
establish a population distribution or prove genuine class heterogeneity.

For `K` exchangeable classes, `M` targets per crater and `R` seeds per target,
the projected variance of the pooled log ratio is

`tau2/K + (target_component + seed_component/R)/(K*M)`.

The fixed-cohort alternative omits `tau2/K`. Both are emitted by the native
analyzer. Future-class projections assume the current average components apply
to the new classes; changing subgroup size may invalidate that assumption.
For a normal log-estimator with standard error `se`, the projected probability
that its 98% interval fits inside the equivalence band is the probability its
estimate falls between `log(0.95)+2.32635*se` and
`log(1.05)-2.32635*se`. An empty acceptance interval has probability zero.

The following scenarios use the observed ratio 1.01112520 and the hypothetical
between-class component. They are approximate planning probabilities with
unquantified model uncertainty, not guarantees.

| Classes | Distinct targets per crater | Seeds per target | Rho calls | Approximate probability of establishing equivalence |
|--:|--:|--:|--:|--:|
| 8 | 100 | 1 | 3,200 | 0% |
| 8 | 100 | 5 | 16,000 | 46.5% |
| 8 | 1,000 | 1 | 32,000 | 62.3% |
| 8 | 1,000 | 5 | 160,000 | 74.6% |
| 16 | 100 | 5 | 32,000 | 84.9% |
| 16 | 1,000 | 1 | 64,000 | 92.9% |
| 16 | 1,000 | 2 | 128,000 | 95.8% |

![Native model projections at the observed crater ratio, including estimated class heterogeneity. These are design scenarios, not additional observations.](figures/repetition-power.pdf)

**Recommendation:** use 1,000 distinct logarithms per crater, at least two
independent solver seeds per logarithm, and a preselected 16-class cohort as a
starting design. With one S1 and three S2 craters per class, this is 128,000
solves, plus separate correctness replays. Two seeds allow variance separation;
extra budget should also buy fresh targets and classes. The counts must be
reassessed with a disjoint pilot at the chosen larger subgroup sizes and a
preregistered precision/power criterion before the primary campaign is frozen.
Increasing the repeat count until an equivalence test passes is not allowed.
An honest difference or inconclusive result remains admissible.

This recommendation addresses precision of a pooled scoped comparison. It does
not establish equivalence for every class or endomorphism-aware algorithm.
The existing class-7 ratio near 1.12 is not explained away by pooling or by the
variance model.

## The key-size limitation

The relevant size of the unknown scalar is the prime subgroup order `r`, not
just the field prime `p`. The current replication has 32/34-bit fields and
25–32-bit subgroup orders. Its generic square-root work scale is roughly
`2^12` through `2^16` group operations, before algorithm constants and setup.
That is deliberately small and easily solvable; it is not production-size
security evidence. Small-size setup and walk effects can matter.

| Instance set | Field bits | Subgroup bits | Approximate generic square-root exponent |
|:--|--:|--:|:--|
| Current eight-class replication | 32 or 34 | 25–32 | 12.09–15.99 |
| Original large-gap study | 52 | 41 | 20.39 |
| Typical production-scale ECC comparison | about 256 | about 256 | about 128 |

The original subgroup is exactly `1883199430181`, about `2^40.78`, as recorded
in its [parameter table](../deep_volcano_prime_gap_20261022/README.md#the-class-and-the-instance).
The last table row is a generic algorithmic work estimate, not a measured
security assessment of these CM fixtures. Full rho sampling at that scale does
not fit the available local execution model. Parameter, subgroup and map checks
at production size would answer a separate structural question and must not be
presented as completed ECDLP solving comparisons.

A larger solvable size ladder must be specified in **subgroup bits**, with
certified actual orders, eligible conductor gaps, and complete transfer costs.
Adding eight subgroup bits multiplies the generic square-root work scale by
about sixteen. More repetitions on 25–32-bit groups cannot substitute for that
size coverage. No larger-size experiment has been run by this analysis.

## Validation and reproduction

[design.cpp](design.cpp) is a standalone C++17 analyzer. It checks the CSV schema,
crater/model/arm counts and verified statuses, reconstructs all eight class
means and the published headline, and checks the full resampling mass. Built-in
controls cover a null cohort, a constant non-unit ratio, a two-point cohort with
known binomial quantiles, and simple power limits. The build uses warnings as
errors. Input and source digests are retained in [INPUTS.sha256](INPUTS.sha256).
The original measurements, protocol, archive, and verdict are not edited.

Run from the repository root:

```sh
crater_study=research/conductor_gap_crater_design_20261025
sha256sum -c "$crater_study/INPUTS.sha256"
g++ -std=c++17 -O2 -Wall -Wextra -Werror \
  "$crater_study/design.cpp" -o /tmp/crater-design
/tmp/crater-design \
  research/conductor_gap_replication_20261025/analysis/records-A.csv \
  > /tmp/crater-design.json
```

Python in [render.py](render.py) only plots the native JSON and the previously
recorded frozen interval. It performs no bootstrap, variance estimation, power
calculation, curve arithmetic or solver execution. The PDF is rendered from this
report and the same figures. Validation results are retained in `checks/`.

The replication's canonical level-ratio graph, dashboard, progress timeline and
leaderboard were checked for affected claims. They retain the original frozen
measurement and simultaneous crater verdict; this reanalysis changes neither
the ratio nor that verdict. The new diagnostic graphs are kept here. No measured
speed improvement, leaderboard row, benchmark row or new progress point is
promoted. The [sampling/size problem record](../../problems/conductor-gap-crater-sampling-and-size.md)
keeps the unperformed measurement obligations explicit.
