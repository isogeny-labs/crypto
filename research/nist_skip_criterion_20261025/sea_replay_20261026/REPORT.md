# Exact replay of the SEA coverage proxy

26 October 2026. All21 recorded values replay exactly. The three fixed
arithmetic controls passed, both fresh-process cohort arms exited0 and
all outputs were byte-identical. Counts, indices, primes and bounds in this
report are decimal. These are exact finite records, without sampling intervals.

The separate native code recomputes `t=p+1-n`, `D=t^2-4p`, then multiplies
odd primes with Kronecker symbol `(D/l)>=0` until the product exceeds
`2*floor(sqrt(4p))`. The result is compared with every recorded
`sea_elkies_only_lmax` in the existing native feature tables. Despite that
old name, ramified primes are included,2 is omitted and Atkin information
is unused. This is a coverage proxy, not measured SEA point-counting cost.

![All21 recorded coverage bounds](reports/coverage.svg)

A uniform preference for the larger value cannot explain the skipped curves:
published P256 has191, while skipped876 has211. A uniform preference for
the smaller value also has a counterexample: published P224 has257,
while its skipped777,1485 and5736 have139,179 and233. These are exact
counterexamples for this particular metric, not a statistical claim about
random curves or a rejection of every possible point-counter artifact.

The exact [cohort table](results/coverage.tsv) retains all21 rows, including
published P384 and P521. [The native summary](results/summary.json) records
21 matches and three arithmetic controls. The controls are public fixed
integer pairs: p103,t0 gives13 from7,13; p101,t2 gives13 from5,13, while
excluding ramified primes changes the latter to17. They are arithmetic
fixtures, not additional eligible NIST curves. A/A equality checks
reproducibility only; no independent host or variance was measured.

Execution used the committed [protocol](PROTOCOL.md) and C++17/strict-warning
[replay source](replay.cpp), compiled before the run, with PARI2.17.3 and the
existing native common header. The source snapshot was
e028a12d0b43bc8892b13afd67a776501e77ebae. The
[run launcher](run.sh) passed the S3 gate before executing any target.
Source, input and binary hashes, all statuses, copied input features and
both arms are in [the raw archive manifest](evidence/manifest.json).
The four-core/16GiB host also ran GNFS014; a17-second verified process-group
pause covered consistent checkpoints and this small replay. No clock,
throughput, isolation-tier or performance claim follows.

The [reference record](REFERENCES.json) binds the official installed PARI
documentation. Online requests timed out. The existing
[native feature contract](../native/features.cpp), original
[search protocol](../SEARCH_PROTOCOL.md) and
[curve-origins article](https://www.anthropic.com/research/ecdsa-curve-origins)
were checked. That primary article still leaves the historical skip rule
unknown. The shared log's exploratory Python association is not admitted as
a result here; no new p-value or unusualness claim was computed.

The P521 n+1 factorization and historical criterion remain unresolved. This
exact replay does not complete either requirement, execute period ECPP or
establish a selected-curve or skipped-curve vulnerability. Original source
and raw results remain unchanged; only this source/data report and visual
are added. Canonical attack-cost and IC leaderboard graphs stay unchanged.
