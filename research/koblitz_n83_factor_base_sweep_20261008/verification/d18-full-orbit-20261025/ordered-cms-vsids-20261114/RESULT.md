# D18 VSIDS branch policy: positive replay and a capped natural query

On the primary N83 `a=0` curve, VSIDS-only branching with all available
CryptoMiniSat Gaussian matrices solved the exact constructed D18 positive in
2.037 seconds and 3,879 conflicts. The assignment passed the original
2,807,999 CNF clauses and 3,901 XOR rows, then independent characteristic-two
curve, subgroup, and relation-row replay. The first public natural query on
the same base and source ended `INDETERMINATE` at 1,000,001 conflicts after
428.574 seconds. It contributes one censored query and zero checked relations
at that cap. Both local measurement claims have `unverified` PCC standing:
`PCC-20261114-edf4a4` and `PCC-20261114-c3815c`. [pcc:unverified]

| Frozen item | Positive | First natural query |
| --- | ---: | ---: |
| Curve and base | `a=0`, D18, 5,418,738 subgroup points, 32,643 signed-Frobenius columns | Same |
| Public point | `(6594052263204820946437856, 9593508870534830450764009)` | `(1834913121178197242856238, 8797964190732846213981377)` |
| Original XCNF SHA-256 | `c632c152447cb09dbe7dd7cf7e29f713588c37bd56b67dfbf1fa7dac4cb15b74` | `bafed0f783ad93fcc91dcbd4c0ab45a505fc4ee3f2b0607a561e63995d0c89c3` |
| Terminal solver status | SAT, exit 10 | `INDETERMINATE`, exit 15 |
| Reported conflicts | 3,879 | 1,000,001 |
| Solver process wall | 2.037383167 s | 428.573798292 s |
| User CPU | 1.613494 s | 424.595857 s |
| Sampled peak RSS | 188,064 KiB | 475,472 KiB |
| Host one-minute load samples | 6.01, 5.33, 4.98 | 5.85, 5.28, 5.29 |
| Swapouts before / after | 164,320 / 164,320 | 164,320 / 164,320 |
| Replay | Original XCNF and group/row pass | No model returned; raw terminal-status audit pass |

The source commit for both exported formulas is
`afebef39283466fca4ddff01063d26df9b59c45e`, the exact compressed D18
object SHA-256 is
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
and the CryptoMiniSat 5.14.7 binary SHA-256 is
`a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`.
The only changed solver option from the [prior full-Gauss arm](../ordered-cms-fullgauss-20261114/RESULT.md)
is `--branchstr vsids`; both arms use one thread, seed 20261114, at most
one million conflicts or 600 solver seconds, and a 660-second outer guard.
Both natural arms are censored, so their wall times do not rank the policies.

The [protocol](PROTOCOL.md), [positive lock](EXPORT_LOCK.json), and
[natural lock](NATURAL_SOLVER_LOCK.json) were committed before the corresponding
solver executions. The [raw manifest](evidence/RAW_MANIFEST.json) pins the
compressed complete solver transcripts, commands, host samples, process
receipts, original-XCNF and group replays, terminal-status audit, and their
local raw originals under `/Volumes/SSD990-2/tmp/n83-d18-vsids-20261114`.
The two original 50 MB XCNFs and 153 MB compressed D18 object remain in the
local frozen input roots; the locks and manifest pin their hashes.

The next a=0 comparison needs a distinct encoding, base, or relation search
method with a positive gate, then checked natural relations and useful rank.
Full-width column logs, unseen-target descent, complete cold cost, and
same-point rho are needed to select an end-to-end factor base. The separate
N83 `a=1` panel remains governed by its approved Spot route and price gate.
