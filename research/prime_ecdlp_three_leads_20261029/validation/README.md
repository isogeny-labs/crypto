# Software and proof validation

These receipts validate development code and stated Lean implications. They are
not scientific experiment records, a performance comparison, or an admission
certificate for any ECDLP result.

- `lake-build.log`: final eight named theorems, with each transitive axiom list.
- `proof-gate.log` and `proof-gate-detail/`: repository audit and explicit source
  build. Nine declarations include the generated `rowEval.eq_1`; eight are named
  mathematical theorems. Zero custom/trusted exceptions were accepted.
- `initial-standalone-check.log`: preserved earlier check, before the coverage
  theorem was added. It is not the final proof inventory.
- A separate official Lean `leanchecker ThreeLeads` replay completed with exit
  zero and empty stdout. Its invocation and source hash are in the final JSON
  receipt. `lean4checker --fresh` was not installed and is not claimed.
- Checkpoint push and gate logs preserve exit-one transport refusal after a
  completed resume note was supplied. No scientific run was launched.
- `artifact-access-denied.xml`: unchanged S3 AccessDenied response to the public
  GET of the P20 object named in `../five-sum-inputs.json`; HTTP status was 403.
  It proves neither absence of the object nor a failed replay.

Native compilation/tests and formatting/lint outcomes are added only after the
commands finish. Raw command logs retain both failures and successful reruns.
No frozen historical session or audit is overwritten.
