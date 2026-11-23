# H4 n=23 complete one-target pipeline — 2026-11-20 UTC

The cofactor-cleared 250-point base supported a complete index-calculus
path on the deterministic `n=23` curve. A frozen ordinary relation stream
supplied 180 verified rows; with one known generator row, the signed
125-column system reached rank 125. Independently checked base logs then
recovered the previously unseen target scalar **14,984**. A same-point
Floyd rho run recovered the same scalar and passed subgroup replay.

| One-target exploratory record | IC | Same-point rho |
| --- | ---: | ---: |
| Public target | `(62002,638929)` | `(62002,638929)` |
| Verified scalar modulo 232,891 | 14,984 | 14,984 |
| Online wall time, including scalar replay | 1.227 ms | 63.006 ms |
| Target-independent IC preparation | 1,019.717 ms | — |
| IC preparation plus one target | 1,020.944 ms | — |

The observed online ratio is 51.34 for this one local pair. The host is an
ordinary macOS machine without a Tier 0 or Tier 1 isolation receipt,
an A/A calibration panel, a tuned rho implementation, or five fresh
alternating pairs. These
numbers establish correctness and stage costs; they are not a promoted
speedup measurement.

The exact candidate is
`IC1N23Ch4randfb250PDP3mitmRCsampleLAgaussTDdirectISO0hee3873de904e`.
Its [immutable manifest](candidate.json) records the field, curve,
subgroup, generator, all 250 cleared points, inverse folding to 125
columns, pair-index algorithm, relation stream, modular Gaussian
elimination, target descent, and source hashes. The curve ID is
`EC1N23Ch4randh9d96eba41b3a`. The [one-target workload](workload.json)
is `Wfd6260157e20`; its full run ID is in the [result](result.json).
The target was generated from a SHA-256-derived scalar solely as fixture
data. That scalar was excluded from all 180 known-scalar relation queries.

## Charged stages

| Cold stage | Time |
| --- | ---: |
| Curve/setup and control overhead | 56.882 ms |
| Factor-base construction and clearing | 490.435 ms |
| Pair-index construction, 62,500 entries | 440.685 ms |
| Relation-query point generation | 4.583 ms |
| Relation pair lookups, including all attempts | 15.154 ms |
| Relation group checks | 2.266 ms |
| Matrix row construction | 1.719 ms |
| Incremental rank and final linear solve | 5.340 ms |
| Replay of all 125 base logs | 2.652 ms |
| **Cold preparation total** | **1,019.717 ms** |

All 180 ordinary relation queries returned a group-verified triple;
124 raised the rank beyond the known generator row. The final matrix
has 181 rows and 125 columns. Failed queries would have remained in
the trace and their work charged to the relevant stages. The frozen
query cap was 5,000 and process cap 180 seconds. Peak RSS was
271,302,656 bytes. The checked Sage runtime is in
[`sage-runtime-info.json`](sage-runtime-info.json).

The one-target online phases sum exactly to 1.227250 ms: target query
0.133583 ms, pair lookup 0.036584 ms, relation check 1.021167 ms,
descent arithmetic 0.012833 ms, and scalar replay 0.023083 ms. The
relation check includes a full group replay of the signed representative
row. Rho used one target, one worker, no cross-target table, and recorded
1,972 additions, 914 doublings, and zero restarts. Its clock includes
the recovered-scalar replay. Operation totals for both methods remain
unpriced because scalar-multiplication costs were not counted completely.

## Verification and proof map

[`verify.py`](verify.py) independently reconstructs the curve, subgroup,
base, signed quotient, all 181 relation equations, and the target
witness. Sage's finite-field matrix solver confirms rank 125 and the
same 125 logarithms; every logarithm is then replayed as a point
multiple. It checks the candidate, curve, workload, and run identity
hashes, the target exclusion, five exclusive online phases, cold-stage
sum, and both scalar certificates. The [validation receipt](validation.json)
passes. The one-row [measurement record](run.jsonl) also passes the
repository's IC measurement contract v2 validator; its output is in
[`contract-validation.txt`](contract-validation.txt).

The mathematical reduction is direct: each verified relation
`A+B+C=kG` becomes a signed linear equation in the representative
logs modulo prime `r`. Rank 125 makes those logs unique; their point
replays certify the solution. For the fresh target, the same signed
row gives a scalar `s`, checked by `[s]G=Q`. The earlier Lean
[`pair_lookup_iff`](../pair_coverage_20261120/formal/PairIndexCoverage.lean)
proves the group pair-index equivalence, and
[`additive_map_preserves_three_sum`](../subgroup_projection_20261120/formal/CofactorProjection.lean)
proves relation transport under cofactor clearing. Both are checked
without axioms. A full Lean theorem for this run remains open: it needs
the concrete binary-elliptic group and finite matrix certificates
connected to those structural lemmas. The checked finite-instance
replays above are the current evidence for that bridge.

The [protocol](protocol.json) and runner were committed in `c413f41c79d`
before measurement. Reproduce from the repository root:

```sh
/Volumes/SSD990/cryptanalysis/sage --runtime-info > research/exotic_ecdlp_20261021/H4_prime_descent/full_pipeline_20261120/sage-runtime-info.json
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/full_pipeline_20261120/run.py
/Volumes/SSD990/cryptanalysis/sage -python research/exotic_ecdlp_20261021/H4_prime_descent/full_pipeline_20261120/verify.py
```

The next measured comparison needs five distinct preregistered targets
on one Tier 1 allocation, alternating IC and a tuned same-point rho
under the same resource envelope. The 250-point base and candidate
manifest are now fixed for that panel.
