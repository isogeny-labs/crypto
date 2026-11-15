# DRAFT protocol: relation oversampling in the factor-base log precompute

**Status: DRAFT, not frozen.** No measured data exists under this protocol.
The machine-readable draft is [`protocol.draft.json`](protocol.draft.json)
(`experiment.protocol/v1`, `freeze.status = "draft"`). Freezing requires
review, a pilot that confirms the cell choice (below), and the freeze hash
before any measured row. Smoke checks of the instrument (seconds, debug
build, local) are not data.

## Question

In `ic logs`, relations are collected until the coefficient rank is full,
then solved: filter (CADO-style purge + merge) and block Wiedemann. Does
collecting a surplus of `ceil(f · columns)` relations past full rank, and
letting the filter purge spend that surplus on the heaviest rows, lower the
**total** cost of the precompute? Collection extra work is certain; the
linear-algebra saving is not. Only net total counts (AGENTS.md: accounting
trade-offs count only if total cost falls).

## Hypotheses

- **H1:** for some `f ∈ {0.1, 0.25, 0.5}`, the paired total cost
  `C(f) − C(0)` is negative, with a 95% bootstrap interval below zero, and
  the sign holds across the unit-price band (below).
- **H0 (negation):** for every tested `f > 0`, `C(f) − C(0) ≥ 0` or its
  interval contains 0. Prior evidence favours H0: LA was 0.11–1.9% of real
  `ic logs` precomputes (`research/la_engines_20261014/README.md`, 27 runs),
  so a surplus that costs more than ~2% of collection cannot pay for itself
  on those cells. That is an unverified extrapolation to these cells
  `[pcc:unverified]`; this experiment measures it.

## Instrument

`ic oversample` (`src/bin/ic/oversample.rs`), library
`collect_factor_base_log_pool` (`src/cryptanalysis/koblitz_index_calculus.rs`).

- One pool per seed, drawn with the exact collector, batch size and probe
  sequence of `solve_factor_base_logs` (so the f = 0 prefix is the same
  relations `ic logs` sees), continued past full rank to
  `full_rank + ceil(f_max · columns)` (whole batches kept).
- Arm `f` solves the prefix `full_rank + ceil(f · columns)` with
  `solve_factor_base_logs_from_relations` (same filter and solver options in
  every arm; `--target-excess` fixed across arms if set).
- Reference arm `logs_default` (`--logs-arm`): the prefix `ic logs` actually
  first solves (end of the batch where rank became full). This is the
  incumbent; f = 0 is the idealised lower edge.
- Each arm verifies every column (`[x]G == point`) independently of the
  solver flag. **An unverified arm is a failure, not a missing value.**

## Cells

Choose cells where `ic logs` completes in seconds to minutes (release).
The filter is CADO (the `ic logs` default) except in C4.

| id | cell | columns | prior use |
|----|------|---------|-----------|
| C1 (primary) | `--degree 23 --curve-a 0 --summands 2 --factor-index 0` | 44 | `research/la_engines_20261014/scripts/e2e.sh` label `n23-legacy`; `raw/e2e.jsonl` |
| C2 | `--degree 37 --curve-a 0 --summands 3 --factor-base <ic search spec>` | 16 | same script, label `n37-search` |
| C3 (exploratory, not previously used) | degree 37, m = 3, `subgroup_orbits` points 7400 ([`fb37_subgroup_orbits_7400.json`](fb37_subgroup_orbits_7400.json)), CADO filter | 104 | none; a larger column count |
| C4 (legacy-filter cell) | C3 with `--filter-strategy legacy` | 104 | none; added 2026-11-11 so the Krylov core is not empty |

**Why C4.** On smoke checks of C1, C2 and C3 the CADO filter merged the
whole system (core dimension 0–1), so the Krylov solver is not exercised and
any LA saving can only come from filter/reconstruction work. C4 keeps C3's
curve, factor base, seeds and fractions and changes only the filter, which
leaves a nonzero core. Smoke checks (2026-11-11, debug build, local, one
seed, **not data**):

| cell | arm | prefix | core dim | core nnz | row_ops | verified |
|------|-----|--------|----------|----------|---------|----------|
| C3 (CADO) | f0 / f0.25 / f0.5 | 358 / 384 / 410 | 1 / 1 / 1 | 33 / 33 / 33 | 2298 / 2342 / 2400 | yes |
| C4 (legacy) | f0 / f0.25 / f0.5 | 358 / 384 / 410 | 10 / 9 / 9 | 190 / 179 / 175 | 9682 / 8741 / 8697 | yes |

At degree 23 the legacy filter leaves a core of 0–5 (2 seeds), too small to
add as a cell. Because the pool is drawn
deterministically from the seed, C3 and C4 see identical relation pools (the
smoke prefixes match), so C3 vs C4 is a paired filter contrast. It is
secondary, not part of the decision rule: each cell's primary is f vs f0
within the cell. A large-core cell (hundreds of columns; hours of collection, e.g. the
N73 600-column run in la_engines) remains optional.

## Arms, seeds, order

- Arms: `f0` (baseline), `f0.1`, `f0.25`, `f0.5` (candidates),
  `f0_aa` (A/A control: a second, independently timed f = 0 solve),
  `logs_default` (reference).
- Seeds: 30 per cell (seed i = base + i, base `0x4b6f626c69747a00`, the
  `ic logs` default). Pilot of 5 seeds to estimate variance; 30 is the
  draft value, to be fixed at freeze from the pilot.
- Arm order within a seed rotates by seed index (built into `ic oversample`).
- Release build, `RAYON_NUM_THREADS` fixed, run through `isolab-eks` (L2
  for wall claims) per CLAUDE.md; the run is not launched by this draft.

## Metrics

Per row (`ic oversample --rows-out`, one JSON line per (seed, arm), with an
`export_run` object holding the `experiment.export/v1` run fields):

- `collection.trials` (exact, counted), `collection.seconds_prorated`
  (batch time pro-rated by trial), `summands_scanned_prorated` (counted for
  m = 3 pair-table only).
- `linear_algebra.row_ops` (counted: multiply-subtracts over filter, fold,
  Krylov products, checks, reconstruction), `linear_algebra.seconds`,
  filter report (rows/columns/nonzeros in and out), `core_dimension`,
  `core_nonzeros`, Krylov proxy `core_dimension × core_nonzeros`.
- `total_wall_seconds = collection.seconds_prorated + linear_algebra.seconds`.

**Primary metric:** per seed, the paired difference
`ΔC(f) = C(f) − C(f0)` of the total counted cost
`C = trials + λ · row_ops`, where `λ` is the price of one row operation in
trials. `λ̂` = median over seeds of `(la_seconds / row_ops) /
(collection_seconds / trials)` from the f0 arms. Report `ΔC` at
`λ ∈ {λ̂/10, λ̂, 10·λ̂}` (and `λ = 0` as the collection-only floor). Mean
`ΔC(f)` with a 95% paired bootstrap CI (10,000 resamples over seeds).
An arm is ranked against f0 only where the sign holds at all three λ.

**Secondary:** paired `Δ total_wall_seconds`, and every arm against
`logs_default` the same way.

**A/A:** `f0_aa − f0` has identical counted cost by construction (same
prefix); its `la_seconds` spread is the wall noise floor. A wall difference
inside the A/A 95% range is reported as no difference.

## Stopping and validity

- Fixed sample (no optional stopping): all seeds of all cells run once.
- Any arm with `verified = false` is a failure; > 0 failures in an arm
  makes that arm's comparison invalid (reported, not dropped).
- A pool that exhausts `--max-trials` before `f_max` invalidates that seed
  for every arm (reported).
- Positive control: none decisive is available; the instrument check is
  that `logs_default ≥ f0` in prefix length and that collection cost is
  monotone in `f` (unit test).

## Decision rule

H1 supported for an `f` if the primary CI is below 0 at all three λ and the
wall difference exceeds the A/A range. Otherwise H0 for that `f`. A win
would be a measured improvement to the precompute; it would still need the
validation gate (replay certificate, validator report, matched rho with the
§5a band) before any leaderboard row.

## Sources checked

`research/la_engines_20261014/README.md` (LA share of `ic logs`, CADO filter
port), `research/sparse_la_timing_20261018/README.md` (completion rule
dominates sparse arms). The research-kb MCP server was not queried from this
subagent; the main session should add the kb receipt before freezing.

## Pilot (2026-11-11, run-20261111T0451Z)

5 seeds per cell, local unisolated container, owner-requested route; report
[`reports/run-20261111T0451Z/report.pdf`](reports/run-20261111T0451Z/report.pdf),
rows in `pilot/rows/`, summary `pilot/pilot_summary.json`, bench record
`benchmarks/records/20261111T0443Z-logs-oversampling-pilot.json`. 120/120 arms
verified. Every f > 0 arm and `logs_default` had higher total counted cost than
f0 in all four cells at every λ tested `[pcc:unverified]`. The pilot does not
change the protocol; `seeds_per_cell` stays 30 pending the reviewer's freeze.

## Freeze (2026-11-11T05:54Z)

Frozen as [`protocol.json`](protocol.json) (`freeze.protocol_sha256`
`aa928f27f9be1dff19e1d62ee79a148ac14c06d56c0dd80193bef62ae1153076`, scheme in
the file) before any frozen-run data, at the owner's instruction ("run the
whole thing"). Amendments over this draft: 30 seeds per cell; seed base
offset +100 so the frozen run's seeds are disjoint from the pilot's; route
local and unisolated (wall metrics indicative only, primary metric counted).
Gaps recorded in the frozen file: no research-kb receipt (not reachable from
this container) and no independent reviewer before freeze.
