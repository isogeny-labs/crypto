# H4 n=35 complete packed-index candidate

The n=35 candidate reached rank 2,048 on 2,048 signed columns after 6,361 recorded ordinary queries, solved and replayed all representative factor logs, and recovered scalar 109,055,891 for public point `(20906086470, 29095154304)`. Installed Sage rho independently recovered the same scalar from that exact point. The frozen protocol and runner were pushed in commit `0d6a098a445` before measurement.

| One-target exploratory local run | Value |
| --- | ---: |
| Candidate | `IC1N35Ch4randfb4096PDP3mitmRCsampleLAgaussTDdescentISO0hf491f3bcd7b6` |
| Workload | `Wfa9ae6a483b9` |
| Field/subgroup order | `2^35` / `520,604,191` |
| Actual base / signed columns | 4,096 / 2,048 |
| Pair attempts / distinct indexed sums | 16,777,216 / 8,321,091 |
| Verified relation queries / novel rows | 6,361 / 2,047 |
| Final matrix rows / rank | 6,362 / 2,048 |
| Cold preparation, empty caches | 167.669 s |
| Pair-index construction | 136.752 s |
| IC target-online interval | 14.195 ms |
| Same-point rho target-online interval | 2124.351 ms |
| Rho / IC online ratio, one local observation | 149.66 |
| Peak process RSS | 1,775,796,224 bytes |

The online clock began with target-dependent work after the reusable index and factor logs were ready; it includes the first 37 third-point checks, group-law relation verification, log reconstruction, and scalar replay. The target was recovered at shift zero. The cold clock includes curve and base setup, all pair-index construction, the 6,361 relation queries, incremental rank work, and verification of all factor logs. Fixture construction is recorded outside both clocks. This one-process macOS run is an exploratory measurement under an ordinary host envelope; a calibrated Tier 0 or Tier 1 alternating-order same-point panel is the next performance test.

The [protocol](protocol.json), [runner](run.py), [checked Sage runtime](sage-runtime-info.json), [candidate manifest](candidate.json), [workload](workload.json), [result](result.json), and compressed [relation trace](relation-trace.json.gz) preserve the complete workload and outcomes. The trace holds all 6,361 query records, including per-query witness, scan count, rank gain, and cost. [verify.py](verify.py) independently rebuilds the curve and pair table in column-major order with the lexicographic first-pair rule; it replays the seeded query sequence and rank trajectory, solves the relation matrix, verifies all logs, and checks the original target and rho scalar. Its [validation receipt](validation.json) and [IC v2 run row](run.jsonl) give machine-readable checks.

The [proof map](formal/PROOF_MAP.md) links the checked 35-bit key, pair-lookup, cofactor-projection, and shifted-target theorems with their exact hypotheses and verification status. The finite curve order, base size, relation rank, and measured costs are replayed by the Sage audit.
