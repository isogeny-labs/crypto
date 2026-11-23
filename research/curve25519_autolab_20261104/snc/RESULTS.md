# SNC-1 and SNC-2 Stage 0 results

Status: [pcc:unverified]. No pcc claim id. Records: SNC-1 and SNC-2 in `../ideas/SYNTHESIS-NC-20261108.md`, section 4
(on branch `origin/claude/bsgs-k20`; it is not on main) [pcc:unverified]. Zero group operations (A) and counter
reading only (B); no ecbench panel was run.

## Task A: SNC-2 Stage 0, chained C = 2 walk

Tool: `src/bin/nc2_stage0.rs --chained` (design and intervals pre-registered in the source header, commit 8baee9c695,
before the first chained run) [pcc:unverified]. Commands: `sh research/curve25519_autolab_20261104/snc/run_snc2.sh`
from the repository root, seed 20261108, raw output in `raw/snc2-*.txt`. Tests: `cargo test --release --bin nc2_stage0`
(11 passed); `cargo clippy --release --bin nc2_stage0 --tests -- -D warnings` clean.

Controls (all inside their pre-registered intervals) [pcc:unverified]:

| control | interval | observed |
|--|--|--|
| D0i, n = 20000, k20 / k24 / k28 (first r) | 1.652 to 1.672 | 1.6666, 1.6613, 1.6638 (SE 0.0035) |
| D0 (NC-2 walk), n = 2000, nine curves | 1.66 to 1.76 | 1.684 to 1.740 |
| CE (CH with uniform seed coordinates), n = 2000, nine curves | 1.66 to 1.77 | 1.714 to 1.738 |

Mean E in sqrt(r) units, n = 2000 per curve, per-k value = mean of three curves, SE = sqrt(sum se^2)/3 [pcc:unverified]:

| k | CH (chained) | D0 | D0i | CE | falsification (CH > 1.82) |
|--:|--:|--:|--:|--:|--|
| 20 | 1.734 +- 0.0065 | 1.730 +- 0.0066 | 1.672 +- 0.0062 | 1.731 +- 0.0065 | no |
| 24 | 1.722 +- 0.0065 | 1.712 +- 0.0065 | 1.657 +- 0.0062 | 1.728 +- 0.0065 | no |
| 28 | 1.718 +- 0.0066 | 1.730 +- 0.0065 | 1.654 +- 0.0064 | 1.724 +- 0.0065 | no |

Per-curve CH values are in the raw files: 1.713 to 1.749. Reading [pcc:unverified]: CH matches the record's
prediction 1.75 [1.70, 1.82] at all three sizes and sits 3.4 to 4.3 percent above the Poisson floor 1.662, the
same walk excess as D0 (CH minus D0 is within about 2 SE at every k). Chaining and small seed coefficients cost
nothing measurable against random jumps; they also gain nothing over D0 (CE equals CH). SNC-2 is not dead at Stage 0.
This is the E of the walk only: the prediction of S needs set-up and verification (Stage 1). Distance to the Poisson
floor is 0.06 to 0.07 sqrt(r); the K = 3 floor is itself 1.66 to 1.79 times fresh rho at k = 28 and 32, per the record.

Design choices fixed in advance, not tuned: 16 jumps, `a, a'` uniform in [1, M] with M = 30 / 60 / 121 at k = 20 /
24 / 28, jumps, start and stream fresh per sample. Not modelled: the real oracle's probe guard and canonicalisation.

## Task B: SNC-1 Stage 0, verify share of the incumbent C = 2 wwalk arm

Source: committed `c1/sessions/confirm-k{20,24,28,32}/records.jsonl`, arm `inc-c2`, 120 scored records per size
(`ecbench.record/v1`; the multicurve session records are not in the checkout, only their artifact manifests).
The `wwalk` oracle exports no verify counter (`breakdown_counters` is `TargetWalk` only), so the numbers are DERIVED:
per record, `A = oracle_setup.adds - table_insert_attempts` is set-up plus verification adds; `C = free_collisions_checked`
(every collision is verified by recomputing both entries, `verify_pending`, two `eval` per collision). OLS of A on C
over the 120 records gives the adds per collision as the slope. Command: `python3 research/curve25519_autolab_20261104/snc/snc1_verify_share.py`
from `research/curve25519_autolab_20261104`, output in `raw/snc1-verify-share.txt` [pcc:unverified].

| k | incumbent | mean collisions C | adds per collision (OLS, SE) | verify share, sqrt(r) units | share of S (mean S) |
|--:|--|--:|--:|--:|--:|
| 20 | `wsum:coef=15,free=1` (box) | 0.00 | 2 at most (see below) | 0 observed | 0 |
| 24 | `wwalk:x=0.9,g=1,grow=2` | 2.10 | 46.5 (2.6) | 0.067 [0.064, 0.071] | 3.2 percent (2.134) |
| 28 | `wwalk:x=0.9,g=1,grow=2` | 2.16 | 61.5 (2.8) | 0.023 [0.022, 0.024] | 1.3 percent (1.832) |
| 32 | `wwalk:x=0.6,g=1,grow=2,targets=walk` | 1.73 | 65.5 (2.2) | 0.0049 [0.0047, 0.0050] | 0.3 percent (1.874) |

All figures DERIVED, [pcc:unverified]. The intervals carry only the slope SE, not the model assumption (intercept
constant across records; identity steps negligible). Cross-check: a hand estimate of one `eval` (popcount of
alpha, beta, gamma, minus one addition) gives about 25 adds, so 49 per collision at k = 24, against the fitted 46.5.
The record's prediction was 0.08 [0.04, 0.12] / 0.02 / 0.006; the fits are 0.067 / 0.023 / 0.005, inside it.

Falsification (dead for the main track if verify is below 0.04 sqrt(r) at k = 24): 0.067 is above it, with the lower
slope bound 0.064, so SNC-1 survives Stage 0. The lever is small: about 0.07 sqrt(r) at k = 24 (3.2 percent of S)
and under 0.03 beyond. This sizes it; a counter subtraction is not a result.

`wsum` with `free=1` (the record's open item), from the code: `WSumTable::free_rows` (ic_boundary.rs, around line 7897)
recomputes both sums of every key collision with `evaluate` charged to `ops`, so it does recompute. Each sum is at most
C - 1 = 1 addition for the box, so at most 2 adds per collision. In the 120 k = 20 records the counter
`free_collisions_checked` is 0 in every one, so the box's verification cost is 0 in these records and extending SNC-1
to the k = 20 box has nothing to remove [pcc:unverified].

## Verdicts [pcc:unverified]

* SNC-2: survives its Stage 0 line (chained E 1.734 / 1.722 / 1.718, none above 1.82). Controls in range.
* SNC-1: survives its Stage 0 line (derived verify 0.067 sqrt(r) at k = 24, above 0.04). The gain is bounded by
  3.2 percent at k = 24 and about 1 percent at k = 28.
* Neither result is a speedup [pcc:unverified]; both are Stage 0 sizing. Stage 1 (the shared panel) is not run.
