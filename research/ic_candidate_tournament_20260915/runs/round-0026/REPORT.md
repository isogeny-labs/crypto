# Native index-calculus tournament: round 0026

**Decision:** `batch16` reduces complete cold instruction cost on this bounded
panel, with unchanged factor-base support and no exponent claim. The native
decision file passes its registered incumbent gate, including a wall-time
condition. Because the hosted runner recorded no core reservation or contention
receipt, that wall-time condition is **unqualified** under AGENTS.md §10;
promotion awaits an isolated confirmation on a newly registered round. The
instruction result is an **engineering** improvement. The selected configuration
remains above the paired rho reference on all five curves.

The [protocol](../../goal_20261028/native_pilot/PROTOCOL.md) was committed
before [workflow run 40721688128](https://github.com/isogeny-labs/crypto/actions/runs/40721688128)
used seed `2026102801` at source commit
`e50d1b78ea9a0e8a53f1c735b5f5b2151d7c987a`. Ubuntu 24.04 amd64,
Rust 1.94.1 and Valgrind 3.22.0 measured one public ECDLP target per cold
process. The round's contract records the exact host, compiler, limits,
fixtures, executable and source digests. Its native audit re-derived every
summary and decision and verified 1,632 trial receipts and 1,693 frozen
source files. A separate amd64 container replay of the downloaded archive
returned the same audit counts. All 1,632 trial receipts have `VERIFIED`
status: 24 A/A, 96 smoke, 288 development, 144 selection, 540 confirmation
and 540 replay. The holdout is `n19a1`.

## Boundary and accounting

The common measured unit is the full process's user-space **Ir** (guest
instruction reads), from startup through factor-base and table setup,
relation collection and point decomposition, verification/filtering and
scalar-field linear algebra, individual log/descent, final verification and
reporting. `S = Ir / sqrt(r)` is reported in the
[native measurements](measurements.json); `2^x Ir` below is the logarithm of
mean complete cold Ir. Neither is a group-addition equivalent, and there is
no measured conversion to counted GAE. Native cold wall time is an unisolated
secondary diagnostic, excluded from any qualified speed claim.

The deliberately weak rank floor is `Ir >= K`, where `K` is the verified
factor-base rank: at least one relation-producing trial per independent
column. It is not an attack lower bound. At confirmation the winner costs
2.22–4.85 million times that floor across the five cells. The paired rho
cost on the same target is the reference; `1.0×` is parity, and a value above
it means IC costs more. This rho arm has not been qualified as the strongest
available implementation. The older round 0025 used a different source and
panel; its values remain historical rather than a direct before/after pair.

## One-unit candidate table

All seven IC configurations and rho were verified in smoke. The ratios to
the incumbent are the native paired results. The rho and floor columns are
the ranges of **mean Ir / mean reference Ir** over the four development
curves; they are descriptive and have no pooled confidence interval.

| Smoke arm | Ir / incumbent | Ir / rho, per-cell range | Ir / rank floor, per-cell range | Verified | Class |
|:--|--:|--:|--:|--:|:--|
| `incumbent` (`batch_trials=64`) | 1.000× | 15.54–23.22× | 4.19–6.96 million× | 12/12 | reference |
| `batch16` | 0.621× | 10.05–14.42× | 2.23–4.84 million× | 12/12 | engineering candidate |
| `batch32` | 0.749× | 11.85–17.34× | 2.90–5.55 million× | 12/12 | engineering candidate |
| `batch128` | 1.482× | 22.49–34.78× | 6.65–9.63 million× | 12/12 | regression |
| `dense` | 0.984× | 15.34–22.85× | 4.10–6.85 million× | 12/12 | engineering candidate |
| `excess8` | 0.995× | 15.48–23.12× | 4.16–6.90 million× | 12/12 | engineering candidate |
| `window8` | 0.856× | 11.18–21.81× | 3.86–5.20 million× | 12/12 | engineering candidate |
| `rho` | 0.0535× | 1.000× | not applicable | 12/12 | reference |

Development selected `batch16`; selection retained it over `batch32`.
The final confirmation and replay each comprise 60 targets × three repeats
for `incumbent`, `batch16` and rho. Confirmation gives
`batch16/incumbent = 0.630 [0.578, 0.676]` in Ir and
`0.762 [0.740, 0.783]` in unisolated native cold wall time (paired 95% intervals).
Replay gives `0.630 [0.578, 0.676]` Ir and
`0.759 [0.736, 0.779]` wall time. Every cell passes the registered 1.10
per-cell cap. The complete cold **IC/rho** comparison is
`11.524 [10.718, 13.016]×` in Ir and `3.353 [2.865, 3.823]×` in
native wall time on confirmation. Rho parity and the stricter rho win gate
both fail. These paired intervals and the decision are quoted from
[`decision.json`](decision.json), not inferred from the rounded table.

## Curve-by-curve confirmation

The names are canonical ICV1 registry slugs, checked against each frozen
fixture's degree, curve coefficient and subgroup order by the native
[reporter](../../../../examples/ic_tournament_round_report.rs). `B` is the
verified signed factor-base size. Both IC arms use the same support on each
curve, so this round does not rank different factor bases. The `was` column
is the incumbent on these exact targets; the IC/rho and IC/incumbent cells
are the native paired ratios in the frozen decision.

| Curve | Cell | B | Was: incumbent `2^x Ir` | `batch16` `2^x Ir` | `S = Ir/√r` | IC/incumbent | IC/rho (`1.0×` parity) | Verified |
|:--|:--|--:|--:|--:|--:|--:|--:|--:|
| `icv1-f2m13-t181-515ee569` | `n13a0` | 182 | 2^24.80 | 2^23.89 | 346,476 | 0.532× | 10.99× | 36/36 per arm |
| `icv1-f2m17-tm101-00378d4e` | `n17a1` | 272 | 2^25.24 | 2^24.55 | 95,974 | 0.620× | 14.58× | 36/36 per arm |
| `icv1-f2m19-t797-b6cf2467` | `n19a0` | 304 | 2^25.46 | 2^24.86 | 83,887 | 0.658× | 10.66× | 36/36 per arm |
| `icv1-f2m19-tm797-9c54981b` | `n19a1` (holdout) | 304 | 2^25.46 | 2^24.85 | 59,029 | 0.657× | 10.97× | 36/36 per arm |
| `icv1-f2m23-t5197-69e76b73` | `n23a0` | 368 | 2^25.73 | 2^25.21 | 26,803 | 0.696× | 10.85× | 36/36 per arm |

The factor-base job policy is `subgroup_orbits`, seed 43,
`factor_base.points = 6n`. The actual signed sizes above come from the
independently checked certificates, not that input formula. Rank is 7 on
`n13a0` and 8 on the other four cells. The factor-base recipe is unchanged
between paired IC arms; whether another factor base is better was not tested.

## Phase accounting and complexity

For `icv1-f2m19-t797-b6cf2467` at confirmation, the table below shows
mean Ir per complete job, in millions. Its phase counters are exclusive and
sum to the full process cost for each receipt.

| Pipeline phase | Incumbent, M Ir | `batch16`, M Ir | Interpretation |
|:--|--:|--:|:--|
| Startup and input | 0.451 | 0.451 | Process setup |
| Curve and targets | 1.858 | 1.858 | Curve/target construction |
| Factor base and tables | 18.536 | 18.536 | Dominant fixed setup |
| Collection and decomposition | 6.742 | 1.840 | Relation search and PDP are grouped by the worker |
| Log certification | 1.093 | 1.089 | Check recovered base logs |
| Verify, filter and linear algebra | 12.757 | 3.591 | Relation checks, filtering and scalar solve are grouped |
| Individual log | 0.496 | 0.495 | Target descent/scalar recovery |
| Final verification | 0.006 | 0.006 | Verify recovered scalar |
| Reporting and cleanup | 4.138 | 2.481 | Full-process tail |
| **Total** | **46.076** | **30.347** | **0.658× on this cell** |

Reducing the batch from 64 to 16 spends less on surplus collection and
repeated rank/linear-algebra work while retaining the same 18.536 M Ir
factor-base/table setup. It changes a schedule constant, not a demonstrated
asymptotic exponent. The worker does not split PDP from collection or linear
algebra from verification/filtering, so their individual costs and slopes
remain unset. Four narrow degrees and one additional model at degree 19 are
insufficient to infer a family-wide crossover. This round does not discharge
the m=83 confidence gate or establish transfer to m=131.

## Evidence and replay

- Compact frozen files: [contract](contract.json), [calibration](calibration.json),
  [candidates](candidates.json), [fixtures](fixtures.json),
  [stage summaries](summaries/), [decision](decision.json),
  [audit](audit.json), [source manifest](source-manifest.json), and
  [per-curve measurements](measurements.json).
- Raw source, evaluator, 1,632 trial trees, profiles, certificates and
  failures are in `evidence/round-0026.tar.zst`, content-addressed in
  `s3://setec-astronomy-artifacts/store/sha256/f4/f4ff053d9f6093046b9c5cd5ee5fcce90942d02876b0a3853ab353056c4a5156`.
  The tracked [artifact manifest](../../evidence/.artifacts.json) records
  SHA-256 `f4ff053d9f6093046b9c5cd5ee5fcce90942d02876b0a3853ab353056c4a5156`
  and 149,835,581 bytes. A fresh S3 download matched both.
- Pull with `python3 tools/artifacts/artifacts.py pull
  research/ic_candidate_tournament_20260915/evidence/round-0026.tar.zst`,
  then extract with `zstd -dc evidence/round-0026.tar.zst | tar -xf -` from
  `research/ic_candidate_tournament_20260915/`. On Linux amd64 run the
  extracted `runs/round-0026/evaluator/icprog tournament verify --round
  runs/round-0026`. This path reproduces the independent audit. The current
  native `--archive` reader rejects ordinary directory headers in this tar;
  extraction plus `--round` is the tested route.

The raw archive is immutable evidence. The reporter only groups verified
receipt fields; it cannot change the native decision or certify an
unverified target.
