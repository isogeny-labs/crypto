# Round 315: amortized multi-target transported logs reach per-target parity at T=16

Date: 2026-10-17

**With one shared cold known-log base and folded pair table charged once and
divided over 16 public targets, the hash-scheduled transported-log
candidate costs 0.686 times the matched strong signed-Frobenius rho
one-target reference per target, 95% cluster interval [0.630, 0.740].**
This is the first measured per-target parity result in this lane.  It is a
**multi-target amortization mechanism screen, not a one-target speedup**:
Round 314's one-target gates stay failed exactly as measured, and a rho
fleet sharing distinguished points across targets can also amortize
(the standing control of PR #1987).

The experiment follows the frozen
[preregistration](PREREGISTRATION.md).  The frozen target count T=16 was
selected by the preregistered development sweep (`T in {1,4,8,16}` at the
development seed) before the holdout was opened: at `T=8` the amortized
per-target ratio was 0.866 with an `ecbench compare` interval whose upper
bound still included 1.00, at `T=16` it excluded 1.00, so T=16 froze.

## Boundary table

The unit is counted GAE.  The candidate's whole-execution cost is divided
by the named target count `T`; rho solves one cold target per execution
with no reusable precomputation.  Every row is a lower bound (unpriced
native work counted but not priced).  Ratios are paired over 40 measured
one-target workloads on `icv1-f2m37-tm534059-32aad96b` (log2 r = 27.78).

| session | T | candidate mean total GAE | amortized per-target GAE | rho mean GAE | amortized / rho | 95% cluster interval |
|:--|--:|--:|--:|--:|--:|:--|
| holdout-n37-t8 | 8 | 28,255.70 | 3,531.96 | 4,077.92 | 0.866 | [0.756, 0.969] (analyzer bootstrap); [0.731, 1.022] (`ecbench compare`) |
| development-n37-t16 | 16 | 46,628.23 | 2,914.26 | 4,173.79 | 0.698 | [0.645, 0.753] |
| **holdout-n37-t16** | **16** | **44,745.32** | **2,796.58** | **4,077.92** | **0.686** | **[0.630, 0.740]** |

The shared cold cost (base + folded pair table + transport verification) is
7,106.13 counted GAE per execution, identical across all three sessions;
the frozen `K=11`, scalar cap, and hash schedule are Round 314's, not
retuned.  At `T=16` the per-target online work is 84.1% of the amortized
unit; the shared table amortizes to 15.9%.

## Correctness and replay

- 432/432 records across the three sessions verify; the record's recovered
  logarithm answers the workload's own public target.
- Every one of the 16 targets per execution is recovered and verified by a
  charged full-point `[d]G = Q` replay inside the solver; 0 wrong answers.
- 360/360 measured records replayed bit-for-bit under
  `ecbench verify --replay-all`; each session's audit reports
  `144 records, 144 verified, 120 replays reproduced, 0 problems`.
- The A/A control arms are byte-identical in cost (exact counted A/A).
- All executions ran at L0 on the development host (no CPU topology on
  this OS), so wall time is descriptive only; operation counts are the
  metric, as in Round 314.

## Promotion decision

| gate | result |
|:--|:--|
| every record verifies and replays | pass |
| exact counted A/A | pass |
| amortized per-target `S(T)/S_rho <= 1.00` at the frozen T=16 with a 95% paired interval excluding 1.00 | **pass: 0.686 [0.630, 0.740]** |
| shared table/verification/base costs reported separately from per-target online cost | pass |
| T named in every figure | pass |
| one-target Round 314 gates | unchanged: fail (this round makes no one-target claim) |

Round 315 closes as a **positive multi-target amortization result at the
degree-37 cell**: engineering class in AGENTS.md §3 terms (the ratio to
rho moved, not the mechanism's asymptotic class), with the explicit
multi-target framing AGENTS.md requires for amortized measurement.

## What this does and does not claim

- It does not claim the candidate beats rho on one unseen target; at T=1
  the Round 314 result (2.121x) stands.
- It does not transfer to P-256: the P-256 Dickson membership gate
  (rank 0, K=131,458) is untouched, and multi-target amortization does not
  change factor-base membership.
- A distinguished-point rho fleet sharing walk state across many targets
  is the known control that can also amortize; the comparison here is
  against the repository's standard cold one-target rho reference, and any
  "cheaper than rho" reading must carry that caveat.
- Scaling beyond n=37 is not measured; the four-size exponent law of the
  one-target candidate (0.474) is unaffected by amortization, which
  divides a constant away.

## Reproduction

The deterministic analysis is [`ANALYSIS.json`](ANALYSIS.json), produced by
`examples/transported_log_amortize_round315_analyze` over the three
committed session directories.

```bash
cargo test --release --lib transported_logs
cargo build --release --bin ecbench --example transported_log_amortize_round315_analyze

for d in holdout-n37-t8 development-n37-t16 holdout-n37-t16; do
  target/release/ecbench verify \
    --dir research/ecbench_transport_amortize_round315_20261017/sessions/$d \
    --replay-all --exit-code
done

target/release/examples/transported_log_amortize_round315_analyze \
  research/ecbench_transport_amortize_round315_20261017/sessions/holdout-n37-t8 \
  research/ecbench_transport_amortize_round315_20261017/sessions/development-n37-t16 \
  research/ecbench_transport_amortize_round315_20261017/sessions/holdout-n37-t16 \
  /tmp/round315-ANALYSIS.json
cmp research/ecbench_transport_amortize_round315_20261017/ANALYSIS.json /tmp/round315-ANALYSIS.json
```
