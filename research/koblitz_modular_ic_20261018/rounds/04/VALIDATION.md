# Round 04 validation

Raw receipts are in `runs.tar.xz`. Source and input hashes match the sealed
ledger (`record-source-hash-check.log`, `record-raw-hash-check.log`). Assembly
receipts are excluded from that map to avoid circular hashes and are covered
by the complete archive member manifest. No historical output was overwritten.

| Check | Receipt / command | Outcome |
|---|---|---|
| Native tests | `native-tests-02.log`; study `cargo test --release --offline --locked` | 106 passed, 0 failed, 1 ignored |
| Root library | `root-tests-01.log`; `CARGO_TARGET_DIR=/home/ec2-user/crypto/target cargo test --release --lib --offline --locked` | 4396 passed, 0 failed, 118 ignored |
| Exact replay | `replay-02.log`; `./REPLAY.sh` | Seven outputs identical; separate checker passes |
| Quotient certificates | `--verify-round04`, recorded in replay | 14 principal integral forms/Frobenius presentations; BigInt determinants, cofactor bounds and separate exhaustive short-vector counts pass |
| Maintained-library arithmetic | Same separate checker | 1250 five-torsion kernel additions; 24 eleven-basis/Hecke controls; 1088 projected point conditions pass |
| Modular relations | Same separate checker | 544 closed orbits; 102 principal rows reconstructed; all point sums pass; BigInt rank 4 |
| Lean kernel | `lean-build-03.log`; `lake build` | All modules and 11 new named lemmas build |
| Transitive axiom audit | `lean-audit-01.log`; pinned `AuditAxioms.lean KoblitzRound04` | 11 audited, 0 failures, 0 native trusts; standard foundations only |
| Identity certificates | `record-01.log`; `--record-round04` | Three identities pass 64 issuer plus128 extra points; exact Lean scope separately bound |
| Dashboard / site | `node tools/render_ic_dashboard.mjs --check`; `python3 scripts/site/test_build.py` | Dashboard pins/status/hash checks and all 49 site tests pass |
| Archive | Full sha256 manifest, fresh extraction and deterministic repack | Byte checks pass |
| Report | Layout-only build with pinned requirements | Every rendered PDF page inspected; source and full proof map included |

Preserved unsuccessful receipts: Q1's residue-as-integer lift, M1's degree-one
field edge case, M3's first projective-point count, the first two Lean builds,
the ledger builder's first borrow-check failure, and the first replay's
historical metadata mismatch. M4's rank 4 deficit and M5's rejected incomplete
zero divisor are retained as collector outcomes. Supported and unsupported
M3 inputs are all included, not filtered to successful images alone.

Remote CI is a separate execution. The workflow now replays all four rounds
and builds/audits the pinned Lean modules within the existing25-minute cap.
There is no complete formal scheme/Picard model, reverse subgroup transfer,
individual descent, scalar recovery or speed comparison in this round.
