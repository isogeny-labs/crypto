# Terminal-less witness review

Reviewed against source2b2b8bf on2026-10-30. Applied after exact-source2b2b8bf validation passed; this increment awaits compilation.

- groups() assigns Rows.first only after independently checking a matched row; cursor order and count checks still run before the new branch.
- witness() checks exact domain indices/summands, target and sum, recomputes group additions and exceptional positions, verifies original equations where coordinates exist, and checks candidate/lift/independent provenance against the retained first row.
- verify_files() binds canonical witness length/hash to the original capture payload before constructing CapturedWitness. The new branch additionally requires matching bytes/hash and a nonzero sequence. It does not invent a missing terminal publication receipt or producer lifecycle.
- A unit native report with nonexceptional matching coordinates remains inconsistent and is rejected, matching the existing terminal branch.
- Full terminal handling is unchanged. Missing earlier input/setup/native/events or matching first group row still prevents mathematical replay; retained bytes remain available for the future raw-prefix disposition.
- Parent integration must distinguish independently replayed witness facts from producer-local lifecycle fields. It must not interpret first_witness_verified as proof that a missing producer terminal existed.

Next validation: actual both-twist runtime with terminal removed, missing witness/group rejection, promoted completion/negative rejection, then central attempt/manifest and censoring controls. The review preceded source2b2b8bf validation completion; the applied replay increment requires its own exact-source compilation and checks.
