# N83 a=0 K2048 all-summand ordering gate

The existing five-summand chain now exports an exact all-summand-ordered
K2048 XCNF. Its constructed-point control returned a complete model that
independent replay checked against every original formula row, all 339,968
factor-base archive rows, the curve group law and the signed-Frobenius
relation row. Removing every witness-coordinate pin preserves that model:
independent evaluation passed all 1,141,545 CNF clauses and 4,980 XOR rows
of the unpinned formula. The frozen unpinned search reached
`INDETERMINATE` at 1,000,001 conflicts in 283.605 seconds. The natural
fixture arm was gated off by the frozen protocol. Claim
`PCC-20261114-0e5133` has unverified standing. [pcc:unverified]

The curve is `icv1-f2m83-tm6151469093347-debefd74`, with subgroup order
`2417851639230796216685689` over
`F₂[z]/(z⁸³+z⁴⁵+z²+z+1)`. The retained K2048 base contains 339,968
usable points in 2,048 signed-Frobenius columns. Five first representatives,
sorted by unsigned x as columns `[1,3,4,0,2]`, sum to the constructed public
point `(985527933806849610456454,7623568253424067907890851)`. Four
lexicographic x comparisons add 328 variables and 988 clauses to the
corresponding unsorted circuit. The pattern-0 chain has a checked witness;
the natural arm tests that same identity pattern only when admitted.

| Gate | Recorded result |
| --- | --- |
| Export source commits | Pinned `be9a37d2baf1d388101bd8b9aaa10a13d36e5496`; unpinned `3fb352e82246bf7625efcb3320b495150b6ad652` |
| Exporter binary SHA-256 | `847a189547b14a6c8425a2b7bfea9004ff3e00696d7f705db0e96da955a1971b` in both arms |
| Pinned formula | 111,806 variables; 1,142,209 CNF; 4,980 XOR; 747 x-coordinate pins; SHA-256 `579a3719fd54ae600632db3ef2a5e0bd458ff5f6c6b3d4524a2eed6d3e4e3d08` |
| Pinned control | SAT, exit 10, 0.357 s process wall; independent original-XCNF, group and row replay `PASS` |
| Unpinned formula | 111,806 variables; 1,141,545 CNF; 4,980 XOR; zero witness pins; SHA-256 `ed19aa5d9fb013e09244608ff6e75417d1e830bc12000b51e70b2e532ca8b101` |
| Known model on unpinned formula | `PASS_ORIGINAL_XCNF` on all 1,146,525 rows |
| Solver envelope | CryptoMiniSat 5.14.7, SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`; one Darwin arm64 thread, seed 20261114, VSIDS, 32 Gaussian matrices, max 25,000 columns, one-million-conflict and 600 s internal limits, 660 s watchdog |
| Unpinned host preflight | Load 6.08, 5.47, 5.74; swapouts 164,320 unchanged through terminal; scratch gate passed |
| Unpinned terminal | `INDETERMINATE`, exit 15, 1,000,001 conflicts, no model; 283.605 s wall, 280.034 s user and 2.043 s system CPU; 929,169,408 byte wait4 peak RSS |
| Independent raw audit | `PASS_CUSTODY_AND_TERMINAL_CLASSIFICATION` on copied compressed originals, transcripts, commands, receipts and host records |

The [frozen protocol](PROTOCOL.md), [solver lock](LOCK.json),
[known-model auditor](audit_known_model.py), [runner](run_locked.py),
[raw auditor](audit_raw.py), and [versioned evidence](evidence/) make this
gate reproducible. The raw audit is a producer-side checksum and terminal
classification, not a third-party attestation. Original uncompressed XCNFs
and transcripts remain at
`/Volumes/SSD990-2/tmp/n83-a0-k2048-sorted-20261114/raw/`.

The unpinned search is a censored solver-stage result: no relation or novel
rank was produced within the cap, so the all-summand ordering change does
not select an end-to-end factor base. The earlier matched unsorted unpinned
control also reached its conflict cap in 153.815 seconds; both rows are
censored, and their wall times are search diagnostics rather than times to a
checked relation. The next a=0 step needs a producer that yields checked
ordinary relations and rank on the common public corpus. The approved a=1
Spot run remains price-gated at `$0.27` per instance-hour; complete
same-target IC/rho measurements for both curves remain the selection gate.
