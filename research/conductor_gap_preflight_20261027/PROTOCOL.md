# Native allocation preflight for the larger crater comparison

Date: 2026-10-27. This is software preparation, not an experiment or a
replacement for the frozen size-ladder protocol. No searches, curve
construction, solver calls, timed measurements or paid resources are authorized
by this preflight. Historical inputs, certificates and measured archives stay
unchanged. PR #2634 merged into the still-open #2430; merging the parameter
design did not validate or execute its proposed measurements.

## Question and exact deliverable

Can the proposed pilot's allocation be generated reproducibly and audited
before any solver runs, so that a replay, a new logarithm and a new walk seed
cannot be counted as interchangeable repetitions?

Implement `examples/conductor_gap_confirmation.rs` with `plan`, `audit` and
`preflight` commands. It has no execution command. Bind the unchanged v2 CSV
by SHA-256, retain its 128 native parameter classes, and allocate exactly
32 fresh nonzero logarithms per class, four crater slots and two distinct
walk seeds per target/slot: 32,768 prospective primary invocations. The six
upper classes remain structural parameters and are excluded from this pilot.
Crater slots are placeholders, not certified curve identities or models.

The 1,000-target/1,024,000-solve confirmation is still provisional. This tool
does not select its final count, generate its observations or promote the
blocked backlog card. Controls and independent replays are additional work,
not included in the 32,768 primary-invocation count.

## Fixed allocation contract

- The canonical CSV digest is
  `1f7a8c3789c8acb46fe5407e3a23e6f227026c34d82513b42bc28951167d7f5f`.
  Reject substitutions, malformed rows, missing/duplicate classes and wrong
  rung/family counts. Four field rungs (40/47/54/61), four discriminants
  (11/19/43/67) and eight classes per cell are fixed.
- Canonical class order is `(field_bits, abs_d, p, t)`. Parameter identifiers
  are not ICV1 identities. No curve identity is assigned before construction.
- SHA-256 with a versioned domain, the input digest, a purpose tag, class
  identity and explicit counters derives public reproducible allocation
  bytes. Targets use rejection sampling into `1..r`, rejecting previously
  selected scalars within the class. Walk seeds have separate domains for
  each crater slot and replicate and are nonzero and globally distinct.
  These are reproducible design draws, not a proof of statistical
  independence or a security RNG claim.
- Store one scalar per target, shared by all four slots. A future executor
  must transport both G and Q through certified maps; a shared scalar field
  does not prove that it did so. Distinct numeric seeds do not prove that
  coordinate-dependent walks are the same transported random walk.
- Hash-order complete class/curve-slot blocks before outcomes. An audit
  checks the entire permutation as well as every target and seed. This is
  the frozen protocol's block randomization, not an admission for wall-time
  comparisons. Future controls must retain their own interleaving policy.
- Write allocation files with create-new semantics. Never overwrite an old
  plan. Audit the parsed structure and the derivation, not just its summary
  counts. Report exact primary/target/class counts and leave execution false.

## Confounder ledger and gates still open

| Factor | Treatment here | Required before measurement |
|---|---|---|
| Class/size/discriminant | Fixed complete certified parameter cohort | Exact models, orders, strata and three distinct S2 endpoints |
| Logarithm | 32 distinct public nonzero scalars per class; same scalar across four slots | Transport G and Q and verify scalar law on every endpoint |
| Rho randomness | Separate derived seed for every target/slot/replicate | Same pinned solver and budgets, cold per-target state; A/A/coordinate/plain-rho controls |
| Replays | Never counted as fresh targets or seeds | Full independent scalar checks and exact correctness replay |
| Execution order | Prospective complete-block hash permutation | Preserve allocation identity; record process/layout/host and interruptions |
| Instrument/setup | No instrument is executed | Pinned native build; full setup/map/solve/verification cost ledger |
| Analyst/inference | No observations or interval produced | Freeze analysis and masking; pilot coverage, variance and joint-power calibration |
| Resources/backend | No job or capacity is created | Feasibility under authorized resources; current backend and isolation requirements |
| Confirmation | Count remains a proposal | Disjoint target namespace and explicit no-overlap audit after pilot, final fixed count and reviewed protocol |

`audit` can pass allocation validity while `preflight` must fail with a
machine-readable blocked receipt until those obligations are implemented and
verified. A passing allocation audit is not measured crater equivalence,
production-size coverage, hardness evidence or a full-cost speed result.

## Sources checked before implementation

- [Handbook of Applied Cryptography, chapter 3](https://cacr.uwaterloo.ca/hac/about/chap3.pdf),
  section 3.6.3, Algorithm 3.60 and Fact 3.62 (full text read): rho is
  randomized; the expected square-root bound assumes random-function
  behavior. One transported invocation does not average its run variation.
- [Corrected crater design](../conductor_gap_crater_design_20261025/REPORT.md),
  corrected interval and repetition sections: eight classes and overlapping
  two-seed controls; replay is not a fresh observation.
- [Frozen size protocol](../conductor_gap_size_ladder_20261026/PROTOCOL.md)
  and [v2 amendment](../conductor_gap_size_ladder_20261026/AMENDMENT-v2.md):
  fixed cohort, unchanged pilot count, pending maps and calibration.
- [Transfer theorem](../deep_volcano_prime_gap_20261022/THEOREMS.md),
  Theorem 2: the logarithm is preserved when both points pass through a map
  injective on the selected subgroup; common orders alone do not certify it.
- [Controlled-experiment skill](../../.claude/skills/controlled-experiment/SKILL.md)
  and [report-evidence](../../.agents/skills/report-evidence/SKILL.md):
  keep allocation checks separate from statistical claims; preserve all
  still-unperformed requirements. No comparative claim is added here.

## Validation scope

Run native example tests for malformed plans, target/seed reuse, missing
classes/blocks, tampering and create-new behavior, plus a full-cohort plan
and audit. These are functional software tests, not pilot observations.
Run the required release-library suite and relevant site/validation-gate
suites before pushing, reporting any inherited failures with their exact
scope. Do not repair unrelated parent integration or alter a gate to pass.
