# DRAFT protocol: published prime-field IC reference arm

**DRAFT. Not frozen. No data has been taken.** The machine-readable form is
[`protocol.draft.json`](protocol.draft.json). It passes
`python3 tools/experiment_protocol.py validate` (crypto-autoresearcher,
2026-10-29) with `freeze.status = "draft"`. Every `FILL-AT-FREEZE` field must
be replaced and the record frozen before the first measured execution. Labels:
*measured* (with source), *derived* (computed here from measured values, with
the formula), *plan*.

## 1. Question

What does the published naive Semaev index calculus cost on this repository's
prime-field instrument? That is: `ecbench ic.pipeline`, S = GAE/√r, cold whole
solve, with the AGENTS.md §5a band at 0, 0.1 and 1 GAE per native operation and
msolve's work counted as native operations. How far is it from matched
`rho.negation`, and where does it sit against the Table B incumbent on the
same base and the same trial stream?

This study measures a reference. It does not test a new method, and it
licenses no improvement claim for any arm.

## 2. Arms

All arms run in one ecbench session per curve: same binary, same targets,
same algorithm seeds, interleaved (`order: alternate`), one thread.

| id | role | configuration | track |
|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | generic |
| `rho-neg-aa` | A/A | the same | generic |
| `inc` | baseline (context) | `ic.pipeline`, `prime-abscissa:size=N_inc(b)`, `mitm:negation_folded=1`, `targets=walk` | table: reference only (generic track once `../DECISIONS.md` R1 merges) |
| `pub-m2` | candidate (the reference being established) | `ic.pipeline`, `prime-abscissa:size=N_inc(b)`, `semaev-naive:m=2,mode=full`, solver `msolve:recover=param` | algebraic (subject to owner ruling D1) |
| `pub-m2-tuned` | candidate | the same at `N_pub(b)`, chosen on the train panel | algebraic |
| `pub-m2-aa` | A/A | `pub-m2` again, on 25% of the targets | |
| `pub-m2-pos` | positive control | `ecbench oracle-check`: 1000 planted points `P_i ± P_j` through the same oracle and engine | |
| `pub-m2-grid` | negative control | `pub-m2` with the counted exhaustive grid engine `fp-grid` | |
| `pub-m2-f4fp` | candidate (engine comparison only) | `pub-m2` with the in-house `f4-fp` engine | |
| `pub-m3-tuned` | candidate | `semaev-naive:m=3,mode=full` (S₄), `N_pub3(b)` from train, 10–18 bits | algebraic |
| `inc-m3` | baseline (context) | `rootfind-subtract:m=3` on the same base | unclassified (generic track once `../DECISIONS.md` R1 merges) |

Note (cross-track review): the Table B rows themselves were measured by `ic boundary`
(`docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json`, `src/cryptanalysis/ic_boundary.rs`),
not by `ecbench` `ic.pipeline`. `inc` is a re-measurement of the Table B recipe in the
same session as `pub-m2`; its S is not expected to equal the Table B S exactly, and the
comparison that matters is `inc` against `pub-m2` inside one session.

`N_inc(b)` is the incumbent's base size, filled at freeze:
- roster curves: from `docs/ic/leaderboard.json` (*measured*: 51 abscissae
  at 20 bits, the Table B recipe `mitm_m2_negfold_walk_balanced`);
- holdout curves: from `research/prime_base_oracle_tuning_20261029`
  (*measured*: 45 at 20 bits, pending second-host replay; 162 at 24 bits).

Why `pub-m2` uses the incumbent's base. `inc` and `pub-m2` then share the
FB1h factor-base id, the walk and the trial points, and differ in exactly one
factor, the decomposition oracle. That gives two exact checks for free:

- **C-hits.** The trial indices at which `inc` (an exhaustive pair table)
  finds a relation must equal those at which `pub-m2` does.
- **C-grid.** `pub-m2` and `pub-m2-grid` must emit byte-identical relation
  streams.

`pub-m2-tuned` exists so that the published arm is not judged at a base size
tuned for a different oracle. The legacy measurement (§6 of the README)
predicts that msolve's per-call cost favours smaller bases.

## 3. Panels, sizes, targets

| panel | curves | m = 2 sizes (bits) | m = 3 sizes | targets × rounds |
|:--|:--|:--|:--|:--|
| T (Table B) | `prime_roster` at each size: the Table B curves `icv1-fp10…fp24` (check with `ecbench plan` at freeze) | 10, 12, 14, 16, 18, 20, 22, 24 | 10, 12, 14, 16, 18 | 48 × 2 at ≤ 18; 24 × 2 at 20; 12 × 2 at 22; 8 × 2 at 24 |
| H (today's holdout) | `prime_search` bits 20 and 24, seed 20261018202 (the curves of `research/prime_base_oracle_tuning_20261029`) | 20, 24 | none | as panel T |
| train | fresh `prime_search` curves, seeds from the protocol hash | 12, 16, 20, 24 | 10, 14, 18 | 8 × 1 per candidate N |

At 24 bits `pub-m2` (matched N = 111 on the roster curve) runs 4 targets × 2
rounds. The derived per-solve time there is about 2.2 h (README §6). The
preflight gate in §7 decides whether more targets fit.

**Train selection of N_pub(b).** The candidates are N ∈ {8, 12, 16, 24, 32,
48, 64, N_inc(b)}, capped where the derived per-solve time exceeds 1 h. Pick
the N that minimises the maximum, over w ∈ {0, 0.1, 1}, of
`mean S_N(w) / min_N' mean S_N'(w)` (minimax regret across the band). Ties go
to the smaller N. Record the choice in `select/FROZEN.sha256` before any
panel T or H session runs, as the 2026-10-29 tuning study did.

## 4. Seeds

`train` curve seeds, `target_seed` and `measurement.seed` are the first 8
bytes of `sha256(protocol_sha256 ‖ panel ‖ bits ‖ role)`
(`tools/experiment_protocol.py seeds`). The panel-H curve seed is fixed at
20261018202 on purpose, so that today's tuning records give an independent
cross-check of `inc` there. The msolve separating-form coefficients come from
ChaCha8 seeded by (algorithm seed, trial index, retry). Every seed is in the
records.

## 5. Pricing band

For every record: `priced_gae(w) = G − C + w·(U + n_C)` and
`S(w) = priced_gae(w)/√r` (`src/cryptanalysis/ecbench/band.rs`), at
w ∈ {0, 0.1, 1}. The parts:

- **G**: the charged group operations of every phase: factor base, oracle
  set-up, relations (walk and lift), linear algebra, verify.
- **U for `pub-*`**: msolve's counted F_p multiply-adds
  (`solver_F_p_multiply-adds_(msolve_linear_algebra_and_FGLM)_uncharged`, read
  from `solver.ops`), plus `semaev_membership_fp_mults_uncharged`,
  `semaev_system_fp_mults_uncharged`, `semaev_rootfind_fp_mults_uncharged`
  and, for the grid arm, `semaev_grid_fp_mults_uncharged`.
- **U for `inc`, `inc-m3` and the rho arms**: their own unpriced counters
  (lookups, inversions, canonicalisations), priced the same way. Rho is not
  exempt.
- **n_C**: native work the curve's pinned calibration priced. For the
  `prime_search` curves of panels H and train there is no pinned row, so C = 0
  and that work is all in U.

A record that names an unpriced counter without a count is a lower bound and
is not ranked (`band.rs`). msolve's work is never priced by host wall time
inside S (§9).

## 6. Metrics

- **Primary**: `S(0)`, `S(0.1)`, `S(1)` per record. Arms are compared by the
  paired ratio of summed S over matched (target, round) pairs, with a cluster
  bootstrap over targets (95%, 4000 resamples).
- **Secondary**:
  - the fitted exponent α per weight (`ecbench bound fit`);
  - msolve multiply-adds per trial;
  - hit rate, with a Clopper–Pearson interval;
  - msolve's internal elapsed time (start-up excluded), and the solve wall
    time (start-up included). Both are timed and compared within one host only;
  - peak RSS.

## 7. Stopping and gates

1. **Preflight** (before freeze, not part of the data). Run
   `ecbench solver-selftest msolve` at every session's p. Run 4 targets × 1
   round of `pub-m2` at each size. Record per-solve seconds, peak RSS and
   counter sanity, and replace the derived runtime in
   `sample_size.preflight_runtime` with these measurements. If a size's
   per-solve time exceeds 2 h, that size runs 4 targets × 1 round and its
   rows are `descriptive_only`.
2. **Fixed N.** Stop when every planned execution has a record.
3. **Invalidation.** Any of the following voids every `pub-*` record of that
   session:
   - a C-hits or C-grid mismatch;
   - a self-test failure;
   - a `pub-m2-pos` miss;
   - a record whose msolve sha256 differs from the pin;
   - a non-identical replay.
4. **Censoring.** An execution that reaches `timeout_seconds` (6 h) is
   censored and recorded. It is never re-run with a longer timeout. Spot
   preemption allows one infrastructure retry from scratch; counted values
   are valid after a retry, wall time is not.

## 8. Decision rules

**Admission: is the reference established?** Every item must hold:

- every `pub-*` run verifies;
- C-hits and C-grid are identical;
- `pub-m2-pos` detects 1000/1000;
- msolve returns `[-1]` on every trial point the grid finds non-decomposing
  (at least 1000 per size);
- the self-test passes;
- `ecbench verify --replay` is identical;
- the counted A/A ratios `pub-m2-aa/pub-m2` and `rho-neg-aa/rho-neg` are
  exactly 1.000.

Otherwise the verdict is `not_established` and nothing below is reported as a
result.

**C2, distance remaining.** For each size and each w, report
`Σ S_pub-m2-tuned / Σ S_rho-neg` with its 95% interval, and the same for
`pub-m3-tuned`. These are distances to rho, not a pass/fail test (owner
criterion 2026-10-15). Also report α per weight for each arm.

**C3, incumbent against the reference (context).** Report
`Σ S_inc / Σ S_pub-m2` with intervals at each w, and the `ecbench band
--ranks` order: `inc_cheaper_across_band`, `pub_cheaper_across_band`, or
`unresolved_in_band`. Under the current `docs/ic/SWEEPER.md` track rule this
is descriptive. It is not an improvement or regression verdict for either arm
until the owner rules on D2 (README §8).

*Derived expectation:* `inc` and `pub-m2` share the walk, so their G is
close. At w = 0 msolve's work is free, and `pub-m2` may even look cheaper,
because it builds no pair table. At w ≥ 0.1 msolve dominates. So
`unresolved_in_band` is a likely and legitimate outcome. The table also
reports the S(0.1) and S(1) ratios.

**C5, engine.** Report `f4-fp / msolve` per size, both as counted units side
by side and as msolve's internal time against f4_fp's time on the same host,
with A/A spreads. Say who is faster at which sizes. This comparison counts
only if every system's solution set is equal between the two engines.

**Multiplicity.** C2, C3 and C5 form one family, corrected by Holm. The three
weights are read jointly (order across the band), not as three tests.

## 9. Inadmissible

- Changing N, sizes, targets, seeds, the unit, the band or the accounting
  after seeing results.
- Pricing msolve work by host wall time inside S, or leaving it out of the
  band.
- Treating msolve start-up or I/O as algorithm work, or silently as zero.
- Quoting S(0) alone for an arm with unpriced solver work.
- Calling `inc` versus `pub-m2` an improvement for either arm while D2 is open.
- Using an msolve build that differs from the pin or has not passed the
  self-test at the session's p.
- Dropping censored executions.

## 10. Analysis

`analysis.sh` (to be written and hashed before freeze) is a thin shell over
`ecbench compare --weight W`, `ecbench band --ranks`, `ecbench bound fit` and
`tools/experiment_protocol.py blind`. Labels stay masked until the decision
table is written. Outputs go to `analysis/`. Reports and graphs follow
`research-visuals`: S against bits per arm and weight, distance to rho
against bits, and msolve multiply-adds per trial against N. The study then
writes `VALIDATION.json` (replay certificate, validator report, matched rho
reference with the band) before any row goes to the leaderboard or to
`benchmarks/README.md`.

## 11. Phase 2 (not in this freeze)

- **`pub-pkm`**: the second published arm. `pkm-semaev` with a new
  `engine=msolve` on `prime-kummer:t=T`, on the Kummer panel curves already
  used by the tuning study (sessions `kummer-P22` and `kummer-P24`), at m = 2,
  3 and, per `RESEARCH_PKM_TOWER_ORACLE.md` §3.7, m = 4 where it finishes.
- **`pub-m2-sym`**: a *derived* F_p analogue of FGHR S_m symmetrisation.
  It uses variables e_1…e_m, `S_(m+1)` written in the e's, and membership as
  the m coefficients of `F_V(T) mod χ_e(T)`. FGHR's symmetrisation is
  published for F_(q^n). Its 2-torsion invariants need rational 2-torsion,
  which the panel curves lack (odd group orders in `docs/curves/registry.json`).
  So this arm is labelled derived, never "published".
- **`pub-m4-chain`**: a feasibility probe at ≤ 12 bits.
