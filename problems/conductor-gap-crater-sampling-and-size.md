# Conductor-gap crater precision and subgroup-size coverage

- **Status:** investigating
- **Kind:** empirical precision limit; size-coverage gap
- **Affected work:** [replication](../research/conductor_gap_replication_20261025/README.md), [crater design analysis](../research/conductor_gap_crater_design_20261025/REPORT.md)
- **Next action owner:** conductor-gap research workstream
- **Last updated:** 2026-10-25

## Problem

The corrected crater comparison does not establish equivalence within ±5%,
and the current 25–32-bit prime subgroups do not support production-size or
scaling conclusions.

The eight fixed classes span four CM discriminants and 32/34-bit fields.
Each has one S1 and three S2 craters, 192 matched target indices and one
primary rho seed per geometric workload. A second seed exists on 32 crater
targets per class. These are public synthetic, known-answer fixtures.

## Evidence and uncertainty

- **Observed:** The frozen 98% whole-class interval is [0.97429, 1.05660]
  at a ratio of 1.01112520. The crater is inconclusive under the original
  simultaneous five-level rule.
- **Derived:** Native enumeration of the complete finite bootstrap
  distribution gives [0.97323, 1.05706], with the same verdict. No new
  solver observations are added. See the linked report, native source,
  input hashes and analysis JSON.
- **Derived planning scenario:** Sixteen exchangeable classes, 1,000
  fresh targets per crater and two seeds per target give 128,000 calls and
  an approximate 95.8% probability of establishing equivalence under the
  fitted model at the observed ratio. This is not measured power or a
  guaranteed minimum; variance components use only 32 two-seed targets per
  class, and six estimated target covariances are negative before truncation.
- **Unknown:** How much apparent class heterogeneity survives fresh targets,
  how the variance components change with subgroup size, and whether larger
  eligible conductor-gap families show the same pooled behavior.
- **Size limit:** The earlier 52-bit-field experiment has subgroup order
  1883199430181 (41 bits), also far below production ECC subgroup sizes.
- **Not established:** Uniform equivalence for every class, a scaling
  theorem, a full-cost speed advantage, or deployed-curve hardness.

## What would unblock it

- [ ] Freeze a follow-up with exact certified subgroup sizes, disjoint target
  selection, multiple seeds per target, a specified class cohort, uncertainty
  method, equivalence margin and resource envelope before primary outcomes.
- [ ] Use a separate pilot at the chosen subgroup sizes to check the proposed
  precision/power model. Freeze the final count before confirmation; do not
  continue sampling until equivalence passes.
- [ ] Execute all prescribed attempts, retain failures and censoring, verify
  recovered scalars and independent replays, and report the corrected interval
  even if it remains inconclusive or shows a difference.
- [ ] Establish larger-size coverage using actual subgroup-order bits. More
  repeats on the existing toy groups alone do not discharge that requirement.

These conditions can support a scoped empirical conclusion. Even successful
larger solvable groups do not prove security equivalence at cryptographic size.
CM full-cost accounting and algebraic-oracle coverage remain separate open
obligations in [the existing problem record](conductor-gap-upper-floor-and-cost.md).

## Next discriminating step

- **Action:** Select the size priority, certify the proposed fixtures and run a
  disjoint pilot under the existing local execution and task-card rules; use it
  to freeze a confirmation design. The current recommendation is provisional.
- **Controls:** Same-point generator/target transport, fixed subgroup/method/
  budgets, separate solver seeds, grouped inference, complete retention and
  replay; counted work is primary and L0 timing stays descriptive.
- **Limits:** No paid resources are provisioned. Exact primary sizes, counts
  and execution guards must be recorded before a new campaign; none is claimed
  to have run in the design analysis.
- **Outcomes:** A sufficiently narrow interval inside the margin supports
  scoped equivalence; a reproducible difference is reported as such; wide or
  censored results remain inconclusive. No outcome is forced by rerunning.
- **Recheck trigger:** The user's size priority, a new pilot, or new certified
  larger-subgroup fixtures.

## Resolution and history

2026-10-25: named after the target/seed audit and corrected crater reanalysis.
The comparison and provisional sizing calculation are complete; the larger
or higher-repetition measurements remain unperformed. Unresolved.
