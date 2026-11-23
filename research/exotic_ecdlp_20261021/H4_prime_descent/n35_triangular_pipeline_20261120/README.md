# Complete n35 triangular-index candidate

The triangular pair-index builder completed the full n35 index-calculus
pipeline on a new public target. It recovered and independently replayed
scalar 250,797,008 for point `(17961865824, 9816811964)`. The same-point
installed Sage rho run recovered the same scalar. The candidate reached rank
2,048 after 6,361 verified relation queries and checked all 2,048 factor
logs. This ordinary local-host run measured 94.928 seconds of empty-cache
preparation, 14.693 ms target-online IC, and 554.927 ms target-online rho.

## Identity and outcome

| Field | Recorded value |
| --- | --- |
| Curve | `EC1N35Ch4randha95af81414ea` |
| Candidate | `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h40e80a647b9b` |
| Workload | `W030ac34fa3fe`; one new SHA-256-derived public point |
| Run | `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0h40e80a647b9bW030ac34fa3feR1` |
| Base and folded columns | 4,096 distinct usable points; 2,048 signed columns |
| Index | 8,390,656 triangular pair visits; 8,321,091 distinct sums |
| Relation collection | 6,361 verified queries; 2,047 novel rows plus one anchor; rank 2,048 |
| Scalar | 250,797,008, verified independently for both arms |
| Target-online IC / rho | 14.693 ms / 554.927 ms; observed ratio 37.77 |
| Host | ordinary macOS local run; exploratory timing |

The [protocol](protocol.json), [runner](run.py), checked
[Sage runtime](sage-runtime-info.json), [candidate](candidate.json),
[workload](workload.json), [result](result.json), and
[relation trace](relation-trace.gzip.b64.txt) preserve the exact computation.
The protocol and runner were pushed before measurement. The trace retains all
query attempts, witnesses, rank gains, and per-query costs, with hashes of
both compressed and uncompressed bytes in the result record.

## Charged costs

| Exclusive interval | This run |
| --- | ---: |
| Pair-index precomputation | 66.569 s |
| Base construction | 9.048 s |
| Relation collection PDP | 4.794 s |
| Final relation linear algebra | 13.116 s |
| Other cold phases | 1.401 s |
| **Empty-cache preparation** | **94.928 s** |
| Target query and PDP | 0.838 ms |
| Target relation check | 13.713 ms |
| Target descent and recovery check | 0.141 ms |
| **One-target IC online** | **14.693 ms** |
| **Same-point rho online** | **554.927 ms** |

The five online phases sum exactly to the target-online interval, and the
cold phases sum exactly to empty-cache preparation. The known-scalar fixture
was generated outside both online intervals. The signed target relation,
log recovery, and original-point scalar replay are charged to the IC online
interval; rho's scalar replay is charged to rho's interval.

For context, the prior ordered-index run on the same curve and base measured
136.752 seconds for index precomputation and 167.669 seconds for empty-cache
preparation. This run retained the identical relation-matrix and factor-log
hashes while measuring 66.569 and 94.928 seconds respectively. These are
two local observations on different target fixtures, so the setup difference
is descriptive. The separately frozen [paired component experiment](../n35_triangular_pair_20261120/README.md)
compared the builders in alternating order and found exact index equality.

## Independent checks and next gate

The [verifier](verify.py) rebuilt the triangular index in column-major order
with an explicit minimum-payload update, replayed the seeded relation stream
and rank trajectory, independently solved the matrix, checked every factor
log, and replayed the IC and rho target scalars. Its
[validation receipt](validation.json) passed; the [IC v2 run row](run.jsonl)
passed the [measurement contract analysis](contract-report.json). The
component's [Lean ordering proof](../n35_triangular_pair_20261120/formal/TriangularPair.lean)
and the existing packed-key proof cover the structural index argument; the
verifier checks the concrete Sage encoding and complete finite run.

The next performance gate is a calibrated Tier 0 or Tier 1, alternating-order
same-point panel with topology, affinity, throttling, steal-time, and noise
receipts. This run establishes correctness and local stage costs for the new
candidate; the isolated-host aggregate remains unmeasured.
