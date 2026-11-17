# Batched S3 index construction: exact keys, no local stage gain

The optional batched builder produced byte-identical N83 K512 compact states
and indexed root slots. All four runs had 10,879,232 states, 21,756,398
indexed roots, 84,992 usable factor-base points, and 512 orbit columns. The
full state BLAKE3 digest was
`73ce87c21a6bb20472daafff1a1c72a4cebb78c53d63beada5c7d76f635e2687`;
the full root-table digest was
`67dac08b8514076544f8ca53e2306ba39118c90f1ce6a3edc308a1fff2e75a70`
in every run. The first-witness unit test passed on the deterministic N83
fixture.

| Fixed order | Builder | Index build (ms) | Setup (ms) | Exit |
| --- | --- | ---: | ---: | ---: |
| 1 | Scalar | 52,105.641 | 53,149.556 | 0 |
| 2 | Batch | 52,474.837 | 53,120.614 | 0 |
| 3 | Batch | 68,153.459 | 70,080.517 | 0 |
| 4 | Scalar | 59,379.933 | 61,613.663 | 0 |

The paired scalar/batch index ratios were 0.993 and 0.871. The observed mean
batch time was 60,314.148 ms versus 55,742.787 ms for scalar. This local
host had a load average of 96.96 at the start and is an exploratory stage
diagnostic. The result does not promote batching to a pipeline comparison;
`candidate_id` remains `null`. The [native analysis](local_pilot1/analysis.json)
checks all four identities, counts, digests, timings, and exit statuses. The
[raw receipts](local_pilot1/) include the exact binary SHA-256
`c836e62e0251265b6640308a93885405d749adfb5b8e23202cbbcbc95faa3be5`,
source and base hashes, process resource logs, and timestamps.

An instrumented scalar K512 run reached its 180-second cap under heavier host
load; its [capped receipt](profile_receipts/k512_capped/) remains recorded. The
same instrumented source completed at K128: root solving took 1,747.260 ms
and compact insertion 2,528.455 ms of a 4,397.421 ms index build. Its
[receipt](profile_receipts/k128_complete/) and [source patch](profile.patch)
pin that diagnostic. This smaller profile suggests insertion merits the next
controlled change; its phase proportions are not a K512 measurement.

The high-K candidate manifests enable `KIC_SPARSE_CANONICAL=1`, whereas this
first pilot used the retained K512 control's non-sparse selector. The next
paired pilot should use the high-K flag with the same base, digest gates, and
alternating order. Retain batching as opt-in until that result, then use fresh
Tier 1 same-point IC/rho pairs for any complete-pipeline promotion.
