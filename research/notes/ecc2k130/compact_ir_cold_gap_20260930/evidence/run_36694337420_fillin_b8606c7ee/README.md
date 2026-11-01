# Cold-CPU fill-in archive, kept verbatim

These 12 files are byte for byte the contents of `evidence/run_36694337420/`
at commit `b8606c7ee` ("Archive the one cold-CPU fill-in for the four pending
compact cells"). They are the other generation of the archive that a union
merge (`6cf567da1`, for #1054) fused with `d373295dc`'s into invalid JSON.

Nothing here was edited. They moved from `evidence/run_36694337420/` to this
directory only because both generations used the same path and cannot share it.

| | this directory | `../run_36694337420/` |
|:--|:--|:--|
| source commit | `b8606c7ee` | `d373295dc` ("Archive matched cold CPU gap panel and no-crossover decision") |
| `verify_cold.py` named in its records | SHA-256 `02ddbe38…` (the file at `b8606c7ee`) | SHA-256 `2c74ab8a…` (the file now in the tree) |
| `MANIFEST.json` | 1,804 bytes, schema `ecc2k130-compact-ir-cold-gap-archive-v1` | 64 KB, per-case `member_sha256` tables |
| includes `n37_L1_smoke` | yes | no |

`analyze_cold.py` and the `compact-ir-cold-gap` workflow read only
`../run_36694337420/`, whose analysis passes and matches `ANALYSIS.json`.
Nothing reads this directory, and `verify_cold.py` at `d373295dc` has not been
run against it, so it is preserved evidence, not checked evidence.
