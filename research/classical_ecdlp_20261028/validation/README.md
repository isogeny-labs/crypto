# Native development checks

These receipts concern the frozen public archive, not a new performance run.
They are not a `VALIDATION.json` bundle and do not admit benchmark or leaderboard
rows. The strongest protocol-matched rho reference and historical environment
evidence required by that gate are absent.

| Receipt | Meaning |
| --- | --- |
| [native-replay.json](native-replay.json) | Rust replay: 60 known-answer fixtures, 120 recorded arm answers, six complete witnesses, six independently recomputed group orders, 84 small APS S3/sign triples, and two finite candidate-count populations. Includes the SHA-256 of the original 60-case JSON. |
| [native-solve.json](native-solve.json) | The existing W12 engine solves the six public trial-zero targets with a fresh fixed seed schedule, and the wrapper checks each answer outside the solver. No timing output is recorded. |
| [historical-summary.json](historical-summary.json) | Rust recomputation of the six historical medians, minima, maxima and ratios from the original raw JSON. Units are milliseconds; ten samples per curve. These remain unqualified historical Python diagnostics. |
| [development-checks.json](development-checks.json) | Toolchain, source/base identities and actual development-check outcomes for this import. |

Reproduce the three JSON receipts from the repository root:

```sh
cargo run --quiet --release --locked --example classical_ecdlp_archive -- check > research/classical_ecdlp_20261028/validation/native-replay.json
cargo run --quiet --release --locked --example classical_ecdlp_archive -- summary > research/classical_ecdlp_20261028/validation/historical-summary.json
cargo run --quiet --release --locked --example classical_ecdlp_archive -- solve > research/classical_ecdlp_20261028/validation/native-solve.json
```

These invocations are development correctness checks and analysis of already
recorded data. They are not a timing comparison or a reproduction of the
historical Python RNG trajectory. The solver receives public instance data;
the wrapper uses the known scalar only to check the result.

The example test target runs seven archive checks and the seven inherited
W12 engine tests. Its corruption tests reject a changed kernel entry, changed
point label and duplicate selected minor column. The original engine, Cargo
configuration and Cargo lockfile are unchanged by the import.

The full library invocation completed with 4,968 passed, 26 failed and 122
ignored. All 26 failures were missing sparse-checkout inputs, and all 26
affected tests pass in exact focused reruns after restoring their original
tracked files. Thus all 4,994 non-ignored library tests pass across those
checks; the original full invocation itself failed, and no clean full-suite
invocation is claimed. No assertion or ignore list was changed.

Those full-library counts belong to `tested_base_revision` in the development
receipt. Later base updates and their archive regression checks are recorded
under `landing_followups`; they do not turn the earlier run into a current-head
full-suite pass.

The PDF was compiled with Typst 0.15.0 and both pages visually inspected.
Its final render was compared byte for byte with the inspected page renders.
Matplotlib 3.10.8 reads frozen counts only; `reports/render_visuals.py` performs
document layout, not field arithmetic, verification or result computation.
