# Current-root interface recovery 119

The actual library check now reports 29 diagnostics, compared with 109 before this recovery. The focused harness passes 75 existing tests: SAT/packed-codec plus actual ExactScalar, canonical JSON and SHA256 modules. These are source compatibility results, not full library or speedup qualification. [pcc:unverified]

Workload recovery combines current source, EFD metadata ancestor 7b56b2fc38 and wide source f57e537f51; BigInstance, exact order metadata, EFD representations and later counted operations remain. Claw uses complete f57 source. U192 implements the current checked narrowing contract; the existing Wiedemann two-word preconditioner refuses wider moduli instead of truncating. BigClaw retains full-width arithmetic. Coordinate Wiedemann is recovered from caae5eca9f. Other repairs export existing hashed_native, remove identical duplicate declarations, retain narrow-only orbit constraints and wrap legacy report scalars losslessly.

SOURCE_CHECKS_119.json pins touched source and successful test logs. SCALAR_FIXTURE_SOURCES_117.json pins the three historical test inputs absent from the current tree. `root-source-tests/hydrate-fixtures.sh` restores only those exact Git blobs and refuses to overwrite differing files; the harness links to original repository fixture paths. Earlier compile and fixture failures110/113/115 remain. The original tests and assertions are retained.

ROOT_BUILD_BLOCKERS_114.json and the complete stderr retain the failed actual-root check. Remaining interfaces concern dataset admission/export, factor-base configuration verification, binary chain encoding/domain APIs, and snapshot/configuration fields. A naive line merge of dataset branches duplicates moved functions; item-level review offf02728b58 and d4f4dc3799 against 45c84d06ef is still pending and has not changed runtime source. No module, algorithm, test or proof obligation was disabled.

The packed adapter remains unapplied; its 36-case actual-root matrix is unrun. Frozen comparison 96 remains unchanged: primary ratios 0.9960–1.0489, inside A/A spread. The 15x objective remains active. No new timing was collected. [pcc:unverified]

Index erratum 120: ROOT_BUILD_BLOCKERS_114.json used a greedy multiline parser and folded the diagnostics into one index entry. Its raw log and headline 29 were correct. ROOT_BUILD_BLOCKERS_120.json contains all 29 separate diagnostic entries; both files and the original raw bytes remain. [pcc:unverified]
