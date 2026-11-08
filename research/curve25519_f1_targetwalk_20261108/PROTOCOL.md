# Protocol: F = 1 walk with the target inside it, single-target ecbench (frozen 2026-11-07)

Frozen before any measured data. Nothing below changes after the first session runs; failed
predictions stay failed and are reported as failed. Before freezing, the only executions were
unit tests (on a 12-bit test curve and the `train-k32` test curve, not the measured curves),
`ecbench plan` (no run), and the cost model `scripts/model.py`, which uses no measured data from
this study.

## 1. Question

The F = 1 walk-table arm of `research/curve25519_f1_walk_20261107` (`wwalk:x=0.7,g=1,grow=2` on
a one-abscissa base) first walks a table until its first self-collision, which pins `log P1`. It
then probes target combinations `Q + kG` against the table until one hits. Its search phase is
about 1.6 sqrt(r): about 1.04 sqrt(r) of walk and 0.57 sqrt(r) of probes (measured in that
study). If the target is put inside the walk, every collision carries information about `log Q`,
and the search phase should drop to the two-collision floor E[sqrt(Gamma(2,1))] = 1.329 sqrt(r)
(`research/curve25519_rho_parity_bound_20261104/README.md`). Does it, and does the end-to-end
cost fall below the current F = 1 arm, measured in ecbench with one target per session?

## 2. Implementation under test

`TargetWalk` (`src/cryptanalysis/ic_boundary.rs`), reached as `wwalk:...,tw=1` (needs `g=1`,
`grow=2` and a base of exactly one abscissa). It uses jumps `J_j = [a_j]P1 + [c_j]G + [b_j]Q` and
entries `E_t = u_t P1 + z_t G + b_t Q`, with `z` and `b` modulo r. Steps are drawn independently
of the point, and one key is stored per negation orbit. Every verified key collision is the row
`du*s1 + db*log Q = -dz`. The walk stops at the first two rows that are independent over
(s1, log Q), or at the cap 3T, T = round(x sqrt(r)). The rows reach the relation loop with the
first trial, and the loop's matrix solves for log Q, which is checked by [d]G = Q. Probes run only
if the cap was reached below rank 2.

`cj=1` (needs `tw=1`) is a separate option. It builds the jumps' G/Q parts by chained additions
from two base combinations, K_0 = [c_0]G + [b_0]Q and K_1, with K_j = K_(j-1) + K_(j-2), and
sets J_j = [a_j]P1 + K_j. Plain `tw=1` uses three independent scalar multiplications per jump.
Every existing `wwalk` path is unchanged: `tw` and `cj` default to 0, and with `tw=0` the oracle
runs the previous code.

## 3. Design (frozen)

- Binary: `ecbench` built from this working tree into `/home/user/ctarget-f1` (release). Its
  SHA-256 is recorded per size in `data/binary-k{20,24}.sha256` when the sessions start.
- Curves: k = 20 (`p = 1,048,573`, `r = 131,203`) primary; k = 24 (`p = 16,777,213`,
  `r = 2,097,349`) replicate. These are the same explicit curves as in the earlier study.
- Sessions: 8 per k, each with `targets_per_curve = 1` (one public target per session) and its
  own `target_seed` and measurement `seed` (`scripts/make_specs.py`). These are new seeds,
  disjoint from the earlier study's. Each arm runs 64 measured rounds plus 1 warm-up, with
  `order = alternate`.
- Arms (identical in every session; all pipeline arms use
  `prime-projected-abscissa:size=1,scan_cap=32` and `targets=stride-generator`):
  - `rho-neg` (reference): `rho.negation`.
  - `f1-x07` (baseline, the current F = 1 arm): `wwalk:x=0.7,g=1,grow=2,seed=run`.
  - `tw-x15` (candidate): `wwalk:x=1.5,g=1,grow=2,tw=1,seed=run` (cap 4.5 sqrt(r)).
  - `twcj-x15` (candidate, **primary**): `wwalk:x=1.5,g=1,grow=2,tw=1,cj=1,seed=run`.
  - No A/A arm. In the earlier study, rho-aa and rho-neg were identical run for run, because
    ecbench seeds runs by workload and round, so an A/A arm measures nothing here. Noise is
    carried by the between-session t intervals.
- Counted unit: ecbench GAE; `S = cost/sqrt(r)`; every comparison at w = 0, 0.1, 1 (AGENTS.md section 5a).
- Audit: `ecbench verify --replay 1` per session.
- Pooling: `scripts/pooled_compare.py`, copied unchanged from the earlier study, including its
  warm-up fix (records with `"warmup": true` are skipped). Per session, sum S over the measured
  rounds per arm; take the ratio of sums; use a linearised t interval with df = 7. One session is
  one target and one cluster.
- Phase breakdown, `scripts/analyse.py`. Tw arms: the `tw_*` relation counters (set-up additions
  and doublings, walk additions, verification additions, steps). `f1-x07`: walk steps =
  `table_insert_attempts`; set-up plus verification = oracle-set-up GAE minus walk; probes =
  relations-phase GAE. `rho-neg`: its `setup`, `search` and `internal_verification` phases.
- Runtime cap: about 30 minutes in total. The earlier study took about 1 minute per size.

## 4. Model (derived, before data)

`scripts/model.py` (output `data/model.json`) is a Monte Carlo (20,000 draws) of the counted
operations. It follows the build rules in the code: doubling chains, jump construction from
popcounts, walk to the first or second collision with n_k^2/r ~ Gamma(k, 1) (negation-folded
uniform walk), per-collision verification by recomputation, and stride probes for `f1-x07`.
Fixed factor-base and [d]G-check costs are taken from the earlier study. The model's `f1-x07`
total, 2.273 (k = 20) and 1.770 (k = 24), sits 2-3 % below that study's measured 2.321 and 1.828.

| k | arm | set-up | walk | verify | probes | total S (w = 0) |
|--:|:--|--:|--:|--:|--:|--:|
| 20 | f1-x07 | 0.507 | 0.982 | 0.077 | 0.588 | 2.273 |
| 20 | tw-x15 | 0.947 | 1.329 | 0.245 | 0 | 2.640 |
| 20 | twcj-x15 | 0.368 | 1.331 | 0.245 | 0 | 2.063 |
| 24 | f1-x07 | 0.159 | 0.980 | 0.024 | 0.572 | 1.770 |
| 24 | tw-x15 | 0.294 | 1.335 | 0.077 | 0 | 1.740 |
| 24 | twcj-x15 | 0.110 | 1.333 | 0.076 | 0 | 1.554 |

Model ratios at w = 0: twcj/f1 = 0.907 (k = 20) and 0.878 (k = 24); tw/f1 = 1.162 and 0.983;
twcj/tw = 0.781 and 0.893. Plain `tw=1` pays three scalar multiplications per jump (G, Q and the
small P1 part). At k = 20 that set-up outweighs the shorter search. The chained jumps remove most
of it.

## 5. Predictions and thresholds (frozen)

"Ratio" is the pooled ratio of sums with its 95 % interval, over 8 sessions per size.

- **P1 (primary).** k = 20: S(twcj-x15) / S(f1-x07) < 1 at all of w = 0, 0.1 and 1 (upper end
  < 1.00 at each weight). The w = 0 point estimate is expected in [0.80, 0.97] (P1b, scored
  separately).
- **P2.** k = 20: S(tw-x15) / S(f1-x07) at w = 0 has its lower end > 1.00. Without chained
  jumps the target-in-walk arm is predicted to lose at k = 20.
- **P3 (replicate).** k = 24: S(twcj-x15) / S(f1-x07) at w = 0 has its upper end < 1.00.
- **P4 (search phase).** For both tw arms at both sizes, the mean walk length (`tw_steps`)
  divided by sqrt(r) lies in [1.26, 1.45]. This is the two-collision floor 1.329 times a walk
  overhead factor of 0.95-1.09. The earlier study's table walk ran about 6 % above its model.
- **P5 (cheap jumps, measured on their own).** S(twcj-x15) / S(tw-x15) at w = 0 has its upper
  end < 1.00 at both sizes.
- **P6 (no parity).** S(twcj-x15) / S(rho-neg) has its lower end > 1.00 at w = 0, 0.1 and 1 at
  both sizes. The w = 0 point estimate is expected in [1.20, 1.55] (P6b, scored separately).
- **P7 (rank-2 stop).** At least 99.9 % of tw-arm records (`tw_rank = 2`) solve from the walk's
  two rows with zero probe hits, on the first trial.
- Every record must be `verified`, and every session's `ecbench verify --replay 1` must
  reproduce. A session that fails the audit is excluded and reported; it is not rerun with
  another seed.

## 6. Reporting

README.md tabulates measured S per arm and weight (with source file), the pooled ratios, the
held/failed status of P1-P7, the per-arm phase breakdown, and the distance to rho-neg at w = 0,
0.1 and 1. Every result sentence ends with [pcc:unverified]. Scope: an F = 1 method with the
target inside the walk is one unknown away from rho. Setting P1 := G (known log) turns it into
a rho-style random walk on `[c]G + [b]Q` (F = 0). The README states this plainly, and the method
is not presented as decomposition-based index calculus.
