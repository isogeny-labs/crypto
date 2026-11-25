# Source-pinned degree-32 equal-18 panel — result — 2026-11-25

## What was run

Binary SHA-256
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`, built
from commit `a5503f4bb66ac3aa4eac3125fba01534d2574096`. The previous
16-abscissa replay failed the size gate (source-pinned isogeny arm produced 18
abscissae on `icv1-fp13-t19-0d05ae31`). This panel is the replacement
source-pinned test at the size the binary actually produces.

Selection: a frozen size-only scan of 13-bit seeds `20261181..20261280`
(`source_pinned_size_scan_20261125.jsonl`) compared
`prime-isogeny:t=5,seed=5,aux=ordinary` with `prime-abscissa:below=32`. Three
seeds matched at exactly 18 abscissae; the panel is those three instances, per
`AMENDMENT_F4_DEGREE32_SOURCE_PINNED_EQUAL18_PANEL_20261125.md`.

Each instance ran interval, candidate, candidate, interval with 16 repeats,
m=2 Semaev F4 chain mode, degree bound 8, 1,000 target trials, a five-second
solver-call budget, one rho context run and a 240-second cap. All four
invocations completed, both arms stayed at 18 abscissae and all 16 recovered
scalars agreed.

## Per-instance results

Ratios are candidate / interval. Lower is better.

| Seed | Instance | Combined GAE | Pair 1 GAE / wall / CPU | Pair 2 GAE / wall / CPU | Relations (int/cand) | Target trials (int/cand) | Median s/rho (int/cand) |
|---:|:--|---:|:--|:--|---:|---:|---:|
| 20261185 | `icv1-fp13-tm13-9e2d11f6` | **0.5100** | 0.6917 / 0.5794 / 0.2260 | 0.3195 / 0.4826 / 0.1677 | 438 / 416 | 3332 / 3252 | 906,007 / 397,508 |
| 20261221 | `icv1-fp13-tm101-1737775c` | **0.1702** | 0.0933 / 0.1597 / 0.2380 | 0.2599 / 0.2760 / 0.1615 | 426 / 444 | 4432 / 4260 | 1,180,590 / 167,650 |
| 20261276 | `icv1-fp13-tm137-9a89da76` | **0.2442** | 0.2804 / 0.2140 / 0.2764 | 0.2153 / 0.2083 / 0.2744 | 378 / 408 | 5006 / 5942 | 1,161,246 / 247,586 |
| **Combined** | three instances | **0.3045** | all pairs below 0.9 / 1 / 1 | | **1242 / 1268** | **12770 / 13454** | |

Combined charged GAE: 16,651,120,237 interval / 5,069,800,769 candidate.
Raw relations are 1.021x higher for the candidate; target trials are 1.053x
higher for the candidate. The component gate passed on all three instances.

Stores:

- `degree32_source_pinned_equal18.jsonl` (20261185)
- `degree32_source_pinned_equal18_20261221.jsonl`
- `degree32_source_pinned_equal18_20261276.jsonl`

## Claim boundary

This is a source-pinned, selected-instance, small-field relation-extraction
component lead. It shows the five-step degree-32 PKM map produces useful
two-summand relations at roughly 0.17–0.51x the charged GAE of an equal-size
interval on three 13-bit instances, with all 48 scalar checks passing.

It does not establish:

- a complete same-target advantage over optimized rho — candidate median
  `s_over_rho` remains **1.7e5–4.0e5**;
- a source-exact replay of the original 16-abscissa result — the rebuilt
  binary produces 18 abscissae on that instance;
- a random-instance or curve-family replication — the three instances were
  selected from a 100-seed size scan;
- an end-to-end advantage after full relation collection, matrix construction,
  linear algebra, descent and failed-attempt charging;
- transfer to 14-bit, s59, P-192 or the NIST curves;
- any asymptotic improvement.

The next gates are a 14-bit equal-size panel, a complete cold same-target
pipeline comparison, and an instrumented explanation of why the degree-32
membership equations reduce F4 cost.
