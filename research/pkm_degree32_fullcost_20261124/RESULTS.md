# PKM degree-32 cold full-rank panel — censored result

The frozen four-invocation panel is incomplete because the first interval
invocation reached its 600-second outer wall cap. The append-only store remains
immutable. The other three invocations completed checked rank 17, verified all
16 planted scalars, and retained their matched counted-rho context.

| Invocation | Exit | External wall (s) | Child CPU (s) | Total GAE | Relations / trials | F4 multiply-adds | Rank |
|:--|--:|--:|--:|--:|:--|--:|--:|
| interval 1 | 124 | 600.007 | 643.820 | — | — | — | — |
| degree-32 candidate 1 | 0 | 400.535 | 216.835 | 8,405,039,414 | 426 / 6,673 | 25,669,064,653 | 17 |
| degree-32 candidate 2 | 0 | 562.555 | 183.283 | 9,682,255,813 | 426 / 6,673 | 25,669,064,653 | 17 |
| interval 2 | 0 | 286.912 | 868.909 | 6,002,325,663 | 430 / 6,653 | 821,836,343,852 | 17 |

The two candidate invocations used identical target, relation, solver-operation
and linear-algebra counts, while their external wall times differed by 40%.
That divergence and the censored interval invocation show that this shared-host
run is not a stable timing panel. The candidate used about 32 times fewer F4
multiply-adds than the completed interval arm, but its charged total was 1.40
to 1.61 times the completed interval total. This component accounting is
descriptive because the missing mirrored interval prevents the frozen paired
gate from being evaluated.

The store is `fullrank.jsonl`, six records, SHA-256
`028f11b7b278d56a3cb18dd29fc8e352e3b5d9dde1e0c0f7c5c16a1ce0b1f8e3`.
Its terminal verdict correctly records `complete=false`,
`component_gate=false`, and `cold_rho_crossover=false`. A follow-up must use a
new protocol and store, a source-pinned executable, a controlled benchmark
host, and an outer cap that admits the already observed interval runtime.

## Source-pinned successor, 2026-11-25

The original seed 20261149 is not equal-size under the retained source-pinned
binary (16 interval versus 18 candidate abscissae). A new protocol instead
uses previously selected seed 20261276, instance
`icv1-fp13-tm137-9a89da76`, with the same source-pinned `ic` SHA-256
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`.
Fresh preflights in `source_pinned_fullrank_quiet_20261125.jsonl` passed at
18/18 abscissae. Timed full-rank and matched-rho calls are waiting for the
host's separate compilations and research benchmarks to exit. The preflight
store's current SHA-256 is
`ec96bd9f6868c84800eda3a596d5e6ad7603dee4c4c81ecdc8286d2669b259de`.
No new full-cost ratio is claimed from this pending store.
