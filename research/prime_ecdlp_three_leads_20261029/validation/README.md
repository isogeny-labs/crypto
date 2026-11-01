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

The final new-example test command completed with exit 0: four Hecke-control
checks and four PKM checks passed. The latter includes a positive m=4 relation
and an exhaustive two-sum decision control over the fixture's nonzero targets.
The fixtures are E/F17: y^2=x^3+2x+2, subgroup order 19, generator (5,1).
PKM uses (t,g,zeta)=(2,1,4) and (3,3,9); these are finite development examples,
not a depth/scaling study or a complete ECDLP benchmark.

## Full-library validation limit

The first full attempt encountered six missing sparse-runtime-input failures
and was interrupted for restoration. The tracked Certicom emulator directory
and the frozen version-one bounds session were materialized unchanged; the
focused checks then passed6/6 and1/1. A bounds rerun launched before hydration
finished is also preserved as `bounds-fixture-premature-rerun.log`.

The restarted full run was capped at40minutes. Unique completion lines record
5,232 passed, zero failed and123 ignored; one unchanged existing case remained:
`cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
It was stopped with SIGTERM, and Cargo returned101. This is **incomplete
broader validation, not a full-suite pass**. No test or solver was modified,
skipped, disabled or quarantined. `full-library-cap.json` and `receipt.json`
record the outcome. Committed excerpts are explicitly partial. Complete raw
outputs exceed the artifact-store threshold and are NOT_UPLOADED because no
store credential is available; their byte counts, hashes and local paths are
in `unuploaded-artifacts.json`. No extension change or compression exception
is used to bypass that policy. This limit says nothing about the new
leads' cryptanalytic performance.

Small command transcripts preserve the captured output bytes as their prefix.
A clearly marked final footer, where present, records that prefix's byte count
and SHA256; it is transcript metadata, not native-program output.
