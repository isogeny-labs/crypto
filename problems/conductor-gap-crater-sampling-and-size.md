# Conductor-gap crater precision and subgroup-size coverage

- **Status:** investigating
- **Kind:** empirical precision limit; size-coverage gap
- **Affected work:** [replication](../research/conductor_gap_replication_20261025/README.md), [crater design analysis](../research/conductor_gap_crater_design_20261025/REPORT.md), [broader size design](../research/conductor_gap_size_ladder_20261026/REPORT.md)
- **Next action owner:** conductor-gap research workstream
- **Last updated:** 2026-10-27

## Problem

The corrected crater comparison does not establish equivalence within ±5%,
and the measured 25–32-bit prime subgroups do not support production-size or
scaling conclusions.

The measured eight fixed classes span four CM discriminants and 32/34-bit fields.
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

The 2026-10-26 [follow-up protocol](../research/conductor_gap_size_ladder_20261026/PROTOCOL.md)
and [explicit amendment](../research/conductor_gap_size_ladder_20261026/AMENDMENT-v2.md)
broaden the candidate design to 32 distinct classes per 40/47/54/61-bit
field rung, actual subgroup sizes at least field bits minus six, plus
separate 112/160/256-bit parameter checks. A candidate parameter class is
not an exact curve model, floor, point map or completed solver workload.
The proposed four-rung primary objective is nominal 99% simultaneous
coverage and intervals wholly inside [0.95,1.05]. Pilot calibration and
resource feasibility remain mandatory: the old variance model does not
establish power or coverage at new sizes.

The amended parameter search completed: all 128 native and six upper
classes pass the independent native verifier. Actual candidate subgroup
sizes are 34–59 bits on the native ladder and 110/158/254 bits on the
upper track; prime-gap parameters extend to approximately 6.49e36.
No new curve model, transported target or solver measurement is supplied
by these certificates. This does not meet the measured-size exit condition.

V1's D=-19, trace=-2, cofactor-four upper search exhausted all three cells.
For prime ell>5, either 1216 ell²+1 or 304 ell²+1 is divisible by 5.
The amendment preserves that failure and substitutes D=-67 only in those
three cells. This obstruction is specific to that parameter family, not
to D=-19 generally. The supporting native residue and identity certificates
are linked by the amendment; no checked Lean proof is claimed.

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
- [ ] Construct and independently certify the broader parameter candidates'
  exact models, orders, strata and explicit maps before using them as
  transported-target comparisons. Upper parameter certificates alone cannot
  close this condition.

These conditions can support a scoped empirical conclusion. Even successful
larger solvable groups do not prove security equivalence at cryptographic size.
CM full-cost accounting and algebraic-oracle coverage remain separate open
obligations in [the existing problem record](conductor-gap-upper-floor-and-cost.md).

## Next discriminating step

- **Action:** The user selected broader gaps and stronger confidence. Certify
  actual models/maps at the new rungs, establish execution feasibility and
  run a disjoint pilot under the local execution/task-card rules; use it to
  freeze confirmation. The [blocked backlog card](../backlog/XP-20261026-conductor-gap-size-confirmation/README.md)
  records the unimplemented driver, full replay and resource requirements.
- **Preparation available:** The [native allocation preflight](../research/conductor_gap_preflight_20261027/PROTOCOL.md)
  generates and audits the complete prospective pilot allocation, with one
  shared scalar per class/target across four crater slots, separate walk
  seeds, fixed block randomization and explicit zero executed solves.
  Exact models/maps, measurement execution and independent replay remain
  unimplemented; a passing allocation audit leaves preflight blocked.
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

2026-10-26: froze a wider parameter search, a separate upper structural
track and a four-comparison confidence objective. Proposed confirmation:
32 classes/rung, 1,000 fresh targets/crater and two seeds, four craters,
four rungs = 1,024,000 one-target solves before replays/controls. This is
not a guaranteed minimum, selected execution budget or launched campaign.
The complete disjoint pilot would contain 32,768 solves before replays.
Neither count has been executed; the measured crater verdict is unchanged.

2026-10-27: implemented prospective pilot allocation and corruption checks
without constructing new curves or invoking a solver. The 128 native classes
receive 32 distinct public nonzero scalars each (4,096 class/target pairs),
shared across four prospective crater slots with two separate seeds per slot.
The allocation has 32,768 prospective primary invocations, plus separately
pending full replay and controls. No measurement, calibrated power, final
confirmation count or crater-equivalence conclusion is added. The blocked
backlog card distinguishes this implemented preparation from its still-missing
execution mode.
