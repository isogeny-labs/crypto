# Local projection/solver accounting diagnostic25

The once-only cohort on published source
`2c5e02b34cb2d56dc392cd3b1f57d706d0d7b24a` completes in59.361seconds,
exit0, with2starts/2completions. Both actualscalars102559623889752301 pass
native and outer group replay. Pivotgraph projects525rows to394 and replays
394witnesses with zero fallback. Both foreign-thread and other-modulus hook
counts are zero. Original copied Rust analyzer reports no global problems
and passes endpoint/detail checks; complete totals remain null and
comparative eligibility/followup stay false. [pcc:unverified]

| Observed quantity | NativeIC | PivotgraphIC |
|---|---:|---:|
| Solver rows | 525 | 394 |
| Solver matrix nonzeros | 4721 | 12982 |
| Original solve bitop subtotal | 364321367580 | 0 |
| Projected solve bitop subtotal | 0 | 666381953262 |
| Filter bitop subtotal | 0 | 806228620 |
| Witness replay bitop subtotal | 0 | 869597544 |
| Other observed LA bitops | 1039744 | 1039744 |

The denser projected system has the larger observed solver cost. Filtering
and certificate reconstruction are small relative to the recorded solver
subtotals on this fixture. This locates the observed work; it does not
establish complete cost, physical gate counts or a performance ranking.
Mapping/output snapshots see zero hooks because their bookkeeping remains
uncounted; their complete costs are unknown. [pcc:unverified]

Every component is recomputed from primitive counts in Rust. The analyzer
checks stage/attempt/solve/refusal order, matrix dimensions/nonzeros, positive
filter/replay work, aggregate graph counters, phase sums, exact prices and
nonnegative primitive remainder within LA. Detailed records contain both
successful and refused paths by contract; no refusal occurs in this cohort.
These are producer checks, without independent signed evidence.
[pcc:unverified]

Amendment20: fresh p1009seed271, one nativeIC then Pivotgraph public-input
worker, one validation round/zero warmups, identical incidence/shared recipe,
one claimed Rayon thread, CPU0,8GiB,900seconds each/1900whole/no retry.
No rho arm, A/A, ratio, interval or comparative timing inference. Earlier
24/23/22results and all acceptance gates remain unchanged. [pcc:unverified]

Exact raw52524bytes and original summary39727bytes, progress/runtime/flow and
pre/post hashes are copied here. Original copied driver/analyzer, source
archive and host metadata remain at
`/workspace/ic-local-runs/local-cover-subcounts-validation-20261116-25`.
All96source and pre/post hashes pass. The final single PDF page was visually
inspected; exact PDF/SVG/150dpiPNG copies/hashes are in
`benchmarks/20261116-cover-subcounts-diagnostic`, originals also local.
[pcc:unverified]

Remaining work: graph/native/scalar/RNG operation coverage and strongest-rho
pricing. Future filtering changes should control nonzero growth and measured
solve work; this observation does not choose a new algorithm or protocol by
itself. The primary literature's adapted two-large-prime finder is a separate
source pointer requiring its own review/selection and native integration.
[pcc:unverified]
