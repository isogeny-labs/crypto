# Radix gradings of the Kummer tower: does F4's cost track tower levels or bits?

**Protocol, frozen before any study row.** 2026-11-04. Study id `EXP-PKM-RADIX-001`.
Machine-readable form: [`protocol.json`](protocol.json) (`experiment.protocol/v1`). Runner: [`run_cells.sh`](run_cells.sh).
Motivation: [`RESEARCH_PRIME_FIELD_BARRIER_MAP_20261104.md`](../notes/ecdlp-general/RESEARCH_PRIME_FIELD_BARRIER_MAP_20261104.md) §6.1.

## 1. Question

A PKM Kummer tower of `t` levels writes membership in `V = g·μ_{2^t}` as `t` quadrics in `t` variables
(`RESEARCH_PKM_TOWER_ORACLE.md` §2.4). A radix-`2^k` tower writes the same `V` with `t/k` variables of degree `2^k`.
Its quotient ring, rules and standard monomials are those of the radix-2 tower with every block of `k` levels
graded `1, 2, …, 2^{k−1}`: `u = y_0`, `u² = y_1`, `u⁴ = y_2`, … (`TowerWeights`, added in this PR to
`f4_fp_tower`). So this study compares one ideal, one factor base and one set of systems under different
gradings. Only the order in which F4 meets monomials changes.

**Question.** On the same refutation systems, does a radix-4 or radix-8 grading change F4's counted work
(multiply-adds) by a factor of 2 or more against the unit (radix-2) grading? If it does, does the factor grow
with `N = m·t`?

**Why it matters.** The barrier note's `k`-SUM reading predicts that cost is governed by information
(`log₂|V|`), not by the number of tower variables, so regrading cannot buy a factor that grows with `N`. A
growing gain would contradict that reading and would be the first presentation-level lever on the tower's
cost found in this repository. A constant gain would be engineering. No outcome here moves an exponent at
cryptographic size, and no outcome is a sub-rho claim.

## 2. Claims and their classes

| id | claim | class | decided by |
|:--|:--|:--|:--|
| C1 | The unit-graded instrument at this commit reproduces the committed `R-t9-t10` rows exactly (`D`, `max_cols`, `muladds`, `inconsistent`, both draws) | exact computation | G0 cells |
| C2 | Every graded run decides every system as the unit run does (`inconsistent` agrees; every planted row has `planted_ok = true`) | exact computation | all cells |
| C3 | The radix-`2^k` grading changes refutation multiply-adds by the ratio `r_k(t)` reported, per `(m, t, control)` | exact computation per system; comparative across the ladder | main ladder |
| C4 | The ratio grows, fades or holds with `N` | comparative, descriptive only over `t ≤ 10` (`m = 2`), `t ≤ 5` (`m = 3`) | §5 rule |
| — | Anything about `N ≥ 24`, cryptographic sizes or rho | out of scope | not established |

## 3. Arms, controls, cells

**Arms.** `radix ∈ {2, 4, 8}`, the pilot's `--radix`. Radix 2 is the unit grading and the reference arm.

**Instrument.** `examples/pkm_tower_pilot.rs --engine tower` at this PR's commit, one binary (SHA-256 recorded
by `run_cells.sh`), `RAYON_NUM_THREADS=2`. Same flags on every arm except `--radix`. Build:
`CARGO_BUILD_JOBS=1 CARGO_PROFILE_RELEASE_CODEGEN_UNITS=64 CARGO_PROFILE_RELEASE_OPT_LEVEL=1 cargo build --release
--no-default-features --example pkm_tower_pilot`. The library at `opt-level=3` was killed by this host's memory
limit three times (2026-11-04). Optimization level cannot change a counted metric, and G0 checks that this binary
reproduces the committed counts exactly. Clocks from it would not be comparable with the committed rows' clocks,
and none is read.

**Controls.**
- *Regression (C1, positive control of the instrument):* G0 = the `R-t9-t10` command of
  `research/prime_fall_degree_t14_20261101/run_cells.sh`, radix 2. Expected exactly
  `t=9: D=5, max_cols=8644, muladds=45957046778`; `t=10: D=6, max_cols=33446, muladds=481960954036`; both draws,
  `inconsistent=true`. Any mismatch stops the study before any graded cell is read.
- *Positive control of the grading (detector sees a grading):* the unit test
  `a_weighted_key_order_is_a_monomial_order` checks that weighted keys carry the weighted degree, and the study
  rows record `weights` per row. A graded row whose `weights` field is missing or unit is invalid. There is no
  input where the size of a grading effect is known in advance. The detector shows only that the instrument
  runs a different grading; it cannot show a minimum effect it would catch.
- *Negative / null control:* `--controls null` (`S₃`'s monomial support with random coefficients and a planted
  zero, under the same tower), run under every grading. If the tower and the null agree on the §5 verdict, the
  effect belongs to the system's shape, not to the elliptic curve.
- *A/A:* the counts are deterministic in the system. A/A is therefore a determinism check: cell A1 reruns the
  radix-4, `m = 2`, `t = 8` tower cell, and its counts must be bit-identical. It measures determinism, not
  variance. There is no variance to measure in counted metrics.
- *Correctness:* `--planted 1` up to `t = 8` (`--planted-max-t 8`). Every planted row must have
  `planted_ok = true` and `inconsistent = false`.

**Cells** (all Kummer; `--ladder-t none --random 2`).

| cell | `p` | `m` | `t` | controls | arms |
|:--|:--|:--|:--|:--|:--|
| G0 | 2013265921 | 2 | 9, 10 | tower | 2 |
| L2 | 2013265921 | 2 | 4, 5, 6, 7, 8, 9, 10 | tower, null | 2, 4, 8 |
| L3 | 2013265921 | 3 | 3, 4, 5 | tower | 2, 4, 8 |
| X2 (replication) | 786433 | 2 | 6, 8, 10 | tower | 2, 4, 8 |
| A1 | 2013265921 | 2 | 8 | tower | 4 (rerun) |

**Order.** Within each `(cell, t)` the arms run in a rotation fixed now: `t` even → `2, 4, 8`, `t` odd →
`8, 4, 2`. Counts do not depend on order. The rotation guards only the clock, which is not a study metric.

**Budgets.** `--budget 1800 --max-nnz 2000000000`, `ulimit -v 6500000` (this host: 2 vCPU, 7 GiB). A row stopped
by budget or memory is *censored*. It is reported with its lower bound and is never imputed. No cell is rerun
with a larger budget to get a different outcome.

## 4. Metrics

- **Primary:** `muladds`, the `F_p` multiply-adds of all eliminations (counted, exact, per system).
- **Secondary:** `max_cols` (widest matrix), `max_dense_entries` (largest `B'`), `solving_degree_max` in the
  arm's own grading, `steps`.
- **Clock:** `ms` is recorded, not used. The host is unisolated and the protocol uses no clock.
- **Reduction:** the two random draws of a cell are separate systems. Where their counts are equal (as in
  every committed Kummer refutation row) the cell has one value. Where they differ, both are reported and
  `r_k` is computed per draw.

## 5. Decision rule (frozen)

Let `r_k(t) = muladds(radix 2^k) / muladds(radix 2)` on the tower refutation systems, per draw, and `t_max` the
largest `t` of the ladder at which all three arms completed (not censored).

1. **Validity first.** C1 and C2 must hold. Otherwise the verdict is `instrument_invalid` and nothing below
   is read.
2. Per `(m, k)`:
   - `gain_growing`: `r_k(t_max) ≤ 0.5` on every draw, and `r_k` is non-increasing over the last three
     completed `t`.
   - `gain_constant`: `r_k(t_max) ≤ 0.5` on every draw, but not non-increasing over the last three.
   - `loss`: `r_k(t_max) ≥ 2` on every draw.
   - `no_material_effect`: `0.5 < r_k(t_max) < 2` on every draw.
   - `mixed`: draws disagree on the band. This is reported as such.
3. **Attribution.** Compute the same verdict for the null control (L2 only). If tower and null agree, write
   "the effect belongs to the system shape". If they differ, write "elliptic-specific at these sizes".
4. **Replication.** X2 must give the same verdict band as L2 at `t = 10`, or the verdict is reported as
   `not_replicated`.
5. **Descriptive only:** least-squares slope of `log₂ muladds` against `N` per arm, over completed `t`, with
   no interval and no decision (at most seven deterministic points).

**Pre-registered prediction (this author's):** `no_material_effect` for `m = 2` and `m = 3`, `k = 2` and `k = 3`,
on the tower and the null alike. **It is falsified by** `gain_growing` or `gain_constant` for any `(m, k)` on
the tower.

**Multiple comparisons.** Four primary verdicts (`m ∈ {2, 3}` × `k ∈ {2, 3}`). They are band rules on exact
counts with no p-values, so no correction applies. Every verdict is reported, including the uninteresting
ones.

## 6. What does not count

Changing cells, budgets, bands or `t_max` after seeing a row. Dropping a censored row. Reading a clock.
Calling a constant factor an exponent. Extrapolating past `t = 10` (`m = 2`) or `t = 5` (`m = 3`). Any of
these needs a dated amendment below, stating whether it preceded the data it affects.

## 7. Execution and pre-flight

- **Backend.** This container: local, 2 vCPU, 7 GiB, unisolated. The repository default is Iso Lab (`AGENTS.md`
  §9a). This session has no route to it: no `isolab_k8s_*` tool, the environment's AWS credentials are
  rejected by STS (`InvalidClientTokenId`, checked 2026-11-04), and `isolab-experiment.yml` takes ecbench specs
  only. The primary metric is counted and host-independent, so the backend changes no study value, only
  which cells finish inside the budget. Rows record `host: local-ccr-2vcpu`.
- **Pre-flight estimate.** From the committed rows at 4 vCPU: `m = 2` unit `t = 9`: 5 s; `t = 10`: 47 s.
  Assume ×2 at 2 vCPU and a grading factor in `[0.25, 4]`. L2 + G0 + X2 + A1 then fit in under 1.5 h. The
  `t = 10` graded cells are the expensive end and may censor. L3 is unpriced: `m = 3`, `N = 15` was minutes
  at 4 vCPU in round 7. The estimate is recorded next to the actual time in the report.
- **Checkpointing.** None (local, minutes per system). An interrupted cell is rerun from the start under the
  same name, and the interruption is logged in `progress.txt`.

## 8. Amendments

None.
