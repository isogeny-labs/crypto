#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set par(justify: true)

= N83 m5: deferred XOR cuts counted capped work
Run 20261027T2135Z · 27 October 2026

On the primary N83 K2048 natural point, native XOR elimination every eighth
decision used *9.555× fewer solver word operations for U* and *10.224× fewer
for D* than every-decision elimination at the same 5,000-conflict endpoint.
All completed searches returned Unknown. Each setting had two identical
work-count observations, with one U baseline observation supplemental after
a scheduled path failure.

#image("cadence_work.svg", width: 100%)

_Figure 1._ Exact `Solver::word_ops()` values, two observations per setting.
The figure data is derived from `evidence/audit.json`. The scheduled eighth
launch failed before model construction and remains a separate row.

== Exact workload and result boundary

Curve `EC1N83Ckb0h252dacc102a4`, subgroup order
`2417851639230796216685689`. Both bases have 2,048 signed-Frobenius columns
and 339,968 points. The same public point was used for U and D, from corpus
SHA-256 `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`.
Every model had 111,478 variables, 1,140,557 initial clauses and 4,980 XOR
rows; five summands and zero split bits. The worker source SHA-256 is
`6402da58e23b2b3e8badf7f8794c615d227b52b9550e3cd639ffd258eba77f03`.

At cadence 1, U used 404,355,869,814 word ops and D used 432,670,001,825.
At cadence 8 both used 42,317,365,862. The two-run spread in each cell was
zero. Cadence-8 search wall ranges were U 15.359–15.430 s, D 15.446–15.657
s; cadence-1 ranges were U 137.989–138.355 s, D 148.408–157.289 s. These
are descriptive shared-host Apple M4 Pro Darwin arm64 intervals. Cadence 8's
identical U/D work trace through the cap may reflect base-independent early
search, so the work reduction alone does not establish relation progress.

== Audit and next gate

The Rust auditor rehashed both archives and the frozen source, replayed the
public point on the exact curve and subgroup, validated eight completed
receipts and the failed-launch row, and recomputed both work ratios. A
one-word-op mutation failed without an audit result. The protocol, source
lock, launch amendment, raw receipts, figure data and verifier are retained
in `research/n83_k2048_m5_cadence_20261027/` and `examples/`.

The next experiment compares at equal word-work allowances and requires an
independently lifted natural relation and rank increment before cadence 8
becomes the pipeline default. A factor-base choice then needs complete log
recovery and a same-point IC/rho pair under a calibrated CPU tier. The
canonical IC leaderboard and progress timeline receive no candidate row
from these capped searches.
