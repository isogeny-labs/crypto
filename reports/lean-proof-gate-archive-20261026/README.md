# Archived Newton source in the Lean gate

The repository-wide gate discovers every tracked standalone Lean file. That
includes `research/isogeny_world_frontier_20261016/replay-optimization-v1/implementation-01/Newton.failed.lean`, retained by the study as its first
failed attempt. Before this change the gate treated that archive as a positive
proof target, so a complete run could not pass even if the corrected source
and all other proofs were valid.

The archived source has SHA-256
`b439d1edbe04798a56012688254ea7bd120494e4fd10856f4bdc0c07796e9d1c`.
The original `implementation-01/lean.log` records the unknown constant
`Int.mul_two`; `implementation-02/lean.log` records the corrected source's
axiom output. A fresh pinned Lean 4.34.1 compile of the archive independently
reproduced the unknown-constant error. The local gate now requires exactly that
source hash, exit status 1, and the single recorded diagnostic. Its corrected
companion remains a positive build and axiom-audit target. A changed archive
requires an explicit review and updated expected evidence.

Validation on the 2026-10-26 checkout:

- `bash -n tools/lean/proof-gate.sh` passed.
- The targeted gate returned `PASS_expected_failure`; its preserved
  [build log](archive-negative-build.log) and [status](archive-negative-status.txt)
  are in this directory. The build log SHA-256 is
  `e27367b76f9615f54b146acbd130b980f5596e30f7836de45bee91950ec86bda`.
- A streamed test copy with a deliberately wrong expected source hash returned
  `FAIL_archive_digest` and exit status 1.
- The pinned `tools/lean/selftest/run.sh` passed all seven fixtures: two
  positive builds and five distinct expected failures.

This is a gate classification correction, not a new cryptanalytic result or
proof. It changes no benchmark or dashboard metric. A complete native proof
gate run and hosted CI are still needed before calling the whole corpus green.

Prior work checked: the study's `README.md`, its two implementation logs and
sources, and [Lean's language reference on unknown identifiers](https://lean-lang.org/doc/reference/latest/Error-Explanations/About___--unknownIdentifier/).
The reference identifies this as a name-resolution error; it does not confer
proof validity on the failed file.
