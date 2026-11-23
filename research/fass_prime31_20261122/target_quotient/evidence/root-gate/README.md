# Root repository gate scope

This is a local setup record, separate from the completed standalone target-quotient qualifier and its immutable-source build. No whole-library success is inferred. [pcc:unverified]

- Root `cargo metadata --offline --no-deps --format-version 1` returned zero. Its raw output, command, exit status and before/after Cargo file hashes are retained here.
- `check-attempt1/` ran bounded `cargo check --lib --locked --offline` and returned 101 before compilation because the sparse checkout omitted `crates/certified-relations`.
- The tracked `crates/` subtree was materialized without changing its content. `check-attempt2/` returned 101 before compilation because `tools/factor-base` was still outside the sparse checkout.

Both failed attempts remain intact. They are checkout setup failures, not evidence of a defect in upstream source or in the new qualifier. There is no root-library compile or test result. The frozen protocol's local retry envelope was not expanded; no third root check was launched. A subsequent full-library qualification needs the remaining tracked path dependencies and a separately declared build envelope. This limitation does not change the standalone qualifier inputs, which include their exact published dependencies in the retained source snapshots.

The original mathematical qualifier and fixed-degree certificate each built and ran on their first attempt. Their outcomes are described in the main report and separate producer/reviewer receipts; they are not executions of the root Cargo target. [pcc:unverified]
