# Recorded commands

Commands ran in `/workspace/audit/next-ternary-digits` with the per-command
prefix `PATH=/workspace/audit/toolchains/lean-4.35.0-linux/bin:$PATH`.

| Command | Receipt / outcome |
| --- | --- |
| `MATHLIB_NO_CACHE_ON_UPDATE=1 lake update` | `lake-update-01.log`, exit 0 |
| `lake build`, initial three-theorem module | `build-01.log`, exit 0; exact source snapshot `TernaryDigits.build-01.lean.txt` |
| `lake build`, final eight-theorem module | `build-02.log`, exit 0 |
| `lake build +TernaryPacket +TernaryDigits` | `explicit-module-build.log`, exit 0 |
| `lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean TernaryPacket TernaryDigits` | `axiom-audit-01.log`, exit 0; 28 audited, 0 failing, 0 native-trusted |
| `lake env leanchecker --verbose TernaryDigits` | `module-checker-01.log`, exit 0 |
| `lake env leanchecker --fresh --verbose TernaryDigits` | `fresh-checker-01.log`, exit 0 |
| `bash .lake/packages/CryptoLean/tools/proof-gate.sh . --out receipts/shared-gate` | `shared-gate-driver.log`, per-target receipts `shared-gate/`, exit 0, PASS, 28 theorems and 0 trusted |

Final exact outcomes are recorded in `VERIFICATION.json`. A separate local
Git index tracks only the five core project files so the unmodified shared
gate can discover the two real source modules. The parent repository and the
frozen earlier package's source/receipts were not edited.

The independent valuation probe's first build failed due to the argument
order of a library lemma; its second build and two-theorem axiom audit passed.
The exact failed and successful sources and logs are retained in
`receipts/valuation-probe/`, and the corrected proof is included in
`TernaryDigits.lean`.
