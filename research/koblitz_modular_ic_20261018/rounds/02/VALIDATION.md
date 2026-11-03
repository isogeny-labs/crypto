# Round 02 validation

All records below are members of `runs.tar.xz`; extract as in README.md.
No frozen output was rewritten. The finalized ledger input hashes and
source hashes passed `ledger-hash-replay.log` and `source-hash-replay.log`.
`runs.sha256` independently covers every archived record, including failures.

| Check | Command / receipt | Actual outcome |
|---|---|---|
| Native research tests | `cargo test --release --offline --locked --manifest-path ../../native/Cargo.toml`; `native-tests-final2.log` | 39 passed, 0 failed |
| Full repository library | `CARGO_TARGET_DIR=/home/ec2-user/crypto/target cargo test --release --lib --offline --locked`; `root-tests-01.log` | 4396 passed, 0 failed, 118 ignored |
| Frozen output replay | `./REPLAY.sh`; `replay-02.log` | Thirteen files identical; transfer and relation checkers pass |
| Transfer certificates | `--verify-klein`; `kt1-verify-01.log` and `replay-02.log` | 393 principal divisors, 56 polynomial automorphism checks; 42 nonzero /14 zero maps |
| Relation certificates | `--verify-klein-relations`; `mr2b-verify-final.log` | 12036 geometric rows reconstructed; separate BigInt rank64; 65 class certificates pass; no interpolation solver |
| Pinned Lean kernel | `lake build` in `../../lean`; `lean-build-05.log` | 13 theorem declarations build |
| Transitive axiom audit | `lake env lean --run ../../../tools/lean/AuditAxioms.lean KoblitzRelations`; `lean-audit-final.log` | 13 audited, 0 failures, 0 native trusts; only standard foundations |
| Repository proof gate before last lemma | `tools/lean/proof-gate.sh .../lean`; `lean-gate-01.log` and `lean-gate/` | Earlier 12-theorem module passes; final 13th lemma has the subsequent kernel build and same auditor |
| PDF | `python build_report.py`, layout only | Five pages, all inspected; diagram, source links and full proof-map appendix included |

The first native build and first Lean build failed; later final builds passed.
The first replay failed only on historical KT0 v2/v3 metadata; the revised
replay compares matching v3 output. The first ledger draft included hashes of
still-open log files; it is preserved as `ledger-draft-v1.json` and is not the
canonical ledger. The final ledger excludes its own open assembly receipt.
The initial cache setup rejected an obsolete path after downloading Mathlib;
subsequent cache use and builds succeeded. See REPORT.md for scope limits.

CI now runs the round replay and the complete pinned Lean proof gate, in
addition to the original frozen round-01 replay. A successful local check is
not represented as a completed remote CI run.
