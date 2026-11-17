---
title: "Protocol: k-sum table against a tuned mitm incumbent on fresh curves, with a frozen choice of k"
date: 2026-10-29
status: registered before any timed run (hashes in REGISTRATION.sha256; the holdout specs are hashed in select/FROZEN.sha256 before the holdout runs)
---

# Protocol: k-sum table against a tuned mitm incumbent on fresh curves, with a frozen choice of k

**Track: generic decomposition (bounded by √r; cannot beat rho).** Both arm families are generic decomposition oracles (AGENTS.md "Tracks of index calculus", 2026-10-29). Nothing here can move the algebraic track.

## 1. Questions (follow-ups 2 to 4 of the #2866 list)

#2866 ranked the k = 5 table against the *recorded* mitm incumbent on one holdout curve per size. Three things limit it.

1. **Q2, one curve per size.** Does the ranking hold on fresh curves, pooled, with an exponent?
2. **Q3, an untuned baseline.** The incumbent's base sizes are the recorded ones, not re-tuned. How much of the 0.22–0.70 ratio is method and how much is tuning?
3. **Q4, no rule for k.** k = 4 against k = 5 is unresolved at 20 and 24 bits. Does a rule frozen on train data choose the better k on fresh curves?

## 2. Design

All sizes are 20, 24, 28 and 32 bits. All curves are fresh `prime_search` curves (none appears in the earlier studies; the resolved ids are in `plans/`).

**Train panel** (`specs/train-P<b>.json`): 2 curves per size (seeds 20261029100 + 10b + {0, 1}), 12 public targets per curve (target seed 20261029111 + b), 2 rounds, 1 warm-up (measurement seed 20261029121 + b). Arms:

- `rho-neg`, `bsgs-neg`: references.
- `inc-n<N>`: `ic.pipeline`, `prime-abscissa:size=N`, `mitm:negation_folded=1`, N in 0.5, 0.71, 1, 1.41, 2 times N0, with N0 = 32, 162, 407, 1024 (the incumbent recipe's base before #2811). At 20 bits N = 45 (the promoted selection) is also run.
- `ksum4-a<A>`: `ksum:m=4`, A in {0.5, 0.71, 1, 1.41, 2} times A0, A0 = 10, 14, 21, 30 (#2808).
- `ksum5-a<A>`: `ksum:m=5`, A in the same grid times A0 = 9, 11, 15, 19 (#2852).

**Selection** (`select.mjs`, run once, output hashed in `select/FROZEN.sha256` before any holdout run):

- **Rule.** For each size and each family (`inc`, `ksum4`, `ksum5`), take the cell with the smallest mean S **at w = 1** over cells verified on every train run. Ties go to the smaller table.
- **Why w = 1.** It charges the native work in full, so a selection cannot be won by moving work into uncharged operations (the 24- and 28-bit selections of `prime_base_oracle_tuning_20261029` were rejected as relabelling for that reason). The same rule applies to the incumbent and to both k-sum families.
- **Edge picks.** A pick on the edge of the grid (0.5 or 2) is run once with no extension and is reported as a limit.
- **Choice of k (Q4).** `k*(b)` is the family among `ksum4` and `ksum5` with the lower selected train mean S at w = 1. Ties go to k = 4. `k*` is a label on the holdout results; both selected k-sum arms are run on every holdout curve, so the rule's choice can be scored against the arm it did not choose.

**Holdout panel** (`specs/holdout-P<b>.json`, generated from the selection): 4 curves per size (seeds 20261029200 + 10b + {0..3}), 24 targets per curve (target seed 20261029211 + b), 2 rounds, 1 warm-up (measurement seed 20261029221 + b). Arms: `rho-neg`, `rho-neg-aa` (A/A), `bsgs-neg`, `inc-sel` (the tuned incumbent), `inc-w0` (the incumbent cell with the smallest train mean S at w = 0; run only when its N differs from `inc-sel`'s), `inc-rec` (the recorded recipe of #2866: N = 45, 162, 407, 1024), `ksum4-sel` and `ksum5-sel`. Every run is one cold target, with the table built and charged inside the run.

**Binary.** `ecbench-v2`, sha256 fc86ee11178da3bc373abecfaa1de532939637ab92fb86ecadf92e9ee1f9c195 (`BINARY.sha256`), the binary of #2852 and #2866. It contains both oracles. If the session count or a replay differs, the study stops at that size and the cause is recorded before anything is compared.

## 3. Registered tests (AGENTS.md §5a band: w = 0, 0.1, 1)

All tests use `ecbench band --ranks` and `ecbench compare --weight`, pooled over the 4 holdout curves of a size (cluster-bootstrap 95% intervals, 4,000 resamples, seed 20261002).

**Primary (Q2 and Q3 together).** At each of 24, 28 and 32 bits, `ksum5-sel < inc-sel` **and** `ksum4-sel < inc-sel` at all three weights, with the upper bound of the 95% interval below 1 at w = 0. The test is on the tuned incumbent, not the recorded one. At w = 0 the comparison must also hold against `inc-w0` when that arm exists: the incumbent gets its best cell at that weight, and the k-sum arms keep their w = 1 picks, which handicaps k-sum. It holds only if it holds in all three sizes.

**Secondary.**

- **(a) Per-curve consistency.** In each size, the k-sum family with lower pooled S is below `inc-sel` on at least 3 of the 4 curves, at w = 0.
- **(b) 20 bits.** The same rule as the primary test.
- **(c) Recorded incumbent.** The `inc-rec` ratios, to link to #2866, and the tuning effect `inc-sel / inc-rec`.
- **(d) Rule for k.** At each size, the family chosen by `k*(b)` is not above the other family, at w = 1, outside the pooled interval. Scored per size, with no pooling across sizes.
- **(e) Exponent.** `ecbench bound fit --tier toy --param-rule` at w = 0, 0.1 and 1 for `inc-sel`, `ksum4-sel`, `ksum5-sel`, `rho-neg` and `bsgs-neg` over the four sizes, with the exponent difference reported with its interval. A refusal by the fit is reported as a refusal.
- **(f) Distance remaining.** The pooled ratios `ksum*-sel / rho-neg` and `/ bsgs-neg`.

**What the pooled interval covers.** `ecbench compare` reports an interval over workloads (targets). If its `interval_scope` stays `workloads` on a pooled multi-curve session, the pooled interval is target-level only: it does not carry between-curve variance. Robustness across curves is then judged by test (a), from the per-curve means of `ecbench band`, and the primary test additionally needs test (a) to hold. The `interval_scope` is read from the train sessions and recorded in `analysis/interval-scope.txt` before the holdout runs.

**Holdout curve freshness (checked before `FROZEN.sha256`).** The resolved holdout curve ids must share nothing with this study's train curves, the four curves of #2866, the train curves of `prime_base_oracle_tuning_20261029`, or the frontier curves of #2808 and #2852. The check is written to `plans/freshness.txt`. If a curve repeats, its seed is bumped by 100 and the plan re-resolved, and the bump is recorded.

## 4. Predictions (registered)

| bits | `ksum5-sel / inc-sel` at w = 0 | basis |
|--:|:--|:--|
| 20 | 0.65–0.95 | #2866: 0.70 against N = 45, which is already the promoted selection |
| 24 | 0.40–0.55 | #2866: 0.45; the recorded N = 162 is near the incumbent's train optimum |
| 28 | 0.30–0.45 | #2866: 0.33 |
| 32 | 0.20–0.32 | #2866: 0.22; the incumbent's own train grid had N = 724 at 3% below N = 1024, so tuning moves it by a few percent, not a factor |

- **Tuning effect.** `inc-sel / inc-rec` lies in 0.9–1.05 at 24, 28 and 32 bits (Q3: the gap is the method).
- **Rule for k.** At 20 and 24 bits k = 4 is chosen at least once; at 28 and 32 bits k = 5 is chosen.
- **Exponent.** The `ksum5-sel` toy-tier exponent is below the `inc-sel` exponent by at least 0.1.
- **Limit.** `ksum*-sel` stays above `bsgs-neg` at every size.

## 5. Reading rules

- If the primary test holds, k-sum is the better generic-track recipe against a *tuned* incumbent on fresh curves, pooled. That is a measured improvement of the generic baseline on these sizes (class engineering, labelled generic).
- If it fails or is unresolved in any size, #2866's ranking is reported as not robust to tuning or to the curve, with the sizes that fail.
- Registered, one host, no second-host replay: the result is UNQUALIFIED. The second-host replay specs are prepared in `second-host/` and are not run by this session.
- Counted unit only. Wall time is recorded, never admitted.

## 6. Analysis

Native `ecbench` only: `verify --replay-all` on every session, `band`, `compare`, `bound fit`. Tables and graphs come from node scripts (tooling). The report, graphs and PDF follow the research-visuals skill. The `benchmarks/` entry and the §7a dashboard point (the best cold one-target IC cell of the holdout panel) are owed **whatever the outcome**: every arm yields an IC/rho ratio, and a failed primary test is still a measured result. The dashboard script is Python-free.
