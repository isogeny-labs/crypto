# Commands and outcomes

All project-relative commands ran in `/workspace/audit/next-ternary-lean`.
The parent repository and its index/branch were untouched. A separate local
Git index in this audit directory contains only the four Lean project files,
solely so the unmodified shared gate can enumerate the project.

`PATH=/workspace/audit/toolchains/lean-4.35.0-linux/bin:$PATH` was set only for
each command requiring the installed toolchain.

| Operation | Exact command or source | Outcome / receipt |
| --- | --- | --- |
| Official installation documentation | `curl -fLsS https://lean-lang.org/install/manual/` | Saved as `official-manual.html` |
| Shared pin | `gh api repos/aburan28/crypto-lean/commits/main --jq .sha`, then checkout exact commit `148f5dc389b341401e06d76abe7783c4ccf42f2e` | Clone/checkout retained; lake manifest pins exact commit |
| Toolchain release metadata | `gh api repos/leanprover/lean4/releases/tags/v4.35.0` selecting the Linux `.tar.zst` asset | `official-toolchain-asset.json` |
| Official binary archive | `curl -fL --retry 2 https://github.com/leanprover/lean4/releases/download/v4.35.0/lean-4.35.0-linux.tar.zst -o /workspace/audit/toolchains/lean-downloads/lean-4.35.0-linux.tar.zst` | Succeeded; SHA256 matches asset digest; `toolchain-download.log`, `toolchain-sha256.txt` |
| Binary extraction | `tar --zstd -xf /workspace/audit/toolchains/lean-downloads/lean-4.35.0-linux.tar.zst -C /workspace/audit/toolchains` | Succeeded; `toolchain-extract.log` |
| First dependency update | `lake update` | Automatic full Mathlib cache download intentionally terminated; `lake-update-01.log` |
| Dependency update without automatic full cache | `MATHLIB_NO_CACHE_ON_UPDATE=1 lake update` | Exit 0; `lake-update-02.log` |
| Targeted dependency cache | `lake exe cache get Mathlib.Algebra.Group.Basic Mathlib.Data.ZMod.Basic Mathlib.Tactic.Abel Mathlib.Tactic.FinCases Mathlib.Tactic.NormNum` | Exit 0; `cache-targeted-01.log` |
| Build attempts 1–4 | `lake build` | Exit 1; exact sources `TernaryPacket.build-01.lean.txt` through `-04.lean.txt`; logs `build-01.log` through `-04.log` |
| Build attempt 5 | `lake build` | Exit 0; initial 15-theorem success, `build-05.log`, source snapshot `TernaryPacket.build-05.lean.txt` |
| Final build attempt 6 | `lake build` | Exit 0; final 20 reported theorems, `build-06.log` |
| Explicit source module | `lake build +TernaryPacket` | Exit 0; `explicit-module-build.log` |
| Transitive axiom audit | `lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean TernaryPacket` | Exit 0; 20 checked, 0 failing, 0 native-trusted; `axiom-audit-01.log` |
| Module kernel replay | `lake env leanchecker --verbose TernaryPacket` | Exit 0; `module-checker-01.log` |
| Fresh checker | `lake env leanchecker --fresh --verbose TernaryPacket` | Exit 0; `fresh-checker-01.log` |
| Literal shared gate | `bash crypto-lean/tools/proof-gate.sh . --out receipts/shared-gate` | Exit 0, PASS, 20 theorems, 0 trusted; `shared-gate-driver.log` and `shared-gate/` |

The failed raw logs remain intact in the audit workspace. Deterministic
`gzip -n -c` copies are supplied for repository packaging. The raw SHA256
manifest verifies their decompressed bytes. Failed source snapshots use the
`.lean.txt` extension so they do not become unintended proof-gate targets.
