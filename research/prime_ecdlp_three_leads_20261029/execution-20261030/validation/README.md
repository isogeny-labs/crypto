# Development validation receipts

The final native suite passes12tests, including complete m=4 supported sets on
all35finite points of two fixed F17 development curves. All8legacy regressions,
strict lint, formatting, ordinary CLI build, gate-refusal control and fresh native
backlog validation pass. These checks are software correctness evidence, not a
scientific panel or cost comparison.

The initial proof-gate/ directory and11-declaration log retain the version
at6970ab05. The affine-proof-gate/ directory and13-declaration log validate the
stronger final module, including actual Mathlib affine points. Both fresh final
module build and official kernel replay pass; replay is against imported
modules, not --fresh or an external prover. Only standard Lean foundations are
used and there are zero trusted exceptions. The failed scratch source is stored
as plaintext, clearly separate from checked .lean modules.

full-library-status.json records a capped incomplete5553-case serial check:
2173passed,0failed,47ignored,one case active at the cap and3332not reached.
No complete library pass is claimed. The entire raw output is NOT_UPLOADED;
unuploaded-artifacts.json identifies its exact local path, size and SHA256.
Only a bounded excerpt is committed. No text was compressed or split to evade
artifact policy. Initial setup failures and the exact captured-byte hashes are
retained. A footer, where used, leaves original captured bytes as its prefix.

receipt.json binds final source/report and binary hashes, command outcomes,
proof scope and execution gaps. Historical receipt directories outside this
versioned development work remain unchanged. G1–G6 scientific acceptance,
complete cost/rank/external qualification and the central cost theorem remain open.
