# Graph-IC M31 retained-capture continuation [pcc:unverified]

## What was done
User fixed the field to M31. Read current AGENTS (unchanged blob bc5217fe), ACTIVE, newest dated journal, current PR #5076 and concurrent PR #5097. Found an existing 645-row public-query B677 capture on the exact group-admission curve at source 577d6c2c. Source/input were retrieved and hashed. No producer run was repeated. Preregistered bounded native conversion/group replay and graph comparison at prefixes 256/512/613.

Implemented `prime31_b677_capture.rs`, using the unchanged original Arithmetic and B677/B509 builders. It preserves query coefficients, target orientation, source row ordinals and hybrid point indices; replays every query and all 645 equations before retaining the 613 rows with at most two LP terms; and records all 32 exclusions. Producer rank, scalar-recovery and timing fields are ignored. Added `replay_m31_capture.sh` and one bounded stage in the existing native workflow, with the same artifact destination. The script preserves input/source/toolchain/binary/lock identities, runs the original admission/evaluator on fixed prefixes, and requires exit 1 with empty stdout for three corruptions of the same M31 input. Shell syntax, static workflow invariants and whitespace checks passed. Native execution is pending.

## Decisions made
Consume the immutable ten-query panel. Preserve its generic exhaustive-pair/known-answer labels and incomplete custody/cost boundaries. Reuse original Rust arithmetic and fixture construction; add no dependencies, new collector, tiny-field workload or performance job. Keep PR #5076 open under no-merge/no-paid-compute. Use the existing CI job for retained-data analysis, with a 300-second stage, 4 GiB address-space limit and one build thread.

## Open threads
Execute the saved-data native replay in the existing CI job. Record actual ranks and refusals, source/code/output identities and invalid-input exits. The full algebraic extractor and complete cold-cost advantage remain separate open goals.

## Errors
Local Rust/Cargo and Conductor are unavailable. Fetching main obtained e1552f1e but returned a separate missing-submodule error; no source working tree changed. Used exact GitHub source reads and the fetched commit for current instructions/journal. Native validation is pending; no pass inferred from source review.
