# KUMMER-N13: concrete 16-bit state replay

This bounded replay enumerates every signed state in the three frozen 16-bit
odd-prime subgroups (119,573 states). It calls the actual cached theta xADD
and signed-recovery source and compares outputs to independent Jacobian
arithmetic. It does not establish an all-state bound for the 18 larger frozen
models or an end-to-end collector cost. The cohort gate remains open.

| Field (p) | subgroup (r) | xADD calls | xADD undefined or mismatched | signed lifts returning expected divisor | signed lifts returning no divisor | enumerated exception lifts succeeding |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 739 | 34,147 | 102,441 | 0 | 33,856 | 291 | 170 |
| 1,151 | 41,399 | 124,197 | 0 | 41,256 | 143 | 190 |
| 1,187 | 44,027 | 132,081 | 0 | 43,892 | 135 | 134 |
| **total** | **119,573 states** | **358,719** | **0** | **119,004** | **569** | **494** |

The fast signed-recover branch failed on 1,063 states. The enumerated
exception fallback recovered 494 of them; the final 569 no-divisor states
are **after** that fallback. Thus no states are unaccounted for. The final
routine returned no divisor on 108 `Delta_zero`, 108
`b1_coefficient_zero`, and 353 `general_conversion_inapplicable` states.
These are implementation branch failures, not ambiguity of the abstract
pair-sign theorem. The raw [per-state CSV](results/states.csv) contains every
scalar, the three raw and repaired xADD outcomes, and each signed-lift status
and operation count. The [summary](results/summary.json) binds source and
input SHA-256 digests and the complete CSV digest. The independent native
validator recomputes the counts, operation totals, state continuity, and
digests from those rows.

The CSV, PDF, and graph PNG live in the repository artifact store under the
adjacent `.artifacts.json` manifests. Hydrate them before validation with
`python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n13-concrete-gate`.

The 119,573 lift calls cost 41,887,801 M + 33,639,052 A + 2,268,451 I in
the signed-recovery module after one-time theta inverse setup. Exceptional
enumeration adds 92,925 field M, 1,482 field S, 3,677 field I, 169,982 field
additions, and 741 Jacobian additions. Those module and Jacobian counters
are separate; squarings have not been converted to M, and setup, labels,
collector work, relation processing, and final end-to-end checks are not
included. No rho ratio follows from these counts.

The [graph](status-graph.svg) plots the three 16-bit lift failure counts and
successful enumerated fallback counts. The [PDF](REPORT.pdf) embeds it. The
abstract sign-safe sidecar is [N11](../n11-compressed-sidecar/REPORT.md).

Regenerate and validate from the repository root:

```sh
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n13-concrete-gate
cargo run --offline --release --manifest-path research/kummer_differential_collector_20261022/n13-concrete-gate/Cargo.toml --bin enumerate
cargo run --offline --release --manifest-path research/kummer_differential_collector_20261022/n13-concrete-gate/Cargo.toml --bin validate
python3 research/kummer_differential_collector_20261022/n13-concrete-gate/render.py
convert -background white -density 150 research/kummer_differential_collector_20261022/n13-concrete-gate/status-graph.svg research/kummer_differential_collector_20261022/n13-concrete-gate/status-graph.png
pdflatex -halt-on-error -output-directory research/kummer_differential_collector_20261022/n13-concrete-gate research/kummer_differential_collector_20261022/n13-concrete-gate/REPORT.tex
```

Host: migrated Spot worker, local process, no timing isolation claim.
Source basis: `9d2c5754b8e` plus this packet. Inputs are the frozen
`theta-models.jsonl`, `generic-models.jsonl`, and `target-gate-16.jsonl`.
Source-revision digests are in `results/summary.json`; this packet's exact
files are in `SHA256SUMS`.

**Remaining gate:** classify all concrete xADD states and signed-lift branch
costs on each of the 21 frozen models, prove or independently verify the
exception fallback against every signed Jacobian class, and only then freeze
a new 1,260-target collector and matched rho comparison. This 16-bit census
is a finite exhaustive result for those three subgroups, not a general
formula proof.
