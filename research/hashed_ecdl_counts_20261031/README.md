# Hashed/tensor ECDL vs rho: counted-S-only amendment (2026-10-31), protocol v2

This directory holds counts-only specs and one `experiment.protocol/v1` record
(`protocol.json`, `crypto-hashed-ecdl-counts-20261031`, **version 2**) for two
panels. They supersede the originals for counted S. The frozen originals are
not edited.

| track | superseding spec | frozen original | binary built from |
|---|---|---|---|
| C, 10/12/14 bits | `spec-C-tensor-counts-10-14.json` | `docs/ecbench/specs/tensor-ic-vs-rho.json` | origin/main `4df337e64d59bbe0145c204048ab1dc69a43c187` |
| C, 16 bits | `spec-C-tensor-counts-16.json` | `docs/ecbench/specs/tensor-ic-vs-rho-16.json` | the same C binary |
| B (native hashed) | `spec-B-hashed-native-counts.json` | `frozen-source/hashed_ecdl_ecbench_20261030/{spec,protocol}.json` (byte-identical copies from `3ae1ac2bd`) | `3ae1ac2bd63d3fb0e86b7e8c0af2b5ec0f1bd09d` (`ic.hashed_native_v1` is not on main) |

The C protocol being superseded is `research/ecdl_tensor_ic_speed_20261030/protocol.json`.
File hashes are in `SOURCES.sha256`. The decision procedure is
`analysis/decide.py`; the protocol freezes its hash.

**Why v2.** The Coordinator decided to revise v1 (`DECISION.md`, reviewed at
`8a678a6b9`). v1 and its merged `spec-C-tensor-counts.json` were withdrawn
before any data and remain in git history. No session has run under either
version.

## What changes and why

Curves (prime roster 10/12/14/16), targets, target seeds, arms, arm
parameters, measurement seeds and per-execution timeouts are the originals'.
Only `measurement` changes.

### Gate-forced (both tracks)

- **`isolation_required` L2 to L0.** On EKS the run-ecbench L2 gate grades any
  solve under 1 ms as L1 (`record.rs` `MIN_SOLVE_WALL_NS_V3`) and exits 42.
  rho at 10-16 bits solves in microseconds: in run gh-41008826427-1, 176 of 192
  records were L1. ecbench documents L0 as enough for operation counts. Run
  with `ecbench run --cpus none`.

### Discretionary (filed before any data)

- **Rounds.** C on 10-14 bits goes from 6 to 1; B goes from 5 to 1. C on 16
  bits already had 1. S is a deterministic function of (workload, algorithm
  seed) and replays exactly, so repeating a seed adds nothing. More rounds
  would add walk seeds per target, which buys power. P1 and P4 do not need
  that power at their expected effect sizes. The cost: **C3 (tensor vs
  ic-mitm) is probably underpowered at one round**, and so is
  B-source-selectors. `no_ranking` is the expected outcome there. It is not
  evidence that the two arms cost the same.
- **Warmup 1 to 0** in all three specs. A warm-up round only absorbs one-time
  wall costs, which counted S does not see. With warmup 0 the measured round
  uses absolute seed index 0, which no measured round of the originals used.
- **C order `alternate` to `alternate_measured` on 10-14 bits.** With warmup 0
  and one round, both orders run the single round in spec order, so the
  execution order is the same. `alternate_measured` matches the 16-bit
  original and avoids ecbench's warning that `alternate` parity counts
  warm-ups.
- **Reverted: the merge and the timeout.** v1 merged the two C specs into one
  with `timeout_seconds: 1800`. That raised the 10-14-bit per-execution
  timeout from 900 s, while v1 claimed the timeouts were unchanged. The
  1800 s in the original C protocol is its job limit. v2 splits C again:
  10-14 bits keep 900 s and 16 bits keep 1800 s, so every per-execution
  timeout now equals its original's.

### Timeouts are wall-clock

`timeout_seconds` is a wall-clock `Duration` (`runner.rs` L1245). At L0,
whether a row ends `verified` or `timeout` depends on the host, most of all
for B's 30 s native rows. As a result, the set of complete (backend, curve)
cells that P4 is judged on depends on the host.

A timeout row is host censoring. It is unknown, never evidence that a method
fails. In an A/A pair, a status mismatch involving `timeout` is censoring and
makes the cell incomplete. It is not a determinism failure. A track is
invalidated only when S or total GAE differs on an A/A pair whose two rows are
both `verified` or both `exhausted`.

## Decision rule

Implemented by `analysis/decide.py decide`, which runs every command:

1. For each family comparison, curve and weight w in {0, 0.1, 1}, run
   `ecbench compare --dir S --a A --b B --weight w --resamples 4000
   --seed 20261002 --json`. The statistic is R = sum S(B) / sum S(A) over the
   8 paired targets, reported as the distance remaining. The primary interval
   is compare's **linearised_t** (df 7, log scale). The cluster bootstrap is
   reported and decides nothing.
2. **Censored rows stay out of rankings.** A cell is complete only when all
   8 + 8 rows are `verified` and compare reports 8 pairs and 0 censored pairs.
   Any other cell is `incomplete_unknown`: it is never ranked and never used
   for P1 or P4. A ratio with an exhausted pair is shown only as descriptive.
   A ratio with a timeout pair is not shown, because `compare.rs` `run_cost`
   would enter that pair at an S implied by wall time. No wall-implied S is
   reported.
3. At each weight, p_w is the two-sided t(7) p-value of ln R / cv, where cv
   is backed out of compare's log-scale interval. The cell's p is the maximum
   of p_w over the three weights.
4. `ecbench band --weights 0,0.1,1 --ranks` supplies only the condition that
   the order holds at 0, 0.1 and 1. Band ranks have no interval.
5. Holm step-down runs over the 32-cell family (C-rho, C-ic, and
   B-rho-{source, selectors, width, partial, ic}, B-source-selectors, each on
   4 curves) at alpha 0.05. The k-th smallest p is tested at
   0.05/(32 - k + 1). Native lower-bound arms can be ranked only for R > 1.

The power check, computed with `analysis/decide.py power` and using no data,
gives the minimum detectable ratio at 8 targets, df 7. It is about 2.5 to 3.7
at the strictest Holm step (0.05/32) and about 1.5 to 1.9 at the loosest, for
rho against arms of different spread. P1 and P4 expect ratios far above these;
C3 does not. `sample_size.power.passes` is therefore false for the family as
a whole.

No runtime, wall-time or speedup number from these sessions is a result. Each
row's gap to rho is reported as the distance remaining, at 0, 0.1 and 1 GAE
per unpriced operation on every arm, rho included (AGENTS.md §5a).

**Caveat.** On 64-bit builds lambda <= 64, so root_u = lambda/131072 = 0 and
public inputs take the finite-size fallback. Nothing here exercises the
recursive subexponential kernel. `ic.tensor_contraction_v1` is an exponential
toy (protocol H1).

## Sizing

These estimates come from the 2026-10-30 session-log sizing (12 bits 3 s,
14 bits 21 s, 16 bits about 417 s per tensor solve):

- **C-small** (96 executions): a few minutes.
- **C-16** (32 executions): about 0.93 h. Each tensor solve fits well inside
  the 1800 s timeout.
- **B** (288 executions): at most about 1.4 h, if every native row uses its
  full 30 s.

## Before running

- Fill and push the resume checkpoint (`scripts/checkpoint.sh init`, `push`,
  `gate`).
- Free disk. On 2026-10-31 the host had about 81 MB free on `/`, which is not
  enough for either release build.
- Build both binaries and record each one's sha256 and `rustc -vV`.
- Check that `ecbench plan` gives 96 (C-small), 32 (C-16) and 288 (B)
  measured executions. This was not checked at freeze, because no binary
  could be built.
- Run B with no concurrent build. Pin each track to its own CPU (`taskset`)
  to cut avoidable wall-clock timeouts.
- Conductor is not installed on this host. The hooks warn and let edits
  through.
