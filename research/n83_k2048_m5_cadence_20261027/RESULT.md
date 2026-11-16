# Deferred XOR reduces bounded N83 m5 solver work

On the primary N83 K2048 five-summand natural query, running native XOR
elimination every eighth decision used **9.555× fewer counted word operations
for U and 10.224× fewer for D** than the cadence-1 default at the same
5,000-conflict endpoint. Both repeats in every setting reproduced the same
work count. All eight completed searches returned `Unknown`; cadence 8 also
gave identical work counters for U and D, so the early search may be dominated
by base-independent circuit work. The measured result is a bounded solver
work reduction, with natural relation yield and factor-base selection still
open.

| Base | Cadence 1 word ops | Cadence 8 word ops | 1 / 8 | Cadence 1 search wall (s) | Cadence 8 search wall (s) |
| --- | ---: | ---: | ---: | ---: | ---: |
| U `public_x_hash` | 404,355,869,814 | 42,317,365,862 | 9.555318 | 137.989–138.355 | 15.359–15.430 |
| D `direct_subgroup_x_d18_prefix2048` | 432,670,001,825 | 42,317,365,862 | 10.224408 | 148.408–157.289 | 15.446–15.657 |

Each work value occurred twice. The U cadence-1 second occurrence is the
post-failure supplemental run described below. These wall intervals are
shared-host diagnostics on an Apple M4 Pro (`Darwin 25.6.0 arm64`) and have
no Tier 0 or Tier 1 timing qualification. The operation ratios divide exact
two-run sums from the [independent audit](evidence/audit.json), which uses
the repository's `Solver::word_ops()` boundary. The [figure](reports/run-20261027T2135Z/cadence_work.svg)
and [PDF](reports/run-20261027T2135Z/REPORT.pdf) visualize that bounded
work comparison.

## Exact workload and method

The [protocol](PROTOCOL.md) fixed the primary `a=0` curve
`EC1N83Ckb0h252dacc102a4`, subgroup order
`2417851639230796216685689`, public corpus SHA-256
`760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`,
and fixture 0 point x/y `1834913121178197242856238` /
`8797964190732846213981377`. Both factor bases have 2,048
signed-Frobenius columns and 339,968 subgroup points. Their compressed
object SHA-256 values are U
`aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`
and D `2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`.
The prior [base construction](../n83_d18_k2048_20261027/RESULT.md)
audited both exact archives. The unchanged chained-S3 model in every run
contained 111,478 variables, 1,140,557 initial clauses and 4,980 native
XOR rows, with five summands and zero split bits.

The worker calls `Solver::set_xor_decision_cadence(1|8)` after model
construction. Cadence 8 skips some intermediate XOR passes while still
checking at root and a complete assignment. The public
[`n83_k2048_m5_cadence.rs`](../../examples/n83_k2048_m5_cadence.rs)
source SHA-256 and release binary SHA-256 were frozen in
[SOURCE_LOCK.md](SOURCE_LOCK.md) before the first search. One fresh process
ran at a time, each with the same 5,000-conflict and 240-second caps. All
completed records have `Fixpoint` root, `Unknown` solver status, and null
candidate, rank and online-time fields. Cadence 8's 8,251 XOR passes and
15,257,233 XOR row scans matched across bases; cadence 1 had 54,410 /
217,750,632 for U and 57,792 / 233,271,852 for D. That exact convergence
at cadence 8 is a reason to test base-specific progress at a larger work
boundary before changing the default.

## Schedule, audit and next gate

The frozen eight-position order was U1, D8, D1, U8, U8, D1, D8, U1.
Positions 1–7 completed. Position 8 exited before model construction due to
a mistyped corpus path and is retained as
[`INPUT_PATH_FAILURE`](evidence/launch-error-08.json). The
[amendment](AMENDMENT_1.md) declared a corrected supplemental U1 run as
position 9 before that run started. The table uses its result as a separately
labeled second U1 observation; it does not overwrite the failed row.

The [Rust verifier](../../examples/n83_k2048_m5_cadence_verify.rs)
rehashes both base archives and the frozen source, checks the public point on
the exact curve and subgroup, validates all eight completed receipt
identities, statuses and model dimensions, and recomputes the two work ratios
from the raw rows. It passed and rejected a changed word-op count in one U1
receipt without writing an audit; the
[negative-control record](evidence/negative-control.json) retains that
failure. The verifier checks receipt consistency and input identity; the
solver's internal work counters are produced by the frozen worker.

The next experiment should compare cadence 1 and 8 at an equal **word-work
budget**, then independently lift any natural model and measure novel rank.
The identical early cadence-8 traces across U and D make that progress gate
essential. A complete factor-base choice still needs target-log recovery and
paired same-point rho under a calibrated Tier 1 or Tier 0 envelope. The
canonical IC leaderboard and progress timeline receive no new candidate row
from these capped searches.
