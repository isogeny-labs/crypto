# Round 03 validation

Raw receipts are members of `runs.tar.xz`; extract as in README.md.
No historical output was overwritten. The final ledger hashes match all
recorded sources and inputs (`record-source-hash-check.log`,
`record-raw-hash-check.log`). The assembly/check receipts are excluded from
the ledger's raw map to avoid self-references; `runs.sha256` covers them too.

| Check | Command / receipt | Actual outcome |
|---|---|---|
| Native tests | `cargo test --release --offline --locked --manifest-path ../../native/Cargo.toml`; `native-tests-final.log` | 106 passed, 0 failed, 1 ignored |
| Full repository library | `CARGO_TARGET_DIR=/home/ec2-user/crypto/target cargo test --release --lib --offline --locked`; `root-tests-01.log` | 4396 passed, 0 failed, 118 ignored |
| Frozen replay | `./REPLAY.sh`; `replay-02.log` | Eight final outputs identical; both separate checkers pass |
| Polarization verifier | `--verify-polarized`; `gl2b-verify-01.log` | 24 positive integral forms, integer determinants, kernel pairings and Frobenius matrices pass; 14 graph cases verified by exhaustive 2x2 matrix enumeration |
| Divisor/recovery verifier | `--verify-n5`; `n5d-verify-01.log` | 1851 principal rows; independent BigInt ranks 42 and 20; 64 projected classes; all 22 descents and elliptic multiplication checks pass |
| Lean kernel | `lake build` in `../../lean`; `lean-build-05.log` | Original module and all 13 new named lemmas build |
| Transitive axiom audit | `lake env lean --run ../../../tools/lean/AuditAxioms.lean KoblitzRound03`; `lean-audit-02.log` | 14 declarations audited including the defining equation; 0 failures, 0 native trusts; standard foundations only |
| Identity certificates | `--record-round03`; `record-01.log` | Three integer-lift identities issued with 64 sample points plus 128 extra points each; exact Lean scope separately recorded |
| Archive | `sha256sum -c`, deterministic tar/xz repack | Complete member manifest and byte-identical repack checked |

The final replay includes the independent conic-restriction assertions added
after the first DP3 run; the JSON bytes remain identical. The initial N5
compile failure, insufficient relation matrices, first Lean path error,
and failed proof elaborations remain archived. N5c's polynomial-count
metadata error is explicitly corrected in the ledger and report without
rewriting its output.

The PDF is regenerated from REPORT.md and PROOF_MAP.md with its diagram;
all rendered pages are visually inspected before commit. CI now replays
rounds 01--03 and builds/audits the pinned Lean targets. Remote CI completion
is reported separately from local validation. No complete polarization
geometry or census classification is claimed as a Lean theorem.
