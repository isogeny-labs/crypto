# Complete n35 triangular-index and sparse-target candidate

The combined candidate completed the full n35 pipeline on a new public
target. It reached rank 2,048 from 6,361 verified relation queries, checked
all 2,048 factor logs, and recovered scalar 225,434,797 for point
`(30429161600, 15541934848)`. Installed Sage rho recovered and independently
replayed the same scalar on that point. In this ordinary local-host run,
empty-cache preparation took 100.541 s, target-online IC took 0.737 ms,
and same-point rho took 1,885.250 ms.

## Frozen identity and result

| Field | Value |
| --- | --- |
| Curve | `EC1N35Ch4randha95af81414ea` |
| Candidate | `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h21421ad7a3b3` |
| Workload | `W6ecbaecbfe6f`; one new SHA-256-derived public point |
| Run | `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h21421ad7a3b3W6ecbaecbfe6fR1` |
| Factor base | 4,096 usable points; 2,048 signed columns |
| Pair index | 8,390,656 triangular visits; 8,321,091 distinct sums |
| Relation matrix | 6,361 verified queries; 2,047 novel rows plus anchor; rank 2,048 |
| Target certificate | Scalar 225,434,797 for `(30429161600, 15541934848)` |
| One-target online | IC 0.737 ms; same-point rho 1,885.250 ms; observed ratio 2,556.85 |
| Host boundary | Ordinary macOS local run; exploratory timing |

The [protocol](protocol.json) and [runner](run.py) were pushed before
measurement. The exact [candidate](candidate.json), [workload](workload.json),
[result](result.json), checked [Sage runtime](sage-runtime-info.json), and
[relation trace](relation-trace.gzip.b64.txt) retain the identities, inputs,
all attempts, and costs. The trace has hashes of both its compressed and
uncompressed bytes in the result record.

## Phase accounting

| Exclusive interval | This run |
| --- | ---: |
| Pair-index precomputation | 68.797 s |
| Factor-base construction | 12.338 s |
| Relation collection PDP | 4.812 s |
| Relation linear algebra | 12.962 s |
| Other cold phases | 1.632 s |
| **Empty-cache preparation** | **100.541 s** |
| Target query and PDP | 0.421 ms |
| Sparse target relation check | 0.198 ms |
| Target descent and recovery check | 0.118 ms |
| **One-target IC online** | **0.737 ms** |
| **Same-point rho online** | **1,885.250 ms** |

The cold phases and the five target-online phases sum to their recorded
totals. The known-scalar fixture is outside both online intervals. The IC
online interval includes the third-point scan, signed-column relation check,
log recovery, and original-point scalar replay. Rho includes scalar replay.
This is one local observation; the online ratio is descriptive until a
calibrated Tier 0 or Tier 1 alternating-order panel passes its noise gates.

The same base, relation seed, and factor-log procedure as the preceding
[triangular-index candidate](../n35_triangular_pipeline_20261120/README.md)
produced identical relation-matrix and factor-log hashes. Its separate target
had a 14.693 ms dense-check IC interval; the earlier
[sparse-check panel](../n35_sparse_check_20261120/README.md) paired dense and
sparse checks on the same ten points. The complete combined run here uses a
new point and charges its own cold setup and rho solve.

## Verification and proof map

The [independent verifier](verify.py) rebuilt the triangular index by a
different traversal, replayed the 6,361-query relation stream and rank
trajectory, solved the matrix, checked every factor log, reconstructed the
three signed target columns, and replayed both target scalars. Its
[validation receipt](validation.json) passed, and the
[IC v2 measurement analyzer](contract-report.json) accepted [run.jsonl](run.jsonl).

The [triangular ordering Lean proof](../n35_triangular_pair_20261120/formal/TriangularPair.lean)
and the [sparse-check proof map](../n35_sparse_check_20261120/formal/PROOF_MAP.md)
cover the structural implications. The finite-instance bridge, including the
exact Sage point encoding and all scalar certificates, is checked by the
independent verifier. The next gate is an isolated-host single-target panel
with matched resource and calibration receipts.
