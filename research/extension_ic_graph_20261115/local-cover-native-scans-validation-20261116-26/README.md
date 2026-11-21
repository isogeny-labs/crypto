# Local native scan/scalar validation26

The once-only cohort on published source
`d30b6e4a81c48e4c4a7302f0a1b535bb956ac70b` finishes in73.538seconds,
exit0,2starts/2completions. Both actualscalars197564087855217396 pass native
and outer group replay. Pivotgraph509-to-382rows is accepted with382witness
replays and zero fallback. Foreign-thread/other-modulus hooks are zero.
Original copied Rust analyzer reports globalproblems[] and passes all
endpoint/detail/required-counter checks. [pcc:unverified]

| Exercised counter | NativeIC | PivotgraphIC |
|---|---:|---:|
| Descent residuals | 3008 | 3008 |
| Descent modr additions | 6016 | 6016 |
| Solver word events | 16843827 | 30794405 |
| Solver RNG words, existing scope | 509 | 382 |
| Solver rows | 509 | 382 |
| Solver nonzeros | 4577 | 12722 |

Both required markers occur and are supported by counters: solver word work
exceeds RNG, and the positive descent stream records exactly two scalar
additions per residual. The qualifier also checks exact integer repricing,
phase/component sums, nonnegative LA remainder, matrix shapes and call flow.
These are producer checks, without independent signed evidence.
[pcc:unverified]

The shared u64 solver counts data scans/comparisons; scalar labels retain
their values and streams while using the modular-add hook. Existing
arithmetic and legacyops are unchanged. Pure allocation, copying and loop
overhead are outside this patch. RNG rejection/internal work and other
native/container coverage, plus strongest-rho pricing, remain open. Complete
totals stay null and complete/followup/comparative flags remain false.
[pcc:unverified]

Amendment21: fresh p1009seed277, nativeIC then Pivotgraph,1validation round,
zero warmups, identical incidence/shared recipe, one claimed Rayon thread,
CPU0,8GiB,900seconds each/1900whole/no retry. No rho arm, A/A, ratio, interval
or performance verdict. No comparison of costs across prior trials is made.
[pcc:unverified]

Exact raw59502bytes/fullsummary40074bytes, progress/runtime/fullflow and
pre/post hashes are copied here. Original driver/analyzer/source archive and
host information remain in
`/workspace/ic-local-runs/local-cover-native-scans-validation-20261116-26`.
All101source/prepost hashes pass. Final one-page PDF visually inspected;
exact PDF/SVG/150dpiPNG browsing copies/hashes are in
`benchmarks/20261116-cover-native-scans-validation`, originals also local.
No admitted-ranking, canonical encrypted-schema or execution-proof assertion.
[pcc:unverified]

Next: source review for exact RNG draw/rejection coverage, remaining native
containers and complete strongest-reference pricing. The native counters
validated here are a completed component; the complete comparison remains
unresolved. [pcc:unverified]
