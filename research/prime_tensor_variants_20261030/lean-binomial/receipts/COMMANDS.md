# Commands and outcomes

Commands ran in `/workspace/audit/levelwise-binomial-lean` with per-command
`PATH=/workspace/audit/toolchains/lean-4.35.0-linux/bin:$PATH`.

| Command | Actual receipt/outcome |
| --- | --- |
| `MATHLIB_NO_CACHE_ON_UPDATE=1 lake update` | `lake-update-01.log`, exit 0 |
| `lake build`, attempts 1–3 | Exit 1; exact sources `LevelwiseBinomial.build-01.lean.txt` through `-03.lean.txt`; full logs `build-01.log` through `-03.log` |
| `lake build`, attempt 4 | Exit 0; general signed digit identity; `build-04.log`, exact source snapshot `LevelwiseBinomial.build-04.lean.txt` |
| `lake build`, attempt 5 | Exit 0; final nine-theorem module; `build-05.log` |
| `lake build +LevelwiseBinomial` | `explicit-module-build.log`, exit 0 |
| `lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean LevelwiseBinomial` | `axiom-audit-01.log`, exit 0, 9 audited, 0 failures/native-trusted |
| `lake env leanchecker --verbose LevelwiseBinomial` | `module-checker-01.log`, exit 0 |
| `lake env leanchecker --fresh --verbose LevelwiseBinomial` | `fresh-checker-01.log`, exit 0 |
| `bash .lake/packages/CryptoLean/tools/proof-gate.sh . --out receipts/shared-gate` | `shared-gate-driver.log` and `shared-gate/`, exit 0, PASS, 9 theorems and 0 trusted |

Attempts 1–3 failed while normalizing casts between natural and integer
division and matching notation `/` against the literal `Int.ediv` head.
The corrected proof supplies an explicit checked bridge with the exact head
symbol. These failures do not contradict the mathematical parity identity.

Independent natural and signed probes passed; source snapshots/logs are
retained in `receipts/nat-parity-probe/` as `.lean.txt` where applicable.
A separate local Git index lists only the four project files for the
unmodified shared gate's source discovery. Neither frozen ternary package
nor the parent repository/index was modified.
