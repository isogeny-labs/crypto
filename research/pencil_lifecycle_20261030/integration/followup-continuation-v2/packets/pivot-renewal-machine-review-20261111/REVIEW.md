# Independent machine review — pivot renewal

The retained receipts are internally consistent with a failed bounded run. Pre-custody passed, `EliminationField` compiled, and `PivotSwap` stopped at `PivotSwap.lean:59:2` with the equal-row branch of `swap_involutive` unsolved. Post-custody passed. `Controls`, `Readback`, and `ExpectedFailure` were not attempted.

## Observed sequence

| Stage | Result | Observed outer time |
|---|---:|---:|
| Pre-custody | passed | 45.61097849800717 s |
| Compiler | failed | 42.47305457800394 s |
| Post-custody | passed | 40.05902259401046 s |
| Overall | failed | 128.24891154497163 s |

The command receipt contains exactly two Lean invocations. Their exits are `EliminationField=0` and `PivotSwap=1`; both child groups were reaped with no remaining members. All three phase owners are terminal, and no outer wait timed out. Only `EliminationField.olean` exists.

Both custody scans checked 16,736 dependency files totaling 3,111,892,318 bytes. Their complete maps are byte-identical at SHA-256 `bd79b0566f9d19dbf1bdecbc1e94cd5dd119bffcf3f0bb639b9d921ff41ed2ad`. The receipt itself notes that matching shared-cache snapshots do not exclude transient concurrent changes between scans.

## Binding note

The bound repository release is SHA-256 `b3acbb8650a1f370579e067fa89874d6348278a63517adb9cb1ee54aae8eecff`, and the admission receipt binds that digest. The outer coordinator copy is a canonicalized JSON serialization with SHA-256 `336d1b10ee96615d5efd6ceb97a84296de9d488691eeec22325ca0a7587305e1`; its parsed value equals the bound release, but its bytes do not. The first audit attempt incorrectly required byte equality and is preserved in `audit-attempt1.log`.

## Review boundary

This review recomputed hashes and checked retained receipts and source bytes. It did not rerun Lean or independently prove the statements. The supported component result is the successful `EliminationField` compilation. The full five-module replay did not complete.
