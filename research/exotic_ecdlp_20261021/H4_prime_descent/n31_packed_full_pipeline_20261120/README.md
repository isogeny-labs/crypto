# H4 n=31 complete direct-integer-index candidate

The complete n=31 pipeline with the direct-integer pair index recovered scalar 27,775,558 for the exact public point `(1818443913, 706820564)` used by the earlier point-key candidate. It reached rank 1,045 on 1,045 signed columns after the same 2,737 ordinary queries, solved and replayed all 1,045 representative factor logs, and verified both IC and installed Sage rho scalar certificates on the original point. The candidate has a distinct immutable ID because its pair-index representation and source changed.

| Same-target local run | Point-key candidate | Direct-integer-key candidate |
| --- | ---: | ---: |
| Candidate digest suffix | `5b630846fb43` | `65526ac19689` |
| Actual base / signed columns | 2,090 / 1,045 | 2,090 / 1,045 |
| Verified / exhausted relation queries | 2,737 / 0 | 2,737 / 0 |
| Matrix rows / final rank | 2,738 / 1,045 | 2,738 / 1,045 |
| Empty-cache cold preparation | 39.592 s | 41.206 s |
| Pair-index construction | 33.423 s | 35.012 s |
| Peak process RSS | 1,556,496,384 bytes | 634,617,856 bytes |
| IC target-online interval | 7.410 ms | 7.198 ms |
| Same-point rho target-online interval | 148.829 ms | 152.672 ms |

The complete direct-integer run used **59.2% less peak process RSS** than the point-key run. Its measured cold preparation was 1.615 s longer, primarily from the index build. The online intervals are single exploratory observations on an ordinary local host; the 0.211 ms IC difference is not a calibrated performance conclusion. Both runs used the same workload ID `W55a408d4617a`, target point, generator, base, relation query scalars, witness indices, rank trajectory, solved logs, and first direct target witness. The two rho intervals are separate same-point runs. A calibrated alternating-order Tier 0 or Tier 1 panel is needed before promoting wall-time differences.

The [protocol](protocol.json) and [runner](run.py) were committed before measurement in `acac86ab1e5`. The [checked Sage runtime](sage-runtime-info.json), [candidate manifest](candidate.json), [workload](workload.json), and complete [result](result.json) preserve the mathematical identity, source hashes, failures, phase costs, target witness, and scalar certificates. The result JSON is compacted after execution without dropping trace rows, remaining below the artifact-migration threshold. [verify.py](verify.py) independently rebuilds the original curve and base, uses a point-only pair-sum set, checks all 2,737 relation outcomes and rank gains against both the new trace and the point-key reference, solves the matrix again, replays every log, and verifies both target scalars. Its [validation receipt](validation.json) passed, and the [IC v2 run row](run.jsonl) passed the measurement contract, recorded in [contract-validation.txt](contract-validation.txt).

The index key's bit-packing injectivity and identity-sentinel separation are proved in the [Lean packet](../n31_packed_index_fast_20261120/formal/PROOF_MAP.md). The prior pair-lookup, cofactor-projection, and shifted-target theorems cover the pipeline's structural steps under their stated hypotheses. Exact instance counts and timings remain audited computations. The next test is a same-point, alternating-order paired panel under a Tier 0 or Tier 1 host envelope with both candidate variants and rho, charging each variant's target path and retaining cold setup separately.
