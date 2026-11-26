# M31 graph witness source-ID correctness protocol

Status: preregistered development regression, before execution. PR #5118; no merge or paid compute.
Base: ceba74bb64854027c4dd16f00428bf676383bfdb.
Production witness blob: c6caf51dbcea0436294d662ac48c70df151b7202.

## Objective and scope

Correct an ordering mismatch when input-position-sorted witness terms are mapped to arbitrary unique original source IDs. The mathematical combination and exact replay checks must remain unchanged. This is relation-algebra correctness work; it does not collect new curve relations, time a solver or qualify the eleven-query M31 recovery.

All new scalar regression arithmetic uses ell=2147446831, the existing M31 curve's subgroup modulus; p=2147483647 remains the coordinate-field modulus and is not substituted for ell. The synthetic coefficient fixture is not asserted to be a group-generated relation stream. Historical tests retain their recorded fields.

## Sources and ownership

Read current AGENTS, ACTIVE, newest dated session log, PR #5076 handoff and source. Prior audit: https://github.com/isogeny-labs/crypto/pull/5076#issuecomment-7119361207.
Inspect production witness materialization, original replay/sparse merging, LpIndex construction and existing integration tests. Separate source reviewer confirmed the ordering mismatch on 81f0308c, whose witness blob is identical to this base. Agent review is not signed validation.

Research-KB MCP is unavailable; scripts/research-kb status exited 127 with no executable found. Conductor is unavailable. Reserve witness.rs, additive integration-test cases and this report/evidence directory in ACTIVE/draft #5118. Existing #5076 converter, branch synchronization and eleven-query replay remain with their owners. No existing workflow dispatch or collector will be duplicated.

## Frozen regression fixture

Use three PartialRelation rows:
- ID u64::MAX: small [(0,1)], large [(101,1)], rhs 1.
- ID 0: small [(0,2)], large [(102,2)], rhs 2.
- ID 42: small [(0,1)], large [(101,3),(102,4)], rhs 3.

Expected rank 2 and one LP-cancelling witness. Expected canonical source certificate:
[(0,ell-2),(42,1),(u64::MAX,ell-3)].
Expected small [(0,ell-6)] and rhs ell-4.
These are pre-execution exact-algebra expectations, not measured results. [pcc:unverified]

## Required checks and falsification

1. Add separate eager and lazy positive tests. Run both against the unchanged production witness module and preserve their expected failures. An unexpected pass requires investigating the premise before applying the fix.
2. Map complete (ID,coefficient) pairs and sort by ID before constructing CompleteRelation. Keep uniqueness, alignment, LP cancellation, replay equality and caps.
3. Run both positive tests against the candidate; assert the exact certificate and small/RHS values above, plus independent replay through the existing kernel.
4. Check all six input-row permutations over ell. Do not require identical normalized witnesses; check rank and the unique relation up to nonzero scaling, canonical original IDs, exact small/RHS and replay.
5. Rebuild the index and witness batch from duplicate-ID inputs before asserting refusal, isolating the uniqueness guard. Preserve mismatched IDs, insufficient source/small caps and tampered-coefficient refusal.
6. Run the existing original-module graph Rust integration suite on the final candidate revision. Existing historical fixtures remain regression evidence only.

## Execution envelope and records

Local development builds/unit tests only, as allowed by AGENTS section 9a. One build job and one test thread, offline dependencies where available, separate local target directory, no paid compute, benchmark or new curve collection. Stop on failed source identity, unavailable native dependencies, unexpected regression, or 5-minute build/test timeout; retain any failed attempt. Do not fix unrelated root-crate modules.

Before execution retain source/input hashes, Rust/Cargo versions, command, original tests and new test identities. Retain pre-fix failures, candidate output and exit status with separate filenames. Record source-bound standalone-module scope; do not imply full root-crate or independently signed PCC success. Existing source-exact harness is reused without modifying production modules other than the candidate fix.

## Acceptance and next step

Accept the correctness change only when the unchanged code exhibits the intended false refusal, the candidate passes exact positives and guards, and the existing graph suite has no new failure. Report additional source-order sorting cost explicitly. Handoff to the owning graph-comparison continuation; subsequent grounded-BFS/eleven-query tests require their own admission and remain unexecuted by this task.
