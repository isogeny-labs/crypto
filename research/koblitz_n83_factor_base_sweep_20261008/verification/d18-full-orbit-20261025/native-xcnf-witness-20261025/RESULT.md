# Full-D18 constructed XCNF witness: passing control

The source-bound local control at commit `d3a09099cb1f16c39981dce95786b2a857be38e4`
returned a native SAT assignment for the exact frozen positive XCNF. The
independent audit checked all 2,807,752 original CNF clauses and 3,901 XOR
rows, then replayed the four summands, group sum, signed-Frobenius row and
rank-one insertion over the original field. It rejected a flipped assignment,
altered target and altered XCNF digest. The relation uses point indices
`[0, 166, 332, 498]` and four nonzero orbit columns.

| Measurement | Observed value |
| --- | ---: |
| Full process wall, worker | 71.562 s |
| Independent supervisor wall | 72.765 s |
| Native search | 2.493 s, 0 conflicts |
| Maximum resident set size | 4,745,379,840 bytes |
| Native variables | 329,726 |
| Exported XCNF bytes | 50,518,392 |
| XCNF SHA-256 | `0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c` |
| Packed model SHA-256 | `7aa34fd2bafa7ac11a2f6384f4436b1d74f16942661351b17ef7bbfe474b1853` |
| Packed model bytes | 41,216 |
| Compressed XCNF SHA-256 | `fd637df7c9cda21bdf29a7b0d871b932d09e384bbd82876f46c756d1c74c1a9b` |
| Compressed XCNF bytes | 4,616,126 |

The export hash is identical to the prior
[`local-xcnf-20261025`](../local-xcnf-20261025/RESULT.md) positive formula,
whose external CryptoMiniSat attempt reached its 1,210-second independent
cap without a terminal answer. This control establishes satisfiability of
that exact formula and shows a native witness can be checked against both
representations. The runs use different solvers and hosts; the observations
do not establish a solver speed ratio for natural relation search.

The [worker result](result.json), [phase stream](result.json.phases.jsonl),
[supervisor](supervisor.json), [original independent audit](independent-audit.json),
[portable replay](portable-replay.json), [command](command.json),
[stdout](stdout.txt), [stderr and resource usage](stderr-time.txt), and
[provider GET metadata](object-get-metadata.json) are preserved here. Their
hashes are in [SHA256SUMS](SHA256SUMS). The raw 50.5 MB XCNF and 41 KB model
remain in local scratch under
`/Volumes/SSD990/llm/tmp/n83-d18-native-xcnf-witness-run-20261025/witness/`.
The 4.4 MB compressed XCNF remains there too. Automatic approval review
rejected its S3 upload because the earlier authorization covered a different
source archive, so this result does not claim an S3 copy of the XCNF.
The [replay procedure](AWS_REPLAY.md) regenerates the payloads from the
frozen source and the immutable S3 base.

This is one constructed, known-answer control. Natural relation yield, useful
rank growth, factor-base logarithms, unseen-target descent, full cold
one-target accounting and matched same-target rho remain open. The selected
best total-runtime base remains null.
